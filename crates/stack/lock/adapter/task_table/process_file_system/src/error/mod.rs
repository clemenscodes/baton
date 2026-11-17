use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    ScanFailure {
        text: String,
    },
    SignalFailure {
        process_identifier: u32,
        text: String,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ScanFailure { text } => {
                write!(formatter, "the processes could not be read, {text}")
            }
            Self::SignalFailure {
                process_identifier,
                text,
            } => write!(
                formatter,
                "the process {process_identifier} could not be signalled, {text}"
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_task_table_reads_as_a_phrase() {
        let error = [
            Error::ScanFailure {
                text: String::from("Permission denied"),
            },
            Error::SignalFailure {
                process_identifier: 7,
                text: String::from("Operation not permitted"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "the processes could not be read, Permission denied",
                "the process 7 could not be signalled, Operation not permitted",
            ]
        );
    }
}
