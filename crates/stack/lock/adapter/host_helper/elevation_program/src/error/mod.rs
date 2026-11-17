use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    StartFailure {
        text: String,
    },
    Request {
        request_name: String,
        status: String,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StartFailure { text } => write!(
                formatter,
                "the elevation program could not be started, {text}"
            ),
            Self::Request {
                request_name,
                status,
            } => write!(formatter, "the request {request_name} ended with {status}"),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_helper_reads_as_a_phrase() {
        let error = [
            Error::StartFailure {
                text: String::from("No such file or directory"),
            },
            Error::Request {
                request_name: String::from("acquire"),
                status: String::from("exit status: 1"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "the elevation program could not be started, No such file or directory",
                "the request acquire ended with exit status: 1",
            ]
        );
    }
}
