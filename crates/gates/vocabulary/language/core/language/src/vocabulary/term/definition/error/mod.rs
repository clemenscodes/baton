use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Definition { text: String },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Definition { text } => {
                write!(formatter, "`{text}` is not a definition")
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_text_that_yields_no_definition_is_named_in_the_error() {
        let text_literal = "   ";
        let text = String::from(text_literal);
        let error = Error::Definition { text };

        let error_text = error.to_string();

        let expectation = "`   ` is not a definition";
        assert_eq!(error_text, expectation);
    }
}
