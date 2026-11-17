use crate::Error;
use crate::gate::Judgment;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::Query;
use std::io::Read;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum ReportJudgment<InputType> {
    File { path: PathBuf },
    Input { input: InputType },
}

impl<InputType> Layer for ReportJudgment<InputType> {
    type Layer = ApplicationLayer;
}

impl<InputType: Read> Query for ReportJudgment<InputType> {
    type Output = Result<Judgment, Error>;
}
