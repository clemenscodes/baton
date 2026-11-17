use crate::error::Error;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
#[serde(from = "Vec<PathBuf>")]
pub(super) struct Filenames {
    filenames: Vec<PathBuf>,
}

impl Layer for Filenames {
    type Layer = InfrastructureLayer;
}

impl Filenames {
    pub(super) fn try_locate_filename(&self, file_index: usize) -> Result<&Path, Error> {
        let Self { filenames } = self;
        match filenames.get(file_index) {
            Some(filename) => Ok(filename),
            None => {
                let file_count = filenames.len();
                let file_error = Error::File {
                    file_index,
                    file_count,
                };
                Err(file_error)
            }
        }
    }
}

impl From<Vec<PathBuf>> for Filenames {
    fn from(filenames: Vec<PathBuf>) -> Self {
        Self { filenames }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_file_index_locates_the_filename_at_its_index() {
        let text = r#"["crates/harbor/src/lib.rs", "crates/lighthouse/src/lib.rs"]"#;
        let filenames: Filenames = serde_json::from_str(text).unwrap();
        let file_index: usize = 1;

        let filename = filenames.try_locate_filename(file_index).unwrap();

        let path_text = "crates/lighthouse/src/lib.rs";
        let expectation = Path::new(path_text);
        assert_eq!(filename, expectation);
    }

    #[test]
    fn a_file_index_past_the_filenames_is_an_error() {
        let text = r#"["crates/harbor/src/lib.rs"]"#;
        let filenames: Filenames = serde_json::from_str(text).unwrap();
        let file_index: usize = 1;

        let filename = filenames.try_locate_filename(file_index);

        let error_text = filename.unwrap_err().to_string();
        let expectation = "[COVERAGE] a region refers to file index 1, but the file indexes of its function end before 1.";
        assert_eq!(error_text, expectation);
    }
}
