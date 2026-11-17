use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    LockName { text: String },
    ApplicationName { text: String },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::LockName { text } => write!(
                formatter,
                "`{text}` is no name of a lock, which holds only letters, digits, `_` and `-` and never begins with `-`."
            ),
            Self::ApplicationName { text } => write!(
                formatter,
                "`{text}` is no name of an application, which holds only letters, digits, `_` and `-` and never begins with `-`."
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_error_of_a_name_reads_as_a_sentence() {
        let error = [
            Error::LockName {
                text: String::from("--help"),
            },
            Error::ApplicationName {
                text: String::from("a b"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "`--help` is no name of a lock, which holds only letters, digits, `_` and `-` and never begins with `-`.",
                "`a b` is no name of an application, which holds only letters, digits, `_` and `-` and never begins with `-`."
            ]
        );
    }
}
