pub mod defect;

use crate::defects::defect::Defect;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::CandidateDecision;
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Defects {
    defects: BTreeSet<Defect>,
}

impl Layer for Defects {
    type Layer = DomainLayer;
}

impl ValueObject for Defects {}

impl Defects {
    pub fn decide_acceptance(&self) -> CandidateDecision {
        let Self { defects } = self;
        match defects.is_empty() {
            true => CandidateDecision::Acceptance,
            false => CandidateDecision::Rejection,
        }
    }
}

impl FromIterator<Defect> for Defects {
    fn from_iter<DefectsType: IntoIterator<Item = Defect>>(defect_sequence: DefectsType) -> Self {
        let defects = defect_sequence.into_iter().collect();
        Self { defects }
    }
}

impl<'defects> IntoIterator for &'defects Defects {
    type Item = &'defects Defect;
    type IntoIter = Iter<'defects, Defect>;

    fn into_iter(self) -> Self::IntoIter {
        let Defects { defects } = self;
        defects.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defects_of_no_defect_admit_their_source() {
        let defects = Defects::default();

        let decision = defects.decide_acceptance();

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn defects_of_a_defect_reject_their_source() {
        let line: usize = 2;
        let column: usize = 1;
        let comment = Defect::Comment { line, column };
        let defect_list = [comment];
        let defects: Defects = defect_list.into_iter().collect();

        let decision = defects.decide_acceptance();

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn defects_yield_every_defect_in_the_order_of_their_kinds_lines_and_columns() {
        let first_line: usize = 1;
        let next_line: usize = 2;
        let column: usize = 1;
        let last_comment = Defect::Comment {
            line: next_line,
            column,
        };
        let first_comment = Defect::Comment {
            line: first_line,
            column,
        };
        let test_attribute = Defect::TestAttribute {
            line: next_line,
            column,
        };
        let defect_list = [last_comment, first_comment, test_attribute];
        let defects: Defects = defect_list.into_iter().collect();

        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();

        let expectation = [
            "2:1: the test attribute does not open the `tests` module at the bottom of the source",
            "1:1: the `doc` attribute is a comment, and code must not contain comments",
            "2:1: the `doc` attribute is a comment, and code must not contain comments",
        ];
        assert_eq!(defect_texts, expectation);
    }
}
