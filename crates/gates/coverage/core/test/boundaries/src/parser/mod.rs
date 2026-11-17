use crate::TestBoundaries;
use gates_coverage_core_source::Source;
use stack_domain_driven_design_core::port::DrivenPort;
use std::fmt;

pub trait Parser: DrivenPort {
    type Error: fmt::Display;

    fn try_parse_test_boundaries(&self, source: &Source) -> Result<TestBoundaries, Self::Error>;
}
