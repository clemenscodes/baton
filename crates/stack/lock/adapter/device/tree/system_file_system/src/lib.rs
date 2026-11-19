mod configuration;
mod error;

pub use configuration::TreeConfiguration;
pub use error::Error;

use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::device::tree::DeviceTree as DeviceTreePort;
use stack_lock_core_device_address::DeviceAddress;
use stack_lock_core_device_file::{DeviceFile, DeviceKind};
use stack_lock_core_graphics_user::GraphicsUser;
use stack_process_core_invocation::{Completion, Invocation, Spawner, Stream};
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::MetadataExt;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceTree<SpawnerType> {
    tree_configuration: TreeConfiguration,
    spawner: SpawnerType,
}

impl<SpawnerType> Layer for DeviceTree<SpawnerType> {
    type Layer = InfrastructureLayer;
}

impl<SpawnerType> Adapter for DeviceTree<SpawnerType> {}

impl<SpawnerType> Port for DeviceTree<SpawnerType> {}

impl<SpawnerType> DrivenPort for DeviceTree<SpawnerType> {}

impl<SpawnerType> From<(TreeConfiguration, SpawnerType)> for DeviceTree<SpawnerType> {
    fn from((tree_configuration, spawner): (TreeConfiguration, SpawnerType)) -> Self {
        Self {
            tree_configuration,
            spawner,
        }
    }
}

impl<SpawnerType: Spawner> DeviceTreePort for DeviceTree<SpawnerType> {
    type Error = Error;

    fn try_read_device_files(
        &self,
        device_address: &DeviceAddress,
    ) -> Result<Vec<DeviceFile>, Error> {
        let mut device_files = Vec::new();
        for function_text in self.try_read_slot_functions(device_address)? {
            let function_path = self.compose_device_path(&function_text);
            Self::scan_device_files(&function_path, &mut device_files);
            let information_path = self
                .tree_configuration
                .system_directory()
                .join(Self::NVIDIA_DIRECTORY)
                .join(&function_text)
                .join(Self::INFORMATION_NAME);
            let information_text = fs::read_to_string(information_path).unwrap_or_default();
            let nvidia_files = information_text.lines().filter_map(|line| {
                line.strip_prefix(Self::MINOR_LABEL)
                    .and_then(|minor_text| minor_text.trim().parse::<u32>().ok())
                    .map(|minor| {
                        DeviceFile::from((DeviceKind::CharacterDevice, Self::NVIDIA_MAJOR, minor))
                    })
            });
            device_files.extend(nvidia_files);
            if self.maybe_driver_name(&function_text).as_deref() == Some(Self::VFIO_DRIVER) {
                let group_path = self.compose_group_file(&function_text);
                device_files.extend(
                    fs::metadata(group_path)
                        .ok()
                        .map(|metadata| Self::compose_device_file(metadata.rdev())),
                );
            }
        }
        device_files.sort();
        device_files.dedup();
        Ok(device_files)
    }

    fn try_read_console_users(
        &self,
        device_address: &DeviceAddress,
    ) -> Result<Vec<GraphicsUser>, Error> {
        let function_texts = self.try_read_slot_functions(device_address)?;
        let mut console_users = Vec::new();
        let mut framebuffer_texts = Vec::new();
        for function_text in &function_texts {
            let graphics_path = self
                .compose_device_path(function_text)
                .join(Self::GRAPHICS_NAME);
            framebuffer_texts.extend(Self::read_names(&graphics_path).into_iter().filter_map(
                |graphics_name| {
                    graphics_name
                        .strip_prefix(Self::FRAMEBUFFER_PREFIX)
                        .map(String::from)
                },
            ));
        }
        if !framebuffer_texts.is_empty() && self.try_check_console_map(&framebuffer_texts)? {
            let vtconsole_path = self.compose_system_path(Self::VTCONSOLE_DIRECTORY);
            for vtconsole_name in Self::read_names(&vtconsole_path) {
                let console_path = vtconsole_path.join(&vtconsole_name);
                let name_text = Self::read_text(&console_path.join(Self::NAME_FILE));
                let bind_text = Self::read_text(&console_path.join(Self::BIND_FILE));
                let maybe_console_index = vtconsole_name
                    .strip_prefix(Self::VTCONSOLE_PREFIX)
                    .and_then(|index_text| index_text.parse::<u32>().ok());
                if let Some(console_index) = maybe_console_index
                    && name_text.contains(Self::FRAMEBUFFER_CONSOLE)
                    && bind_text == Self::BIND_PRESENCE
                {
                    console_users.push(GraphicsUser::ConsoleUser { console_index });
                }
            }
        }
        let boot_presence = function_texts.iter().any(|function_text| {
            let boot_path = self
                .compose_device_path(function_text)
                .join(Self::BOOT_NAME);
            Self::read_text(&boot_path) == Self::BIND_PRESENCE
        });
        if boot_presence {
            for driver_text in Self::FIRMWARE_DRIVERS {
                let driver_path = self
                    .compose_system_path(Self::PLATFORM_DIRECTORY)
                    .join(driver_text);
                for device_text in Self::read_names(&driver_path) {
                    if device_text.starts_with(driver_text) {
                        let driver_text = String::from(*driver_text);
                        console_users.push(GraphicsUser::FramebufferUser {
                            device_text,
                            driver_text,
                        });
                    }
                }
            }
        }
        Ok(console_users)
    }

    fn try_unbind_console_user(&self, console_user: &GraphicsUser) -> Result<(), Error> {
        match console_user {
            GraphicsUser::ConsoleUser { console_index } => {
                let bind_path = self.compose_console_bind(*console_index);
                self.try_write_within_deadline(&bind_path, Self::BIND_ABSENCE)
            }
            GraphicsUser::FramebufferUser {
                device_text,
                driver_text,
            } => {
                let unbind_path = self
                    .compose_system_path(Self::PLATFORM_DIRECTORY)
                    .join(driver_text)
                    .join(Self::UNBIND_FILE);
                self.try_write_within_deadline(&unbind_path, device_text)
            }
            GraphicsUser::ProcessUser { .. } => Ok(()),
        }
    }

