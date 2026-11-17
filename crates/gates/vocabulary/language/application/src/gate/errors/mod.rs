use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::slice::Iter;
use std::vec::IntoIter;

#[derive(Debug)]
pub struct Errors {
    errors: Vec<Error>,
}

impl Layer for Errors {
    type Layer = ApplicationLayer;
}

impl FromIterator<Error> for Errors {
    fn from_iter<ErrorsType: IntoIterator<Item = Error>>(error_sequence: ErrorsType) -> Self {
        let errors = error_sequence.into_iter().collect();
        Self { errors }
    }
}

impl From<Error> for Errors {
    fn from(error: Error) -> Self {
        let errors = vec![error];
        Self { errors }
    }
}

impl IntoIterator for Errors {
    type Item = Error;
    type IntoIter = IntoIter<Error>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { errors } = self;
        errors.into_iter()
    }
}

impl<'errors> IntoIterator for &'errors Errors {
    type Item = &'errors Error;
    type IntoIter = Iter<'errors, Error>;

    fn into_iter(self) -> Self::IntoIter {
        let Errors { errors } = self;
        errors.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn borrowed_errors_yield_every_error_they_hold() {
        let path_text = "/workspace/crates/ship";
        let path = PathBuf::from(path_text);
        let source_absence = Error::SourceAbsence { path };
        let error_list = [source_absence];
        let errors: Errors = error_list.into_iter().collect();

        let error_texts: Vec<String> = (&errors).into_iter().map(ToString::to_string).collect();

        let expectation = ["[LANGUAGE] there is no Rust source under `/workspace/crates/ship`."];
        assert_eq!(error_texts, expectation);
    }

    #[test]
    fn an_error_alone_is_errors_of_that_error() {
        let path_text = "/workspace/crates/ship";
        let path = PathBuf::from(path_text);
        let source_absence = Error::SourceAbsence { path };

        let errors = Errors::from(source_absence);

        let error_texts: Vec<String> = errors.into_iter().map(|error| error.to_string()).collect();
        let expectation = ["[LANGUAGE] there is no Rust source under `/workspace/crates/ship`."];
        assert_eq!(error_texts, expectation);
    }
}
