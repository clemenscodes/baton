use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TaskConfiguration {
    task_directory: PathBuf,
}

impl Layer for TaskConfiguration {
    type Layer = InfrastructureLayer;
}

impl TaskConfiguration {
    pub fn task_directory(&self) -> &Path {
        &self.task_directory
    }
}

impl From<PathBuf> for TaskConfiguration {
    fn from(task_directory: PathBuf) -> Self {
        Self { task_directory }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_directory_of_processes() {
        let task_configuration = TaskConfiguration::from(PathBuf::from("/proc"));

        assert_eq!(task_configuration.task_directory(), Path::new("/proc"));
    }
}
