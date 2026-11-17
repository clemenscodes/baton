use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::error;
use std::ffi::{OsStr, OsString};
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    Words,
    Invocation {
        error: gates_coverage_core_invocation::Error,
    },
    ProgramStart {
        program: OsString,
        error: io::Error,
    },
    Failure {
        program: OsString,
        status: String,
    },
    TestBuild {
        text: String,
    },
    ProfileTools {
        error: gates_coverage_core_profile_tools::Error,
    },
    ProfileAbsence,
    ProfileDirectory {
        error: io::Error,
    },
    ReportFile {
        path: PathBuf,
        error: io::Error,
    },
    ReportExport {
        path: PathBuf,
        error: io::Error,
    },
    Report {
        text: String,
    },
    Source {
        path: PathBuf,
        error: io::Error,
    },
    Syntax {
        text: String,
    },
}

impl Layer for Error {
    type Layer = ApplicationLayer;
}

impl From<gates_coverage_core_invocation::Error> for Error {
    fn from(error: gates_coverage_core_invocation::Error) -> Self {
        Self::Invocation { error }
    }
}

impl From<gates_coverage_core_profile_tools::Error> for Error {
    fn from(error: gates_coverage_core_profile_tools::Error) -> Self {
        Self::ProfileTools { error }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Words => Self::try_write_words_error(formatter),
            Self::Invocation { error } => write!(formatter, "{error}"),
            Self::ProgramStart { program, error } => {
                Self::try_write_program_start_error(formatter, program, error)
            }
            Self::Failure { program, status } => {
                Self::try_write_failure(formatter, program, status)
            }
            Self::TestBuild { text } | Self::Report { text } | Self::Syntax { text } => {
                write!(formatter, "{text}")
            }
            Self::ProfileTools { error } => write!(formatter, "{error}"),
            Self::ProfileAbsence => Self::try_write_profile_absence(formatter),
            Self::ProfileDirectory { error } => {
                Self::try_write_profile_directory_error(formatter, error)
            }
            Self::ReportFile { path, error } => {
                Self::try_write_report_file_error(formatter, path, error)
            }
            Self::ReportExport { path, error } => {
                Self::try_write_report_export_error(formatter, path, error)
            }
            Self::Source { path, error } => Self::try_write_source_error(formatter, path, error),
        }
    }
}

