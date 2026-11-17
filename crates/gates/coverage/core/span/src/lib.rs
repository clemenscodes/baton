mod error;

pub mod position;

pub use error::Error;

use crate::position::Position;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    path: PathBuf,
    start_position: Position,
    end_position: Position,
}

impl Layer for Span {
    type Layer = DomainLayer;
}

impl ValueObject for Span {}

impl Span {
    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn start_position(&self) -> &Position {
        &self.start_position
    }
}

impl TryFrom<(PathBuf, Position, Position)> for Span {
    type Error = Error;

    fn try_from(
        (path, start_position, end_position): (PathBuf, Position, Position),
    ) -> Result<Self, Self::Error> {
        if end_position < start_position {
            let order_error = Error::Order {
                path,
                start_position,
                end_position,
            };
            return Err(order_error);
        }
        let span = Self {
            path,
            start_position,
            end_position,
        };
        Ok(span)
    }
}

impl fmt::Display for Span {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            path,
            start_position,
            ..
        } = self;
        let path_text = path.display();
        write!(formatter, "{path_text}:{start_position}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

        pub(super) fn span_factory(start_line: usize) -> Span {
            let path_text = "crates/harbor/src/lib.rs";
            let path = PathBuf::from(path_text);
            let start_column: usize = 1;
            let end_column: usize = 2;
            let start_position = Position::try_from((start_line, start_column)).unwrap();
            let end_position = Position::try_from((start_line, end_column)).unwrap();
            Span::try_from((path, start_position, end_position)).unwrap()
        }
    }

    #[test]
    fn a_span_is_shown_as_its_path_and_where_it_starts() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let start_line: usize = 3;
        let start_column: usize = 9;
        let end_line: usize = 4;
        let end_column: usize = 2;
        let start_position = Position::try_from((start_line, start_column)).unwrap();
        let end_position = Position::try_from((end_line, end_column)).unwrap();
        let span = Span::try_from((path, start_position, end_position)).unwrap();

        let span_text = span.to_string();

        let expectation = "crates/harbor/src/lib.rs:3:9";
        assert_eq!(span_text, expectation);
    }

    #[test]
    fn a_span_that_starts_where_it_ends_is_a_span() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let line: usize = 3;
        let column: usize = 9;
        let start_position = Position::try_from((line, column)).unwrap();
        let end_position = start_position;

        let span = Span::try_from((path, start_position, end_position));

        let error = span.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }

    #[test]
    fn a_span_keeps_its_path() {
        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);

        let span_path = span.path();

        let path_text = "crates/harbor/src/lib.rs";
        let expectation = Path::new(path_text);
        assert_eq!(span_path, expectation);
    }

    #[test]
    fn a_span_keeps_its_start() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let start_line: usize = 3;
        let start_column: usize = 9;
        let end_line: usize = 4;
        let end_column: usize = 2;
        let start_position = Position::try_from((start_line, start_column)).unwrap();
        let end_position = Position::try_from((end_line, end_column)).unwrap();
        let span = Span::try_from((path, start_position, end_position)).unwrap();

        let span_start = span.start_position();

        assert_eq!(span_start, &start_position);
    }

    #[test]
    fn a_span_that_ends_on_an_earlier_line_is_an_error() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let start_line: usize = 4;
        let start_column: usize = 1;
        let end_line: usize = 3;
        let end_column: usize = 9;
        let start_position = Position::try_from((start_line, start_column)).unwrap();
        let end_position = Position::try_from((end_line, end_column)).unwrap();

        let span = Span::try_from((path, start_position, end_position));

        let expectation_path = PathBuf::from(path_text);
        let order_error = Error::Order {
            path: expectation_path,
            start_position,
            end_position,
        };
        let expectation = Err(order_error);
        assert_eq!(span, expectation);
    }

    #[test]
    fn a_span_that_ends_on_an_earlier_column_of_its_line_is_an_error() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let line: usize = 3;
        let start_column: usize = 9;
        let end_column: usize = 8;
        let start_position = Position::try_from((line, start_column)).unwrap();
        let end_position = Position::try_from((line, end_column)).unwrap();

        let span = Span::try_from((path, start_position, end_position));

        let expectation_path = PathBuf::from(path_text);
        let order_error = Error::Order {
            path: expectation_path,
            start_position,
            end_position,
        };
        let expectation = Err(order_error);
        assert_eq!(span, expectation);
    }
}
