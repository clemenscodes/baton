use gates_coverage_core_region::Region;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct Regions {
    regions: Vec<Region>,
}

impl Layer for Regions {
    type Layer = InfrastructureLayer;
}

impl FromIterator<Region> for Regions {
    fn from_iter<RegionsType: IntoIterator<Item = Region>>(region_sequence: RegionsType) -> Self {
        let regions = Vec::from_iter(region_sequence);
        Self { regions }
    }
}

impl IntoIterator for Regions {
    type Item = Region;
    type IntoIter = IntoIter<Region>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { regions } = self;
        regions.into_iter()
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
    fn regions_yield_the_sequence_they_come_from_in_its_order() {
        let start_lines: [usize; 2] = [5, 3];
        let spans = start_lines.map(fixture::span_factory);
        let execution_count: usize = 0;
        let region = |span| Region::from((span, execution_count));
        let regions: Regions = spans.iter().cloned().map(region).collect();

        let region_sequence = Vec::from_iter(regions);

        let expectation = spans.map(region);
        assert_eq!(region_sequence, expectation);
    }
}
