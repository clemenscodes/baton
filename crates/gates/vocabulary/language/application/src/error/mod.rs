use gates_vocabulary_language_core_language::defects::Defects;
use gates_vocabulary_language_core_language::{UnknownVocabulary, UnusedTerms};
use gates_vocabulary_language_core_languages as languages;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::error;
use std::fmt;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum Error {
    File {
        path: PathBuf,
        error: io::Error,
    },
    Manifest {
        path: PathBuf,
        error: io::Error,
    },
    Syntax {
        path: PathBuf,
        error: io::Error,
    },
    Language {
        path: PathBuf,
        unknown_vocabulary: UnknownVocabulary,
        defects: Defects,
    },
    Terms {
        path: PathBuf,
        unused_terms: UnusedTerms,
    },
    SourceAbsence {
        path: PathBuf,
    },
    Book {
        path: PathBuf,
        error: io::Error,
    },
    Directory {
        path: PathBuf,
        error: io::Error,
    },
    Vocabulary {
        error: languages::Error,
    },
}

impl Layer for Error {
    type Layer = ApplicationLayer;
}

impl Error {
    const LINE_SEPARATOR: &'static str = "\n";

    fn try_write_file_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(formatter, "[LANGUAGE] cannot read `{path_text}`: {error}.")
    }

    fn try_write_manifest_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        let manifest_error_text = error.to_string();
        let reason = manifest_error_text.trim_end();
        write!(
            formatter,
            "[LANGUAGE] `{path_text}` is not a valid crate manifest: {reason}."
        )
    }

    fn try_write_syntax_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(formatter, "[LANGUAGE] {path_text}:{error}.")
    }

    fn try_write_language_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        unknown_vocabulary: &UnknownVocabulary,
        defects: &Defects,
    ) -> fmt::Result {
        let path_text = path.display();
        let occurrence_line = |occurrence| {
            format!(
                "[LANGUAGE] {path_text}:{occurrence} is not made of words of this crate's vocabulary."
            )
        };
        let defect_line = |defect| format!("[LANGUAGE] {path_text}:{defect}.");
        let occurrence_lines = unknown_vocabulary.into_iter().map(occurrence_line);
        let defect_lines = defects.into_iter().map(defect_line);
        let error_lines: Vec<String> = occurrence_lines.chain(defect_lines).collect();
        let error_text = error_lines.join(Self::LINE_SEPARATOR);
        write!(formatter, "{error_text}")
    }

    fn try_write_terms_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        unused_terms: &UnusedTerms,
    ) -> fmt::Result {
        let path_text = path.display();
        let spelling_line = |spelling| {
            format!(
                "[LANGUAGE] `{path_text}` defines the term `{spelling}`, but no package name and no identifier in production code uses it."
            )
        };
        let error_lines: Vec<String> = unused_terms.into_iter().map(spelling_line).collect();
        let error_text = error_lines.join(Self::LINE_SEPARATOR);
        write!(formatter, "{error_text}")
    }

    fn try_write_source_absence(formatter: &mut fmt::Formatter<'_>, path: &Path) -> fmt::Result {
        let path_text = path.display();
        write!(
            formatter,
            "[LANGUAGE] there is no Rust source under `{path_text}`."
        )
    }

    fn try_write_book_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(formatter, "[LANGUAGE] cannot write `{path_text}`: {error}.")
    }

    fn try_write_directory_error(
        formatter: &mut fmt::Formatter<'_>,
        path: &Path,
        error: &io::Error,
    ) -> fmt::Result {
        let path_text = path.display();
        write!(
            formatter,
            "[LANGUAGE] cannot check whether `{path_text}` is a directory: {error}."
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::File { path, error } => Self::try_write_file_error(formatter, path, error),
            Self::Manifest { path, error } => {
                Self::try_write_manifest_error(formatter, path, error)
            }
            Self::Syntax { path, error } => Self::try_write_syntax_error(formatter, path, error),
            Self::Language {
                path,
                unknown_vocabulary,
                defects,
            } => Self::try_write_language_error(formatter, path, unknown_vocabulary, defects),
            Self::Terms { path, unused_terms } => {
                Self::try_write_terms_error(formatter, path, unused_terms)
            }
            Self::SourceAbsence { path } => Self::try_write_source_absence(formatter, path),
            Self::Book { path, error } => Self::try_write_book_error(formatter, path, error),
            Self::Directory { path, error } => {
                Self::try_write_directory_error(formatter, path, error)
            }
            Self::Vocabulary { error } => write!(formatter, "[LANGUAGE] {error}."),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::File { error, .. } => Some(error),
            Self::Manifest { error, .. } => Some(error),
            Self::Syntax { error, .. } => Some(error),
            Self::Language { .. } => None,
            Self::Terms { .. } => None,
            Self::SourceAbsence { .. } => None,
            Self::Book { error, .. } => Some(error),
            Self::Directory { error, .. } => Some(error),
            Self::Vocabulary { error } => Some(error),
        }
    }
}

