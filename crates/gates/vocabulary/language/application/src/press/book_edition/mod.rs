use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct BookEdition {
    directories: Vec<PathBuf>,
}

impl Layer for BookEdition {
    type Layer = ApplicationLayer;
}

impl BookEdition {
    pub fn directories(&self) -> &[PathBuf] {
        &self.directories
    }
}

impl From<Vec<PathBuf>> for BookEdition {
    fn from(directories: Vec<PathBuf>) -> Self {
        Self { directories }
    }
}
