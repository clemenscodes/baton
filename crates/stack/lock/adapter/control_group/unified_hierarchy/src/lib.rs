mod configuration;
mod error;

pub use configuration::GroupConfiguration;
pub use error::Error;

use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::control_group::ControlGroup as ControlGroupPort;
use std::fs;
use std::io;
use std::os::unix::fs::MetadataExt;
use std::path::PathBuf;
use std::process;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ControlGroup {
    group_configuration: GroupConfiguration,
}

impl Layer for ControlGroup {
    type Layer = InfrastructureLayer;
}

impl Adapter for ControlGroup {}

impl Port for ControlGroup {}

impl DrivenPort for ControlGroup {}

impl From<GroupConfiguration> for ControlGroup {
    fn from(group_configuration: GroupConfiguration) -> Self {
        Self {
            group_configuration,
        }
    }
}

impl ControlGroupPort for ControlGroup {
    type Error = Error;

    fn try_create_group(&self, path_text: &str) -> Result<u64, Error> {
        let group_path = self.compose_group_path(path_text);
        fs::create_dir_all(&group_path)
            .and_then(|()| fs::metadata(&group_path))
            .map(|metadata| metadata.ino())
            .map_err(|error| Self::compose_error(group_path.clone(), &error))
    }

    fn try_read_group_identifier(&self, path_text: &str) -> Result<u64, Error> {
        let group_path = self.compose_group_path(path_text);
        fs::metadata(&group_path)
            .map(|metadata| metadata.ino())
            .map_err(|error| Self::compose_error(group_path.clone(), &error))
    }

    fn try_check_population(&self, path_text: &str) -> Result<bool, Error> {
        let events_path = self.compose_group_path(path_text).join(Self::EVENTS_NAME);
        match fs::read_to_string(&events_path) {
            Ok(events_text) => Ok(events_text
                .lines()
                .any(|line| line.trim() == Self::POPULATION_LINE)),
            Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(false),
            Err(error) => Err(Self::compose_error(events_path, &error)),
        }
    }

    fn try_move_process(&self, path_text: &str, process_identifier: u32) -> Result<(), Error> {
        let processes_path = self
            .compose_group_path(path_text)
            .join(Self::PROCESSES_NAME);
        fs::write(&processes_path, process_identifier.to_string())
            .map_err(|error| Self::compose_error(processes_path.clone(), &error))
    }

    fn try_enter_group(&self, path_text: &str) -> Result<(), Error> {
        self.try_move_process(path_text, process::id())
    }

    fn try_remove_group(&self, path_text: &str) -> Result<(), Error> {
        let group_path = self.compose_group_path(path_text);
        match fs::remove_dir(&group_path) {
            Err(error) if error.kind() != io::ErrorKind::NotFound => {
                Err(Self::compose_error(group_path, &error))
            }
            _ => Ok(()),
        }
    }
}

impl ControlGroup {
    const EVENTS_NAME: &'static str = "cgroup.events";
    const PROCESSES_NAME: &'static str = "cgroup.procs";
    const POPULATION_LINE: &'static str = "populated 1";

    fn compose_group_path(&self, path_text: &str) -> PathBuf {
        let relative_text = path_text.trim_start_matches('/');
        self.group_configuration
            .hierarchy_directory()
            .join(relative_text)
    }

