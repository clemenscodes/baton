use crate::vocabulary::word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Defect {
    TestAttribute {
        line: usize,
        column: usize,
    },
    Comment {
        line: usize,
        column: usize,
    },
    Identifier {
        line: usize,
        column: usize,
        text: String,
        error: word::Error,
    },
}

impl Layer for Defect {
    type Layer = DomainLayer;
}

impl ValueObject for Defect {}

impl fmt::Display for Defect {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::TestAttribute { line, column } => write!(
                formatter,
                "{line}:{column}: the test attribute does not open the `tests` module at the bottom of the source"
            ),
            Self::Comment { line, column } => write!(
                formatter,
                "{line}:{column}: the `doc` attribute is a comment, and code must not contain comments"
            ),
            Self::Identifier {
                line, column, text, ..
            } => write!(
                formatter,
                "{line}:{column}: `{text}` is not an identifier made of words"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test_attribute_is_shown_with_its_line_and_its_column() {
        let line: usize = 3;
        let column: usize = 1;
        let defect = Defect::TestAttribute { line, column };

        let defect_text = defect.to_string();

        let expectation = "3:1: the test attribute does not open the `tests` module at the bottom of the source";
        assert_eq!(defect_text, expectation);
    }

    #[test]
    fn a_comment_is_shown_with_its_line_and_its_column() {
        let line: usize = 2;
        let column: usize = 5;
        let defect = Defect::Comment { line, column };

        let defect_text = defect.to_string();

        let expectation = "2:5: the `doc` attribute is a comment, and code must not contain comments";
        assert_eq!(defect_text, expectation);
    }

    #[test]
    fn an_identifier_of_no_words_is_shown_with_its_line_its_column_and_its_text() {
        let line: usize = 1;
        let column: usize = 5;
        let text_literal = "letter_2";
        let text = String::from(text_literal);
        let word_text_literal = "2";
        let word_text = String::from(word_text_literal);
        let error = word::Error::Word { text: word_text };
        let defect = Defect::Identifier {
            line,
            column,
            text,
            error,
        };

        let defect_text = defect.to_string();

        let expectation = "1:5: `letter_2` is not an identifier made of words";
        assert_eq!(defect_text, expectation);
    }
}
