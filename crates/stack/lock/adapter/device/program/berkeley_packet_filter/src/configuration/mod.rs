use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramConfiguration {
    hierarchy_directory: PathBuf,
    pin_directory: PathBuf,
    ancestry_depth: u32,
}

impl Layer for ProgramConfiguration {
    type Layer = InfrastructureLayer;
}

impl ProgramConfiguration {
    pub fn hierarchy_directory(&self) -> &Path {
        &self.hierarchy_directory
    }

    pub fn pin_directory(&self) -> &Path {
        &self.pin_directory
    }

    pub fn ancestry_depth(&self) -> u32 {
        self.ancestry_depth
    }
}

impl From<(PathBuf, PathBuf, u32)> for ProgramConfiguration {
    fn from((hierarchy_directory, pin_directory, ancestry_depth): (PathBuf, PathBuf, u32)) -> Self {
        Self {
            hierarchy_directory,
            pin_directory,
            ancestry_depth,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_directories_and_its_depth() {
        let program_configuration = ProgramConfiguration::from((
            PathBuf::from("/sys/fs/cgroup"),
            PathBuf::from("/sys/fs/bpf/baton"),
            16,
        ));

        assert_eq!(
            program_configuration.hierarchy_directory(),
            Path::new("/sys/fs/cgroup")
        );
        assert_eq!(
            program_configuration.pin_directory(),
            Path::new("/sys/fs/bpf/baton")
        );
        assert_eq!(program_configuration.ancestry_depth(), 16);
    }
}
