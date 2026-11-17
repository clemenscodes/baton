use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GroupConfiguration {
    hierarchy_directory: PathBuf,
}

impl Layer for GroupConfiguration {
    type Layer = InfrastructureLayer;
}

impl GroupConfiguration {
    pub fn hierarchy_directory(&self) -> &Path {
        &self.hierarchy_directory
    }
}

impl From<PathBuf> for GroupConfiguration {
    fn from(hierarchy_directory: PathBuf) -> Self {
        Self {
            hierarchy_directory,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_hierarchy() {
        let group_configuration = GroupConfiguration::from(PathBuf::from("/sys/fs/cgroup"));

        assert_eq!(
            group_configuration.hierarchy_directory(),
            Path::new("/sys/fs/cgroup")
        );
    }
}
