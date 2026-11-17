use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::ops::BitOr;
use std::process::ExitCode;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub(super) enum Outcome {
    Success,
    Error,
}

impl Layer for Outcome {
    type Layer = InfrastructureLayer;
}

impl BitOr for Outcome {
    type Output = Self;

    fn bitor(self, outcome: Self) -> Self::Output {
        match self {
            Self::Success => outcome,
            Self::Error => Self::Error,
        }
    }
}

impl From<Outcome> for ExitCode {
    fn from(outcome: Outcome) -> Self {
        match outcome {
            Outcome::Success => Self::SUCCESS,
            Outcome::Error => Self::FAILURE,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_success_leaves_the_exit_code_of_a_success() {
        let outcome = Outcome::Success;

        let exit_code = ExitCode::from(outcome);

        let expectation = ExitCode::SUCCESS;
        assert_eq!(exit_code, expectation);
    }

    #[test]
    fn an_error_leaves_the_exit_code_of_a_failure() {
        let outcome = Outcome::Error;

        let exit_code = ExitCode::from(outcome);

        let expectation = ExitCode::FAILURE;
        assert_eq!(exit_code, expectation);
    }

    #[test]
    fn a_success_joined_with_an_error_is_an_error() {
        let success = Outcome::Success;
        let error = Outcome::Error;

        let outcome = success | error;

        let expectation = Outcome::Error;
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn an_error_joined_with_a_success_is_an_error() {
        let error = Outcome::Error;
        let success = Outcome::Success;

        let outcome = error | success;

        let expectation = Outcome::Error;
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn two_successes_joined_are_a_success() {
        let augend = Outcome::Success;
        let addend = Outcome::Success;

        let outcome = augend | addend;

        let expectation = Outcome::Success;
        assert_eq!(outcome, expectation);
    }
}
