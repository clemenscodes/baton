use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Line {
        path: PathBuf,
        opening_line: usize,
        closing_line: usize,
    },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Line {
                path,
                opening_line,
                closing_line,
            } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[COVERAGE] a test module in `{path_text}` opens on line {opening_line} and closes on line {closing_line}, but its opening line must be at least 1 and at most its closing line."
                )
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_test_module_that_is_no_range_of_lines_names_its_path_its_opening_line_and_its_closing_line()
     {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let opening_line: usize = 5;
        let closing_line: usize = 3;
        let error = Error::Line {
            path,
            opening_line,
            closing_line,
        };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] a test module in `crates/harbor/src/lib.rs` opens on line 5 and closes on line 3, but its opening line must be at least 1 and at most its closing line.";
        assert_eq!(error_text, expectation);
    }
}
