use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::io;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    PinDirectory {
        path: PathBuf,
        text: String,
    },
    Hierarchy {
        path: PathBuf,
        text: String,
    },
    Call {
        operation_text: String,
        text: String,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PinDirectory { path, text } => write!(
                formatter,
                "the directory of the pins {} could not be reached, {text}, is bpffs mounted",
                path.display()
            ),
            Self::Hierarchy { path, text } => write!(
                formatter,
                "the root of the unified hierarchy {} could not be opened, {text}",
                path.display()
            ),
            Self::Call {
                operation_text,
                text,
            } => write!(formatter, "the bpf {operation_text} failed, {text}"),
        }
    }
}

impl error::Error for Error {}

impl From<(String, io::Error)> for Error {
    fn from((operation_text, error): (String, io::Error)) -> Self {
        let text = error.to_string();
        Self::Call {
            operation_text,
            text,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_program_reads_as_a_phrase() {
        let error = [
            Error::PinDirectory {
                path: PathBuf::from("/sys/fs/bpf/baton"),
                text: String::from("No such file or directory"),
            },
            Error::Hierarchy {
                path: PathBuf::from("/sys/fs/cgroup"),
                text: String::from("Permission denied"),
            },
            Error::Call {
                operation_text: String::from("attachment"),
                text: String::from("Operation not permitted"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "the directory of the pins /sys/fs/bpf/baton could not be reached, No such file or directory, is bpffs mounted",
                "the root of the unified hierarchy /sys/fs/cgroup could not be opened, Permission denied",
                "the bpf attachment failed, Operation not permitted",
            ]
        );
    }
}
