use gates_coverage_core_span::Span;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Gap {
    span: Span,
}

impl Layer for Gap {
    type Layer = DomainLayer;
}

impl ValueObject for Gap {}

impl From<Span> for Gap {
    fn from(span: Span) -> Self {
        Self { span }
    }
}

impl fmt::Display for Gap {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { span } = self;
        write!(formatter, "[COVERAGE] {span}: no test covers this region.")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
    fn a_gap_names_where_its_span_starts() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let gap = Gap::from(span);

        let gap_text = gap.to_string();

        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:3:1: no test covers this region.";
        assert_eq!(gap_text, expectation);
    }
}
