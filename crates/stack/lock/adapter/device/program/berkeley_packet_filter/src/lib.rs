mod configuration;
pub mod device;
mod error;
mod instruction;
mod system_kernel;

pub use configuration::ProgramConfiguration;
pub use error::Error;
pub use system_kernel::SystemKernel;

use crate::device::filter::DeviceFilter;
use crate::device::kernel::DeviceKernel;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::device::program::DeviceProgram as DeviceProgramPort;
use stack_lock_core_device_file::{DeviceFile, DeviceKind};
use std::fs::{self, File};
use std::io;
use std::os::fd::{AsFd, AsRawFd, BorrowedFd, OwnedFd};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeviceProgram<DeviceKernelType> {
    program_configuration: ProgramConfiguration,
    device_kernel: DeviceKernelType,
}

impl<DeviceKernelType> Layer for DeviceProgram<DeviceKernelType> {
    type Layer = InfrastructureLayer;
}

impl<DeviceKernelType> Adapter for DeviceProgram<DeviceKernelType> {}

impl<DeviceKernelType> Port for DeviceProgram<DeviceKernelType> {}

impl<DeviceKernelType> DrivenPort for DeviceProgram<DeviceKernelType> {}

impl<DeviceKernelType> From<(ProgramConfiguration, DeviceKernelType)>
    for DeviceProgram<DeviceKernelType>
{
    fn from(
        (program_configuration, device_kernel): (ProgramConfiguration, DeviceKernelType),
    ) -> Self {
        Self {
            program_configuration,
            device_kernel,
        }
    }
}

impl<DeviceKernelType: DeviceKernel> DeviceProgramPort for DeviceProgram<DeviceKernelType> {
    type Error = Error;

    fn try_install_program(&self) -> Result<(), Error> {
        if self.try_check_program()? {
            return Ok(());
        }
        let pin_directory = self.program_configuration.pin_directory();
        fs::create_dir_all(pin_directory).map_err(|error| {
            let text = error.to_string();
            Error::PinDirectory {
                path: PathBuf::from(pin_directory),
                text,
            }
        })?;
        let device_map = self.try_create_map(
            Self::DEVICE_KEY_SIZE,
            Self::LOCK_SIZE,
            Self::DEVICE_LIMIT,
        )?;
        self.try_fasten_object(device_map.as_fd(), &self.device_pin())?;
        let permission_map = self.try_create_map(
            Self::PERMISSION_KEY_SIZE,
            Self::PRESENCE_SIZE,
            Self::PERMISSION_LIMIT,
        )?;
        self.try_fasten_object(permission_map.as_fd(), &self.permission_pin())?;
        let program = self.try_load_program(device_map.as_fd(), permission_map.as_fd())?;
        let hierarchy_file = self.try_open_hierarchy()?;
        self.device_kernel
            .try_attach_device_program(hierarchy_file.as_fd(), program.as_fd())
            .map_err(|error| Error::from((String::from("attachment"), error)))?;
        self.try_fasten_object(program.as_fd(), &self.program_pin())
    }

    fn try_insert_device_files(
        &self,
        map_index: u32,
        device_files: &[DeviceFile],
    ) -> Result<(), Error> {
        if device_files.is_empty() {
            return Ok(());
        }
        let device_map = self.try_open_object(&self.device_pin())?;
        let lock_value = map_index.to_le_bytes();
        for device_file in device_files {
            let device_key = Self::compose_device_key(device_file);
            self.device_kernel
                .try_update_element(device_map.as_fd(), &device_key, &lock_value)
                .map_err(|error| Error::from((String::from("insertion of a device"), error)))?;
        }
        Ok(())
    }

    fn try_delete_device_files(&self, device_files: &[DeviceFile]) -> Result<(), Error> {
        if device_files.is_empty() {
            return Ok(());
        }
        let device_map = self.try_open_object(&self.device_pin())?;
        for device_file in device_files {
            let device_key = Self::compose_device_key(device_file);
            self.try_delete_key(device_map.as_fd(), &device_key)?;
        }
        Ok(())
    }

