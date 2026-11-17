mod export_region;
mod kind;

use crate::error::Error;
use crate::export::data::functions::function::export_regions::export_region::ExportRegion;
use crate::export::data::functions::function::export_regions::kind::Kind;
use crate::export::data::functions::function::filenames::Filenames;
use crate::regions::Regions;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
#[serde(from = "Vec<ExportRegion>")]
pub(super) struct ExportRegions {
    export_regions: Vec<ExportRegion>,
}

impl Layer for ExportRegions {
    type Layer = InfrastructureLayer;
}

impl ExportRegions {
    pub(super) fn try_decode_regions(&self, filenames: &Filenames) -> Result<Regions, Error> {
        let Self { export_regions } = self;
        let mut region_sequence = Vec::new();
        for export_region in export_regions {
            let kind = export_region.kind();
            match kind {
                Kind::Code => {
                    let region = export_region.try_decode_region(filenames)?;
                    region_sequence.push(region);
                }
                Kind::Annotation => {}
            }
        }
        let regions = Regions::from_iter(region_sequence);
        Ok(regions)
    }
}

impl From<Vec<ExportRegion>> for ExportRegions {
    fn from(export_regions: Vec<ExportRegion>) -> Self {
        Self { export_regions }
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
    fn every_code_region_is_a_region_of_the_file_it_names() {
        let export_regions_text = "[[3, 1, 3, 2, 7, 1, 0, 0], [5, 1, 5, 2, 0, 0, 0, 0]]";
        let filenames_text = r#"["crates/harbor/src/main.rs", "crates/harbor/src/lib.rs"]"#;
        let export_regions: ExportRegions = serde_json::from_str(export_regions_text).unwrap();
        let filenames: Filenames = serde_json::from_str(filenames_text).unwrap();

        let regions = export_regions.try_decode_regions(&filenames).unwrap();

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
    fn an_annotation_region_is_passed_over() {
        let export_regions_text = "[[3, 1, 3, 2, 0, 0, 0, 1]]";
        let filenames_text = r#"["crates/harbor/src/lib.rs"]"#;
        let export_regions: ExportRegions = serde_json::from_str(export_regions_text).unwrap();
        let filenames: Filenames = serde_json::from_str(filenames_text).unwrap();

        let regions = export_regions.try_decode_regions(&filenames).unwrap();

        let expectation = Regions::default();
        assert_eq!(regions, expectation);
    }

    #[test]
    fn a_code_region_of_a_file_its_function_lacks_is_an_error() {
        let export_regions_text = "[[3, 1, 3, 2, 0, 1, 0, 0]]";
        let filenames_text = r#"["crates/harbor/src/lib.rs"]"#;
        let export_regions: ExportRegions = serde_json::from_str(export_regions_text).unwrap();
        let filenames: Filenames = serde_json::from_str(filenames_text).unwrap();

        let regions = export_regions.try_decode_regions(&filenames);

        let error_text = regions.unwrap_err().to_string();
        let expectation = "[COVERAGE] a region refers to file index 1, but the file indexes of its function end before 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_code_region_that_is_no_span_is_an_error() {
        let export_regions_text = "[[4, 1, 3, 9, 0, 0, 0, 0]]";
        let filenames_text = r#"["crates/harbor/src/lib.rs"]"#;
        let export_regions: ExportRegions = serde_json::from_str(export_regions_text).unwrap();
        let filenames: Filenames = serde_json::from_str(filenames_text).unwrap();

        let regions = export_regions.try_decode_regions(&filenames);

        let error_text = regions.unwrap_err().to_string();
        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:4:1: the span ends at 3:9, before it starts.";
        assert_eq!(error_text, expectation);
    }
}