impl Error {
    fn try_write_words_error(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] a measurement takes the path of its report, the command of the build, `--` and the command of the tests."
        )
    }

    fn try_write_program_start_error(
        formatter: &mut fmt::Formatter<'_>,
        program: &OsStr,
        error: &io::Error,
    ) -> fmt::Result {
        let program_text = program.display();
        write!(
            formatter,
            "[COVERAGE] cannot start `{program_text}`: {error}."
        )
    }

    fn try_write_failure(
        formatter: &mut fmt::Formatter<'_>,
        program: &OsStr,
        status: &str,
    ) -> fmt::Result {
        let program_text = program.display();
        write!(
            formatter,
            "[COVERAGE] `{program_text}` ended with {status}."
        )
    }

    fn try_write_profile_absence(formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] the tests left no profile, so their build was not instrumented for coverage."
        )
    }

    fn try_write_profile_directory_error(
        formatter: &mut fmt::Formatter<'_>,
        error: &io::Error,
    ) -> fmt::Result {
        write!(
            formatter,
            "[COVERAGE] cannot keep the profiles of the measurement in a directory of their own: {error}."
        )
    }

    fn try_write_report_file_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(
            formatter,
            "[COVERAGE] cannot write the report `{path_text}`: {error}."
        )
    }

    fn try_write_report_export_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(
            formatter,
            "[COVERAGE] cannot read the report `{path_text}`: {error}."
        )
    }

    fn try_write_source_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(formatter, "[COVERAGE] cannot read `{path_text}`: {error}.")
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_words_error_names_what_a_measurement_takes() {
        let error = Error::Words;

        let text = error.to_string();

        let expectation = "[COVERAGE] a measurement takes the path of its report, the command of the build, `--` and the command of the tests.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn an_invocation_error_is_the_error_of_the_invocation() {
        let invocation_error = gates_coverage_core_invocation::Error::ProgramAbsence;
        let error = Error::from(invocation_error);

        let text = error.to_string();

        let expectation = invocation_error.to_string();
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_start_error_names_the_program_and_the_cause() {
        let program = OsString::from("cargo");
        let error_kind = io::ErrorKind::NotFound;
        let start_error = io::Error::from(error_kind);
        let error = Error::ProgramStart {
            program,
            error: start_error,
        };

        let text = error.to_string();

        let expectation = "[COVERAGE] cannot start `cargo`: entity not found.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_failure_names_the_program_and_its_status() {
        let program = OsString::from("cargo");
        let status = String::from("exit status: 101");
        let error = Error::Failure { program, status };

        let text = error.to_string();

        let expectation = "[COVERAGE] `cargo` ended with exit status: 101.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_test_build_error_is_the_text_of_its_decoder() {
        let text = String::from("[COVERAGE] the build made no executable of a test.");
        let error = Error::TestBuild { text };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] the build made no executable of a test.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_profile_tools_error_is_the_error_of_the_profile_tools() {
        let profile_tools_error = gates_coverage_core_profile_tools::Error::LibraryDirectoryAbsence;
        let error = Error::from(profile_tools_error);

        let text = error.to_string();

        let expectation = profile_tools_error.to_string();
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_profile_absence_says_that_the_build_was_not_instrumented() {
        let error = Error::ProfileAbsence;

        let text = error.to_string();

        let expectation = "[COVERAGE] the tests left no profile, so their build was not instrumented for coverage.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_profile_directory_error_names_its_cause() {
        let error_kind = io::ErrorKind::PermissionDenied;
        let profile_directory_error = io::Error::from(error_kind);
        let error = Error::ProfileDirectory {
            error: profile_directory_error,
        };

        let text = error.to_string();

        let expectation = "[COVERAGE] cannot keep the profiles of the measurement in a directory of their own: permission denied.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_report_file_error_names_the_path_and_the_cause() {
        let path = PathBuf::from("target/moon/coverage/harbor.json");
        let error_kind = io::ErrorKind::PermissionDenied;
        let report_error = io::Error::from(error_kind);
        let error = Error::ReportFile {
            path,
            error: report_error,
        };

        let text = error.to_string();

        let expectation = "[COVERAGE] cannot write the report `target/moon/coverage/harbor.json`: permission denied.";
        assert_eq!(text, expectation);
    }

    #[test]
    fn a_source_that_cannot_be_read_names_its_path_and_the_error_of_the_reader() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let reader_error_text = "permission denied";
        let reader_error = io::Error::other(reader_error_text);
        let error = Error::Source {
            path,
            error: reader_error,
        };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] cannot read `crates/harbor/src/lib.rs`: permission denied.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_report_that_cannot_be_read_names_its_path_and_the_error_of_the_reader() {
        let path_text = "target/moon/coverage/harbor.json";
        let path = PathBuf::from(path_text);
        let reader_error_text = "entity not found";
        let reader_error = io::Error::other(reader_error_text);
        let error = Error::ReportExport {
            path,
            error: reader_error,
        };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] cannot read the report `target/moon/coverage/harbor.json`: entity not found.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_report_the_decoder_refuses_and_a_source_that_is_no_rust_are_the_texts_of_their_errors() {
        let report_text = String::from("[COVERAGE] the report is no export of llvm-cov.");
        let syntax_text = String::from(
            "[COVERAGE] crates/harbor/src/lib.rs:1:13: the source is not valid Rust: unexpected token.",
        );
        let report_error = Error::Report {
            text: report_text.clone(),
        };
        let syntax_error = Error::Syntax {
            text: syntax_text.clone(),
        };

        let report_error_text = report_error.to_string();
        let syntax_error_text = syntax_error.to_string();

        assert_eq!(report_error_text, report_text);
        assert_eq!(syntax_error_text, syntax_text);
    }
}
