mod decoder;
mod file;
mod paths;

pub use decoder::Decoder;
pub use file::ReportFile;

use crate::paths::Paths;
use gates_coverage_core_region::Region;
use gates_coverage_core_span::Span;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeMap;
use std::collections::btree_map::Values;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Report {
    regions: BTreeMap<Span, Region>,
}

impl Layer for Report {
    type Layer = DomainLayer;
}

impl ValueObject for Report {}

impl Report {
    pub fn list_paths(&self) -> Paths {
        let Self { regions } = self;
        let mut path_sequence = Vec::new();
        for span in regions.keys() {
            let path = span.path().to_path_buf();
            path_sequence.push(path);
        }
        Paths::from_iter(path_sequence)
    }
}

impl FromIterator<Region> for Report {
    fn from_iter<RegionsType: IntoIterator<Item = Region>>(region_sequence: RegionsType) -> Self {
        let mut regions = BTreeMap::new();
        for region in region_sequence {
            let span = region.span().clone();
            let execution_count = region.execution_count();
            let first_region = || region.clone();
            let stored_region = regions.entry(span).or_insert_with(first_region);
            let stored_region_execution_count = stored_region.execution_count();
            if execution_count > stored_region_execution_count {
                *stored_region = region;
            }
        }
        Self { regions }
    }
}

impl<'report> IntoIterator for &'report Report {
    type Item = &'report Region;
    type IntoIter = Values<'report, Span, Region>;

    fn into_iter(self) -> Self::IntoIter {
        let Report { regions } = self;
        regions.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

        pub(super) fn region_factory(execution_count: usize) -> gates_coverage_core_region::Region {
            let start_line: usize = 3;
            let span = span_factory(start_line);
            gates_coverage_core_region::Region::from((span, execution_count))
        }
    }

    #[test]
    fn a_report_keeps_the_copy_of_a_region_that_counts_highest() {
        let execution_counts: [usize; 3] = [0, 4, 0];
        let regions = execution_counts.map(fixture::region_factory);

        let report = Report::from_iter(regions);

        let region_sequence = Vec::from_iter(&report);
        let execution_count: usize = 4;
        let expectation_region = fixture::region_factory(execution_count);
        let expectation = vec![&expectation_region];
        assert_eq!(region_sequence, expectation);
    }

    #[test]
    fn a_report_names_every_path_of_its_regions_once() {
        let start_lines: [usize; 2] = [3, 4];
        let spans = start_lines.map(fixture::span_factory);
        let main_path_spans = start_lines.map(fixture::main_path_span_factory);
        let execution_count: usize = 0;
        let span_sequence = spans.into_iter().chain(main_path_spans);
        let region = |span| Region::from((span, execution_count));
        let regions = span_sequence.map(region);
        let report = Report::from_iter(regions);

        let paths = report.list_paths();

        let path_text = "crates/harbor/src/lib.rs";
        let main_path_text = "crates/harbor/src/main.rs";
        let path = PathBuf::from(path_text);
        let main_path = PathBuf::from(main_path_text);
        let path_sequence = [path, main_path];
        let expectation = Paths::from_iter(path_sequence);
        assert_eq!(paths, expectation);
    }

    #[test]
    fn a_report_that_keeps_no_region_names_no_path() {
        let report = Report::default();

        let paths = report.list_paths();

        let expectation = Paths::default();
        assert_eq!(paths, expectation);
    }
}
