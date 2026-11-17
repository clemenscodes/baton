use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::fmt;
use std::path::PathBuf;

#[derive(Debug)]
pub enum BookEntry {
    Publication { path: PathBuf, term_count: usize },
    Omission { path: PathBuf },
    Error { error: Error },
}

impl Layer for BookEntry {
    type Layer = ApplicationLayer;
}

impl fmt::Display for BookEntry {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Publication { path, term_count } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[UBIQUITOUS] `{path_text}` now contains {term_count} term(s)."
                )
            }
            Self::Omission { path } => {
                let path_text = path.display();
                write!(
                    formatter,
                    "[UBIQUITOUS] skipped `{path_text}`: its directory does not exist."
                )
            }
            Self::Error { error } => write!(formatter, "{error}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_publication_names_its_book_and_the_count_of_its_terms() {
        let path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let path = PathBuf::from(path_text);
        let term_count: usize = 2;
        let publication = BookEntry::Publication { path, term_count };

        let publication_text = publication.to_string();

        let expectation = "[UBIQUITOUS] `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md` now contains 2 term(s).";
        assert_eq!(publication_text, expectation);
    }

    #[test]
    fn an_error_entry_reads_as_its_error() {
        let path_text = "crates/harbor";
        let path = PathBuf::from(path_text);
        let error = Error::SourceAbsence { path };
        let error_entry = BookEntry::Error { error };

        let error_text = error_entry.to_string();

        let expectation = "[LANGUAGE] there is no Rust source under `crates/harbor`.";
        assert_eq!(error_text, expectation);
    }
}
