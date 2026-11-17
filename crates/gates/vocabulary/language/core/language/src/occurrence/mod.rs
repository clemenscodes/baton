use crate::identifier::Identifier;
use crate::vocabulary::word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Occurrence {
    line: usize,
    column: usize,
    text: String,
    identifier: Identifier,
}

impl Layer for Occurrence {
    type Layer = DomainLayer;
}

impl ValueObject for Occurrence {}

impl Occurrence {
    pub(super) fn identifier(&self) -> &Identifier {
        match self {
            Self { identifier, .. } => identifier,
        }
    }
}

impl TryFrom<(&str, usize, usize)> for Occurrence {
    type Error = word::Error;

    fn try_from((text, line, column): (&str, usize, usize)) -> Result<Self, Self::Error> {
        let identifier = Identifier::try_from(text)?;
        let occurrence_text = text.to_string();
        let occurrence = Self {
            line,
            column,
            text: occurrence_text,
            identifier,
        };
        Ok(occurrence)
    }
}

impl fmt::Display for Occurrence {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            line, column, text, ..
        } = self;
        write!(formatter, "{line}:{column}: `{text}`")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_occurrence_is_shown_as_its_line_its_column_and_its_text_as_the_source_writes_it() {
        let text = "JsonReport";
        let line: usize = 3;
        let column: usize = 8;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();

        let occurrence_text = occurrence.to_string();

        let expectation = "3:8: `JsonReport`";
        assert_eq!(occurrence_text, expectation);
    }

    #[test]
    fn an_occurrence_lends_the_identifier_its_text_falls_apart_into() {
        let text = "JsonReport";
        let identifier_text = "json_report";
        let line: usize = 3;
        let column: usize = 8;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();

        let identifier = occurrence.identifier();

        let identifier_expectation = Identifier::try_from(identifier_text).unwrap();
        let expectation = &identifier_expectation;
        assert_eq!(identifier, expectation);
    }

    #[test]
    fn an_occurrence_of_a_text_with_a_piece_that_is_no_word_is_an_error() {
        let text = "letter_2";
        let line: usize = 4;
        let column: usize = 9;

        let occurrence = Occurrence::try_from((text, line, column));

        let word_text_literal = "2";
        let word_text = String::from(word_text_literal);
        let error = word::Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(occurrence, expectation);
    }
}
