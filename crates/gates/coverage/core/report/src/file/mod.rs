use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ReportFile {
    path: PathBuf,
    export: Vec<u8>,
}

impl Layer for ReportFile {
    type Layer = DomainLayer;
}

impl ValueObject for ReportFile {}

impl ReportFile {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn export(&self) -> &[u8] {
        &self.export
    }
}

impl From<(PathBuf, Vec<u8>)> for ReportFile {
    fn from((path, export): (PathBuf, Vec<u8>)) -> Self {
        Self { path, export }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_report_file_keeps_its_path_and_its_export() {
        let path = PathBuf::from("target/moon/coverage/harbor.json");
        let export = b"{\"data\":[]}".to_vec();

        let report_file = ReportFile::from((path, export));

        let path_expectation = Path::new("target/moon/coverage/harbor.json");
        let export_expectation = b"{\"data\":[]}";
        assert_eq!(report_file.path(), path_expectation);
        assert_eq!(report_file.export(), export_expectation);
    }
}
