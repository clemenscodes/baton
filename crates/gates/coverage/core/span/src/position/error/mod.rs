use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum Error {
    Position { line: usize, column: usize },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Position { line, column } => {
                write!(
                    formatter,
                    "{line}:{column} is not a valid position: lines and columns start at 1"
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
    fn a_position_below_one_names_its_line_and_its_column() {
        let line: usize = 0;
        let column: usize = 9;
        let error = Error::Position { line, column };

        let error_text = error.to_string();

        let expectation = "0:9 is not a valid position: lines and columns start at 1";
        assert_eq!(error_text, expectation);
    }
}