impl From<languages::Error> for Error {
    fn from(error: languages::Error) -> Self {
        Self::Vocabulary { error }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gate::tests::fixture as source_fixture;
    use gates_vocabulary_language_core_language::Language;
    use gates_vocabulary_language_core_language::defects::defect::Defect;
    use std::error::Error as _;

    #[test]
    fn a_file_that_could_not_be_read_names_the_path_and_the_error() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let file_error_text = "No such file or directory (os error 2)";
        let file_error = io::Error::other(file_error_text);
        let error = Error::File {
            path,
            error: file_error,
        };

        let error_text = error.to_string();

        let expectation = "[LANGUAGE] cannot read `crates/harbor/src/lib.rs`: No such file or directory (os error 2).";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_that_could_not_be_decoded_names_the_path_and_the_error_without_its_last_line_break()
     {
        let path_text = "crates/harbor/Cargo.toml";
        let path = PathBuf::from(path_text);
        let manifest_error_text = r#"TOML parse error at line 1, column 14
invalid table header
"#;
        let manifest_error = io::Error::other(manifest_error_text);
        let error = Error::Manifest {
            path,
            error: manifest_error,
        };

        let error_text = error.to_string();

        let expectation = r#"[LANGUAGE] `crates/harbor/Cargo.toml` is not a valid crate manifest: TOML parse error at line 1, column 14
invalid table header."#;
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_source_that_writes_unknown_words_names_the_path_the_line_and_the_column_of_each() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let language = Language::default();
        let identifier_texts: &[&str] = &["helper", "JsonReport"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();
        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let defects = Defects::default();
        let error = Error::Language {
            path,
            unknown_vocabulary,
            defects,
        };

        let error_text = error.to_string();

        let expectation = r#"[LANGUAGE] crates/harbor/src/lib.rs:1:1: `helper` is not made of words of this crate's vocabulary.
[LANGUAGE] crates/harbor/src/lib.rs:1:2: `JsonReport` is not made of words of this crate's vocabulary."#;
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_source_with_defects_names_the_path_the_line_and_the_column_of_each() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let unknown_vocabulary = UnknownVocabulary::default();
        let line: usize = 2;
        let column: usize = 1;
        let comment = Defect::Comment { line, column };
        let test_attribute = Defect::TestAttribute { line, column };
        let defect_list = [comment, test_attribute];
        let defects = defect_list.into_iter().collect();
        let error = Error::Language {
            path,
            unknown_vocabulary,
            defects,
        };

        let error_text = error.to_string();

        let expectation = r#"[LANGUAGE] crates/harbor/src/lib.rs:2:1: the test attribute does not open the `tests` module at the bottom of the source.
[LANGUAGE] crates/harbor/src/lib.rs:2:1: the `doc` attribute is a comment, and code must not contain comments."#;
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn the_source_of_a_file_error_is_the_error_of_the_file() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let file_error_text = "torn";
        let file_error = io::Error::other(file_error_text);
        let error = Error::File {
            path,
            error: file_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "torn";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_a_manifest_error_is_the_error_of_the_decoder() {
        let path_text = "crates/harbor/Cargo.toml";
        let path = PathBuf::from(path_text);
        let manifest_error_text = "no table";
        let manifest_error = io::Error::other(manifest_error_text);
        let error = Error::Manifest {
            path,
            error: manifest_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "no table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn a_source_the_parser_refuses_names_the_path_and_the_error() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let syntax_error_text = "1:4: the source is not valid Rust";
        let syntax_error = io::Error::other(syntax_error_text);
        let error = Error::Syntax {
            path,
            error: syntax_error,
        };

        let error_text = error.to_string();

        let expectation = "[LANGUAGE] crates/harbor/src/lib.rs:1:4: the source is not valid Rust.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn the_source_of_a_syntax_error_is_the_error_of_the_parser() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let syntax_error_text = "torn";
        let syntax_error = io::Error::other(syntax_error_text);
        let error = Error::Syntax {
            path,
            error: syntax_error,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation_text = "torn";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn an_error_of_the_language_has_no_source() {
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);
        let unknown_vocabulary = UnknownVocabulary::default();
        let defects = Defects::default();
        let error = Error::Language {
            path,
            unknown_vocabulary,
            defects,
        };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn an_error_of_unused_terms_has_no_source() {
        let path_text = "crates/harbor";
        let path = PathBuf::from(path_text);
        let unused_terms = UnusedTerms::default();
        let error = Error::Terms { path, unused_terms };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn a_directory_without_a_source_names_its_path() {
        let path_text = "crates/harbor";
        let path = PathBuf::from(path_text);
        let error = Error::SourceAbsence { path };

        let error_text = error.to_string();

        let expectation = "[LANGUAGE] there is no Rust source under `crates/harbor`.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_of_a_source_absence_has_no_source() {
        let path_text = "crates/harbor";
        let path = PathBuf::from(path_text);
        let error = Error::SourceAbsence { path };

        let source = error.source();

        let source_text = source.map(ToString::to_string);
        let expectation = None;
        assert_eq!(source_text, expectation);
    }

    #[test]
    fn the_source_of_a_book_error_and_of_a_directory_error_is_the_error_of_the_writer() {
        let book_path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
        let book_path = PathBuf::from(book_path_text);
        let book_error_text = "no room";
        let book_io_error = io::Error::other(book_error_text);
        let book_error = Error::Book {
            path: book_path,
            error: book_io_error,
        };
        let directory_path_text = "crates/harbor/docs";
        let directory_path = PathBuf::from(directory_path_text);
        let directory_error_text = "no sight";
        let directory_io_error = io::Error::other(directory_error_text);
        let directory_error = Error::Directory {
            path: directory_path,
            error: directory_io_error,
        };

        let book_source = book_error.source().map(ToString::to_string);
        let directory_source = directory_error.source().map(ToString::to_string);

        let book_expectation = Some(String::from(book_error_text));
        let directory_expectation = Some(String::from(directory_error_text));
        assert_eq!(book_source, book_expectation);
        assert_eq!(directory_source, directory_expectation);
    }

    #[test]
    fn a_vocabulary_error_reads_as_the_error_of_the_languages_and_has_it_as_its_source() {
        let path_text = "contexts/treasure";
        let path = PathBuf::from(path_text);
        let languages_error = languages::Error::Context { path };
        let languages_text = languages_error.to_string();
        let error = Error::from(languages_error);

        let error_text = error.to_string();
        let source_text = error.source().map(ToString::to_string);

        let text_expectation = format!("[LANGUAGE] {languages_text}.");
        let source_expectation = Some(languages_text);
        assert_eq!(error_text, text_expectation);
        assert_eq!(source_text, source_expectation);
    }
}
