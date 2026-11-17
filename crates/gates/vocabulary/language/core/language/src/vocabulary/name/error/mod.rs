use crate::vocabulary::word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Name { text: String },
    Word { text: String, error: word::Error },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name { text } => {
                write!(formatter, "`{text}` is not a name")
            }
            Self::Word { text, error } => {
                write!(formatter, "`{text}` is not a name: {error}")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Name { .. } => None,
            Self::Word { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn a_text_that_yields_no_name_is_named_in_the_error() {
        let text_literal = "_";
        let text = String::from(text_literal);
        let error = Error::Name { text };

        let error_text = error.to_string();

        let expectation = "`_` is not a name";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_text_with_a_piece_that_is_no_word_is_named_with_the_error_of_the_word() {
        let text_literal = "is empty";
        let text = String::from(text_literal);
        let word_text_literal = "is empty";
        let word_text = String::from(word_text_literal);
        let word_error = word::Error::Word { text: word_text };
        let error = Error::Word {
            text,
            error: word_error,
        };

        let error_text = error.to_string();

        let expectation = "`is empty` is not a name: `is empty` is not a word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_text_of_no_word_has_no_source() {
        let text_literal = "_";
        let text = String::from(text_literal);
        let error = Error::Name { text };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_a_name_with_a_piece_that_is_no_word_is_the_error_of_the_word() {
        let text_literal = "level_2";
        let text = String::from(text_literal);
        let word_text_literal = "2";
        let word_text = String::from(word_text_literal);
        let word_error = word::Error::Word { text: word_text };
        let error = Error::Word {
            text,
            error: word_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "`2` is not a word";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }
}
