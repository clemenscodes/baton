mod parser;

pub mod test_boundary;

pub use parser::Parser;

use crate::test_boundary::TestBoundary;
use gates_coverage_core_span::Span;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::IntoIter;
use std::ops::BitAnd;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct TestBoundaries {
    test_boundaries: BTreeSet<TestBoundary>,
}

impl Layer for TestBoundaries {
    type Layer = DomainLayer;
}

impl ValueObject for TestBoundaries {}

impl Specification<Span> for TestBoundaries {
    fn decide_acceptance(&self, span: &Span) -> CandidateDecision {
        let Self { test_boundaries } = self;
        let test_boundary_decision = |test_boundary: &TestBoundary| {
            test_boundary.decide_acceptance(span)
        };
        let acceptance = CandidateDecision::Acceptance;
        test_boundaries
            .iter()
            .map(test_boundary_decision)
            .fold(acceptance, BitAnd::bitand)
    }
}

impl FromIterator<TestBoundary> for TestBoundaries {
    fn from_iter<TestBoundariesType: IntoIterator<Item = TestBoundary>>(
        test_boundary_sequence: TestBoundariesType,
    ) -> Self {
        let test_boundaries = BTreeSet::from_iter(test_boundary_sequence);
        Self { test_boundaries }
    }
}

impl IntoIterator for TestBoundaries {
    type Item = TestBoundary;
    type IntoIter = IntoIter<TestBoundary>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { test_boundaries } = self;
        test_boundaries.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_boundary;
    use std::path::PathBuf;

    mod fixture {
        pub(super) fn span_factory(start_line: usize) -> gates_coverage_core_span::Span {
            let path_text = "crates/harbor/src/lib.rs";
            let path = std::path::PathBuf::from(path_text);
            let start_column: usize = 1;
            let end_column: usize = 2;
            let start_position = gates_coverage_core_span::position::Position::try_from((
                start_line,
                start_column,
            ))
            .unwrap();
            let end_position = gates_coverage_core_span::position::Position::try_from((
                start_line, end_column,
            ))
            .unwrap();
            gates_coverage_core_span::Span::try_from((path, start_position, end_position)).unwrap()
        }
    }

    #[test]
    fn a_span_in_a_test_module_of_its_source_lies_in_the_test_module() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 7;
        let closing_line: usize = 9;
        let test_boundary = test_boundary::tests::fixture::test_boundary_factory();
        let span_test_boundary = TestBoundary::try_from((path, opening_line, closing_line))
            .unwrap();
        let test_boundary_sequence = [test_boundary, span_test_boundary];
        let test_boundaries = TestBoundaries::from_iter(test_boundary_sequence);
        let start_line: usize = 8;
        let span = fixture::span_factory(start_line);

        let decision = test_boundaries.decide_acceptance(&span);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_span_outside_every_test_module_of_its_source_lies_in_production() {
        let test_boundary = test_boundary::tests::fixture::test_boundary_factory();
        let test_boundary_sequence = [test_boundary];
        let test_boundaries = TestBoundaries::from_iter(test_boundary_sequence);
        let start_line: usize = 6;
        let span = fixture::span_factory(start_line);

        let decision = test_boundaries.decide_acceptance(&span);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_span_of_a_source_without_a_test_boundary_lies_in_production() {
        let test_boundaries = TestBoundaries::default();
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);

        let decision = test_boundaries.decide_acceptance(&span);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn test_boundaries_yield_every_test_boundary_once_in_its_order() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 1;
        let closing_line: usize = 1;
        let test_boundary = test_boundary::tests::fixture::test_boundary_factory();
        let first_line_test_boundary = TestBoundary::try_from((path, opening_line, closing_line))
            .unwrap();
        let source_test_boundaries = [&test_boundary, &first_line_test_boundary, &test_boundary];
        let test_boundaries: TestBoundaries = source_test_boundaries.into_iter().cloned().collect();

        let test_boundary_sequence = Vec::from_iter(test_boundaries);

        let expectation = vec![first_line_test_boundary, test_boundary];
        assert_eq!(test_boundary_sequence, expectation);
    }
}