    fn try_bind_console_user(&self, console_user: &GraphicsUser) -> Result<(), Error> {
        match console_user {
            GraphicsUser::ConsoleUser { console_index } => {
                let bind_path = self.compose_console_bind(*console_index);
                self.try_write_within_deadline(&bind_path, Self::BIND_PRESENCE)
            }
            GraphicsUser::FramebufferUser {
                device_text,
                driver_text,
            } => {
                let bind_path = self
                    .compose_system_path(Self::PLATFORM_DIRECTORY)
                    .join(driver_text)
                    .join(Self::BIND_FILE);
                self.try_write_within_deadline(&bind_path, device_text)
            }
            GraphicsUser::ProcessUser { .. } => Ok(()),
        }
    }

    fn try_bind_passthrough(&self, device_address: &DeviceAddress) -> Result<bool, Error> {
        let address_text = device_address.to_string();
        let function_texts = self.try_read_group_functions(device_address)?;
        let vfio_presence = function_texts.iter().all(|function_text| {
            self.maybe_driver_name(function_text).as_deref() == Some(Self::VFIO_DRIVER)
        });
        if vfio_presence {
            return Ok(false);
        }
        let nvidia_presence = self.maybe_driver_name(&address_text).as_deref()
            == Some(Self::NVIDIA_DRIVER);
        let mut persistence_presence = false;
        if nvidia_presence {
            let query_texts = [
                "-i",
                address_text.as_str(),
                "--query-gpu=persistence_mode",
                "--format=csv,noheader",
            ];
            let query_output = self.try_spawn_persistence(&query_texts)?;
            persistence_presence = query_output.trim() == Self::PERSISTENCE_ANSWER;
            if persistence_presence {
                self.try_spawn_persistence(&["-i", address_text.as_str(), "-pm", "0"])?;
            }
            let drain_outcome = self.try_spawn_persistence(&[
                "drain",
                "-p",
                address_text.as_str(),
                "-m",
                "1",
            ]);
            if let Err(error) = drain_outcome {
                self.restore_nvidia(&address_text, persistence_presence);
                return Err(error);
            }
        }
        for function_text in &function_texts {
            self.try_move_function(function_text)?;
        }
        Ok(persistence_presence)
    }

    fn check_identity_presence(&self, device_file: &DeviceFile) -> bool {
        let identity_directory = match device_file.device_kind() {
            DeviceKind::CharacterDevice => Self::CHARACTER_IDENTITY_DIRECTORY,
            DeviceKind::BlockDevice => Self::BLOCK_IDENTITY_DIRECTORY,
        };
        self.compose_system_path(identity_directory)
            .join(format!("{}:{}", device_file.major(), device_file.minor()))
            .exists()
    }

    fn try_release_passthrough(
        &self,
        device_address: &DeviceAddress,
        persistence_presence: bool,
    ) -> Result<(), Error> {
        let address_text = device_address.to_string();
        for function_text in self.try_read_group_functions(device_address)? {
            if self.maybe_driver_name(&function_text).as_deref() == Some(Self::VFIO_DRIVER) {
                self.try_return_function(&function_text)?;
            }
        }
        if persistence_presence {
            self.try_spawn_persistence(&["-i", address_text.as_str(), "-pm", "1"])?;
        }
        Ok(())
    }
}

