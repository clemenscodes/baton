mod files;
mod functions;

use crate::error::Error;
use crate::export::data::files::Files;
use crate::export::data::functions::Functions;
use crate::regions::Regions;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
pub(super) struct Data {
    files: Files,
    functions: Functions,
}

impl Layer for Data {
    type Layer = InfrastructureLayer;
}

impl Data {
    pub(super) fn try_decode_regions(&self) -> Result<Regions, Error> {
        let Self { files, functions } = self;
        let function_regions = functions.try_decode_regions()?;
        let regions = files.select_regions(function_regions);
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
    }

    #[test]
    fn the_regions_of_every_function_in_a_file_of_the_data_are_the_regions_of_the_data() {
        let text = r#"{"files": [{"filename": "crates/harbor/src/lib.rs"}], "functions": [
            {"filenames": ["crates/harbor/src/lib.rs"], "regions": [[3, 1, 3, 2, 7, 0, 0, 0]]},
            {"filenames": ["crates/lighthouse/src/lib.rs"], "regions": [[5, 1, 5, 2, 0, 0, 0, 0]]}
        ]}"#;
        let data: Data = serde_json::from_str(text).unwrap();

        let regions = data.try_decode_regions().unwrap();

        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 7;
        let region = Region::from((span, execution_count));
        let region_sequence = [region];
        let expectation = Regions::from_iter(region_sequence);
        assert_eq!(regions, expectation);
    }

    #[test]
    fn a_data_with_a_function_whose_regions_are_an_error_is_an_error() {
        let text = r#"{"files": [], "functions": [
            {"filenames": [], "regions": [[3, 1, 3, 2, 7, 0, 0, 0]]}
        ]}"#;
        let data: Data = serde_json::from_str(text).unwrap();

        let regions = data.try_decode_regions();

        let error_text = regions.unwrap_err().to_string();
        let expectation = "[COVERAGE] a region refers to file index 0, but the file indexes of its function end before 0.";
        assert_eq!(error_text, expectation);
    }
}