    fn try_permit_group(&self, map_index: u32, group_identifier: u64) -> Result<(), Error> {
        let permission_map = self.try_open_object(&self.permission_pin())?;
        let permission_key = Self::compose_permission_key(group_identifier, map_index);
        let presence_value: [u8; 1] = [1];
        self.device_kernel
            .try_update_element(permission_map.as_fd(), &permission_key, &presence_value)
            .map_err(|error| Error::from((String::from("permission of a cgroup"), error)))
    }

    fn try_revoke_group(&self, map_index: u32, group_identifier: u64) -> Result<(), Error> {
        let permission_map = self.try_open_object(&self.permission_pin())?;
        let permission_key = Self::compose_permission_key(group_identifier, map_index);
        self.try_delete_key(permission_map.as_fd(), &permission_key)
    }

    fn try_remove_program(&self) -> Result<(), Error> {
        if self.try_check_program()? {
            let program = self.try_open_object(&self.program_pin())?;
            let hierarchy_file = self.try_open_hierarchy()?;
            self.device_kernel
                .try_detach_device_program(hierarchy_file.as_fd(), program.as_fd())
                .map_err(|error| Error::from((String::from("detachment"), error)))?;
        }
        for pin_path in [self.program_pin(), self.permission_pin(), self.device_pin()] {
            if let Err(error) = fs::remove_file(&pin_path)
                && error.kind() != io::ErrorKind::NotFound
            {
                let text = error.to_string();
                return Err(Error::PinDirectory {
                    path: pin_path,
                    text,
                });
            }
        }
        Ok(())
    }
}

impl<DeviceKernelType: DeviceKernel> DeviceProgram<DeviceKernelType> {
    const DEVICE_KEY_SIZE: u32 = 12;
    const PERMISSION_KEY_SIZE: u32 = 16;
    const LOCK_SIZE: u32 = 4;
    const PRESENCE_SIZE: u32 = 1;
    const DEVICE_LIMIT: u32 = 1024;
    const PERMISSION_LIMIT: u32 = 4096;
    const CHARACTER_TYPE: u32 = 2;
    const BLOCK_TYPE: u32 = 1;
    const ABSENCE_CODE: i32 = 2;
    const DEVICE_PIN: &'static str = "devices";
    const PERMISSION_PIN: &'static str = "permissions";
    const PROGRAM_PIN: &'static str = "program";

    fn device_pin(&self) -> PathBuf {
        self.program_configuration
            .pin_directory()
            .join(Self::DEVICE_PIN)
    }

    fn permission_pin(&self) -> PathBuf {
        self.program_configuration
            .pin_directory()
            .join(Self::PERMISSION_PIN)
    }

    fn program_pin(&self) -> PathBuf {
        self.program_configuration
            .pin_directory()
            .join(Self::PROGRAM_PIN)
    }

    fn try_check_program(&self) -> Result<bool, Error> {
        let program_pin = self.program_pin();
        program_pin.try_exists().map_err(|error| {
            let text = error.to_string();
            Error::PinDirectory {
                path: program_pin.clone(),
                text,
            }
        })
    }

    fn try_create_map(
        &self,
        key_size: u32,
        value_size: u32,
        entry_limit: u32,
    ) -> Result<OwnedFd, Error> {
        self.device_kernel
            .try_create_hash_map(key_size, value_size, entry_limit)
            .map_err(|error| Error::from((String::from("creation of a map"), error)))
    }

    fn try_load_program(
        &self,
        device_map: BorrowedFd<'_>,
        permission_map: BorrowedFd<'_>,
    ) -> Result<OwnedFd, Error> {
        let ancestry_depth = self.program_configuration.ancestry_depth();
        let device_filter = DeviceFilter::from((
            device_map.as_raw_fd(),
            permission_map.as_raw_fd(),
            ancestry_depth,
        ));
        let instructions = device_filter.compose_bytes();
        let license = c"GPL";
        self.device_kernel
            .try_load_device_program(&instructions, license)
            .map_err(|error| Error::from((String::from("load of the program"), error)))
    }

