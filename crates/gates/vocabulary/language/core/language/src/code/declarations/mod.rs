use crate::occurrence::Occurrence;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Declarations {
    occurrences: BTreeSet<Occurrence>,
}

impl Layer for Declarations {
    type Layer = DomainLayer;
}

impl ValueObject for Declarations {}

impl FromIterator<Occurrence> for Declarations {
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

impl<'declarations> IntoIterator for &'declarations Declarations {
    type Item = &'declarations Occurrence;
    type IntoIter = Iter<'declarations, Occurrence>;

    fn into_iter(self) -> Self::IntoIter {
        let Declarations { occurrences } = self;
        occurrences.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_occurrence_without_a_word_is_no_declaration() {
        let text = "_";
        let line: usize = 1;
        let column: usize = 5;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();
        let occurrence_list = [occurrence];

        let declarations: Declarations = occurrence_list.into_iter().collect();

        let expectation = Declarations::default();
        assert_eq!(declarations, expectation);
    }

    #[test]
    fn the_declarations_hold_every_declared_occurrence_once() {
        let text = "treasure";
        let line: usize = 1;
        let column: usize = 5;
        let occurrence = Occurrence::try_from((text, line, column)).unwrap();
        let occurrence_copy = occurrence.clone();
        let occurrence_list = [occurrence, occurrence_copy];

        let declarations: Declarations = occurrence_list.into_iter().collect();

        let occurrence_count = declarations.into_iter().count();
        let expectation: usize = 1;
        assert_eq!(occurrence_count, expectation);
    }
}
