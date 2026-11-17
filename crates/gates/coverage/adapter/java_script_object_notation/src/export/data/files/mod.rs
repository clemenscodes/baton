mod file;

use crate::export::data::files::file::File;
use crate::regions::Regions;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default, Deserialize)]
#[serde(from = "Vec<File>")]
pub(super) struct Files {
    files: Vec<File>,
}

impl Layer for Files {
    type Layer = InfrastructureLayer;
}

impl Files {
    pub(super) fn select_regions(&self, function_regions: Regions) -> Regions {
        let Self { files } = self;
        let mut file_paths = BTreeSet::new();
        for file in files {
            let filename = file.filename();
            file_paths.insert(filename);
        }
        let mut region_sequence = Vec::new();
        for region in function_regions {
            let span = region.span();
            let path = span.path();
            if file_paths.contains(path) {
                region_sequence.push(region);
            }
        }
        Regions::from_iter(region_sequence)
    }
}

impl From<Vec<File>> for Files {
    fn from(files: Vec<File>) -> Self {
        Self { files }
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
    fn a_region_of_a_file_the_files_keep_is_selected() {
        let text = r#"[{"filename": "crates/harbor/src/lib.rs"}]"#;
        let files: Files = serde_json::from_str(text).unwrap();
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let expectation_region = region.clone();
        let region_sequence = [region];
        let function_regions = Regions::from_iter(region_sequence);

        let regions = files.select_regions(function_regions);

        let expectation_sequence = [expectation_region];
        let expectation = Regions::from_iter(expectation_sequence);
        assert_eq!(regions, expectation);
    }

    #[test]
    fn a_region_of_a_file_the_files_do_not_keep_is_passed_over() {
        let text = r#"[{"filename": "crates/harbor/src/lib.rs"}]"#;
        let files: Files = serde_json::from_str(text).unwrap();
        let start_line: usize = 5;
        let span = fixture::main_path_span_factory(start_line);
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let region_sequence = [region];
        let function_regions = Regions::from_iter(region_sequence);

        let regions = files.select_regions(function_regions);

        let expectation = Regions::default();
        assert_eq!(regions, expectation);
    }
}
