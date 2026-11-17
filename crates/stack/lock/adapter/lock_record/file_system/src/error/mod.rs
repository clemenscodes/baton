use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    LeaseFailure { path: PathBuf, text: String },
    Input { path: PathBuf, text: String },
    Output { path: PathBuf, text: String },
    Document { path: PathBuf, text: String },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LeaseFailure { path, text } => {
                write!(formatter, "{} could not be leased, {text}", path.display())
            }
            Self::Input { path, text } => {
                write!(formatter, "{} could not be read, {text}", path.display())
            }
            Self::Output { path, text } => {
                write!(formatter, "{} could not be written, {text}", path.display())
            }
            Self::Document { path, text } => {
                write!(formatter, "{} holds no lock, {text}", path.display())
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_record_reads_as_a_phrase() {
        let path = PathBuf::from("/run/baton/rtx3080.json");
        let text = String::from("it broke");
        let error = [
            Error::LeaseFailure {
                path: path.clone(),
                text: text.clone(),
            },
            Error::Input {
                path: path.clone(),
                text: text.clone(),
            },
            Error::Output {
                path: path.clone(),
                text: text.clone(),
            },
            Error::Document { path, text },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "/run/baton/rtx3080.json could not be leased, it broke",
                "/run/baton/rtx3080.json could not be read, it broke",
                "/run/baton/rtx3080.json could not be written, it broke",
                "/run/baton/rtx3080.json holds no lock, it broke",
            ]
        );
    }
}
