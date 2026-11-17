use crate::TermText;
use crate::vocabulary::name;
use crate::vocabulary::term;
use crate::vocabulary::word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Term { error: term::Error },
    Name { error: name::Error },
    Word { error: word::Error },
    Redefinition { term_text: TermText },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Term { error } => write!(formatter, "{error}"),
            Self::Name { error } => write!(formatter, "{error}"),
            Self::Word { error } => write!(formatter, "{error}"),
            Self::Redefinition { term_text } => {
                write!(formatter, "the term `{term_text}` is defined twice")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Term { error } => Some(error),
            Self::Name { error } => Some(error),
            Self::Word { error } => Some(error),
            Self::Redefinition { .. } => None,
        }
    }
}

impl From<term::Error> for Error {
    fn from(error: term::Error) -> Self {
        Self::Term { error }
    }
}

impl From<name::Error> for Error {
    fn from(error: name::Error) -> Self {
        Self::Name { error }
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
    use crate::vocabulary::term::spelling;
    use std::error::Error as _;

    #[test]
    fn a_language_states_the_error_of_the_term_it_is_defined_with() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "A definition.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let spelling_error = spelling::Error::Term;
        let term_error = term::Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let error = Error::from(term_error);

        let error_text = error.to_string();

        let expectation = "the term `: A definition.` is invalid: a term needs at least one word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_language_states_the_error_of_the_name_it_admits() {
        let text_literal = "is empty";
        let text = String::from(text_literal);
        let name_error = name::Error::Name { text };
        let error = Error::from(name_error);

        let error_text = error.to_string();

        let expectation = "`is empty` is not a name";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_language_states_the_error_of_the_word_it_admits() {
        let text_literal = "clan tag";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let error = Error::from(word_error);

        let error_text = error.to_string();

        let expectation = "`clan tag` is not a word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_term_defined_a_second_time_is_named_with_its_text_in_the_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let error = Error::Redefinition { term_text };

        let error_text = error.to_string();

        let expectation = "the term `Treasure: A thing.` is defined twice";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn the_source_of_an_error_of_a_term_is_that_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "A definition.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let spelling_error = spelling::Error::Term;
        let term_error = term::Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let error = Error::from(term_error);

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "the term `: A definition.` is invalid: a term needs at least one word";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_an_error_of_a_name_is_that_error() {
        let text_literal = "_";
        let text = String::from(text_literal);
        let name_error = name::Error::Name { text };
        let error = Error::from(name_error);

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "`_` is not a name";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_an_error_of_a_word_is_that_error() {
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

    #[test]
    fn an_error_of_a_redefinition_has_no_source() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let error = Error::Redefinition { term_text };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }
}
