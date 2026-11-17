mod error;

pub use error::Error;

use crate::identifier::Identifier;
use crate::vocabulary::word::Word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::borrow::Borrow;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct Name {
    identifier: Identifier,
}

impl Layer for Name {
    type Layer = DomainLayer;
}

impl ValueObject for Name {}

impl TryFrom<&str> for Name {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let name_text = text.to_string();
        let identifier = Identifier::try_from(text);
        match identifier {
            Ok(name_identifier) => match name_identifier.as_ref().is_empty() {
                true => {
                    let error = Error::Name { text: name_text };
                    Err(error)
                }
                false => {
                    let name = Self {
                        identifier: name_identifier,
                    };
                    Ok(name)
                }
            },
            Err(word_error) => {
                let error = Error::Word {
                    text: name_text,
                    error: word_error,
                };
                Err(error)
            }
        }
    }
}

impl Borrow<[Word]> for Name {
    fn borrow(&self) -> &[Word] {
        let Self { identifier } = self;
        identifier.as_ref()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { identifier } = self;
        fmt::Display::fmt(identifier, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::identifier::tests::fixture;
    use crate::vocabulary::word;

    #[test]
    fn a_name_is_shown_as_its_words_apart_by_an_underscore() {
        let text = "HarborMaster";
        let name = Name::try_from(text).unwrap();

        let name_text = name.to_string();

        let expectation = "Harbor_Master";
        assert_eq!(name_text, expectation);
    }

    #[test]
    fn a_name_lends_the_words_it_falls_apart_into() {
        let text = "as_ref";
        let type_text = "AsRef";
        let name = Name::try_from(text).unwrap();

        let name_words: &[Word] = name.borrow();

        let identifier = fixture::identifier_factory(type_text);
        let words = identifier.as_ref();
        assert_eq!(name_words, words);
    }

    #[test]
    fn an_empty_text_is_an_error() {
        let text = "";

        let name = Name::try_from(text);

        let name_text = String::new();
        let error = Error::Name { text: name_text };
        let expectation = Err(error);
        assert_eq!(name, expectation);
    }

    #[test]
    fn an_underscore_alone_is_an_error() {
        let text = "_";

        let name = Name::try_from(text);

        let name_text_literal = "_";
        let name_text = String::from(name_text_literal);
        let error = Error::Name { text: name_text };
        let expectation = Err(error);
        assert_eq!(name, expectation);
    }

    #[test]
    fn a_text_of_two_names_apart_by_a_space_is_an_error() {
        let text = "is empty";

        let name = Name::try_from(text);

        let name_text_literal = "is empty";
        let name_text = String::from(name_text_literal);
        let word_text_literal = "is empty";
        let word_text = String::from(word_text_literal);
        let word_error = word::Error::Word { text: word_text };
        let error = Error::Word {
            text: name_text,
            error: word_error,
        };
        let expectation = Err(error);
        assert_eq!(name, expectation);
    }
}
