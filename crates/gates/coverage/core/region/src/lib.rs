use gates_coverage_core_span::Span;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Region {
    span: Span,
    execution_count: usize,
}

impl Layer for Region {
    type Layer = DomainLayer;
}

impl ValueObject for Region {}

impl Region {
    pub fn span(&self) -> &Span {
        &self.span
    }

    pub fn execution_count(&self) -> usize {
        self.execution_count
    }
}

impl From<(Span, usize)> for Region {
    fn from((span, execution_count): (Span, usize)) -> Self {
        Self {
            span,
            execution_count,
        }
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
    fn a_region_keeps_its_span() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let span_expectation = span.clone();
        let execution_count: usize = 7;
        let region = Region::from((span, execution_count));

        let region_span = region.span();

        assert_eq!(region_span, &span_expectation);
    }

    #[test]
    fn a_region_keeps_how_often_it_ran() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 7;
        let region = Region::from((span, execution_count));

        let region_execution_count = region.execution_count();

        assert_eq!(region_execution_count, execution_count);
    }
}
