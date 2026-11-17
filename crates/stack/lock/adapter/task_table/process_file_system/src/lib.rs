mod configuration;
mod error;
mod signal_call;
mod system_signal;

pub use configuration::TaskConfiguration;
pub use error::Error;
pub use signal_call::{SignalCall, SignalKind};
pub use system_signal::SystemSignal;

use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::task_table::TaskTable as TaskTablePort;
use stack_lock_core_device_file::{DeviceFile, DeviceKind};
use stack_lock_core_graphics_user::GraphicsUser;
use std::fs;
use std::os::unix::fs::{FileTypeExt, MetadataExt};
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskTable<SignalCallType> {
    task_configuration: TaskConfiguration,
    signal_call: SignalCallType,
}

impl<SignalCallType> Layer for TaskTable<SignalCallType> {
    type Layer = InfrastructureLayer;
}

impl<SignalCallType> Adapter for TaskTable<SignalCallType> {}

impl<SignalCallType> Port for TaskTable<SignalCallType> {}

impl<SignalCallType> DrivenPort for TaskTable<SignalCallType> {}

impl<SignalCallType> From<(TaskConfiguration, SignalCallType)> for TaskTable<SignalCallType> {
    fn from((task_configuration, signal_call): (TaskConfiguration, SignalCallType)) -> Self {
        Self {
            task_configuration,
            signal_call,
        }
    }
}

impl<SignalCallType: SignalCall> TaskTablePort for TaskTable<SignalCallType> {
    type Error = Error;

    fn try_scan_process_users(
        &self,
        device_files: &[DeviceFile],
    ) -> Result<Vec<GraphicsUser>, Error> {
        let task_directory = self.task_configuration.task_directory();
        let entries = fs::read_dir(task_directory).map_err(|error| {
            let text = error.to_string();
            Error::ScanFailure { text }
        })?;
        let mut process_identifiers = Vec::from_iter(entries.filter_map(Result::ok).filter_map(
            |entry| {
                entry
                    .file_name()
                    .to_str()
                    .and_then(|name_text| name_text.parse::<u32>().ok())
            },
        ));
        process_identifiers.sort_unstable();
        let process_users = process_identifiers
            .into_iter()
            .filter_map(|process_identifier| {
                let process_path = task_directory.join(process_identifier.to_string());
                let holder_presence = Self::check_holder(&process_path, device_files);
                holder_presence.then(|| {
                    let command_text = Self::read_text(&process_path.join(Self::COMMAND_NAME));
                    let group_text = fs::read_to_string(process_path.join(Self::GROUP_NAME))
                        .unwrap_or_default();
                    let control_group_text = String::from(
                        group_text
                            .lines()
                            .find_map(|group_line| group_line.strip_prefix(Self::GROUP_PREFIX))
                            .unwrap_or_default()
                            .trim(),
                    );
                    GraphicsUser::ProcessUser {
                        process_identifier,
                        command_text,
                        control_group_text,
                    }
                })
            });
        Ok(Vec::from_iter(process_users))
    }

    fn check_process_presence(&self, process_identifier: u32) -> bool {
        let task_directory = self.task_configuration.task_directory();
        task_directory.join(process_identifier.to_string()).is_dir()
    }

    fn try_request_end(&self, process_identifier: u32) -> Result<(), Error> {
        self.try_signal_process(process_identifier, SignalKind::Termination)
    }

    fn try_force_end(&self, process_identifier: u32) -> Result<(), Error> {
        self.try_signal_process(process_identifier, SignalKind::Kill)
    }
}

impl<SignalCallType: SignalCall> TaskTable<SignalCallType> {
    const COMMAND_NAME: &'static str = "comm";
    const GROUP_NAME: &'static str = "cgroup";
    const DESCRIPTOR_NAME: &'static str = "fd";
    const GROUP_PREFIX: &'static str = "0::";
    const ABSENCE_CODE: i32 = 3;

    fn check_holder(process_path: &Path, device_files: &[DeviceFile]) -> bool {
        let descriptor_path = process_path.join(Self::DESCRIPTOR_NAME);
        let Ok(entries) = fs::read_dir(descriptor_path) else {
            return false;
        };
        entries.filter_map(Result::ok).any(|entry| {
            fs::metadata(entry.path()).is_ok_and(|metadata| {
                let file_type = metadata.file_type();
                let maybe_device_kind = file_type
                    .is_char_device()
                    .then_some(DeviceKind::CharacterDevice);
                maybe_device_kind.is_some_and(|device_kind| {
                    let device_identity = metadata.rdev();
                    let major = rustix::fs::major(device_identity);
                    let minor = rustix::fs::minor(device_identity);
                    device_files.contains(&DeviceFile::from((device_kind, major, minor)))
                })
            })
        })
    }

    fn read_text(path: &Path) -> String {
        let text = fs::read_to_string(path).unwrap_or_default();
        String::from(text.lines().next().unwrap_or_default().trim())
    }

