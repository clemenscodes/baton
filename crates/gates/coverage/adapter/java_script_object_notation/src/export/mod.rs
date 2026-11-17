mod data;

use crate::error::Error;
use crate::export::data::Data;
use crate::regions::Regions;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
pub(super) struct Export {
    data: Vec<Data>,
}

impl Layer for Export {
    type Layer = InfrastructureLayer;
}

impl Export {
    pub(super) fn try_decode_regions(&self) -> Result<Regions, Error> {
        let Self {
            data: data_sequence,
        } = self;
        let mut region_sequence = Vec::new();
        for data in data_sequence {
            let data_regions = data.try_decode_regions()?;
            region_sequence.extend(data_regions);
        }
        let regions = Regions::from_iter(region_sequence);
        Ok(regions)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_coverage_core_region::Region;

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
    }

    #[test]
    fn the_regions_of_every_data_are_the_regions_of_the_export() {
        let text = r#"{"data": [
            {"files": [{"filename": "crates/harbor/src/lib.rs"}], "functions": [{"filenames": ["crates/harbor/src/lib.rs"], "regions": [[3, 1, 3, 2, 7, 0, 0, 0]]}]},
            {"files": [{"filename": "crates/harbor/src/main.rs"}], "functions": [{"filenames": ["crates/harbor/src/main.rs"], "regions": [[5, 1, 5, 2, 0, 0, 0, 0]]}]}
        ]}"#;
        let export: Export = serde_json::from_str(text).unwrap();

        let regions = export.try_decode_regions().unwrap();

        let start_line: usize = 3;
        let main_path_start_line: usize = 5;
        let span = fixture::span_factory(start_line);
        let main_path_span = fixture::main_path_span_factory(main_path_start_line);
        let execution_count: usize = 7;
        let main_path_execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let main_path_region = Region::from((main_path_span, main_path_execution_count));
        let region_sequence = [region, main_path_region];
        let expectation = Regions::from_iter(region_sequence);
        assert_eq!(regions, expectation);
    }

    #[test]
    fn an_export_with_a_data_whose_regions_are_an_error_is_an_error() {
        let text = r#"{"data": [{"files": [], "functions": [{"filenames": [], "regions": [[3, 1, 3, 2, 7, 0, 0, 0]]}]}]}"#;
        let export: Export = serde_json::from_str(text).unwrap();

        let regions = export.try_decode_regions();

        let error_text = regions.unwrap_err().to_string();
        let expectation = "[COVERAGE] a region refers to file index 0, but the file indexes of its function end before 0.";
        assert_eq!(error_text, expectation);
    }
}
