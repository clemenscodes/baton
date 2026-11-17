mod error;

pub(crate) use error::Error;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Definition {
    text: String,
}

impl Layer for Definition {
    type Layer = DomainLayer;
}

impl ValueObject for Definition {}

impl TryFrom<&str> for Definition {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let bare_text = text.trim();
        match bare_text.is_empty() {
            true => {
                let definition_text = text.to_string();
                let error = Error::Definition {
                    text: definition_text,
                };
                Err(error)
            }
            false => {
                let definition_text = bare_text.to_string();
                let definition = Self {
                    text: definition_text,
                };
                Ok(definition)
            }
        }
    }
}

impl fmt::Display for Definition {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { text } = self;
        fmt::Display::fmt(text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_same_text_yields_the_same_definition() {
        let text = "The object a team brings to the chest.";
        let definition = Definition::try_from(text).unwrap();

        let definition_copy = Definition::try_from(text).unwrap();

        assert_eq!(definition, definition_copy);
    }

    #[test]
    fn a_definition_drops_the_padding_around_its_text() {
        let bare_text = "The object a team brings to the chest.";
        let definition_text = "  The object a team brings to the chest.  ";
        let bare_text_definition = Definition::try_from(bare_text).unwrap();

        let definition = Definition::try_from(definition_text).unwrap();

        assert_eq!(definition, bare_text_definition);
    }

    #[test]
    fn two_different_texts_yield_two_different_definitions() {
        let first_text = "The object a team brings to the chest.";
        let next_text = "The chest a team delivers to.";
        let first_definition = Definition::try_from(first_text).unwrap();

        let next_definition = Definition::try_from(next_text).unwrap();

        assert_ne!(first_definition, next_definition);
    }

    #[test]
    fn a_definition_is_shown_as_its_text() {
        let text = "The object a team brings to the chest.";
        let definition = Definition::try_from(text).unwrap();

        let definition_text = definition.to_string();

        assert_eq!(definition_text, text);
    }

    #[test]
    fn an_empty_text_is_an_error() {
        let text = "";

        let definition = Definition::try_from(text);

        let definition_text = String::new();
        let error = Error::Definition {
            text: definition_text,
        };
        let expectation = Err(error);
        assert_eq!(definition, expectation);
    }

    #[test]
    fn padding_alone_is_an_error() {
        let text = "   ";

        let definition = Definition::try_from(text);

        let definition_text_literal = "   ";
        let definition_text = String::from(definition_text_literal);
        let error = Error::Definition {
            text: definition_text,
        };
        let expectation = Err(error);
        assert_eq!(definition, expectation);
    }
}
