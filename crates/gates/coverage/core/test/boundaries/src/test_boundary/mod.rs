mod error;

pub use error::Error;

use gates_coverage_core_span::Span;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TestBoundary {
    path: PathBuf,
    opening_line: usize,
    closing_line: usize,
}

impl Layer for TestBoundary {
    type Layer = DomainLayer;
}

impl ValueObject for TestBoundary {}

impl TestBoundary {
    const FIRST_LINE: usize = 1;
}

impl Specification<Span> for TestBoundary {
    fn decide_acceptance(&self, span: &Span) -> CandidateDecision {
        let Self {
            path,
            opening_line,
            closing_line,
        } = self;
        let span_path = span.path();
        let start_position = span.start_position();
        let start_line = start_position.line();
        if span_path != path || start_line < *opening_line || start_line > *closing_line {
            CandidateDecision::Acceptance
        } else {
            CandidateDecision::Rejection
        }
    }
}

impl TryFrom<(PathBuf, usize, usize)> for TestBoundary {
    type Error = Error;

    fn try_from(
        (path, opening_line, closing_line): (PathBuf, usize, usize),
    ) -> Result<Self, Self::Error> {
        let first_line = Self::FIRST_LINE;
        if opening_line < first_line || closing_line < opening_line {
            let line_error = Error::Line {
                path,
                opening_line,
                closing_line,
            };
            return Err(line_error);
        }
        let test_boundary = Self {
            path,
            opening_line,
            closing_line,
        };
        Ok(test_boundary)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) mod fixture {
        use super::*;

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

        pub(super) fn main_path_span_factory(start_line: usize) -> gates_coverage_core_span::Span {
            let path_text = "crates/harbor/src/main.rs";
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

        pub(crate) fn test_boundary_factory() -> TestBoundary {
            let path_text = "crates/harbor/src/lib.rs";
            let path = PathBuf::from(path_text);
            let opening_line: usize = 2;
            let closing_line: usize = 5;
            TestBoundary::try_from((path, opening_line, closing_line)).unwrap()
        }
    }

    #[test]
    fn a_test_boundary_that_opens_and_closes_on_the_first_line_is_a_test_boundary() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 1;
        let closing_line: usize = 1;

        let test_boundary = TestBoundary::try_from((path, opening_line, closing_line));

        let error = test_boundary.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }

    #[test]
    fn a_test_boundary_that_opens_on_the_line_zero_is_an_error() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 0;
        let closing_line: usize = 3;

        let test_boundary = TestBoundary::try_from((path, opening_line, closing_line));

        let expectation_path = PathBuf::from(path_text);
        let line_error = Error::Line {
            path: expectation_path,
            opening_line,
            closing_line,
        };
        let expectation = Err(line_error);
        assert_eq!(test_boundary, expectation);
    }

    #[test]
    fn a_test_boundary_that_closes_before_it_opens_is_an_error() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 5;
        let closing_line: usize = 3;

        let test_boundary = TestBoundary::try_from((path, opening_line, closing_line));

        let expectation_path = PathBuf::from(path_text);
        let line_error = Error::Line {
            path: expectation_path,
            opening_line,
            closing_line,
        };
        let expectation = Err(line_error);
        assert_eq!(test_boundary, expectation);
    }

    #[test]
    fn a_span_before_the_test_module_lies_in_production() {
        let test_boundary = fixture::test_boundary_factory();
        let start_line: usize = 1;
        let span = fixture::span_factory(start_line);

        let decision = test_boundary.decide_acceptance(&span);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_span_on_the_line_of_the_test_attribute_lies_in_the_test_module() {
        let test_boundary = fixture::test_boundary_factory();
        let start_line: usize = 2;
        let span = fixture::span_factory(start_line);

        let decision = test_boundary.decide_acceptance(&span);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_span_on_the_line_that_closes_the_test_module_lies_in_the_test_module() {
        let test_boundary = fixture::test_boundary_factory();
        let start_line: usize = 5;
        let span = fixture::span_factory(start_line);

        let decision = test_boundary.decide_acceptance(&span);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_span_after_the_test_module_lies_in_production() {
        let test_boundary = fixture::test_boundary_factory();
        let start_line: usize = 6;
        let span = fixture::span_factory(start_line);

        let decision = test_boundary.decide_acceptance(&span);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_span_of_another_source_lies_in_production() {
        let test_boundary = fixture::test_boundary_factory();
        let start_line: usize = 3;
        let span = fixture::main_path_span_factory(start_line);

        let decision = test_boundary.decide_acceptance(&span);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }
}
