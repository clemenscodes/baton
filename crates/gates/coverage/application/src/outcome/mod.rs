use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::process::ExitCode;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Outcome {
    Success,
    Error,
}

impl Layer for Outcome {
    type Layer = ApplicationLayer;
}

impl From<&Result<(), Error>> for Outcome {
    fn from(measurement: &Result<(), Error>) -> Self {
        match measurement {
            Ok(()) => Self::Success,
            Err(_) => Self::Error,
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
    fn a_success_leaves_the_status_of_a_success() {
        let outcome = Outcome::Success;

        let exit_code = ExitCode::from(outcome);

        let expectation = ExitCode::SUCCESS;
        assert_eq!(exit_code, expectation);
    }

    #[test]
    fn an_error_leaves_the_status_of_a_failure() {
        let outcome = Outcome::Error;

        let exit_code = ExitCode::from(outcome);

        let expectation = ExitCode::FAILURE;
        assert_eq!(exit_code, expectation);
    }

    #[test]
    fn a_measurement_that_wrote_its_report_is_a_success_and_one_that_failed_is_an_error() {
        let success_measurement = Ok(());
        let error_measurement = Err(Error::Words);

        let success_outcome = Outcome::from(&success_measurement);
        let error_outcome = Outcome::from(&error_measurement);

        assert_eq!(success_outcome, Outcome::Success);
        assert_eq!(error_outcome, Outcome::Error);
    }
}
