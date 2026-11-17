mod error;

pub use error::Error;

use crate::vocabulary::name::Name;
use crate::vocabulary::word::Word;
use stack_domain_driven_design_core::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::borrow::Borrow;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Spelling {
    words: Vec<Word>,
}

impl Layer for Spelling {
    type Layer = DomainLayer;
}

impl ValueObject for Spelling {}

impl Identifier for Spelling {}

impl Spelling {
    const SEPARATOR: &'static str = " ";
}

impl From<Word> for Spelling {
    fn from(word: Word) -> Self {
        let words = vec![word];
        Self { words }
    }
}

impl From<&Name> for Spelling {
    fn from(name: &Name) -> Self {
        let name_words: &[Word] = name.borrow();
        let words = name_words.to_vec();
        Self { words }
    }
}

impl TryFrom<&[&str]> for Spelling {
    type Error = Error;

    fn try_from(word_texts: &[&str]) -> Result<Self, Self::Error> {
        let mut words = Vec::new();
        for word_text in word_texts.iter().copied() {
            let word = Word::try_from(word_text)?;
            words.push(word);
        }
        match words.is_empty() {
            true => {
                let error = Error::Term;
                Err(error)
            }
            false => {
                let spelling = Self { words };
                Ok(spelling)
            }
        }
    }
}

impl Borrow<[Word]> for Spelling {
    fn borrow(&self) -> &[Word] {
        let Self { words } = self;
        words
    }
}

impl fmt::Display for Spelling {
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
mod tests {
    use super::*;
    use crate::vocabulary::word;

    #[test]
    fn a_spelling_keeps_the_words_it_is_made_of() {
        let word_texts: &[&str] = &["King", "Hill"];
        let spelling = Spelling::try_from(word_texts).unwrap();

        let spelling_text = spelling.to_string();

        let expectation = "King Hill";
        assert_eq!(spelling_text, expectation);
    }

    #[test]
    fn a_spelling_of_no_word_is_an_error() {
        let word_texts: &[&str] = &[];

        let spelling = Spelling::try_from(word_texts);

        let error = Error::Term;
        let expectation = Err(error);
        assert_eq!(spelling, expectation);
    }

    #[test]
    fn a_spelling_of_a_text_that_is_no_word_is_an_error() {
        let word_texts: &[&str] = &["Treasure", "clan tag"];

        let spelling = Spelling::try_from(word_texts);

        let text_literal = "clan tag";
        let text = String::from(text_literal);
        let word_error = word::Error::Word { text };
        let error = Error::Word { error: word_error };
        let expectation = Err(error);
        assert_eq!(spelling, expectation);
    }

    #[test]
    fn two_spellings_that_read_alike_are_the_same_spelling() {
        let spelling_word_text = "Treasure";
        let spelling_word = Word::try_from(spelling_word_text).unwrap();
        let lower_case_word_text = "treasure";
        let lower_case_word = Word::try_from(lower_case_word_text).unwrap();
        let term_spelling = Spelling::from(spelling_word);

        let lower_case_spelling = Spelling::from(lower_case_word);

        assert_eq!(term_spelling, lower_case_spelling);
    }

    #[test]
    fn a_spelling_lends_the_words_it_is_made_of() {
        let word_text = "Anchor";
        let word = Word::try_from(word_text).unwrap();
        let spelling = Spelling::from(word);

        let spelling_words: &[Word] = spelling.borrow();

        let lower_case_word_text = "anchor";
        let lower_case_word = Word::try_from(lower_case_word_text).unwrap();
        let expectation = [lower_case_word];
        assert_eq!(spelling_words, expectation);
    }
}
