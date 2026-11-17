use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::ffi::{OsStr, OsString};
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TreeConfiguration {
    system_directory: PathBuf,
    console_program: OsString,
    persistence_program: OsString,
    binding_deadline: Duration,
    check_interval: Duration,
}

impl Layer for TreeConfiguration {
    type Layer = InfrastructureLayer;
}

impl TreeConfiguration {
    pub fn system_directory(&self) -> &Path {
        &self.system_directory
    }

    pub fn console_program(&self) -> &OsStr {
        &self.console_program
    }

    pub fn persistence_program(&self) -> &OsStr {
        &self.persistence_program
    }

    pub fn binding_deadline(&self) -> Duration {
        self.binding_deadline
    }

    pub fn check_interval(&self) -> Duration {
        self.check_interval
    }
}

impl From<(PathBuf, (OsString, OsString), (Duration, Duration))> for TreeConfiguration {
    fn from(
        (
            system_directory,
            (console_program, persistence_program),
            (binding_deadline, check_interval),
        ): (PathBuf, (OsString, OsString), (Duration, Duration)),
    ) -> Self {
        Self {
            system_directory,
            console_program,
            persistence_program,
            binding_deadline,
            check_interval,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_directory_its_programs_and_its_times() {
        let tree_configuration = TreeConfiguration::from((
            PathBuf::from("/"),
            (OsString::from("con2fbmap"), OsString::from("nvidia-smi")),
            (Duration::from_millis(10000), Duration::from_millis(200)),
        ));

        assert_eq!(tree_configuration.system_directory(), Path::new("/"));
        assert_eq!(tree_configuration.console_program(), "con2fbmap");
        assert_eq!(tree_configuration.persistence_program(), "nvidia-smi");
        assert_eq!(
            tree_configuration.binding_deadline(),
            Duration::from_millis(10000)
        );
        assert_eq!(
            tree_configuration.check_interval(),
            Duration::from_millis(200)
        );
    }
}
