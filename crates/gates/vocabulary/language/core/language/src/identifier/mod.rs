mod pieces;

use crate::identifier::pieces::Pieces;
use crate::vocabulary::word;
use crate::vocabulary::word::Word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Identifier {
    words: Vec<Word>,
}

impl Layer for Identifier {
    type Layer = DomainLayer;
}

impl ValueObject for Identifier {}

impl Identifier {
    const SEPARATOR: &'static str = "_";
}

impl TryFrom<&str> for Identifier {
    type Error = word::Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let pieces = Pieces::from(text);
        let mut words = Vec::new();
        for piece in &pieces {
            let piece_text = piece.text();
            let word = Word::try_from(piece_text)?;
            words.push(word);
        }
        let identifier = Self { words };
        Ok(identifier)
    }
}

impl AsRef<[Word]> for Identifier {
    fn as_ref(&self) -> &[Word] {
        let Self { words } = self;
        words
    }
}

impl fmt::Display for Identifier {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { words } = self;
        let mut word_texts = Vec::new();
        for word in words {
            let word_text = word.to_string();
            word_texts.push(word_text);
        }
        let separator = Self::SEPARATOR;
        let text = word_texts.join(separator);
        fmt::Display::fmt(&text, formatter)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) mod fixture {
        use super::*;

        pub(crate) fn identifier_factory(text: &str) -> Identifier {
            Identifier::try_from(text).unwrap()
        }
    }

    #[test]
    fn the_words_of_an_identifier_are_shown_apart_by_an_underscore() {
        let text = "HarborMaster";
        let identifier = fixture::identifier_factory(text);

        let identifier_text = identifier.to_string();

        let expectation = "Harbor_Master";
        assert_eq!(identifier_text, expectation);
    }

    #[test]
    fn a_piece_of_an_identifier_that_opens_with_a_digit_is_an_error() {
        let text = "level_2";

        let identifier = Identifier::try_from(text);

        let word_text_literal = "2";
        let word_text = String::from(word_text_literal);
        let error = word::Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(identifier, expectation);
    }

    #[test]
    fn a_piece_of_digits_alone_is_an_error() {
        let text = "_1";

        let identifier = Identifier::try_from(text);

        let word_text_literal = "1";
        let word_text = String::from(word_text_literal);
        let error = word::Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(identifier, expectation);
    }

    #[test]
    fn a_piece_with_a_letter_outside_the_alphabet_of_a_word_is_an_error() {
        let text = "слово";

        let identifier = Identifier::try_from(text);

        let word_text_literal = "слово";
        let word_text = String::from(word_text_literal);
        let error = word::Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(identifier, expectation);
    }

    #[test]
    fn an_underscore_alone_is_no_word_at_all() {
        let text = "_";

        let identifier = Identifier::try_from(text);

        let identifier_expectation = Identifier::default();
        let expectation = Ok(identifier_expectation);
        assert_eq!(identifier, expectation);
    }

    #[test]
    fn two_writings_of_the_same_words_are_the_same_identifier() {
        let lower_case_text = "as_ref";
        let type_text = "AsRef";
        let lower_case_identifier = fixture::identifier_factory(lower_case_text);

        let type_identifier = Identifier::try_from(type_text);

        let expectation = Ok(lower_case_identifier);
        assert_eq!(type_identifier, expectation);
    }

    #[test]
    fn the_same_words_in_another_order_are_another_identifier() {
        let text = "is_empty";
        let reverse_text = "empty_is";
        let identifier = fixture::identifier_factory(text);

        let reverse_identifier = Identifier::try_from(reverse_text);

        let expectation = Ok(identifier);
        assert_ne!(reverse_identifier, expectation);
    }

    #[test]
    fn an_identifier_lends_the_words_it_falls_apart_into() {
        let text = "as_ref";
        let first_word_text = "as";
        let next_word_text = "ref";
        let identifier = fixture::identifier_factory(text);

        let words = identifier.as_ref();

        let first_word = Word::try_from(first_word_text).unwrap();
        let next_word = Word::try_from(next_word_text).unwrap();
        let expectation = [first_word, next_word];
        assert_eq!(words, expectation);
    }
}
