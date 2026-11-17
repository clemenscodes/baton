use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RecordConfiguration {
    record_directory: PathBuf,
}

impl Layer for RecordConfiguration {
    type Layer = InfrastructureLayer;
}

impl RecordConfiguration {
    pub fn record_directory(&self) -> &Path {
        &self.record_directory
    }
}

impl From<PathBuf> for RecordConfiguration {
    fn from(record_directory: PathBuf) -> Self {
        Self { record_directory }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_directory() {
        let record_configuration = RecordConfiguration::from(PathBuf::from("/run/baton"));

        assert_eq!(
            record_configuration.record_directory(),
            Path::new("/run/baton")
        );
    }
}
