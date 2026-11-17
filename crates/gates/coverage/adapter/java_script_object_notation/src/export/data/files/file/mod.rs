use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
pub(super) struct File {
    filename: PathBuf,
}

impl Layer for File {
    type Layer = InfrastructureLayer;
}

impl File {
    pub(super) fn filename(&self) -> &Path {
        match self {
            Self { filename } => filename,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_of_an_export_is_its_filename() {
        let text = r#"{"filename": "crates/harbor/src/lib.rs", "segments": []}"#;
        let file: File = serde_json::from_str(text).unwrap();

        let filename = file.filename();

        let path_text = "crates/harbor/src/lib.rs";
        let expectation = Path::new(path_text);
        assert_eq!(filename, expectation);
    }
}
