use gates_coverage_core_test_boundaries::test_boundary;
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug)]
pub enum Error {
    Syntax {
        path: PathBuf,
        line: usize,
        column: usize,
        error: ::syn::Error,
    },
    TestBoundary {
        error: test_boundary::Error,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl From<(&Path, ::syn::Error)> for Error {
    fn from((source_path, error): (&Path, ::syn::Error)) -> Self {
        let path = source_path.to_path_buf();
        let LineColumn {
            line,
            column: column_offset,
        } = error.span().start();
        let column = column_offset + 1;
        Self::Syntax {
            path,
            line,
            column,
            error,
        }
    }
}

impl From<test_boundary::Error> for Error {
    fn from(error: test_boundary::Error) -> Self {
        Self::TestBoundary { error }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Syntax {
                path,
                line,
                column,
                error,
            } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[COVERAGE] {path_text}:{line}:{column}: the source is not valid Rust: {error}."
                )
            }
            Self::TestBoundary { error } => write!(formatter, "{error}"),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_source_that_is_no_rust_names_its_path_its_line_its_column_and_the_error_of_syn() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let line: usize = 3;
        let column: usize = 9;
        let span = proc_macro2::Span::call_site();
        let syntax_error_text = "expected item";
        let syntax_error = ::syn::Error::new(span, syntax_error_text);
        let error = Error::Syntax {
            path,
            line,
            column,
            error: syntax_error,
        };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:3:9: the source is not valid Rust: expected item.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_test_module_that_is_no_range_of_lines_states_the_error_of_its_test_boundary() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 5;
        let closing_line: usize = 3;
        let line_error = test_boundary::Error::Line {
            path,
            opening_line,
            closing_line,
        };
        let error = Error::from(line_error);

        let error_text = error.to_string();

        let expectation = "[COVERAGE] a test module in `crates/harbor/src/lib.rs` opens on line 5 and closes on line 3, but its opening line must be at least 1 and at most its closing line.";
        assert_eq!(error_text, expectation);
    }
}
