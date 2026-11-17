mod configuration;
mod error;
mod lock_document;
mod record_lease;

pub use configuration::RecordConfiguration;
pub use error::Error;
pub use record_lease::RecordLease;

use crate::lock_document::LockDocument;
use rustix::fs::FlockOperation;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::lock::record::LockRecord as LockRecordPort;
use stack_lock_core_graphics_lock::GraphicsLock;
use stack_lock_core_name::LockName;
use std::fs::{self, File, OpenOptions};
use std::io;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LockRecord {
    record_configuration: RecordConfiguration,
}

impl Layer for LockRecord {
    type Layer = InfrastructureLayer;
}

impl Adapter for LockRecord {}

impl Port for LockRecord {}

impl DrivenPort for LockRecord {}

impl From<RecordConfiguration> for LockRecord {
    fn from(record_configuration: RecordConfiguration) -> Self {
        Self {
            record_configuration,
        }
    }
}

impl LockRecordPort for LockRecord {
    type Error = Error;
    type RecordLease = RecordLease;

    fn try_lease_records(&self) -> Result<RecordLease, Error> {
        let record_directory = self.record_configuration.record_directory();
        let lease_path = record_directory.join(Self::LEASE_NAME);
        fs::create_dir_all(record_directory)
            .and_then(|()| Self::try_open_lease(&lease_path))
            .map(RecordLease::from)
            .map_err(|error| {
                let text = error.to_string();
                Error::LeaseFailure {
                    path: lease_path.clone(),
                    text,
                }
            })
    }

    fn try_load_lock(&self, lock_name: &LockName) -> Result<Option<GraphicsLock>, Error> {
        let record_path = self.record_path(lock_name.text());
        match fs::read_to_string(&record_path) {
            Ok(record_text) => Self::try_parse_lock(&record_path, &record_text).map(Some),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(None),
            Err(error) => Err(Self::compose_input_error(&record_path, &error)),
        }
    }

    fn try_load_locks(&self) -> Result<Vec<GraphicsLock>, Error> {
        let record_directory = self.record_configuration.record_directory();
        let record_paths = match fs::read_dir(record_directory) {
            Ok(entries) => Vec::from_iter(entries.filter_map(Result::ok).map(|entry| entry.path())),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Vec::new(),
            Err(error) => return Err(Self::compose_input_error(record_directory, &error)),
        };
        let mut record_paths = Vec::from_iter(record_paths.into_iter().filter(|record_path| {
            record_path
                .extension()
                .is_some_and(|extension| extension == Self::RECORD_EXTENSION)
        }));
        record_paths.sort();
        let mut graphics_locks = Vec::new();
        for record_path in record_paths {
            let record_text = fs::read_to_string(&record_path)
                .map_err(|error| Self::compose_input_error(&record_path, &error))?;
            graphics_locks.push(Self::try_parse_lock(&record_path, &record_text)?);
        }
        Ok(graphics_locks)
    }

    fn try_store_lock(&self, graphics_lock: &GraphicsLock) -> Result<(), Error> {
        let record_path = self.record_path(graphics_lock.lock_name().text());
        let transfer_path = record_path.with_extension(Self::TRANSFER_EXTENSION);
        let lock_document = LockDocument::from(graphics_lock);
        let record_text = serde_json::to_string_pretty(&lock_document).unwrap_or_default();
        fs::write(&transfer_path, record_text)
            .and_then(|()| fs::rename(&transfer_path, &record_path))
            .map_err(|error| {
                let text = error.to_string();
                Error::Output {
                    path: record_path.clone(),
                    text,
                }
            })
    }

    fn try_delete_lock(&self, lock_name: &LockName) -> Result<(), Error> {
        let record_path = self.record_path(lock_name.text());
        match fs::remove_file(&record_path) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => {
                let text = error.to_string();
                Err(Error::Output {
                    path: record_path,
                    text,
                })
            }
            _ => Ok(()),
        }
    }
}

impl LockRecord {
    const LEASE_NAME: &'static str = "lease";
    const RECORD_EXTENSION: &'static str = "json";
    const TRANSFER_EXTENSION: &'static str = "transfer";

    fn record_path(&self, lock_text: &str) -> PathBuf {
        let record_name = format!("{lock_text}.{}", Self::RECORD_EXTENSION);
        self.record_configuration
            .record_directory()
            .join(record_name)
    }

