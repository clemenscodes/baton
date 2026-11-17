use gates_vocabulary_language_core_foreign_name::ForeignName;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Entry { foreign_name: ForeignName },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Entry { foreign_name } => {
                write!(formatter, "a dependency table lists `{foreign_name}` twice")
            }
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn a_second_entry_of_a_dependency_is_named_with_its_foreign_name_in_the_error() {
        let foreign_name_text = "anchor";
        let foreign_name = ForeignName::from(foreign_name_text);
        let error = Error::Entry { foreign_name };

        let error_text = error.to_string();

        let expectation = "a dependency table lists `anchor` twice";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_of_an_entry_has_no_source() {
        let foreign_name_text = "anchor";
        let foreign_name = ForeignName::from(foreign_name_text);
        let error = Error::Entry { foreign_name };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }
}
