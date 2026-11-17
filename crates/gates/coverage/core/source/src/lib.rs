mod reader;

pub use reader::Reader;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Source {
    path: PathBuf,
    text: String,
}

impl Layer for Source {
    type Layer = DomainLayer;
}

impl ValueObject for Source {}

impl Source {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn text(&self) -> &str {
        &self.text
    }
}

impl From<(PathBuf, String)> for Source {
    fn from((path, text): (PathBuf, String)) -> Self {
        Self { path, text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_source_keeps_its_path() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let text = String::new();
        let source = Source::from((path, text));

        let source_path = source.path();

        let expectation = Path::new(path_text);
        assert_eq!(source_path, expectation);
    }

    #[test]
    fn a_source_keeps_its_text() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let source_line_text = "struct Treasure;";
        let text = String::from(source_line_text);
        let source = Source::from((path, text));

        let source_text = source.text();

        let expectation = "struct Treasure;";
        assert_eq!(source_text, expectation);
    }
}
