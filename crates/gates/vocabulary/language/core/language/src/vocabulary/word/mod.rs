mod error;

pub use error::Error;

use crate::vocabulary::stems::Stems;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::cmp::Ordering;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, Eq)]
pub(crate) struct Word {
    text: String,
}

impl Layer for Word {
    type Layer = DomainLayer;
}

impl ValueObject for Word {}

impl Word {
    const DOUBLE_LETTER_ENDING: &'static str = "ss";
    const PLURAL_ENDING: &'static str = "s";
    const WYE_PLURAL_ENDING: &'static str = "ies";
    const WYE_ENDING: &'static str = "y";
    const SIBILANT_PLURAL_ENDING: &'static str = "es";
    const SIBILANT_LETTERS: [char; 4] = ['s', 'x', 'z', 'h'];

    pub(super) fn lower_case_text(&self) -> String {
        match self {
            Self { text } => text.to_lowercase(),
        }
    }

    pub(super) fn compose_stems(&self) -> Stems {
        let text = self.lower_case_text();
        let plural_ending = Self::PLURAL_ENDING;
        let double_letter_ending = Self::DOUBLE_LETTER_ENDING;
        let wye_plural_ending = Self::WYE_PLURAL_ENDING;
        let wye_ending = Self::WYE_ENDING;
        if !text.ends_with(plural_ending) || text.ends_with(double_letter_ending) {
            return Stems::default();
        }
        match text.strip_suffix(wye_plural_ending) {
            Some(wye_stem) => {
                let wye_ending_stem_text = format!("{wye_stem}{wye_ending}");
                let stem = Self {
                    text: wye_ending_stem_text,
                };
                let stems = [stem];
                Stems::from_iter(stems)
            }
            None => self.compose_singular_stems(),
        }
    }

    fn compose_singular_stems(&self) -> Stems {
        let sibilant_stems = self.compose_sibilant_stems();
        let plural_stems = self.compose_plural_stems();
        let sibilant_stem_words = sibilant_stems.into_iter();
        let plural_stem_words = plural_stems.into_iter();
        sibilant_stem_words
            .chain(plural_stem_words)
            .cloned()
            .collect()
    }

    fn compose_sibilant_stems(&self) -> Stems {
        let text = self.lower_case_text();
        let sibilant_plural_ending = Self::SIBILANT_PLURAL_ENDING;
        let sibilant_letters = Self::SIBILANT_LETTERS;
        let mut stems = Vec::new();
        if let Some(sibilant_stem) = text.strip_suffix(sibilant_plural_ending)
            && sibilant_stem.ends_with(sibilant_letters)
        {
            let sibilant_stem_text = sibilant_stem.to_string();
            let stem = Self {
                text: sibilant_stem_text,
            };
            stems.push(stem);
        }
        stems.into_iter().collect()
    }

    fn compose_plural_stems(&self) -> Stems {
        let text = self.lower_case_text();
        let plural_ending = Self::PLURAL_ENDING;
        let mut stems = Vec::new();
        if let Some(singular) = text.strip_suffix(plural_ending)
            && !singular.is_empty()
        {
            let singular_text = singular.to_string();
            let stem = Self {
                text: singular_text,
            };
            stems.push(stem);
        }
        stems.into_iter().collect()
    }
}

impl TryFrom<&str> for Word {
    type Error = Error;

    fn try_from(text: &str) -> Result<Self, Self::Error> {
        let word_text = text.to_string();
        match text.is_ascii()
            && text.starts_with(char::is_alphabetic)
            && text.chars().all(char::is_alphanumeric)
        {
            true => {
                let word = Self { text: word_text };
                Ok(word)
            }
            false => {
                let error = Error::Word { text: word_text };
                Err(error)
            }
        }
    }
}

impl PartialEq for Word {
    fn eq(&self, word: &Self) -> bool {
        let lower_case_text = self.lower_case_text();
        let word_lower_case_text = word.lower_case_text();
        lower_case_text == word_lower_case_text
    }
}

impl Hash for Word {
    fn hash<HasherType: Hasher>(&self, hasher: &mut HasherType) {
        let lower_case_text = self.lower_case_text();
        lower_case_text.hash(hasher);
    }
}

impl PartialOrd for Word {
    fn partial_cmp(&self, word: &Self) -> Option<Ordering> {
        let order = self.cmp(word);
        Some(order)
    }
}

impl Ord for Word {
    fn cmp(&self, word: &Self) -> Ordering {
        let lower_case_text = self.lower_case_text();
        let word_lower_case_text = word.lower_case_text();
        lower_case_text.cmp(&word_lower_case_text)
    }
}

