use crate::TermText;
use crate::vocabulary::term::definition;
use crate::vocabulary::term::spelling;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Spelling {
        term_text: TermText,
        error: spelling::Error,
    },
    Definition {
        term_text: TermText,
        error: definition::Error,
    },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Spelling { term_text, error } => {
                write!(formatter, "the term `{term_text}` is invalid: {error}")
            }
            Self::Definition { term_text, error } => {
                write!(formatter, "the term `{term_text}` is invalid: {error}")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Spelling { error, .. } => Some(error),
            Self::Definition { error, .. } => Some(error),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocabulary::word;
    use std::error::Error as _;

    #[test]
    fn a_term_states_the_error_of_the_word_it_was_spelled_with_and_its_text() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["clan tag"];
            let definition = "A tag of a clan.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let text_literal = "clan tag";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let spelling_error = spelling::Error::Word { error: word_error };
        let error = Error::Spelling {
            term_text,
            error: spelling_error,
        };

        let error_text = error.to_string();

        let expectation = "the term `clan tag: A tag of a clan.` is invalid: `clan tag` is not a word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_term_states_the_error_of_the_spelling_it_was_named_by_and_its_text() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "A tag of a clan.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let spelling_error = spelling::Error::Term;
        let error = Error::Spelling {
            term_text,
            error: spelling_error,
        };

        let error_text = error.to_string();

        let expectation = "the term `: A tag of a clan.` is invalid: a term needs at least one word";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_term_states_the_error_of_the_definition_it_takes_and_its_text() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "   ";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let text_literal = "   ";
        let text = String::from(text_literal);
        let definition_error = definition::Error::Definition { text };
        let error = Error::Definition {
            term_text,
            error: definition_error,
        };

        let error_text = error.to_string();

        let expectation = "the term `Treasure:    ` is invalid: `   ` is not a definition";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn the_source_of_an_error_of_a_spelling_is_the_error_of_the_spelling() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "A tag of a clan.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let spelling_error = spelling::Error::Term;
        let error = Error::Spelling {
            term_text,
            error: spelling_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "a term needs at least one word";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_an_error_of_a_definition_is_the_error_of_the_definition() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "   ";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let text_literal = "   ";
        let text = String::from(text_literal);
        let definition_error = definition::Error::Definition { text };
        let error = Error::Definition {
            term_text,
            error: definition_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "`   ` is not a definition";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }
}