    fn try_signal_process(
        &self,
        process_identifier: u32,
        signal_kind: SignalKind,
    ) -> Result<(), Error> {
        match self
            .signal_call
            .try_send_signal(process_identifier, signal_kind)
        {
            Err(error) if error.raw_os_error() != Some(Self::ABSENCE_CODE) => {
                let text = error.to_string();
                Err(Error::SignalFailure {
                    process_identifier,
                    text,
                })
            }
            _ => Ok(()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::io;
    use std::os::unix::fs::symlink;
    use std::path::PathBuf;
    use std::process;
    use std::sync::Mutex;

    mod fixture {
        use super::*;

        #[derive(Debug, Default)]
        pub(crate) struct FixtureSignalCall {
            pub(crate) signal_kinds: Mutex<Vec<(u32, SignalKind)>>,
        }

        impl SignalCall for FixtureSignalCall {
            fn try_send_signal(
                &self,
                process_identifier: u32,
                signal_kind: SignalKind,
            ) -> io::Result<()> {
                self.signal_kinds
                    .lock()
                    .unwrap()
                    .push((process_identifier, signal_kind));
                match process_identifier {
                    1 => Err(io::Error::from_raw_os_error(1)),
                    9 => Err(io::Error::from_raw_os_error(3)),
                    _ => Ok(()),
                }
            }
        }

        pub(crate) fn task_directory_factory(name: &str) -> PathBuf {
            let task_directory = env::temp_dir()
                .join(format!("baton-process-{name}-{}", process::id()));
            drop(fs::remove_dir_all(&task_directory));
            fs::create_dir_all(&task_directory).unwrap();
            task_directory
        }

        pub(crate) fn process_factory(
            task_directory: &Path,
            process_identifier: u32,
            command_text: &str,
            group_text: &str,
            entries: &[&str],
        ) {
            let process_path = task_directory.join(process_identifier.to_string());
            let descriptor_path = process_path.join("fd");
            fs::create_dir_all(&descriptor_path).unwrap();
            fs::write(process_path.join("comm"), format!("{command_text}\n")).unwrap();
            fs::write(process_path.join("cgroup"), format!("{group_text}\n")).unwrap();
            for (entry, path) in entries.iter().enumerate() {
                symlink(path, descriptor_path.join(entry.to_string())).unwrap();
            }
        }

        pub(crate) fn task_table_factory(task_directory: &Path) -> TaskTable<FixtureSignalCall> {
            TaskTable::from((
                TaskConfiguration::from(PathBuf::from(task_directory)),
                FixtureSignalCall::default(),
            ))
        }
    }

    #[test]
    fn a_scan_names_every_process_that_holds_a_locked_device_file_with_its_command_and_its_cgroup()
    {
        let task_directory = fixture::task_directory_factory("scan");
        fixture::process_factory(
            &task_directory,
            42,
            "resolve",
            "1:net_cls:/\n0::/baton/rtx3080/resolve",
            &["/dev/null", "/dev/zero"],
        );
        fixture::process_factory(
            &task_directory,
            43,
            "bash",
            "0::/user.slice",
            &["/dev/zero", "/missing"],
        );
        fixture::process_factory(&task_directory, 7, "renderer", "", &["/dev/null"]);
        fs::create_dir_all(task_directory.join("self")).unwrap();
        let path = task_directory.join("regular");
        fs::write(&path, "").unwrap();
        fixture::process_factory(
            &task_directory,
            46,
            "cat",
            "0::/user.slice",
            &[path.to_str().unwrap()],
        );
        fs::create_dir_all(task_directory.join("44")).unwrap();
        let task_table = fixture::task_table_factory(&task_directory);
        let device_files = [
            DeviceFile::from((DeviceKind::CharacterDevice, 1, 3)),
            DeviceFile::from((DeviceKind::BlockDevice, 1, 5)),
        ];

        let process_users = task_table.try_scan_process_users(&device_files).unwrap();

        assert_eq!(
            process_users,
            [
                GraphicsUser::ProcessUser {
                    process_identifier: 7,
                    command_text: String::from("renderer"),
                    control_group_text: String::new(),
                },
                GraphicsUser::ProcessUser {
                    process_identifier: 42,
                    command_text: String::from("resolve"),
                    control_group_text: String::from("/baton/rtx3080/resolve"),
                },
            ]
        );
        assert!(task_table.check_process_presence(42));
        assert!(!task_table.check_process_presence(45));
        fs::remove_dir_all(task_directory).unwrap();
    }

    #[test]
    fn a_missing_directory_of_processes_is_an_error_of_the_scan() {
        let task_directory = fixture::task_directory_factory("absence").join("proc");
        let task_table = fixture::task_table_factory(&task_directory);

        let outcome = task_table.try_scan_process_users(&[]);

        assert!(matches!(outcome, Err(Error::ScanFailure { .. })));
    }

    #[test]
    fn an_end_sends_the_signal_and_a_process_that_already_ended_is_no_error() {
        let task_directory = fixture::task_directory_factory("signal");
        let task_table = fixture::task_table_factory(&task_directory);

        let outcome = [
            task_table.try_request_end(42),
            task_table.try_force_end(42),
            task_table.try_request_end(9),
            task_table.try_force_end(1),
        ];

        assert_eq!(
            outcome,
            [
                Ok(()),
                Ok(()),
                Ok(()),
                Err(Error::SignalFailure {
                    process_identifier: 1,
                    text: io::Error::from_raw_os_error(1).to_string(),
                }),
            ]
        );
        assert_eq!(
            *task_table.signal_call.signal_kinds.lock().unwrap(),
            [
                (42, SignalKind::Termination),
                (42, SignalKind::Kill),
                (9, SignalKind::Termination),
                (1, SignalKind::Kill)
            ]
        );
        fs::remove_dir_all(task_directory).unwrap();
    }
}
