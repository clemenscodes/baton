use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    ProgramAbsence,
    StartFailure {
        program_text: String,
        text: String,
    },
    Signal {
        program_text: String,
        status: String,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProgramAbsence => write!(formatter, "no program was named after `--`"),
            Self::StartFailure { program_text, text } => {
                write!(formatter, "{program_text} could not be started, {text}")
            }
            Self::Signal {
                program_text,
                status,
            } => write!(formatter, "{program_text} ended with {status}"),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_runner_reads_as_a_phrase() {
        let program_text = String::from("resolve");
        let error = [
            Error::ProgramAbsence,
            Error::StartFailure {
                program_text: program_text.clone(),
                text: String::from("No such file or directory"),
            },
            Error::Signal {
                program_text,
                status: String::from("signal: 9 (SIGKILL)"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "no program was named after `--`",
                "resolve could not be started, No such file or directory",
                "resolve ended with signal: 9 (SIGKILL)",
            ]
        );
    }
}
