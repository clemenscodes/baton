use crate::gate::Errors;
use crate::gate::token::TokenReport;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::Query;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TokenSelection {
    paths: Vec<PathBuf>,
}

impl Layer for TokenSelection {
    type Layer = ApplicationLayer;
}

impl Query for TokenSelection {
    type Output = Result<TokenReport, Errors>;
}

impl TokenSelection {
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
}

impl From<Vec<PathBuf>> for TokenSelection {
    fn from(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }
}