    fn try_open_hierarchy(&self) -> Result<File, Error> {
        let hierarchy_directory = self.program_configuration.hierarchy_directory();
        File::open(hierarchy_directory).map_err(|error| {
            let text = error.to_string();
            Error::Hierarchy {
                path: PathBuf::from(hierarchy_directory),
                text,
            }
        })
    }

    fn try_fasten_object(&self, object: BorrowedFd<'_>, pin_path: &Path) -> Result<(), Error> {
        self.device_kernel
            .try_fasten_object(object, pin_path)
            .map_err(|error| Error::from((String::from("pin of an object"), error)))
    }

    fn try_open_object(&self, pin_path: &Path) -> Result<OwnedFd, Error> {
        self.device_kernel
            .try_open_object(pin_path)
            .map_err(|error| Error::from((String::from("lookup of an object"), error)))
    }

    fn try_delete_key(&self, map: BorrowedFd<'_>, key: &[u8]) -> Result<(), Error> {
        match self.device_kernel.try_delete_element(map, key) {
            Err(error) if error.raw_os_error() != Some(Self::ABSENCE_CODE) => {
                Err(Error::from((String::from("deletion of a key"), error)))
            }
            _ => Ok(()),
        }
    }

    fn compose_device_key(device_file: &DeviceFile) -> [u8; 12] {
        let device_type = match device_file.device_kind() {
            DeviceKind::CharacterDevice => Self::CHARACTER_TYPE,
            DeviceKind::BlockDevice => Self::BLOCK_TYPE,
        };
        let mut device_key: [u8; 12] = [0; 12];
        device_key[0..4].copy_from_slice(&device_type.to_le_bytes());
        device_key[4..8].copy_from_slice(&device_file.major().to_le_bytes());
        device_key[8..12].copy_from_slice(&device_file.minor().to_le_bytes());
        device_key
    }