impl fmt::Display for Word {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { text } = self;
        fmt::Display::fmt(text, formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn the_same_word_in_two_writings_reaches_one_entry_of_a_table() {
        let lower_case_word_text = "seat";
        let lower_case_word = Word::try_from(lower_case_word_text).unwrap();
        let spelling_word_text = "Seat";
        let spelling_word = Word::try_from(spelling_word_text).unwrap();
        let words = [lower_case_word, spelling_word];

        let word_set: HashSet<Word> = words.into_iter().collect();

        let word_count = word_set.len();
        let expectation: usize = 1;
        assert_eq!(word_count, expectation);
    }

    #[test]
    fn two_writings_of_the_same_word_are_the_same_word() {
        let spelling_word_text = "TREASURE";
        let lower_case_word_text = "treasure";
        let spelling_word = Word::try_from(spelling_word_text).unwrap();

        let lower_case_word = Word::try_from(lower_case_word_text).unwrap();

        assert_eq!(spelling_word, lower_case_word);
    }

    #[test]
    fn two_different_words_are_not_the_same_word() {
        let first_word_text = "chest";
        let next_word_text = "treasure";
        let first_word = Word::try_from(first_word_text).unwrap();

        let next_word = Word::try_from(next_word_text).unwrap();

        assert_ne!(first_word, next_word);
    }

    #[test]
    fn two_writings_of_the_same_word_are_ordered_together() {
        let spelling_word_text = "TREASURE";
        let spelling_word = Word::try_from(spelling_word_text).unwrap();
        let lower_case_word_text = "treasure";
        let lower_case_word = Word::try_from(lower_case_word_text).unwrap();

        let ordering = spelling_word.partial_cmp(&lower_case_word);

        let ordering_expectation = Ordering::Equal;
        let expectation = Some(ordering_expectation);
        assert_eq!(ordering, expectation);
    }

    #[test]
    fn words_are_ordered_by_their_letters_and_not_by_their_case() {
        let first_word_text = "Chest";
        let first_word = Word::try_from(first_word_text).unwrap();
        let next_word_text = "treasure";
        let next_word = Word::try_from(next_word_text).unwrap();

        let ordering = first_word.cmp(&next_word);

        let expectation = Ordering::Less;
        assert_eq!(ordering, expectation);
    }

    #[test]
    fn a_word_is_shown_in_the_case_it_came_in() {
        let word_text = "Plunder";
        let word = Word::try_from(word_text).unwrap();

        let text = word.to_string();

        assert_eq!(text, word_text);
    }

    #[test]
    fn a_word_may_end_in_digits() {
        let text = "ak47";

        let word = Word::try_from(text);

        let error = word.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }

    #[test]
    fn a_text_of_digits_alone_is_an_error() {
        let text = "40135";

        let word = Word::try_from(text);

        let word_text_literal = "40135";
        let word_text = String::from(word_text_literal);
        let error = Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(word, expectation);
    }

    #[test]
    fn an_empty_text_is_an_error() {
        let text = "";

        let word = Word::try_from(text);

        let word_text = String::new();
        let error = Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(word, expectation);
    }

    #[test]
    fn a_text_of_two_words_apart_by_a_space_is_an_error() {
        let text = "clan tag";

        let word = Word::try_from(text);

        let word_text_literal = "clan tag";
        let word_text = String::from(word_text_literal);
        let error = Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(word, expectation);
    }

    #[test]
    fn a_text_with_an_underscore_is_an_error() {
        let text = "season_profile";

        let word = Word::try_from(text);

        let word_text_literal = "season_profile";
        let word_text = String::from(word_text_literal);
        let error = Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(word, expectation);
    }

    #[test]
    fn a_text_with_a_letter_outside_the_alphabet_of_a_word_is_an_error() {
        let text = "café";

        let word = Word::try_from(text);

        let word_text_literal = "café";
        let word_text = String::from(word_text_literal);
        let error = Error::Word { text: word_text };
        let expectation = Err(error);
        assert_eq!(word, expectation);
    }

    #[test]
    fn a_word_in_the_plural_stems_from_the_word_without_its_plural_ending() {
        let word_text = "letters";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["letter"];
        assert_eq!(stem_texts, expectation);
    }

    #[test]
    fn a_word_with_the_wye_plural_ending_stems_from_the_word_with_the_wye_ending() {
        let word_text = "entries";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["entry"];
        assert_eq!(stem_texts, expectation);
    }

    #[test]
    fn a_word_with_the_sibilant_plural_ending_after_a_sibilant_letter_stems_from_its_sibilant_stem_and_its_singular()
     {
        let word_text = "matches";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["match", "matche"];
        assert_eq!(stem_texts, expectation);
    }

    #[test]
    fn a_word_with_the_sibilant_plural_ending_after_a_sibilant_letter_stems_from_its_singular_as_well()
     {
        let word_text = "phrases";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["phras", "phrase"];
        assert_eq!(stem_texts, expectation);
    }

    #[test]
    fn a_word_with_the_sibilant_plural_ending_after_no_sibilant_letter_stems_from_its_singular_alone()
     {
        let word_text = "notes";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["note"];
        assert_eq!(stem_texts, expectation);
    }

    #[test]
    fn a_word_in_upper_case_stems_from_its_singular_in_lower_case() {
        let word_text = "NAMES";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["name"];
        assert_eq!(stem_texts, expectation);
    }

    #[test]
    fn a_word_with_the_double_letter_ending_stems_from_no_word() {
        let word_text = "success";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let expectation = Stems::default();
        assert_eq!(stems, expectation);
    }

    #[test]
    fn a_word_without_the_plural_ending_stems_from_no_word() {
        let word_text = "letter";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let expectation = Stems::default();
        assert_eq!(stems, expectation);
    }

    #[test]
    fn the_plural_ending_alone_stems_from_no_word() {
        let word_text = "s";
        let word = Word::try_from(word_text).unwrap();

        let stems = word.compose_stems();

        let expectation = Stems::default();
        assert_eq!(stems, expectation);
    }
}