    fn compose_error(path: PathBuf, error: &io::Error) -> Error {
        let text = error.to_string();
        Error::Group { path, text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::process;

    mod fixture {
        use super::*;

        pub(crate) fn hierarchy_directory_factory(name: &str) -> PathBuf {
            let hierarchy_directory = env::temp_dir()
                .join(format!("baton-hierarchy-{name}-{}", process::id()));
            drop(fs::remove_dir_all(&hierarchy_directory));
            fs::create_dir_all(&hierarchy_directory).unwrap();
            hierarchy_directory
        }

        pub(crate) fn control_group_factory(hierarchy_directory: &std::path::Path) -> ControlGroup {
            ControlGroup::from(GroupConfiguration::from(PathBuf::from(hierarchy_directory)))
        }
    }

    #[test]
    fn a_created_group_answers_the_inode_of_its_directory_as_its_identifier() {
        let hierarchy_directory = fixture::hierarchy_directory_factory("creation");
        let control_group = fixture::control_group_factory(&hierarchy_directory);

        let group_identifier = control_group
            .try_create_group("/baton/rtx3080/resolve")
            .unwrap();

        let group_path = hierarchy_directory.join("baton/rtx3080/resolve");
        assert_eq!(group_identifier, fs::metadata(&group_path).unwrap().ino());
        assert_eq!(
            control_group.try_read_group_identifier("/baton/rtx3080/resolve"),
            Ok(group_identifier)
        );
        assert_eq!(
            control_group.try_create_group("/baton/rtx3080/resolve"),
            Ok(group_identifier)
        );
        fs::remove_dir_all(hierarchy_directory).unwrap();
    }

    #[test]
    fn a_group_is_populated_only_where_its_events_say_so() {
        let hierarchy_directory = fixture::hierarchy_directory_factory("population");
        let control_group = fixture::control_group_factory(&hierarchy_directory);
        control_group
            .try_create_group("/baton/rtx3080/resolve")
            .unwrap();
        let events_path = hierarchy_directory.join("baton/rtx3080/resolve/cgroup.events");

        let population_presence = control_group.try_check_population("/baton/rtx3080/resolve");
        fs::write(&events_path, "populated 0\nfrozen 0\n").unwrap();
        let population_line = control_group.try_check_population("/baton/rtx3080/resolve");
        fs::write(&events_path, "populated 1\nfrozen 0\n").unwrap();

        assert_eq!(population_presence, Ok(false));
        assert_eq!(population_line, Ok(false));
        assert_eq!(
            control_group.try_check_population("/baton/rtx3080/resolve"),
            Ok(true)
        );
        assert_eq!(
            control_group.try_check_population("/baton/rtx3080/winix"),
            Ok(false)
        );
        fs::remove_dir_all(hierarchy_directory).unwrap();
    }

    #[test]
    fn a_move_writes_the_process_into_the_processes_of_the_group() {
        let hierarchy_directory = fixture::hierarchy_directory_factory("move");
        let control_group = fixture::control_group_factory(&hierarchy_directory);
        control_group
            .try_create_group("/baton/rtx3080/resolve")
            .unwrap();

        control_group
            .try_move_process("/baton/rtx3080/resolve", 4242)
            .unwrap();

        let processes_path = hierarchy_directory.join("baton/rtx3080/resolve/cgroup.procs");
        assert_eq!(fs::read_to_string(processes_path).unwrap(), "4242");
        fs::remove_dir_all(hierarchy_directory).unwrap();
    }

    #[test]
    fn an_entry_writes_the_own_process_into_the_processes_of_the_group() {
        let hierarchy_directory = fixture::hierarchy_directory_factory("entry");
        let control_group = fixture::control_group_factory(&hierarchy_directory);
        control_group.try_create_group("/baton/host").unwrap();

        control_group.try_enter_group("/baton/host").unwrap();

        let processes_path = hierarchy_directory.join("baton/host/cgroup.procs");
        assert_eq!(
            fs::read_to_string(processes_path).unwrap(),
            process::id().to_string()
        );
        fs::remove_dir_all(hierarchy_directory).unwrap();
    }

    #[test]
    fn a_removal_removes_an_empty_group_and_leaves_a_missing_one_alone() {
        let hierarchy_directory = fixture::hierarchy_directory_factory("removal");
        let control_group = fixture::control_group_factory(&hierarchy_directory);
        control_group
            .try_create_group("/baton/rtx3080/resolve")
            .unwrap();

        control_group
            .try_remove_group("/baton/rtx3080/resolve")
            .unwrap();
        control_group
            .try_remove_group("/baton/rtx3080/resolve")
            .unwrap();
        let outcome = control_group.try_remove_group("/baton");

        assert!(!hierarchy_directory.join("baton/rtx3080/resolve").exists());
        assert!(matches!(outcome, Err(Error::Group { .. })));
        fs::remove_dir_all(hierarchy_directory).unwrap();
    }

    #[test]
    fn a_file_in_the_place_of_a_group_fails_every_request() {
        let hierarchy_directory = fixture::hierarchy_directory_factory("file");
        let control_group = fixture::control_group_factory(&hierarchy_directory);
        fs::write(hierarchy_directory.join("baton"), "").unwrap();

        let outcome = [
            control_group.try_create_group("/baton/rtx3080").map(|_| ()),
            control_group
                .try_read_group_identifier("/baton/rtx3080")
                .map(|_| ()),
            control_group
                .try_check_population("/baton/rtx3080")
                .map(|_| ()),
            control_group.try_move_process("/baton/rtx3080", 7),
        ];

        assert!(
            outcome
                .iter()
                .all(|outcome| matches!(outcome, Err(Error::Group { .. })))
        );
        fs::remove_dir_all(hierarchy_directory).unwrap();
    }
}
