use crate::position::Position;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Order {
        path: PathBuf,
        start_position: Position,
        end_position: Position,
    },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Order {
                path,
                start_position,
                end_position,
            } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[COVERAGE] {path_text}:{start_position}: the span ends at {end_position}, before it starts."
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
    fn a_span_that_ends_before_it_starts_names_its_path_its_start_and_its_end() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let start_line: usize = 4;
        let start_column: usize = 1;
        let end_line: usize = 3;
        let end_column: usize = 9;
        let start_position = Position::try_from((start_line, start_column)).unwrap();
        let end_position = Position::try_from((end_line, end_column)).unwrap();
        let error = Error::Order {
            path,
            start_position,
            end_position,
        };

        let error_text = error.to_string();

        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:4:1: the span ends at 3:9, before it starts.";
        assert_eq!(error_text, expectation);
    }
}
