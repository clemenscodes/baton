use crate::occurrence::Occurrence;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Identifiers {
    occurrences: BTreeSet<Occurrence>,
}

impl Layer for Identifiers {
    type Layer = DomainLayer;
}

impl ValueObject for Identifiers {}

impl FromIterator<Occurrence> for Identifiers {
    fn from_iter<OccurrencesType: IntoIterator<Item = Occurrence>>(
        occurrence_sequence: OccurrencesType,
    ) -> Self {
        let occurrence_criterion = |occurrence: &Occurrence| {
            !occurrence.identifier().as_ref().is_empty()
        };
        let occurrences = occurrence_sequence
            .into_iter()
            .filter(occurrence_criterion)
            .collect();
        Self { occurrences }
    }
}

impl<'identifiers> IntoIterator for &'identifiers Identifiers {
    type Item = &'identifiers Occurrence;
    type IntoIter = Iter<'identifiers, Occurrence>;

    fn into_iter(self) -> Self::IntoIter {
        let Identifiers { occurrences } = self;
        occurrences.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_occurrence_without_a_word_is_no_identifier_of_the_identifiers() {
        let text = "_";
        let line: usize = 1;
        let column: usize = 5;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();
        let occurrence_list = [occurrence];

        let identifiers: Identifiers = occurrence_list.into_iter().collect();

        let expectation = Identifiers::default();
        assert_eq!(identifiers, expectation);
    }

    #[test]
    fn the_identifiers_hold_every_occurrence_of_words_once() {
        let text = "treasure";
        let line: usize = 1;
        let column: usize = 5;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();
        let occurrence_copy = occurrence.clone();
        let occurrence_list = [occurrence, occurrence_copy];

        let identifiers: Identifiers = occurrence_list.into_iter().collect();

        let occurrence_count = identifiers.into_iter().count();
        let expectation: usize = 1;
        assert_eq!(occurrence_count, expectation);
    }
}
