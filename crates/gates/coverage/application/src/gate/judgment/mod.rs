use crate::Outcome;
use gates_coverage_core_gaps::Gaps;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Judgment {
    gaps: Gaps,
}

impl Layer for Judgment {
    type Layer = ApplicationLayer;
}

impl Judgment {
    const SUCCESS_TEXT: &'static str = "[COVERAGE] every region outside the test modules is covered by a test.";

    const LINE_SEPARATOR: &'static str = "\n";

    pub fn decide_outcome(&self) -> Outcome {
        let Self { gaps } = self;
        let gap_count = gaps.into_iter().count();
        if gap_count == 0 {
            Outcome::Success
        } else {
            Outcome::Error
        }
    }
}

impl From<Gaps> for Judgment {
    fn from(gaps: Gaps) -> Self {
        Self { gaps }
    }
}

impl fmt::Display for Judgment {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { gaps } = self;
        let outcome = self.decide_outcome();
        match outcome {
            Outcome::Success => {
                let success_text = Self::SUCCESS_TEXT;
                write!(formatter, "{success_text}")
            }
            Outcome::Error => {
                let gap_lines: Vec<String> = gaps.into_iter().map(ToString::to_string).collect();
                let gap_text = gap_lines.join(Self::LINE_SEPARATOR);
                write!(formatter, "{gap_text}")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_coverage_core_region::Region;
    use gates_coverage_core_report::Report;
    use gates_coverage_core_test_boundaries::TestBoundaries;

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
    fn no_gap_is_a_success() {
        let gaps = Gaps::default();
        let judgment = Judgment::from(gaps);

        let outcome = judgment.decide_outcome();

        let expectation = Outcome::Success;
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn a_gap_is_an_error() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let regions = [region];
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();
        let gaps = Gaps::from((&report, &test_boundaries));
        let judgment = Judgment::from(gaps);

        let outcome = judgment.decide_outcome();

        let expectation = Outcome::Error;
        assert_eq!(outcome, expectation);
    }

    #[test]
    fn a_judgment_of_no_gap_reads_as_the_success_and_of_gaps_as_a_line_for_each() {
        let gaps = Gaps::default();
        let success_judgment = Judgment::from(gaps);
        let regions = [3, 5].map(|start_line| {
            let span = fixture::span_factory(start_line);
            let execution_count: usize = 0;
            Region::from((span, execution_count))
        });
        let report = Report::from_iter(regions);
        let test_boundaries = TestBoundaries::default();
        let report_gaps = Gaps::from((&report, &test_boundaries));
        let gap_judgment = Judgment::from(report_gaps);

        let success_text = success_judgment.to_string();
        let gap_text = gap_judgment.to_string();

        let success_expectation = "[COVERAGE] every region outside the test modules is covered by a test.";
        let gap_expectation = r#"[COVERAGE] crates/harbor/src/lib.rs:3:1: no test covers this region.
[COVERAGE] crates/harbor/src/lib.rs:5:1: no test covers this region."#;
        assert_eq!(success_text, success_expectation);
        assert_eq!(gap_text, gap_expectation);
    }
}
