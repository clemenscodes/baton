use crate::gate::Errors;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::Query;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TermJudgment {
    paths: Vec<PathBuf>,
}

impl Layer for TermJudgment {
    type Layer = ApplicationLayer;
}

impl Query for TermJudgment {
    type Output = Result<(), Errors>;
}

impl TermJudgment {
    pub fn paths(&self) -> &[PathBuf] {
        &self.paths
    }
}

impl From<Vec<PathBuf>> for TermJudgment {
    fn from(paths: Vec<PathBuf>) -> Self {
        Self { paths }
    }
}
