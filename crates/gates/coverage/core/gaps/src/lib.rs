mod gap;

use crate::gap::Gap;
use gates_coverage_core_report::Report;
use gates_coverage_core_test_boundaries::TestBoundaries;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Gaps {
    gaps: Vec<Gap>,
}

impl Layer for Gaps {
    type Layer = DomainLayer;
}

impl ValueObject for Gaps {}

impl Gaps {
    const MAIN_PATH: &'static str = "src/main.rs";
}

impl From<(&Report, &TestBoundaries)> for Gaps {
    fn from((report, test_boundaries): (&Report, &TestBoundaries)) -> Self {
        let main_path = Self::MAIN_PATH;
        let mut gaps = Vec::new();
        for region in report {
            let execution_count = region.execution_count();
            let span = region.span();
            let path = span.path();
            if execution_count > 0 || path.ends_with(main_path) {
                continue;
            }
            let decision = test_boundaries.decide_acceptance(span);
            match decision {
                CandidateDecision::Acceptance => {
                    let gap_span = span.clone();
                    let gap = Gap::from(gap_span);
                    gaps.push(gap);
                }
                CandidateDecision::Rejection => {}
            }
        }
        Self { gaps }
    }
}

impl<'gaps> IntoIterator for &'gaps Gaps {
    type Item = &'gaps Gap;
    type IntoIter = Iter<'gaps, Gap>;

    fn into_iter(self) -> Self::IntoIter {
        let Gaps { gaps } = self;
        gaps.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_coverage_core_region::Region;
    use gates_coverage_core_span::Span;
    use gates_coverage_core_span::position::Position;
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

        pub(super) fn region_factory(execution_count: usize) -> gates_coverage_core_region::Region {
            let start_line: usize = 3;
            let span = span_factory(start_line);
            gates_coverage_core_region::Region::from((span, execution_count))
        }

        pub(super) fn test_boundary_factory()
        -> gates_coverage_core_test_boundaries::test_boundary::TestBoundary {
            let path_text = "crates/harbor/src/lib.rs";
            let path = std::path::PathBuf::from(path_text);
            let opening_line: usize = 2;
            let closing_line: usize = 5;
            gates_coverage_core_test_boundaries::test_boundary::TestBoundary::try_from((
                path,
                opening_line,
                closing_line,
            ))
            .unwrap()
        }
    }

    #[test]
    fn a_region_no_test_reaches_is_a_gap() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let gap_span = span.clone();
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let regions = [region];
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();

        let gaps = Gaps::from((&report, &test_boundaries));

        let gap_sequence = Vec::from_iter(&gaps);
        let gap = Gap::from(gap_span);
        let expectation = vec![&gap];
        assert_eq!(gap_sequence, expectation);
    }

    #[test]
    fn the_gaps_of_a_report_follow_the_order_of_their_spans() {
        let start_lines: [usize; 2] = [5, 3];
        let spans = start_lines.map(fixture::span_factory);
        let execution_count: usize = 0;
        let region = |span| Region::from((span, execution_count));
        let regions = spans.map(region);
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();

        let gaps = Gaps::from((&report, &test_boundaries));

        let gap_sequence = Vec::from_iter(&gaps);
        let gap_start_lines: [usize; 2] = [3, 5];
        let gap_spans = gap_start_lines.map(fixture::span_factory);
        let expectation_gaps = gap_spans.map(Gap::from);
        let expectation = Vec::from_iter(&expectation_gaps);
        assert_eq!(gap_sequence, expectation);
    }

    #[test]
    fn a_region_a_test_reaches_is_no_gap() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 2;
        let region = Region::from((span, execution_count));
        let regions = [region];
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();

        let gaps = Gaps::from((&report, &test_boundaries));

        let expectation = Gaps::default();
        assert_eq!(gaps, expectation);
    }

    #[test]
    fn a_region_whose_copy_a_test_reaches_is_no_gap() {
        let execution_counts: [usize; 3] = [0, 4, 0];
        let regions = execution_counts.map(fixture::region_factory);
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();

        let gaps = Gaps::from((&report, &test_boundaries));

        let expectation = Gaps::default();
        assert_eq!(gaps, expectation);
    }

    #[test]
    fn a_region_of_a_main_is_no_gap() {
        let path_text = "crates/harbor/src/main.rs";
        let path = PathBuf::from(path_text);
        let line: usize = 3;
        let start_column: usize = 1;
        let end_column: usize = 2;
        let start = Position::try_from((line, start_column)).unwrap();
        let end = Position::try_from((line, end_column)).unwrap();
        let span = Span::try_from((path, start, end)).unwrap();
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let regions = [region];
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();

        let gaps = Gaps::from((&report, &test_boundaries));

        let expectation = Gaps::default();
        assert_eq!(gaps, expectation);
    }

    #[test]
    fn a_region_of_the_test_module_is_no_gap() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let regions = [region];
        let report = Report::from_iter(regions);
        let test_boundary = fixture::test_boundary_factory();
        let test_boundary_sequence = [test_boundary];
        let test_boundaries = TestBoundaries::from_iter(test_boundary_sequence);

        let gaps = Gaps::from((&report, &test_boundaries));

        let expectation = Gaps::default();
        assert_eq!(gaps, expectation);
    }
}
