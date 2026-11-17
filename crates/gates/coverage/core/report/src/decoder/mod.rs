use crate::Report;
use gates_coverage_core_test_build::TestBuild;
use stack_domain_driven_design_core::port::DrivenPort;
use std::fmt;
use std::io::Read;

pub trait Decoder: DrivenPort {
    type Error: fmt::Display;

    fn try_decode_report(&self, input: impl Read) -> Result<Report, Self::Error>;

    fn try_decode_test_build(&self, output: &[u8]) -> Result<TestBuild, Self::Error>;
}
