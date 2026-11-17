use crate::vocabulary::word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Term,
    Word { error: word::Error },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Term => {
                write!(formatter, "a term needs at least one word")
            }
            Self::Word { error } => write!(formatter, "{error}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Term => None,
            Self::Word { error } => Some(error),
        }
    }
}

impl From<word::Error> for Error {
    fn from(error: word::Error) -> Self {
        Self::Word { error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn a_term_without_a_word_states_what_a_term_needs() {
        let error = Error::Term;

        let error_text = error.to_string();

        let expectation = "a term needs at least one word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_spelling_of_a_text_that_is_no_word_states_the_error_of_the_word() {
        let text_literal = "clan tag";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let error = Error::from(word_error);

        let error_text = error.to_string();

        let expectation = "`clan tag` is not a word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_term_without_a_word_has_no_source() {
        let error = Error::Term;

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_a_spelling_of_a_text_that_is_no_word_is_the_error_of_the_word() {
        let text_literal = "clan tag";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let error = Error::from(word_error);

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "`clan tag` is not a word";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }
}
