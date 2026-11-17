use gates_vocabulary_language_core_language as language;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Language { error: language::Error },
    Context { path: PathBuf },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Language { error } => write!(formatter, "{error}"),
            Self::Context { path } => {
                let path_text = path.display();
                write!(formatter, "`{path_text}` is not the directory of a context")
            }
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::Language { error } => Some(error),
            Self::Context { .. } => None,
        }
    }
}

impl From<language::Error> for Error {
    fn from(error: language::Error) -> Self {
        Self::Language { error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::error::Error as _;

    #[test]
    fn a_path_that_is_the_directory_of_no_context_is_named_in_the_error() {
        let path_text = "crates/nowhere";
        let path = PathBuf::from(path_text);
        let error = Error::Context { path };

        let error_text = error.to_string();

        let expectation = "`crates/nowhere` is not the directory of a context";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn languages_state_the_error_of_a_language() {
        const TERM_TEXT: language::TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            language::TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let language_error = language::Error::Redefinition { term_text };
        let error = Error::from(language_error);

        let error_text = error.to_string();

        let expectation = "the term `Treasure: A thing.` is defined twice";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn the_source_of_an_error_of_a_language_is_that_error() {
        const TERM_TEXT: language::TermText = {
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            language::TermText::new(spelling, definition)
        };
        let term_text = TERM_TEXT;
        let language_error = language::Error::Redefinition { term_text };
        let error = Error::from(language_error);

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "the term `Treasure: A thing.` is defined twice";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn an_error_of_a_context_has_no_source() {
        let path_text = "crates/nowhere";
        let path = PathBuf::from(path_text);
        let error = Error::Context { path };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }
}
