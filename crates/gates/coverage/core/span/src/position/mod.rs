mod error;

pub use error::Error;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct Position {
    line: usize,
    column: usize,
}

impl Layer for Position {
    type Layer = DomainLayer;
}

impl ValueObject for Position {}

impl Position {
    const FIRST_LINE: usize = 1;
    const FIRST_COLUMN: usize = 1;

    pub fn line(&self) -> usize {
        self.line
    }
}

impl TryFrom<(usize, usize)> for Position {
    type Error = Error;

    fn try_from((line, column): (usize, usize)) -> Result<Self, Self::Error> {
        let first_line = Self::FIRST_LINE;
        let first_column = Self::FIRST_COLUMN;
        if line < first_line || column < first_column {
            let position_error = Error::Position { line, column };
            return Err(position_error);
        }
        let position = Self { line, column };
        Ok(position)
    }
}

impl fmt::Display for Position {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { line, column } = self;
        write!(formatter, "{line}:{column}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_position_is_shown_as_its_line_and_its_column() {
        let line: usize = 3;
        let column: usize = 9;
        let position = Position::try_from((line, column)).unwrap();

        let position_text = position.to_string();

        let expectation = "3:9";
        assert_eq!(position_text, expectation);
    }

    #[test]
    fn a_position_keeps_its_line() {
        let line: usize = 3;
        let column: usize = 9;
        let position = Position::try_from((line, column)).unwrap();

        let position_line = position.line();

        assert_eq!(position_line, line);
    }

    #[test]
    fn a_position_on_the_first_line_and_the_first_column_is_a_position() {
        let line: usize = 1;
        let column: usize = 1;

        let position = Position::try_from((line, column));

        let error = position.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }

    #[test]
    fn a_line_of_zero_is_an_error() {
        let line: usize = 0;
        let column: usize = 9;

        let position = Position::try_from((line, column));

        let position_error = Error::Position { line, column };
        let expectation = Err(position_error);
        assert_eq!(position, expectation);
    }

    #[test]
    fn a_column_of_zero_is_an_error() {
        let line: usize = 3;
        let column: usize = 0;

        let position = Position::try_from((line, column));

        let position_error = Error::Position { line, column };
        let expectation = Err(position_error);
        assert_eq!(position, expectation);
    }
}
