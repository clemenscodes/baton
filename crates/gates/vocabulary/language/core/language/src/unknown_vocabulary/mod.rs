use crate::occurrence::Occurrence;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::CandidateDecision;
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;
use std::ops::BitOr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct UnknownVocabulary {
    occurrences: BTreeSet<Occurrence>,
}

impl Layer for UnknownVocabulary {
    type Layer = DomainLayer;
}

impl ValueObject for UnknownVocabulary {}

impl UnknownVocabulary {
    pub fn decide_acceptance(&self) -> CandidateDecision {
        let Self { occurrences } = self;
        match occurrences.is_empty() {
            true => CandidateDecision::Acceptance,
            false => CandidateDecision::Rejection,
        }
    }
}

impl FromIterator<Occurrence> for UnknownVocabulary {
    fn from_iter<OccurrencesType: IntoIterator<Item = Occurrence>>(
        occurrence_sequence: OccurrencesType,
    ) -> Self {
        let occurrences = occurrence_sequence.into_iter().collect();
        Self { occurrences }
    }
}

impl BitOr for &UnknownVocabulary {
    type Output = UnknownVocabulary;

    fn bitor(self, unknown_vocabulary: Self) -> Self::Output {
        let UnknownVocabulary {
            occurrences: augend_occurrences,
        } = self;
        let UnknownVocabulary {
            occurrences: addend_occurrences,
        } = unknown_vocabulary;
        augend_occurrences
            .union(addend_occurrences)
            .cloned()
            .collect()
    }
}

impl<'unknown_vocabulary> IntoIterator for &'unknown_vocabulary UnknownVocabulary {
    type Item = &'unknown_vocabulary Occurrence;
    type IntoIter = Iter<'unknown_vocabulary, Occurrence>;

    fn into_iter(self) -> Self::IntoIter {
        let UnknownVocabulary { occurrences } = self;
        occurrences.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_unknown_vocabulary_without_an_occurrence_admits_its_source() {
        let unknown_vocabulary = UnknownVocabulary::default();

        let decision = unknown_vocabulary.decide_acceptance();

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn an_unknown_vocabulary_with_an_occurrence_rejects_its_source() {
        let text = "treasure";
        let line: usize = 1;
        let column: usize = 8;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();
        let occurrences = [occurrence];
        let unknown_vocabulary: UnknownVocabulary = occurrences.into_iter().collect();

        let decision = unknown_vocabulary.decide_acceptance();

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn an_unknown_vocabulary_yields_its_occurrences_in_the_order_of_their_lines_and_columns() {
        let first_text = "Anchor";
        let last_text = "treasure_chest";
        let first_line: usize = 1;
        let last_line: usize = 2;
        let column: usize = 8;
        let first_occurrence = Occurrence::try_from((first_text, first_line, column)).unwrap();
        let last_occurrence = Occurrence::try_from((last_text, last_line, column)).unwrap();
        let occurrences = [last_occurrence, first_occurrence];
        let unknown_vocabulary: UnknownVocabulary = occurrences.into_iter().collect();

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();

        let expectation = ["1:8: `Anchor`", "2:8: `treasure_chest`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn two_joined_unknown_vocabularies_hold_every_occurrence_of_both_once() {
        let first_text = "anchor";
        let last_text = "treasure";
        let line: usize = 1;
        let first_column: usize = 1;
        let last_column: usize = 8;
        let augend_first_occurrence = Occurrence::try_from((first_text, line, first_column))
            .unwrap();
        let addend_first_occurrence = Occurrence::try_from((first_text, line, first_column))
            .unwrap();
        let last_occurrence = Occurrence::try_from((last_text, line, last_column)).unwrap();
        let augend_occurrences = [augend_first_occurrence];
        let addend_occurrences = [addend_first_occurrence, last_occurrence];
        let augend: UnknownVocabulary = augend_occurrences.into_iter().collect();
        let addend: UnknownVocabulary = addend_occurrences.into_iter().collect();

        let unknown_vocabulary = &augend | &addend;

        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:1: `anchor`", "1:8: `treasure`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }
}