    fn compose_permission_key(group_identifier: u64, map_index: u32) -> [u8; 16] {
        let mut permission_key: [u8; 16] = [0; 16];
        permission_key[0..8].copy_from_slice(&group_identifier.to_le_bytes());
        permission_key[8..12].copy_from_slice(&map_index.to_le_bytes());
        permission_key
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::process;
    use std::sync::Mutex;

    mod fixture {
        use super::*;

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub(crate) enum Operation {
            TryCreateHashMap,
            TryUpdateElement,
            TryDeleteElement,
            TryLoadDeviceProgram,
            TryFastenObject,
            TryOpenObject,
            TryAttachDeviceProgram,
            TryDetachDeviceProgram,
        }

        #[derive(Debug, Default)]
        pub(crate) struct FixtureDeviceKernel {
            pub(crate) operations: Mutex<Vec<Operation>>,
            pub(crate) maps: Mutex<Vec<(u32, u32, u32)>>,
            pub(crate) rejection_operation: Option<Operation>,
            pub(crate) rejection_position: Option<usize>,
            pub(crate) absence: bool,
        }

        impl FixtureDeviceKernel {
            fn try_record(&self, operation: Operation) -> io::Result<()> {
                let position = {
                    let mut operations = self.operations.lock().unwrap();
                    operations.push(operation);
                    operations.len() - 1
                };
                if self.rejection_operation == Some(operation)
                    || self.rejection_position == Some(position)
                {
                    return Err(io::Error::from(io::ErrorKind::PermissionDenied));
                }
                Ok(())
            }

            fn try_descriptor() -> io::Result<OwnedFd> {
                let file = File::open("/dev/null")?;
                Ok(OwnedFd::from(file))
            }
        }

        impl DeviceKernel for FixtureDeviceKernel {
            fn try_create_hash_map(
                &self,
                key_size: u32,
                value_size: u32,
                entry_limit: u32,
            ) -> io::Result<OwnedFd> {
                self.try_record(Operation::TryCreateHashMap)?;
                self.maps
                    .lock()
                    .unwrap()
                    .push((key_size, value_size, entry_limit));
                Self::try_descriptor()
            }

            fn try_update_element(
                &self,
                _map: BorrowedFd<'_>,
                _key: &[u8],
                _value: &[u8],
            ) -> io::Result<()> {
                self.try_record(Operation::TryUpdateElement)
            }

            fn try_delete_element(&self, _map: BorrowedFd<'_>, _key: &[u8]) -> io::Result<()> {
                self.try_record(Operation::TryDeleteElement)?;
                if self.absence {
                    return Err(io::Error::from_raw_os_error(2));
                }
                Ok(())
            }

            fn try_load_device_program(
                &self,
                _instructions: &[u8],
                _license: &std::ffi::CStr,
            ) -> io::Result<OwnedFd> {
                self.try_record(Operation::TryLoadDeviceProgram)?;
                Self::try_descriptor()
            }

            fn try_fasten_object(&self, _object: BorrowedFd<'_>, _path: &Path) -> io::Result<()> {
                self.try_record(Operation::TryFastenObject)
            }

            fn try_open_object(&self, _path: &Path) -> io::Result<OwnedFd> {
                self.try_record(Operation::TryOpenObject)?;
                Self::try_descriptor()
            }

            fn try_attach_device_program(
                &self,
                _cgroup: BorrowedFd<'_>,
                _program: BorrowedFd<'_>,
            ) -> io::Result<()> {
                self.try_record(Operation::TryAttachDeviceProgram)
            }

            fn try_detach_device_program(
                &self,
                _cgroup: BorrowedFd<'_>,
                _program: BorrowedFd<'_>,
            ) -> io::Result<()> {
                self.try_record(Operation::TryDetachDeviceProgram)
            }
        }

        pub(crate) fn pin_directory_factory(name: &str) -> PathBuf {
            let pin_directory = env::temp_dir()
                .join(format!("baton-program-{name}-{}", process::id()));
            drop(fs::remove_dir_all(&pin_directory));
            fs::create_dir_all(&pin_directory).unwrap();
            pin_directory
        }

        pub(crate) fn program_factory(
            pin_directory: &Path,
            device_kernel: FixtureDeviceKernel,
        ) -> DeviceProgram<FixtureDeviceKernel> {
            let program_configuration = ProgramConfiguration::from((
                PathBuf::from(pin_directory),
                PathBuf::from(pin_directory).join("pins"),
                16,
            ));
            DeviceProgram::from((program_configuration, device_kernel))
        }

        pub(crate) fn device_file_factory(kind: DeviceKind, major: u32, minor: u32) -> DeviceFile {
            DeviceFile::from((kind, major, minor))
        }
    }

    use fixture::Operation;

    #[test]
    fn an_install_creates_the_maps_loads_the_program_attaches_it_and_pins_everything() {
        let pin_directory = fixture::pin_directory_factory("install");
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel::default(),
        );

        device_program.try_install_program().unwrap();

        let operations = device_program
            .device_kernel
            .operations
            .lock()
            .unwrap()
            .clone();
        assert_eq!(
            operations,
            [
                Operation::TryCreateHashMap,
                Operation::TryFastenObject,
                Operation::TryCreateHashMap,
                Operation::TryFastenObject,
                Operation::TryLoadDeviceProgram,
                Operation::TryAttachDeviceProgram,
                Operation::TryFastenObject,
            ]
        );
        let maps = device_program.device_kernel.maps.lock().unwrap().clone();
        assert_eq!(maps, [(12, 4, 1024), (16, 1, 4096)]);
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn an_install_of_a_present_program_does_nothing() {
        let pin_directory = fixture::pin_directory_factory("present");
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel::default(),
        );
        fs::create_dir_all(pin_directory.join("pins")).unwrap();
        fs::write(pin_directory.join("pins/program"), "").unwrap();

        device_program.try_install_program().unwrap();

        assert!(
            device_program
                .device_kernel
                .operations
                .lock()
                .unwrap()
                .is_empty()
        );
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn the_device_files_are_inserted_and_deleted_through_the_map_of_the_devices() {
        let pin_directory = fixture::pin_directory_factory("devices");
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel::default(),
        );
        let device_files = [
            fixture::device_file_factory(DeviceKind::CharacterDevice, 195, 1),
            fixture::device_file_factory(DeviceKind::BlockDevice, 8, 0),
        ];

        device_program
            .try_insert_device_files(3, &device_files)
            .unwrap();
        device_program
            .try_delete_device_files(&device_files)
            .unwrap();
        device_program.try_insert_device_files(3, &[]).unwrap();
        device_program.try_delete_device_files(&[]).unwrap();

        let operations = device_program
            .device_kernel
            .operations
            .lock()
            .unwrap()
            .clone();
        assert_eq!(
            operations,
            [
                Operation::TryOpenObject,
                Operation::TryUpdateElement,
                Operation::TryUpdateElement,
                Operation::TryOpenObject,
                Operation::TryDeleteElement,
                Operation::TryDeleteElement,
            ]
        );
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn a_cgroup_is_permitted_and_revoked_through_the_map_of_the_permissions() {
        let pin_directory = fixture::pin_directory_factory("permissions");
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel::default(),
        );

        device_program.try_permit_group(3, 347348).unwrap();
        device_program.try_revoke_group(3, 347348).unwrap();

        let operations = device_program
            .device_kernel
            .operations
            .lock()
            .unwrap()
            .clone();
        assert_eq!(
            operations,
            [
                Operation::TryOpenObject,
                Operation::TryUpdateElement,
                Operation::TryOpenObject,
                Operation::TryDeleteElement,
            ]
        );
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn a_removal_detaches_the_program_and_removes_the_pins() {
        let pin_directory = fixture::pin_directory_factory("removal");
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel::default(),
        );
        let pin_path = pin_directory.join("pins");
        fs::create_dir_all(&pin_path).unwrap();
        for pin_name in ["devices", "permissions", "program"] {
            fs::write(pin_path.join(pin_name), "").unwrap();
        }

        device_program.try_remove_program().unwrap();

        let operations = device_program
            .device_kernel
            .operations
            .lock()
            .unwrap()
            .clone();
        assert_eq!(
            operations,
            [Operation::TryOpenObject, Operation::TryDetachDeviceProgram]
        );
        assert!(!pin_path.join("program").exists());
        assert!(!pin_path.join("devices").exists());
        device_program.try_remove_program().unwrap();
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn a_deletion_of_an_absent_key_is_no_error_but_a_refused_call_is() {
        let pin_directory = fixture::pin_directory_factory("failure");
        let device_kernel = fixture::FixtureDeviceKernel {
            absence: true,
            ..fixture::FixtureDeviceKernel::default()
        };
        let device_program = fixture::program_factory(&pin_directory, device_kernel);
        let device_files = [fixture::device_file_factory(
            DeviceKind::CharacterDevice,
            195,
            1,
        )];

        let outcome = device_program.try_delete_device_files(&device_files);

        assert_eq!(outcome, Ok(()));
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel {
                rejection_operation: Some(Operation::TryCreateHashMap),
                ..fixture::FixtureDeviceKernel::default()
            },
        );
        assert!(matches!(
            device_program.try_install_program(),
            Err(Error::Call { .. })
        ));
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn a_missing_hierarchy_fails_the_install_and_a_missing_bpffs_fails_the_pin() {
        let pin_directory = fixture::pin_directory_factory("broken");
        let program_configuration = ProgramConfiguration::from((
            pin_directory.join("absent"),
            pin_directory.join("pins"),
            16,
        ));
        let device_program = DeviceProgram::from((
            program_configuration,
            fixture::FixtureDeviceKernel::default(),
        ));
        fs::write(pin_directory.join("pins"), "").unwrap();

        let outcome = device_program.try_install_program();

        assert!(matches!(outcome, Err(Error::PinDirectory { .. })));
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn every_call_of_a_request_surfaces_its_failure() {
        for position in 0..7 {
            let pin_directory = fixture::pin_directory_factory(&format!("install-{position}"));
            let device_kernel = fixture::FixtureDeviceKernel {
                rejection_position: Some(position),
                ..fixture::FixtureDeviceKernel::default()
            };
            let device_program = fixture::program_factory(&pin_directory, device_kernel);

            let outcome = device_program.try_install_program();

            assert!(
                matches!(outcome, Err(Error::Call { .. })),
                "call {position}"
            );
            fs::remove_dir_all(pin_directory).unwrap();
        }
        for (operation, operation_text) in [
            (Operation::TryOpenObject, "insert"),
            (Operation::TryUpdateElement, "insert"),
            (Operation::TryOpenObject, "delete"),
            (Operation::TryDeleteElement, "delete"),
            (Operation::TryOpenObject, "permit"),
            (Operation::TryUpdateElement, "permit"),
            (Operation::TryOpenObject, "revoke"),
            (Operation::TryDeleteElement, "revoke"),
        ] {
            let pin_directory = fixture::pin_directory_factory(&format!(
                "{operation_text}-{operation:?}"
            ));
            let device_kernel = fixture::FixtureDeviceKernel {
                rejection_operation: Some(operation),
                ..fixture::FixtureDeviceKernel::default()
            };
            let device_program = fixture::program_factory(&pin_directory, device_kernel);
            let device_files = [fixture::device_file_factory(
                DeviceKind::CharacterDevice,
                195,
                1,
            )];

            let outcome = match operation_text {
                "insert" => device_program.try_insert_device_files(3, &device_files),
                "delete" => device_program.try_delete_device_files(&device_files),
                "permit" => device_program.try_permit_group(3, 7),
                _ => device_program.try_revoke_group(3, 7),
            };

            assert!(
                matches!(outcome, Err(Error::Call { .. })),
                "{operation_text} {operation:?}"
            );
            fs::remove_dir_all(pin_directory).unwrap();
        }
    }

    #[test]
    fn a_broken_pin_directory_and_a_missing_hierarchy_fail_the_install() {
        let pin_directory = fixture::pin_directory_factory("paths");
        fs::write(pin_directory.join("blocker"), "").unwrap();
        let program_configuration = ProgramConfiguration::from((
            PathBuf::from(&pin_directory),
            pin_directory.join("blocker/pins"),
            16,
        ));
        let device_program = DeviceProgram::from((
            program_configuration,
            fixture::FixtureDeviceKernel::default(),
        ));

        let outcome = device_program.try_install_program();

        assert!(matches!(outcome, Err(Error::PinDirectory { .. })));
        let program_configuration = ProgramConfiguration::from((
            pin_directory.join("absent"),
            pin_directory.join("pins"),
            16,
        ));
        let device_program = DeviceProgram::from((
            program_configuration,
            fixture::FixtureDeviceKernel::default(),
        ));

        let outcome = device_program.try_install_program();

        assert!(matches!(outcome, Err(Error::Hierarchy { .. })));
        let pin_directory = fixture::pin_directory_factory("readonly");
        let mut permissions = fs::metadata(&pin_directory).unwrap().permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&pin_directory, permissions).unwrap();
        let program_configuration = ProgramConfiguration::from((
            PathBuf::from(&pin_directory),
            pin_directory.join("pins"),
            16,
        ));
        let device_program = DeviceProgram::from((
            program_configuration,
            fixture::FixtureDeviceKernel::default(),
        ));
        let outcome = device_program.try_install_program();
        assert!(matches!(outcome, Err(Error::PinDirectory { .. })));
        let mut permissions = fs::metadata(&pin_directory).unwrap().permissions();
        permissions.set_readonly(false);
        fs::set_permissions(&pin_directory, permissions).unwrap();
        fs::remove_dir_all(pin_directory).unwrap();
        let pin_directory = fixture::pin_directory_factory("notadir");
        fs::write(pin_directory.join("blocker"), "").unwrap();
        let program_configuration = ProgramConfiguration::from((
            PathBuf::from(&pin_directory),
            pin_directory.join("blocker/pins"),
            16,
        ));
        let device_program = DeviceProgram::from((
            program_configuration,
            fixture::FixtureDeviceKernel::default(),
        ));
        assert!(matches!(
            device_program.try_install_program(),
            Err(Error::PinDirectory { .. })
        ));
        assert!(matches!(
            device_program.try_remove_program(),
            Err(Error::PinDirectory { .. })
        ));
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn a_removal_surfaces_a_failed_detach_and_a_pin_that_cannot_be_removed() {
        let pin_directory = fixture::pin_directory_factory("remove-detach");
        let pin_path = pin_directory.join("pins");
        fs::create_dir_all(&pin_path).unwrap();
        fs::write(pin_path.join("program"), "").unwrap();
        let device_kernel = fixture::FixtureDeviceKernel {
            rejection_operation: Some(Operation::TryDetachDeviceProgram),
            ..fixture::FixtureDeviceKernel::default()
        };
        let device_program = fixture::program_factory(&pin_directory, device_kernel);

        let outcome = device_program.try_remove_program();

        assert!(matches!(outcome, Err(Error::Call { .. })));
        let pin_directory = fixture::pin_directory_factory("remove-get");
        let pin_path = pin_directory.join("pins");
        fs::create_dir_all(&pin_path).unwrap();
        fs::write(pin_path.join("program"), "").unwrap();
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel {
                rejection_operation: Some(Operation::TryOpenObject),
                ..fixture::FixtureDeviceKernel::default()
            },
        );
        assert!(matches!(
            device_program.try_remove_program(),
            Err(Error::Call { .. })
        ));
        fs::remove_dir_all(pin_directory).unwrap();
        let pin_directory = fixture::pin_directory_factory("remove-hierarchy");
        let pin_path = pin_directory.join("pins");
        fs::create_dir_all(&pin_path).unwrap();
        fs::write(pin_path.join("program"), "").unwrap();
        let program_configuration = ProgramConfiguration::from((
            pin_directory.join("absent"),
            pin_path.clone(),
            16,
        ));
        let device_program = DeviceProgram::from((
            program_configuration,
            fixture::FixtureDeviceKernel::default(),
        ));
        assert!(matches!(
            device_program.try_remove_program(),
            Err(Error::Hierarchy { .. })
        ));
        fs::remove_dir_all(pin_directory).unwrap();
        let pin_directory = fixture::pin_directory_factory("remove-pin");
        let pin_path = pin_directory.join("pins");
        fs::create_dir_all(&pin_path).unwrap();
        fs::write(pin_path.join("program"), "").unwrap();
        fs::create_dir_all(pin_path.join("permissions/holder")).unwrap();
        let device_program = fixture::program_factory(
            &pin_directory,
            fixture::FixtureDeviceKernel::default(),
        );

        let outcome = device_program.try_remove_program();

        assert!(matches!(outcome, Err(Error::PinDirectory { .. })));
        fs::remove_dir_all(pin_directory).unwrap();
    }

    #[test]
    fn the_keys_of_the_maps_hold_the_device_and_the_cgroup() {
        let device_key = [
            fixture::device_file_factory(DeviceKind::CharacterDevice, 195, 1),
            fixture::device_file_factory(DeviceKind::BlockDevice, 8, 0),
        ]
        .map(|device_file| {
            DeviceProgram::<fixture::FixtureDeviceKernel>::compose_device_key(&device_file)
        });
        let permission_key = DeviceProgram::<fixture::FixtureDeviceKernel>::compose_permission_key(
            347348, 3,
        );

        assert_eq!(device_key[0], [2, 0, 0, 0, 195, 0, 0, 0, 1, 0, 0, 0]);
        assert_eq!(device_key[1], [1, 0, 0, 0, 8, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(
            permission_key,
            [212, 76, 5, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0]
        );
    }
}