    fn try_open_lease(lease_path: &Path) -> io::Result<File> {
        OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(lease_path)
            .and_then(|lease_file| {
                rustix::fs::flock(&lease_file, FlockOperation::LockExclusive)
                    .map(|()| lease_file)
                    .map_err(io::Error::from)
            })
    }

    fn try_parse_lock(record_path: &Path, record_text: &str) -> Result<GraphicsLock, Error> {
        serde_json::from_str::<LockDocument>(record_text)
            .map_err(|error| error.to_string())
            .and_then(GraphicsLock::try_from)
            .map_err(|text| Error::Document {
                path: PathBuf::from(record_path),
                text,
            })
    }

    fn compose_input_error(path: &Path, error: &io::Error) -> Error {
        let text = error.to_string();
        Error::Input {
            path: PathBuf::from(path),
            text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stack_lock_core_demand::GraphicsDemand;
    use stack_lock_core_device_address::DeviceAddress;
    use stack_lock_core_device_file::{DeviceFile, DeviceKind};
    use stack_lock_core_graphics_user::GraphicsUser;
    use stack_lock_core_name::ApplicationName;
    use std::env;
    use std::process;

    mod fixture {
        use super::*;

        pub(crate) fn record_directory_factory(name: &str) -> PathBuf {
            let record_directory = env::temp_dir()
                .join(format!("baton-record-{name}-{}", process::id()));
            drop(fs::remove_dir_all(&record_directory));
            record_directory
        }

        pub(crate) fn lock_record_factory(record_directory: &Path) -> LockRecord {
            LockRecord::from(RecordConfiguration::from(PathBuf::from(record_directory)))
        }

        pub(crate) fn application_name_factory(text: &str) -> ApplicationName {
            ApplicationName::try_from(String::from(text)).unwrap()
        }

        pub(crate) fn graphics_lock_factory(lock_text: &str) -> GraphicsLock {
            let lock_name = LockName::try_from(String::from(lock_text)).unwrap();
            let mut graphics_lock = GraphicsLock::from((lock_name, 2));
            let device_address = DeviceAddress::try_from(String::from("0000:03:00.0")).unwrap();
            graphics_lock.try_add_device(device_address).unwrap();
            graphics_lock.replace_device_files(vec![
                DeviceFile::from((DeviceKind::CharacterDevice, 195, 1)),
                DeviceFile::from((DeviceKind::BlockDevice, 8, 0)),
            ]);
            graphics_lock
                .try_admit(application_name_factory("desktop"), true)
                .unwrap();
            graphics_lock
                .try_admit(application_name_factory("winix"), false)
                .unwrap();
            let graphics_demand = GraphicsDemand::from((application_name_factory("desktop"), 9));
            graphics_lock.try_record_demand(graphics_demand).unwrap();
            graphics_lock
                .try_recover_owner(application_name_factory("winix"))
                .unwrap();
            graphics_lock.record_console_users(vec![
                GraphicsUser::ConsoleUser { console_index: 1 },
                GraphicsUser::FramebufferUser {
                    device_text: String::from("efi-framebuffer.0"),
                    driver_text: String::from("efi-framebuffer"),
                },
                GraphicsUser::ProcessUser {
                    process_identifier: 7,
                    command_text: String::from("render"),
                    control_group_text: String::from("/user.slice"),
                },
            ]);
            graphics_lock.record_passthrough(true);
            graphics_lock
        }
    }

    #[test]
    fn a_stored_lock_loads_again_with_every_part() {
        let record_directory = fixture::record_directory_factory("round");
        let lock_record = fixture::lock_record_factory(&record_directory);
        let graphics_lock = fixture::graphics_lock_factory("rtx3080");
        let record_lease = lock_record.try_lease_records().unwrap();

        lock_record.try_store_lock(&graphics_lock).unwrap();
        let maybe_graphics_lock = lock_record
            .try_load_lock(graphics_lock.lock_name())
            .unwrap();

        assert_eq!(maybe_graphics_lock, Some(graphics_lock));
        assert!(record_directory.join("lease").is_file());
        assert!(record_lease.lease_file().metadata().is_ok());
        fs::remove_dir_all(record_directory).unwrap();
    }

    #[test]
    fn every_lock_loads_in_the_order_of_its_name_and_a_deletion_removes_it() {
        let record_directory = fixture::record_directory_factory("order");
        let lock_record = fixture::lock_record_factory(&record_directory);
        let mut graphics_lock = [
            fixture::graphics_lock_factory("quadro"),
            fixture::graphics_lock_factory("arc"),
        ];
        graphics_lock[1].clear_passthrough();
        assert_eq!(lock_record.try_load_locks(), Ok(Vec::new()));
        lock_record.try_lease_records().unwrap();
        lock_record.try_store_lock(&graphics_lock[0]).unwrap();
        lock_record.try_store_lock(&graphics_lock[1]).unwrap();
        fs::write(record_directory.join("notes.text"), "").unwrap();

        let graphics_locks = lock_record.try_load_locks().unwrap();
        lock_record
            .try_delete_lock(graphics_lock[0].lock_name())
            .unwrap();
        lock_record
            .try_delete_lock(graphics_lock[0].lock_name())
            .unwrap();

        assert_eq!(
            graphics_locks,
            [graphics_lock[1].clone(), graphics_lock[0].clone()]
        );
        assert_eq!(
            lock_record.try_load_lock(graphics_lock[0].lock_name()),
            Ok(None)
        );
        fs::remove_dir_all(record_directory).unwrap();
    }

    #[test]
    fn a_directory_in_the_place_of_a_file_fails_every_request() {
        let record_directory = fixture::record_directory_factory("blocked");
        fs::create_dir_all(record_directory.parent().unwrap()).unwrap();
        fs::write(&record_directory, "").unwrap();
        let lock_record = fixture::lock_record_factory(&record_directory);
        let graphics_lock = fixture::graphics_lock_factory("rtx3080");

        let outcome = [
            lock_record.try_lease_records().map(|_| ()),
            lock_record.try_store_lock(&graphics_lock),
            lock_record.try_load_locks().map(|_| ()),
            lock_record
                .try_load_lock(graphics_lock.lock_name())
                .map(|_| ()),
        ];

        assert!(matches!(outcome[0], Err(Error::LeaseFailure { .. })));
        assert!(matches!(outcome[1], Err(Error::Output { .. })));
        assert!(matches!(outcome[2], Err(Error::Input { .. })));
        assert!(matches!(outcome[3], Err(Error::Input { .. })));
        fs::remove_file(record_directory).unwrap();
    }

    #[test]
    fn a_record_that_is_a_directory_fails_its_load_and_its_deletion() {
        let record_directory = fixture::record_directory_factory("directory");
        let lock_record = fixture::lock_record_factory(&record_directory);
        let graphics_lock = fixture::graphics_lock_factory("rtx3080");
        fs::create_dir_all(record_directory.join("rtx3080.json").join("inner")).unwrap();

        let outcome = [
            lock_record.try_load_locks().map(|_| ()),
            lock_record.try_delete_lock(graphics_lock.lock_name()),
        ];

        assert!(matches!(outcome[0], Err(Error::Input { .. })));
        assert!(matches!(outcome[1], Err(Error::Output { .. })));
        fs::remove_dir_all(record_directory).unwrap();
    }

    #[test]
    fn a_record_that_holds_no_lock_is_an_error_of_its_document() {
        let record_directory = fixture::record_directory_factory("document");
        let lock_record = fixture::lock_record_factory(&record_directory);
        let graphics_lock = fixture::graphics_lock_factory("rtx3080");
        lock_record.try_lease_records().unwrap();
        lock_record.try_store_lock(&graphics_lock).unwrap();
        let record_path = record_directory.join("rtx3080.json");
        let record_text = fs::read_to_string(&record_path).unwrap();
        let lock_document = [
            record_text.replacen("\"rtx3080\"", "\"--lock\"", 1),
            record_text.replacen("\"0000:03:00.0\"", "\"03:00\"", 1),
            record_text.replacen("\"desktop\",", "\"--desktop\",", 1),
            record_text.replacen(
                "\"application_name\": \"desktop\"",
                "\"application_name\": \"--desktop\"",
                1,
            ),
            record_text.replacen(
                "\"maybe_owner\": \"winix\"",
                "\"maybe_owner\": \"--winix\"",
                1,
            ),
            record_text.replacen("{", "[", 1),
        ];

        let outcome = lock_document.map(|record_text| {
            fs::write(&record_path, record_text).unwrap();
            lock_record.try_load_lock(graphics_lock.lock_name())
        });
        fs::write(&record_path, "[").unwrap();
        let lock_outcome = lock_record.try_load_locks();

        assert!(
            outcome
                .iter()
                .all(|outcome| matches!(outcome, Err(Error::Document { .. })))
        );
        assert!(matches!(lock_outcome, Err(Error::Document { .. })));
        fs::remove_dir_all(record_directory).unwrap();
    }
}
