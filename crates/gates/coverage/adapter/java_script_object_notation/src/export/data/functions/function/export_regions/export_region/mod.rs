use crate::error::Error;
use crate::export::data::functions::function::export_regions::kind::Kind;
use crate::export::data::functions::function::filenames::Filenames;
use gates_coverage_core_region::Region;
use gates_coverage_core_span::Span;
use gates_coverage_core_span::position::Position;
use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default, Deserialize)]
#[serde(from = "[usize; 8]")]
pub(super) struct ExportRegion {
    start_line: usize,
    start_column: usize,
    end_line: usize,
    end_column: usize,
    execution_count: usize,
    file_index: usize,
    kind: Kind,
}

impl Layer for ExportRegion {
    type Layer = InfrastructureLayer;
}

impl ExportRegion {
    pub(super) fn kind(&self) -> Kind {
        self.kind
    }

    pub(super) fn try_decode_region(&self, filenames: &Filenames) -> Result<Region, Error> {
        let Self {
            start_line,
            start_column,
            end_line,
            end_column,
            execution_count,
            file_index,
            ..
        } = *self;
        let filename = filenames.try_locate_filename(file_index)?;
        let position_error_conversion = |position_error| Error::from((filename, position_error));
        let start_position = Position::try_from((start_line, start_column))
            .map_err(position_error_conversion)?;
        let end_position = Position::try_from((end_line, end_column))
            .map_err(position_error_conversion)?;
        let path = filename.to_path_buf();
        let span = Span::try_from((path, start_position, end_position))?;
        let region = Region::from((span, execution_count));
        Ok(region)
    }
}

impl From<[usize; 8]> for ExportRegion {
    fn from(
        [
            start_line,
            start_column,
            end_line,
            end_column,
            execution_count,
            file_index,
            _,
            export_kind,
        ]: [usize; 8],
    ) -> Self {
        let kind = Kind::from(export_kind);
        Self {
            start_line,
            start_column,
            end_line,
            end_column,
            execution_count,
            file_index,
            kind,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

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

        pub(super) fn filenames_factory() -> Filenames {
            let text = r#"["crates/harbor/src/lib.rs"]"#;
            serde_json::from_str(text).unwrap()
        }
    }

    #[test]
    fn an_export_region_keeps_its_kind() {
        let text = "[3, 1, 3, 2, 7, 4, 5, 1]";
        let export_region: ExportRegion = serde_json::from_str(text).unwrap();

        let kind = export_region.kind();

        let expectation = Kind::Annotation;
        assert_eq!(kind, expectation);
    }

    #[test]
    fn an_export_region_is_a_region_of_the_filename_of_its_file_index() {
        let text = "[3, 1, 3, 2, 7, 0, 0, 0]";
        let export_region: ExportRegion = serde_json::from_str(text).unwrap();
        let filenames = fixture::filenames_factory();

        let region = export_region.try_decode_region(&filenames).unwrap();

        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 7;
        let expectation = Region::from((span, execution_count));
        assert_eq!(region, expectation);
    }

    #[test]
    fn an_export_region_of_a_file_index_past_the_filenames_is_an_error() {
        let text = "[3, 1, 3, 2, 7, 1, 0, 0]";
        let export_region: ExportRegion = serde_json::from_str(text).unwrap();
        let filenames = fixture::filenames_factory();

        let region = export_region.try_decode_region(&filenames);

        let error_text = region.unwrap_err().to_string();
        let expectation = "[COVERAGE] a region refers to file index 1, but the file indexes of its function end before 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_export_region_that_starts_on_the_line_zero_is_an_error() {
        let text = "[0, 1, 3, 2, 0, 0, 0, 0]";
        let export_region: ExportRegion = serde_json::from_str(text).unwrap();
        let filenames = fixture::filenames_factory();

        let region = export_region.try_decode_region(&filenames);

        let error_text = region.unwrap_err().to_string();
        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:0:1 is not a valid position: lines and columns start at 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_export_region_that_ends_on_the_column_zero_is_an_error() {
        let text = "[3, 1, 3, 0, 0, 0, 0, 0]";
        let export_region: ExportRegion = serde_json::from_str(text).unwrap();
        let filenames = fixture::filenames_factory();

        let region = export_region.try_decode_region(&filenames);

        let error_text = region.unwrap_err().to_string();
        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:3:0 is not a valid position: lines and columns start at 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_export_region_that_ends_before_it_starts_is_an_error() {
        let text = "[4, 1, 3, 9, 0, 0, 0, 0]";
        let export_region: ExportRegion = serde_json::from_str(text).unwrap();
        let filenames = fixture::filenames_factory();

        let region = export_region.try_decode_region(&filenames);

        let error_text = region.unwrap_err().to_string();
        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:4:1: the span ends at 3:9, before it starts.";
        assert_eq!(error_text, expectation);
    }
}