impl<SpawnerType: Spawner> DeviceTree<SpawnerType> {
    const DEVICE_DIRECTORY: &'static str = "sys/bus/pci/devices";
    const CHARACTER_IDENTITY_DIRECTORY: &'static str = "sys/dev/char";
    const BLOCK_IDENTITY_DIRECTORY: &'static str = "sys/dev/block";
    const DRIVER_DIRECTORY: &'static str = "sys/bus/pci/drivers";
    const PROBE_FILE: &'static str = "sys/bus/pci/drivers_probe";
    const NVIDIA_DIRECTORY: &'static str = "proc/driver/nvidia/gpus";
    const VTCONSOLE_DIRECTORY: &'static str = "sys/class/vtconsole";
    const PLATFORM_DIRECTORY: &'static str = "sys/bus/platform/drivers";
    const GROUP_DIRECTORY: &'static str = "dev/vfio";
    const INFORMATION_NAME: &'static str = "information";
    const GRAPHICS_NAME: &'static str = "graphics";
    const BOOT_NAME: &'static str = "boot_vga";
    const DEV_NAME: &'static str = "dev";
    const SUBSYSTEM_NAME: &'static str = "subsystem";
    const INPUT_SUBSYSTEM: &'static str = "input";
    const NAME_FILE: &'static str = "name";
    const BIND_FILE: &'static str = "bind";
    const UNBIND_FILE: &'static str = "unbind";
    const OVERRIDE_FILE: &'static str = "driver_override";
    const MINOR_LABEL: &'static str = "Device Minor:";
    const FRAMEBUFFER_PREFIX: &'static str = "fb";
    const VTCONSOLE_PREFIX: &'static str = "vtcon";
    const FRAMEBUFFER_CONSOLE: &'static str = "frame buffer";
    const BIND_PRESENCE: &'static str = "1";
    const BIND_ABSENCE: &'static str = "0";
    const PERSISTENCE_ANSWER: &'static str = "Enabled";
    const VFIO_DRIVER: &'static str = "vfio-pci";
    const NVIDIA_DRIVER: &'static str = "nvidia";
    const BRIDGE_CLASS: &'static str = "0x0604";
    const NVIDIA_MAJOR: u32 = 195;
    const CONSOLE_LIMIT: u32 = 63;
    const FIRMWARE_DRIVERS: &'static [&'static str] = &[
        "efi-framebuffer",
        "simple-framebuffer",
        "vesa-framebuffer",
    ];

    fn compose_system_path(&self, relative_text: &str) -> PathBuf {
        self.tree_configuration
            .system_directory()
            .join(relative_text)
    }

    fn compose_device_path(&self, function_text: &str) -> PathBuf {
        self.compose_system_path(Self::DEVICE_DIRECTORY)
            .join(function_text)
    }

    fn compose_console_bind(&self, console_index: u32) -> PathBuf {
        self.compose_system_path(Self::VTCONSOLE_DIRECTORY)
            .join(format!("{}{console_index}", Self::VTCONSOLE_PREFIX))
            .join(Self::BIND_FILE)
    }

    fn compose_group_file(&self, function_text: &str) -> PathBuf {
        let group_link = self.compose_device_path(function_text).join("iommu_group");
        let group_name = fs::read_link(group_link)
            .ok()
            .and_then(|group_target| Self::maybe_file_name(&group_target))
            .unwrap_or_default();
        self.compose_system_path(Self::GROUP_DIRECTORY)
            .join(group_name)
    }

    fn compose_device_file(device_identity: u64) -> DeviceFile {
        let major = rustix::fs::major(device_identity);
        let minor = rustix::fs::minor(device_identity);
        DeviceFile::from((DeviceKind::CharacterDevice, major, minor))
    }

    fn scan_device_files(directory_path: &Path, device_files: &mut Vec<DeviceFile>) {
        let entries = fs::read_dir(directory_path).into_iter().flatten();
        for entry in entries.filter_map(Result::ok) {
            let entry_path = entry.path();
            let directory_presence = entry.file_type().is_ok_and(|file_type| file_type.is_dir());
            if directory_presence {
                if Self::check_input_presence(&entry_path) {
                    continue;
                }
                Self::scan_device_files(&entry_path, device_files);
            } else if entry.file_name() == Self::DEV_NAME {
                let dev_text = Self::read_text(&entry_path);
                let maybe_device_file = dev_text.split_once(':').and_then(
                    |(major_text, minor_text)| {
                        let major = major_text.parse::<u32>().ok()?;
                        let minor = minor_text.parse::<u32>().ok()?;
                        Some(DeviceFile::from((
                            DeviceKind::CharacterDevice,
                            major,
                            minor,
                        )))
                    },
                );
                device_files.extend(maybe_device_file);
            }
        }
    }

    fn check_input_presence(directory_path: &Path) -> bool {
        fs::read_link(directory_path.join(Self::SUBSYSTEM_NAME))
            .is_ok_and(|subsystem_path| subsystem_path.ends_with(Self::INPUT_SUBSYSTEM))
    }

    fn try_read_slot_functions(
        &self,
        device_address: &DeviceAddress,
    ) -> Result<Vec<String>, Error> {
        let address_text = device_address.to_string();
        if !self.compose_device_path(&address_text).is_dir() {
            return Err(Error::DeviceAbsence { address_text });
        }
        let device_directory = self.compose_system_path(Self::DEVICE_DIRECTORY);
        let mut function_texts = Vec::from_iter(
            Self::read_names(&device_directory)
                .into_iter()
                .filter(|function_text| {
                    DeviceAddress::try_from(function_text.clone())
                        .is_ok_and(|function_address| function_address.share_slot(device_address))
                }),
        );
        function_texts.sort();
        Ok(function_texts)
    }

    fn try_read_group_functions(
        &self,
        device_address: &DeviceAddress,
    ) -> Result<Vec<String>, Error> {
        let address_text = device_address.to_string();
        let device_path = self.compose_device_path(&address_text);
        if !device_path.is_dir() {
            return Err(Error::DeviceAbsence { address_text });
        }
        let mut function_texts = Self::read_names(&device_path.join("iommu_group/devices"));
        if function_texts.is_empty() {
            return Err(Error::GroupAbsence { address_text });
        }
        function_texts.sort();
        for function_text in &function_texts {
            let class_text = Self::read_text(
                &self.compose_device_path(function_text).join("class"),
            );
            if class_text.starts_with(Self::BRIDGE_CLASS) {
                let address_text = function_text.clone();
                return Err(Error::GroupBridge { address_text });
            }
            let member_presence = DeviceAddress::try_from(function_text.clone())
                .is_ok_and(|function_address| function_address.share_slot(device_address));
            if !member_presence {
                let address_text = function_text.clone();
                return Err(Error::GroupMember { address_text });
            }
        }
        Ok(function_texts)
    }

    fn try_check_console_map(&self, framebuffer_texts: &[String]) -> Result<bool, Error> {
        let console_program = self.tree_configuration.console_program();
        for console_index in 1..=Self::CONSOLE_LIMIT {
            let index_text = console_index.to_string();
            let mapping_text = self.try_spawn_program(console_program, &[index_text.as_str()])?;
            let maybe_framebuffer_text = mapping_text.split_whitespace().last();
            let mapping_presence = framebuffer_texts
                .iter()
                .any(|framebuffer_text| maybe_framebuffer_text == Some(framebuffer_text.as_str()));
            if mapping_presence {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn maybe_driver_name(&self, function_text: &str) -> Option<String> {
        let driver_link = self.compose_device_path(function_text).join("driver");
        fs::read_link(driver_link)
            .ok()
            .and_then(|driver_target| Self::maybe_file_name(&driver_target))
    }

    fn maybe_file_name(path: &Path) -> Option<String> {
        path.file_name()
            .map(|file_name| file_name.to_string_lossy().into_owned())
    }

    fn try_move_function(&self, function_text: &str) -> Result<(), Error> {
        let device_path = self.compose_device_path(function_text);
        let maybe_driver_name = self.maybe_driver_name(function_text);
        if maybe_driver_name.as_deref() == Some(Self::VFIO_DRIVER) {
            return Ok(());
        }
        if maybe_driver_name.is_some() {
            let unbind_path = device_path.join("driver").join(Self::UNBIND_FILE);
            self.try_write_within_deadline(&unbind_path, function_text)?;
        }
        let override_path = device_path.join(Self::OVERRIDE_FILE);
        self.try_write_within_deadline(&override_path, Self::VFIO_DRIVER)?;
        let bind_path = self
            .compose_system_path(Self::DRIVER_DIRECTORY)
            .join(Self::VFIO_DRIVER)
            .join(Self::BIND_FILE);
        self.try_write_within_deadline(&bind_path, function_text)?;
        let start_instant = Instant::now();
        let binding_deadline = self.tree_configuration.binding_deadline();
        while self.maybe_driver_name(function_text).as_deref() != Some(Self::VFIO_DRIVER) {
            if start_instant.elapsed() >= binding_deadline {
                let path = device_path.join("driver");
                return Err(Error::BindingDeadline { path });
            }
            thread::sleep(self.tree_configuration.check_interval());
        }
        Ok(())
    }

    fn try_return_function(&self, function_text: &str) -> Result<(), Error> {
        let device_path = self.compose_device_path(function_text);
        let unbind_path = device_path.join("driver").join(Self::UNBIND_FILE);
        self.try_write_within_deadline(&unbind_path, function_text)?;
        let override_path = device_path.join(Self::OVERRIDE_FILE);
        self.try_write_within_deadline(&override_path, "\n")?;
        let probe_path = self.compose_system_path(Self::PROBE_FILE);
        self.try_write_within_deadline(&probe_path, function_text)
    }

    fn restore_nvidia(&self, address_text: &str, persistence_presence: bool) {
        drop(self.try_spawn_persistence(&["drain", "-p", address_text, "-m", "0"]));
        if persistence_presence {
            drop(self.try_spawn_persistence(&["-i", address_text, "-pm", "1"]));
        }
    }

    fn try_write_within_deadline(&self, path: &Path, text: &str) -> Result<(), Error> {
        let (sender, receiver) = mpsc::channel();
        let write_path = PathBuf::from(path);
        let write_text = String::from(text);
        thread::spawn(move || {
            let write_outcome = fs::write(&write_path, write_text).map_err(|error| {
                let text = error.to_string();
                Error::OutputFailure {
                    path: write_path.clone(),
                    text,
                }
            });
            drop(sender.send(write_outcome));
        });
        let binding_deadline = self.tree_configuration.binding_deadline();
        receiver.recv_timeout(binding_deadline).unwrap_or_else(|_| {
            let path = PathBuf::from(path);
            Err(Error::BindingDeadline { path })
        })
    }

    fn try_spawn_persistence(&self, argument_texts: &[&str]) -> Result<String, Error> {
        let persistence_program = self.tree_configuration.persistence_program();
        self.try_spawn_program(persistence_program, argument_texts)
    }

    fn try_spawn_program(
        &self,
        program: &std::ffi::OsStr,
        argument_texts: &[&str],
    ) -> Result<String, Error> {
        let command_text = format!("{} {}", program.to_string_lossy(), argument_texts.join(" "));
        let arguments = Vec::from_iter(argument_texts.iter().map(OsString::from));
        let invocation = Invocation::from((program.to_os_string(), arguments, Stream::Capture));
        let completion = self
            .spawner
            .try_spawn_invocation(&invocation)
            .map_err(|error| {
                let text = error.to_string();
                Error::StartFailure { text }
            })?;
        match completion {
            Completion::Success { output } => Ok(String::from_utf8_lossy(&output).into_owned()),
            Completion::Failure { status, .. } => Err(Error::CompletionFailure {
                command_text,
                status,
            }),
        }
    }

    fn read_names(path: &Path) -> Vec<String> {
        let Ok(entries) = fs::read_dir(path) else {
            return Vec::new();
        };
        Vec::from_iter(
            entries
                .filter_map(Result::ok)
                .map(|entry| entry.file_name().to_string_lossy().into_owned()),
        )
    }

    fn read_text(path: &Path) -> String {
        String::from(fs::read_to_string(path).unwrap_or_default().trim())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::os::unix::fs::symlink;
    use std::process;
    use std::time::Duration;

    mod fixture {
        use super::*;

        pub(crate) type FixtureDeviceTree = DeviceTree<
            stack_process_adapter_spawner_transcript::Spawner,
        >;

        pub(crate) fn system_directory_factory(name: &str) -> PathBuf {
            let system_directory = env::temp_dir()
                .join(format!("baton-tree-{name}-{}", process::id()));
            drop(fs::remove_dir_all(&system_directory));
            let device_directory = system_directory.join("sys/bus/pci/devices");
            for (function_text, driver_text, class_text) in [
                ("0000:03:00.0", "nvidia", "0x030000"),
                ("0000:03:00.1", "snd_hda_intel", "0x040300"),
                ("0000:04:00.0", "nvidia", "0x030000"),
            ] {
                let function_path = device_directory.join(function_text);
                fs::create_dir_all(&function_path).unwrap();
                fs::write(function_path.join("class"), format!("{class_text}\n")).unwrap();
                let driver_path = system_directory
                    .join("sys/bus/pci/drivers")
                    .join(driver_text);
                fs::create_dir_all(&driver_path).unwrap();
                fs::write(driver_path.join("unbind"), "").unwrap();
                symlink(
                    format!("../../drivers/{driver_text}"),
                    function_path.join("driver"),
                )
                .unwrap();
                fs::write(function_path.join("driver_override"), "").unwrap();
            }
            let group_path = system_directory.join("sys/kernel/iommu_groups/16/devices");
            fs::create_dir_all(&group_path).unwrap();
            for function_text in ["0000:03:00.0", "0000:03:00.1"] {
                fs::create_dir_all(group_path.join(function_text)).unwrap();
                symlink(
                    "../../../../kernel/iommu_groups/16",
                    device_directory.join(function_text).join("iommu_group"),
                )
                .unwrap();
            }
            let graphics_path = device_directory.join("0000:03:00.0");
            fs::write(graphics_path.join("boot_vga"), "1\n").unwrap();
            for (relative_text, dev_text) in [
                ("drm/card0", "226:0"),
                ("drm/renderD128", "226:128"),
                ("graphics/fb1", "29:1"),
                ("i2c-11/i2c-dev/i2c-11", "89:11"),
                ("drm/card0/card0-DP-1", "nonsense"),
                ("drm/card0/card0-DP-2", "226:x"),
                ("drm/card0/card0-DP-3", "x:0"),
            ] {
                fs::create_dir_all(graphics_path.join(relative_text)).unwrap();
                fs::write(
                    graphics_path.join(relative_text).join("dev"),
                    format!("{dev_text}\n"),
                )
                .unwrap();
            }
            let group_path = system_directory.join("sys/class/drm/card9");
            fs::create_dir_all(&group_path).unwrap();
            fs::write(group_path.join("dev"), "226:9\n").unwrap();
            symlink("../../../../class/drm", graphics_path.join("subsystem")).unwrap();
            let function_path = device_directory.join("0000:03:00.1/sound/card7/controlC7");
            fs::create_dir_all(&function_path).unwrap();
            fs::write(function_path.join("dev"), "116:7\n").unwrap();
            let directory_path = device_directory.join("0000:03:00.1/sound/card7/input21");
            let event_path = directory_path.join("event29");
            fs::create_dir_all(&event_path).unwrap();
            fs::write(event_path.join("dev"), "13:93\n").unwrap();
            symlink(
                "../../../../../../class/input",
                directory_path.join("subsystem"),
            )
            .unwrap();
            symlink(
                "../../../../../../../class/input",
                event_path.join("subsystem"),
            )
            .unwrap();
            let other_path = device_directory.join("0000:04:00.0/drm/card1");
            fs::create_dir_all(&other_path).unwrap();
            fs::write(other_path.join("dev"), "226:1\n").unwrap();
            let information_path = system_directory.join("proc/driver/nvidia/gpus/0000:03:00.0");
            fs::create_dir_all(&information_path).unwrap();
            fs::write(
                information_path.join("information"),
                "Model: RTX 3080\nDevice Minor: 1\n",
            )
            .unwrap();
            fs::create_dir_all(system_directory.join("dev/vfio")).unwrap();
            symlink("/dev/null", system_directory.join("dev/vfio/16")).unwrap();
            for (vtconsole_name, name_text) in [
                ("vtcon0", "(S) dummy device"),
                ("vtcon1", "(M) frame buffer device"),
                ("vtconx", "(M) frame buffer device"),
            ] {
                let console_path = system_directory
                    .join("sys/class/vtconsole")
                    .join(vtconsole_name);
                fs::create_dir_all(&console_path).unwrap();
                fs::write(console_path.join("name"), format!("{name_text}\n")).unwrap();
                fs::write(console_path.join("bind"), "1\n").unwrap();
            }
            for driver_text in ["efi-framebuffer", "simple-framebuffer"] {
                let driver_path = system_directory
                    .join("sys/bus/platform/drivers")
                    .join(driver_text);
                fs::create_dir_all(&driver_path).unwrap();
                fs::write(driver_path.join("bind"), "").unwrap();
                fs::write(driver_path.join("unbind"), "").unwrap();
            }
            fs::create_dir_all(
                system_directory.join("sys/bus/platform/drivers/efi-framebuffer/efi-framebuffer.0"),
            )
            .unwrap();
            let driver_path = system_directory.join("sys/bus/pci/drivers/vfio-pci");
            fs::create_dir_all(&driver_path).unwrap();
            fs::write(driver_path.join("bind"), "").unwrap();
            fs::write(driver_path.join("unbind"), "").unwrap();
            fs::write(system_directory.join("sys/bus/pci/drivers_probe"), "").unwrap();
            system_directory
        }

        pub(crate) fn device_tree_factory(
            system_directory: &Path,
            completion: Vec<Option<Completion>>,
            binding_deadline: Duration,
        ) -> FixtureDeviceTree {
            let tree_configuration = TreeConfiguration::from((
                PathBuf::from(system_directory),
                (OsString::from("con2fbmap"), OsString::from("nvidia-smi")),
                (binding_deadline, Duration::from_millis(5)),
            ));
            DeviceTree::from((
                tree_configuration,
                stack_process_adapter_spawner_transcript::Spawner::from(completion),
            ))
        }

        pub(crate) fn device_address_factory(address_text: &str) -> DeviceAddress {
            DeviceAddress::try_from(String::from(address_text)).unwrap()
        }

        pub(crate) fn output_factory(output_text: &str) -> Option<Completion> {
            Some(Completion::Success {
                output: output_text.as_bytes().to_vec(),
            })
        }

        pub(crate) fn vfio_driver_factory(system_directory: &Path, function_text: &str) {
            let driver_link = system_directory
                .join("sys/bus/pci/devices")
                .join(function_text)
                .join("driver");
            fs::remove_file(&driver_link).unwrap();
            symlink("../../drivers/vfio-pci", driver_link).unwrap();
        }

        pub(crate) fn bind_path_factory(system_directory: &Path) -> thread::JoinHandle<()> {
            let system_directory = PathBuf::from(system_directory);
            thread::spawn(move || {
                let bind_path = system_directory.join("sys/bus/pci/drivers/vfio-pci/bind");
                let mut function_texts = vec!["0000:03:00.0", "0000:03:00.1"];
                let start_instant = Instant::now();
                while !function_texts.is_empty() && start_instant.elapsed() < Duration::from_secs(5)
                {
                    let bind_text = fs::read_to_string(&bind_path).unwrap_or_default();
                    if let Some(position) = function_texts
                        .iter()
                        .position(|function_text| *function_text == bind_text)
                    {
                        vfio_driver_factory(&system_directory, function_texts.remove(position));
                    }
                    thread::sleep(Duration::from_millis(1));
                }
            })
        }

        pub(crate) fn argument_factory(invocation: &Invocation) -> String {
            let arguments = Vec::from_iter(
                invocation
                    .arguments()
                    .iter()
                    .map(|argument| argument.to_string_lossy().into_owned()),
            );
            format!(
                "{} {}",
                invocation.program().to_string_lossy(),
                arguments.join(" ")
            )
        }
    }

    #[test]
    fn the_device_files_of_a_gpu_are_every_dev_of_its_slot_but_its_input_devices_and_its_file_of_nvidia()
     {
        let system_directory = fixture::system_directory_factory("files");
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );

        let device_files = device_tree
            .try_read_device_files(&fixture::device_address_factory("0000:03:00.0"))
            .unwrap();

        let expectation = [(29, 1), (89, 11), (116, 7), (195, 1), (226, 0), (226, 128)]
            .map(|(major, minor)| DeviceFile::from((DeviceKind::CharacterDevice, major, minor)));
        assert_eq!(device_files, expectation);
        fixture::vfio_driver_factory(&system_directory, "0000:03:00.1");
        let device_files = device_tree
            .try_read_device_files(&fixture::device_address_factory("0000:03:00.1"))
            .unwrap();
        assert!(device_files.contains(&DeviceFile::from((DeviceKind::CharacterDevice, 1, 3))));
        let outcome = device_tree
            .try_read_device_files(&fixture::device_address_factory("0000:05:00.0"));
        assert_eq!(
            outcome,
            Err(Error::DeviceAbsence {
                address_text: String::from("0000:05:00.0")
            })
        );
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn an_identity_is_present_while_a_device_of_the_system_holds_it() {
        let system_directory = fixture::system_directory_factory("numbers");
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );
        fs::create_dir_all(system_directory.join("sys/dev/char/13:84")).unwrap();
        fs::create_dir_all(system_directory.join("sys/dev/block/8:0")).unwrap();

        let presence = [
            DeviceFile::from((DeviceKind::CharacterDevice, 13, 84)),
            DeviceFile::from((DeviceKind::CharacterDevice, 13, 86)),
            DeviceFile::from((DeviceKind::BlockDevice, 8, 0)),
            DeviceFile::from((DeviceKind::BlockDevice, 13, 84)),
        ]
        .map(|device_file| device_tree.check_identity_presence(&device_file));

        assert_eq!(presence, [true, false, true, false]);
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_console_drawn_on_the_gpu_and_the_framebuffer_of_the_firmware_are_users() {
        let system_directory = fixture::system_directory_factory("consoles");
        let completion = vec![
            fixture::output_factory("console 1 is mapped to framebuffer 0\n"),
            fixture::output_factory("console 2 is mapped to framebuffer 1\n"),
        ];
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            completion,
            Duration::from_secs(1),
        );

        let console_users = device_tree
            .try_read_console_users(&fixture::device_address_factory("0000:03:00.0"))
            .unwrap();

        assert_eq!(
            console_users,
            [
                GraphicsUser::ConsoleUser { console_index: 1 },
                GraphicsUser::FramebufferUser {
                    device_text: String::from("efi-framebuffer.0"),
                    driver_text: String::from("efi-framebuffer"),
                },
            ]
        );
        assert_eq!(
            fixture::argument_factory(&device_tree.spawner.invocations()[1]),
            "con2fbmap 2"
        );
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_gpu_whose_framebuffer_draws_no_console_and_that_is_no_boot_device_has_no_console_user() {
        let system_directory = fixture::system_directory_factory("quiet");
        fs::write(
            system_directory.join("sys/bus/pci/devices/0000:03:00.0/boot_vga"),
            "0\n",
        )
        .unwrap();
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );

        let console_users = [
            device_tree.try_read_console_users(&fixture::device_address_factory("0000:03:00.0")),
            device_tree.try_read_console_users(&fixture::device_address_factory("0000:04:00.0")),
        ];

        assert_eq!(console_users, [Ok(Vec::new()), Ok(Vec::new())]);
        assert_eq!(device_tree.spawner.invocations().len(), 63);
        assert!(matches!(
            device_tree.try_read_console_users(&fixture::device_address_factory("0000:05:00.0")),
            Err(Error::DeviceAbsence { .. })
        ));
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![None],
            Duration::from_secs(1),
        );
        let outcome = device_tree
            .try_read_console_users(&fixture::device_address_factory("0000:03:00.0"));
        assert!(matches!(outcome, Err(Error::StartFailure { .. })));
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_console_user_is_unbound_and_bound_again_through_its_files() {
        let system_directory = fixture::system_directory_factory("binding");
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );
        let console_user = GraphicsUser::ConsoleUser { console_index: 1 };
        let framebuffer_user = GraphicsUser::FramebufferUser {
            device_text: String::from("efi-framebuffer.0"),
            driver_text: String::from("efi-framebuffer"),
        };
        let process_user = GraphicsUser::ProcessUser {
            process_identifier: 7,
            command_text: String::from("render"),
            control_group_text: String::from("/user.slice"),
        };
        let console_path = system_directory.join("sys/class/vtconsole/vtcon1/bind");
        let driver_path = system_directory.join("sys/bus/platform/drivers/efi-framebuffer");

        device_tree.try_unbind_console_user(&console_user).unwrap();
        device_tree
            .try_unbind_console_user(&framebuffer_user)
            .unwrap();
        device_tree.try_unbind_console_user(&process_user).unwrap();

        assert_eq!(fs::read_to_string(&console_path).unwrap(), "0");
        assert_eq!(
            fs::read_to_string(driver_path.join("unbind")).unwrap(),
            "efi-framebuffer.0"
        );
        device_tree.try_bind_console_user(&console_user).unwrap();
        device_tree
            .try_bind_console_user(&framebuffer_user)
            .unwrap();
        device_tree.try_bind_console_user(&process_user).unwrap();
        assert_eq!(fs::read_to_string(&console_path).unwrap(), "1");
        assert_eq!(
            fs::read_to_string(driver_path.join("bind")).unwrap(),
            "efi-framebuffer.0"
        );
        let outcome = device_tree
            .try_bind_console_user(&GraphicsUser::ConsoleUser { console_index: 9 });
        assert!(matches!(outcome, Err(Error::OutputFailure { .. })));
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_passthrough_turns_the_persistence_off_drains_the_gpu_and_binds_every_function_to_vfio() {
        let system_directory = fixture::system_directory_factory("passthrough");
        let completion = vec![fixture::output_factory("Enabled\n")];
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            completion,
            Duration::from_secs(5),
        );
        let receiver = fixture::bind_path_factory(&system_directory);

        let persistence_presence = device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"))
            .unwrap();

        receiver.join().unwrap();
        assert!(persistence_presence);
        let command_texts = Vec::from_iter(
            device_tree
                .spawner
                .invocations()
                .iter()
                .map(fixture::argument_factory),
        );
        assert_eq!(
            command_texts,
            [
                "nvidia-smi -i 0000:03:00.0 --query-gpu=persistence_mode --format=csv,noheader",
                "nvidia-smi -i 0000:03:00.0 -pm 0",
                "nvidia-smi drain -p 0000:03:00.0 -m 1",
            ]
        );
        let override_path = system_directory
            .join("sys/bus/pci/devices/0000:03:00.1/driver_override");
        assert_eq!(fs::read_to_string(&override_path).unwrap(), "vfio-pci");
        let vfio_presence = device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"));
        assert_eq!(vfio_presence, Ok(false));
        device_tree
            .try_release_passthrough(&fixture::device_address_factory("0000:03:00.0"), true)
            .unwrap();
        assert_eq!(fs::read_to_string(&override_path).unwrap(), "\n");
        assert_eq!(
            fs::read_to_string(system_directory.join("sys/bus/pci/drivers_probe")).unwrap(),
            "0000:03:00.1"
        );
        let command_texts = Vec::from_iter(
            device_tree
                .spawner
                .invocations()
                .iter()
                .map(fixture::argument_factory),
        );
        assert_eq!(
            command_texts.last().map(String::as_str),
            Some("nvidia-smi -i 0000:03:00.0 -pm 1")
        );
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_passthrough_without_persistence_skips_the_mode_and_its_release_skips_it_too() {
        let system_directory = fixture::system_directory_factory("persistence");
        let completion = vec![fixture::output_factory("Disabled\n")];
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            completion,
            Duration::from_secs(5),
        );
        let receiver = fixture::bind_path_factory(&system_directory);

        let persistence_presence = device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"))
            .unwrap();

        receiver.join().unwrap();
        assert!(!persistence_presence);
        assert_eq!(device_tree.spawner.invocations().len(), 2);
        device_tree
            .try_release_passthrough(&fixture::device_address_factory("0000:03:00.0"), false)
            .unwrap();
        assert_eq!(device_tree.spawner.invocations().len(), 2);
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_failed_drain_gives_the_gpu_back_its_persistence_and_ends_the_passthrough() {
        let system_directory = fixture::system_directory_factory("drain");
        let failure = Some(Completion::Failure {
            status: String::from("exit status: 3"),
            maybe_code: Some(3),
        });
        let completion = vec![
            fixture::output_factory("Enabled\n"),
            fixture::output_factory(""),
            failure,
        ];
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            completion,
            Duration::from_secs(1),
        );

        let outcome = device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"));

        assert_eq!(
            outcome,
            Err(Error::CompletionFailure {
                command_text: String::from("nvidia-smi drain -p 0000:03:00.0 -m 1"),
                status: String::from("exit status: 3"),
            })
        );
        let command_texts = Vec::from_iter(
            device_tree
                .spawner
                .invocations()
                .iter()
                .map(fixture::argument_factory),
        );
        assert_eq!(
            &command_texts[3..],
            [
                "nvidia-smi drain -p 0000:03:00.0 -m 0",
                "nvidia-smi -i 0000:03:00.0 -pm 1"
            ]
        );
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![
                fixture::output_factory("Disabled\n"),
                Some(Completion::Failure {
                    status: String::from("exit status: 3"),
                    maybe_code: Some(3),
                }),
            ],
            Duration::from_secs(1),
        );
        assert!(
            device_tree
                .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"))
                .is_err()
        );
        assert_eq!(device_tree.spawner.invocations().len(), 3);
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![fixture::output_factory("Enabled\n"), None],
            Duration::from_secs(1),
        );
        assert!(matches!(
            device_tree.try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0")),
            Err(Error::StartFailure { .. })
        ));
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![None],
            Duration::from_secs(1),
        );
        assert!(matches!(
            device_tree.try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0")),
            Err(Error::StartFailure { .. })
        ));
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_group_with_a_bridge_or_another_card_is_never_passed_through() {
        let system_directory = fixture::system_directory_factory("group");
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );
        let group_path = system_directory.join("sys/kernel/iommu_groups/16/devices");
        fs::create_dir_all(group_path.join("0000:04:00.0")).unwrap();

        let outcome = device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"));
        fs::write(
            system_directory.join("sys/bus/pci/devices/0000:03:00.1/class"),
            "0x060400\n",
        )
        .unwrap();
        let drain_outcome = device_tree
            .try_release_passthrough(&fixture::device_address_factory("0000:03:00.0"), false);

        assert_eq!(
            outcome,
            Err(Error::GroupMember {
                address_text: String::from("0000:04:00.0")
            })
        );
        assert_eq!(
            drain_outcome,
            Err(Error::GroupBridge {
                address_text: String::from("0000:03:00.1")
            })
        );
        assert!(matches!(
            device_tree.try_bind_passthrough(&fixture::device_address_factory("0000:05:00.0")),
            Err(Error::DeviceAbsence { .. })
        ));
        assert_eq!(
            device_tree.try_bind_passthrough(&fixture::device_address_factory("0000:04:00.0")),
            Err(Error::GroupAbsence {
                address_text: String::from("0000:04:00.0")
            })
        );
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_function_that_never_reaches_vfio_ends_the_passthrough_at_its_deadline() {
        let system_directory = fixture::system_directory_factory("deadline");
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![fixture::output_factory("Disabled\n")],
            Duration::from_millis(30),
        );
        fs::remove_file(system_directory.join("sys/bus/pci/devices/0000:03:00.0/driver")).unwrap();

        let outcome = device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"));

        assert_eq!(
            outcome,
            Err(Error::BindingDeadline {
                path: system_directory.join("sys/bus/pci/devices/0000:03:00.0/driver"),
            })
        );
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_write_that_blocks_ends_at_the_deadline_of_the_binding() {
        let system_directory = fixture::system_directory_factory("blocking");
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_millis(30),
        );
        let bind_path = system_directory.join("sys/class/vtconsole/vtcon1/bind");
        fs::remove_file(&bind_path).unwrap();
        rustix::fs::mknodat(
            rustix::fs::CWD,
            &bind_path,
            rustix::fs::FileType::Fifo,
            rustix::fs::Mode::RUSR | rustix::fs::Mode::WUSR,
            0,
        )
        .unwrap();

        let outcome = device_tree
            .try_bind_console_user(&GraphicsUser::ConsoleUser { console_index: 1 });

        assert_eq!(
            outcome,
            Err(Error::BindingDeadline {
                path: bind_path.clone()
            })
        );
        drop(fs::OpenOptions::new().read(true).open(&bind_path));
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_release_whose_writes_fail_is_an_error_of_the_output() {
        let system_directory = fixture::system_directory_factory("release");
        fixture::vfio_driver_factory(&system_directory, "0000:03:00.0");
        fs::remove_dir_all(system_directory.join("sys/bus/pci/drivers/vfio-pci")).unwrap();
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );

        let outcome = device_tree
            .try_release_passthrough(&fixture::device_address_factory("0000:03:00.0"), false);

        assert!(matches!(outcome, Err(Error::OutputFailure { .. })));
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_partial_passthrough_binds_only_the_function_that_is_not_bound_yet() {
        let system_directory = fixture::system_directory_factory("partial");
        fixture::vfio_driver_factory(&system_directory, "0000:03:00.1");
        fs::write(
            system_directory.join("sys/bus/pci/devices/0000:03:00.1/driver_override"),
            "kept",
        )
        .unwrap();
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![fixture::output_factory("Disabled\n")],
            Duration::from_secs(5),
        );
        let receiver = fixture::bind_path_factory(&system_directory);

        device_tree
            .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"))
            .unwrap();

        drop(receiver);
        let override_path = system_directory
            .join("sys/bus/pci/devices/0000:03:00.1/driver_override");
        assert_eq!(fs::read_to_string(&override_path).unwrap(), "kept");
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn a_release_returns_only_the_functions_of_vfio_and_fails_on_a_failed_persistence() {
        let system_directory = fixture::system_directory_factory("return");
        fixture::vfio_driver_factory(&system_directory, "0000:03:00.0");
        let failure = Some(Completion::Failure {
            status: String::from("exit status: 1"),
            maybe_code: Some(1),
        });
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            vec![failure],
            Duration::from_secs(1),
        );

        let outcome = device_tree
            .try_release_passthrough(&fixture::device_address_factory("0000:03:00.0"), true);

        assert!(matches!(outcome, Err(Error::CompletionFailure { .. })));
        let override_path = system_directory
            .join("sys/bus/pci/devices/0000:03:00.1/driver_override");
        assert_eq!(fs::read_to_string(override_path).unwrap(), "");
        fs::remove_dir_all(system_directory).unwrap();
    }

    #[test]
    fn every_write_of_a_binding_that_fails_is_an_error_of_the_output() {
        let entries = [
            "sys/bus/pci/drivers/nvidia/unbind",
            "sys/bus/pci/devices/0000:03:00.0/driver_override",
            "sys/bus/pci/drivers/vfio-pci/bind",
        ];

        let outcome = entries.map(|entry| {
            let system_directory = fixture::system_directory_factory("blocked");
            let path = system_directory.join(entry);
            fs::remove_file(&path).unwrap();
            fs::create_dir_all(&path).unwrap();
            let device_tree = fixture::device_tree_factory(
                &system_directory,
                vec![fixture::output_factory("Disabled\n")],
                Duration::from_secs(1),
            );
            let outcome = device_tree
                .try_bind_passthrough(&fixture::device_address_factory("0000:03:00.0"));
            fs::remove_dir_all(system_directory).unwrap();
            outcome
        });
        let system_directory = fixture::system_directory_factory("returning");
        fixture::vfio_driver_factory(&system_directory, "0000:03:00.0");
        let path = system_directory.join("sys/bus/pci/devices/0000:03:00.0/driver_override");
        fs::remove_file(&path).unwrap();
        fs::create_dir_all(&path).unwrap();
        let device_tree = fixture::device_tree_factory(
            &system_directory,
            Vec::new(),
            Duration::from_secs(1),
        );
        let release_outcome = device_tree
            .try_release_passthrough(&fixture::device_address_factory("0000:03:00.0"), false);

        assert!(
            outcome
                .iter()
                .all(|outcome| matches!(outcome, Err(Error::OutputFailure { .. })))
        );
        assert!(matches!(release_outcome, Err(Error::OutputFailure { .. })));
        fs::remove_dir_all(system_directory).unwrap();
    }
}
