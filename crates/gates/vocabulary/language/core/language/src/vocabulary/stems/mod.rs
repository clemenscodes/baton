use crate::vocabulary::word::Word;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct Stems {
    words: Vec<Word>,
}

impl Layer for Stems {
    type Layer = DomainLayer;
}

impl ValueObject for Stems {}

impl FromIterator<Word> for Stems {
    fn from_iter<WordsType: IntoIterator<Item = Word>>(word_sequence: WordsType) -> Self {
        let words = word_sequence.into_iter().collect();
        Self { words }
    }
}

impl<'stems> IntoIterator for &'stems Stems {
    type Item = &'stems Word;
    type IntoIter = Iter<'stems, Word>;

    fn into_iter(self) -> Self::IntoIter {
        let Stems { words } = self;
        words.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn stems_yield_every_word_they_are_made_of_in_their_order() {
        let sibilant_stem_text = "match";
        let singular_text = "matche";
        let sibilant_stem = Word::try_from(sibilant_stem_text).unwrap();
        let singular = Word::try_from(singular_text).unwrap();
        let word_list = [sibilant_stem, singular];

        let stems: Stems = word_list.into_iter().collect();

        let stem_texts: Vec<String> = stems.into_iter().map(ToString::to_string).collect();
        let expectation = ["match", "matche"];
        assert_eq!(stem_texts, expectation);
    }
}
