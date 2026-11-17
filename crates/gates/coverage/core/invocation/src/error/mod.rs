use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Error {
    ProgramAbsence,
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProgramAbsence => write!(
                formatter,
                "[COVERAGE] a command names no program: the build and the tests each need one."
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_program_absence_says_that_a_command_names_no_program() {
        let error = Error::ProgramAbsence;

        let text = error.to_string();

        let expectation = "[COVERAGE] a command names no program: the build and the tests each need one.";
        assert_eq!(text, expectation);
    }
}
