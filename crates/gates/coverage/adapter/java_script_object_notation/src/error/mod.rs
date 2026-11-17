use gates_coverage_core_span::position;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Input {
        error: io::Error,
    },
    Export {
        error: serde_json::Error,
    },
    Message {
        error: serde_json::Error,
    },
    TestExecutableAbsence,
    File {
        file_index: usize,
        file_count: usize,
    },
    Position {
        path: PathBuf,
        error: position::Error,
    },
    Span {
        error: gates_coverage_core_span::Error,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Self::Input { error }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Self::Export { error }
    }
}

impl From<(&Path, position::Error)> for Error {
    fn from((region_path, error): (&Path, position::Error)) -> Self {
        let path = region_path.to_path_buf();
        Self::Position { path, error }
    }
}

impl From<gates_coverage_core_span::Error> for Error {
    fn from(error: gates_coverage_core_span::Error) -> Self {
        Self::Span { error }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Input { error } => Self::try_write_input_error(formatter, error),
            Self::Export { error } => Self::try_write_export_error(formatter, error),
            Self::Message { error } => Self::try_write_message_error(formatter, error),
            Self::TestExecutableAbsence => Self::try_write_test_executable_absence(formatter),
            Self::File {
                file_index,
                file_count,
            } => Self::try_write_file_error(formatter, *file_index, *file_count),
            Self::Position { path, error } => {
                Self::try_write_position_error(formatter, path, error)
            }
            Self::Span { error } => write!(formatter, "{error}"),
        }
    }
}

impl Error {
    fn try_write_input_error(formatter: &mut fmt::Formatter<'_>, error: &io::Error) -> fmt::Result {
        write!(formatter, "[COVERAGE] cannot read the input: {error}.")
    }

    fn try_write_export_error(
        formatter: &mut fmt::Formatter<'_>,
        error: &serde_json::Error,
    ) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] the report is not an llvm-cov export: {error}."
        )
    }

    fn try_write_message_error(
        formatter: &mut fmt::Formatter<'_>,
        error: &serde_json::Error,
    ) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] a line of the output of the build is no message of Cargo: {error}."
        )
    }

    fn try_write_test_executable_absence(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] the build made no executable of a test, so no test can be measured."
        )
    }

    fn try_write_file_error(
        formatter: &mut fmt::Formatter<'_>,
        file_index: usize,
        file_count: usize,
    ) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] a region refers to file index {file_index}, but the file indexes of its function end before {file_count}."
        )
    }

    fn try_write_position_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &position::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(formatter, "[COVERAGE] {path_text}:{error}.")
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_input_that_cannot_be_read_names_the_error_of_the_input() {
        let input_error_kind = io::ErrorKind::InvalidData;
        let input_error = io::Error::from(input_error_kind);
        let error = Error::from(input_error);

        let error_text = error.to_string();

        let expectation = "[COVERAGE] cannot read the input: invalid data.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_report_that_is_no_export_names_the_error_of_serde_json() {
        let text = "export";
        let syntax_error = serde_json::from_str::<usize>(text).unwrap_err();
        let error = Error::from(syntax_error);

        let error_text = error.to_string();

        let expectation = "[COVERAGE] the report is not an llvm-cov export: expected value at line 1 column 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_region_of_a_file_index_its_function_lacks_names_the_file_index_and_the_file_count() {
        let file_index: usize = 3;
        let file_count: usize = 1;
        let error = Error::File {
            file_index,
            file_count,
        };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] a region refers to file index 3, but the file indexes of its function end before 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_region_with_a_position_below_one_names_its_path_and_states_the_error_of_the_position() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = Path::new(path_text);
        let line: usize = 0;
        let column: usize = 9;
        let position_error = position::Error::Position { line, column };
        let error = Error::from((path, position_error));

        let error_text = error.to_string();

        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:0:9 is not a valid position: lines and columns start at 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_region_that_is_no_span_states_the_error_of_the_span() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let start_line: usize = 4;
        let start_column: usize = 1;
        let end_line: usize = 3;
        let end_column: usize = 9;
        let start = position::Position::try_from((start_line, start_column)).unwrap();
        let end = position::Position::try_from((end_line, end_column)).unwrap();
        let span_error = gates_coverage_core_span::Error::Order {
            path,
            start_position: start,
            end_position: end,
        };
        let error = Error::from(span_error);

        let error_text = error.to_string();

        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:4:1: the span ends at 3:9, before it starts.";
        assert_eq!(error_text, expectation);
    }
}
