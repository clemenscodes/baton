mod definition;
mod error;
pub mod spelling;

pub use error::Error;

use crate::TermText;
use crate::vocabulary::term::definition::Definition;
use crate::vocabulary::term::spelling::Spelling;
use stack_domain_driven_design_core::entity::Entity;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Term {
    spelling: Spelling,
    definition: Definition,
}

impl Layer for Term {
    type Layer = DomainLayer;
}

impl Entity for Term {
    type Identity = Spelling;

    fn identity(&self) -> &Self::Identity {
        &self.spelling
    }
}

impl Term {
    pub fn definition(&self) -> &Definition {
        &self.definition
    }
}

impl TryFrom<TermText> for Term {
    type Error = Error;

    fn try_from(term_text: TermText) -> Result<Self, Self::Error> {
        let spelling_error = |error| Error::Spelling { term_text, error };
        let definition_error = |error| Error::Definition { term_text, error };
        let spelling_text = term_text.spelling_text();
        let spelling = Spelling::try_from(spelling_text).map_err(spelling_error)?;
        let definition_text = term_text.definition_text();
        let definition = Definition::try_from(definition_text).map_err(definition_error)?;
        let term = Self {
            spelling,
            definition,
        };
        Ok(term)
    }
}

impl fmt::Display for Term {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { spelling, .. } = self;
        fmt::Display::fmt(spelling, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vocabulary::word;

    #[test]
    fn a_term_text_of_words_and_a_definition_yields_a_term() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Column", "Length"];
            let definition = "How wide a column is.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;

        let term = Term::try_from(term_text).unwrap();

        let spelling_text = term.to_string();
        let expectation = "Column Length";
        assert_eq!(spelling_text, expectation);
    }

    #[test]
    fn two_terms_of_the_same_spelling_and_definition_are_the_same_term() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let term = Term::try_from(term_text).unwrap();

        let term_copy = Term::try_from(term_text).unwrap();

        assert_eq!(term, term_copy);
    }

    #[test]
    fn two_terms_of_one_spelling_and_two_definitions_are_two_terms() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        const REDEFINITION_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team delivers.";
            TermText::new(spelling, definition)
        };
        let redefinition_text = REDEFINITION_TEXT;
        let term = Term::try_from(term_text).unwrap();

        let redefinition_term = Term::try_from(redefinition_text).unwrap();

        assert_ne!(term, redefinition_term);
    }

    #[test]
    fn a_term_yields_what_it_means() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let term = Term::try_from(term_text).unwrap();

        let definition = term.definition();

        let definition_text = definition.to_string();
        let expectation = "The object a team brings to the chest.";
        assert_eq!(definition_text, expectation);
    }

    #[test]
    fn a_term_redefined_is_still_the_same_entity() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        const REDEFINITION_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "The object a team delivers.";
            TermText::new(spelling, definition)
        };
        let redefinition_text = REDEFINITION_TEXT;
        let redefinition_term = Term::try_from(redefinition_text).unwrap();

        let identity = redefinition_term.identity();

        let spelling_text = term_text.spelling_text();
        let spelling_expectation = Spelling::try_from(spelling_text).unwrap();
        let expectation = &spelling_expectation;
        assert_eq!(identity, expectation);
    }

    #[test]
    fn a_term_is_shown_as_its_words_apart_by_a_space() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["King", "of", "the", "Hill"];
            let definition = "The match with a hill.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let term = Term::try_from(term_text).unwrap();

        let spelling_text = term.to_string();

        let expectation = "King of the Hill";
        assert_eq!(spelling_text, expectation);
    }

    #[test]
    fn a_spelling_without_a_word_is_an_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "The object a team brings to the chest.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;

        let term = Term::try_from(term_text);

        let spelling_error = spelling::Error::Term;
        let error = Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let expectation = Err(error);
        assert_eq!(term, expectation);
    }

    #[test]
    fn a_spelling_that_is_no_word_is_an_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["season_profile"];
            let definition = "The record of one season.";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;

        let term = Term::try_from(term_text);

        let text_literal = "season_profile";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let spelling_error = spelling::Error::Word { error: word_error };
        let error = Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let expectation = Err(error);
        assert_eq!(term, expectation);
    }

    #[test]
    fn a_spelling_without_a_definition_is_an_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "";
            TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;

        let term = Term::try_from(term_text);

        let text = String::new();
        let definition_error = definition::Error::Definition { text };
        let error = Error::Definition {
            term_text,
            error: definition_error,
        };
        let expectation = Err(error);
        assert_eq!(term, expectation);
    }
}
