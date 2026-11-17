mod outcome;
mod session;

pub use session::Session;

use clap::Parser;
use gates_vocabulary_language_application::vocabulary::Vocabulary;
use outcome::Outcome;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Parser)]
#[command(
    name = "language",
    about = "Checks identifiers against the ubiquitous language and prints its books"
)]
pub struct CommandLine {
    #[arg(required_unless_present = "book", value_name = "PATH")]
    paths: Vec<PathBuf>,

    #[arg(long, conflicts_with = "terms")]
    book: bool,

    #[arg(long)]
    terms: bool,

    #[arg(long, conflicts_with_all = ["book", "terms"])]
    tokens: bool,
}

impl Layer for CommandLine {
    type Layer = InfrastructureLayer;
}

impl CommandLine {
    fn try_judge_session<VocabularyType: Vocabulary, Report: Write, ErrorStream: Write>(
        &self,
        session: Session<VocabularyType, Report, ErrorStream>,
    ) -> io::Result<Outcome> {
        let Self { paths, terms, .. } = self;
        let source_paths = paths.clone();
        match terms {
            true => session.try_judge_terms(source_paths),
            false => session.try_judge_paths(source_paths),
        }
    }

    fn try_examine_session<VocabularyType: Vocabulary, Report: Write, ErrorStream: Write>(
        &self,
        session: Session<VocabularyType, Report, ErrorStream>,
    ) -> io::Result<Outcome> {
        let Self { paths, tokens, .. } = self;
        match tokens {
            true => {
                let source_paths = paths.clone();
                session.try_report_production_tokens(source_paths)
            }
            false => self.try_judge_session(session),
        }
    }

    pub fn try_respond_exit_code<VocabularyType: Vocabulary, Report: Write, ErrorStream: Write>(
        &self,
        session: Session<VocabularyType, Report, ErrorStream>,
    ) -> io::Result<ExitCode> {
        let Self { paths, book, .. } = self;
        let outcome = match book {
            true => {
                let directories = paths.clone();
                session.try_publish_books(directories)?
            }
            false => self.try_examine_session(session)?,
        };
        let exit_code = ExitCode::from(outcome);
        Ok(exit_code)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use clap::error::ErrorKind as ParseErrorKind;

    pub(crate) mod fixture {
        use gates_vocabulary_language_application::Error;
        use gates_vocabulary_language_application::vocabulary::{
            ContextList, ForeignLanguageList, Vocabulary,
        };
        use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
        use stack_domain_driven_design_core::port::Port;
        use std::env::temp_dir;
        use std::fs::{create_dir_all, remove_dir_all, write};
        use std::path::PathBuf;

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
        pub(crate) enum VocabularyAdapter {
            Absence,
            ContextError,
            ForeignLanguageError,
        }

        impl Layer for VocabularyAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Port for VocabularyAdapter {}

        impl Vocabulary for VocabularyAdapter {
            fn try_list_contexts(&self) -> Result<ContextList, Error> {
                match self {
                    Self::Absence | Self::ForeignLanguageError => {
                        let context_list = ContextList::default();
                        Ok(context_list)
                    }
                    Self::ContextError => {
                        let path_text = "contexts/treasure";
                        let path = PathBuf::from(path_text);
                        let error = Error::SourceAbsence { path };
                        Err(error)
                    }
                }
            }

            fn try_list_foreign_languages(&self) -> Result<ForeignLanguageList, Error> {
                match self {
                    Self::Absence | Self::ContextError => {
                        let foreign_language_list = ForeignLanguageList::default();
                        Ok(foreign_language_list)
                    }
                    Self::ForeignLanguageError => {
                        let path_text = "foreign_languages/anchor";
                        let path = PathBuf::from(path_text);
                        let error = Error::SourceAbsence { path };
                        Err(error)
                    }
                }
            }
        }

        pub(crate) fn source_factory(directory_name: &str, source_text: &str) -> PathBuf {
            let root = temp_dir();
            let directory = root.join(directory_name);
            create_dir_all(&directory).unwrap();
            remove_dir_all(&directory).unwrap();
            let source_directory = directory.join("src");
            create_dir_all(&source_directory).unwrap();
            let source_path = source_directory.join("lib.rs");
            write(&source_path, source_text).unwrap();
            source_path
        }
    }

    pub(crate) mod helper {
        use super::*;

        pub(crate) fn respond_exit_code(
            command_line_texts: &[&str],
            report: &mut Vec<u8>,
            errors: &mut Vec<u8>,
        ) -> ExitCode {
            let command_line = CommandLine::try_parse_from(command_line_texts).unwrap();
            let vocabulary_adapter = fixture::VocabularyAdapter::Absence;
            let session = Session::from((vocabulary_adapter, report, errors));
            command_line.try_respond_exit_code(session).unwrap()
        }
    }

    #[test]
    fn a_command_line_without_a_path_is_an_error() {
        let command_line_texts = ["language"];

        let command_line = CommandLine::try_parse_from(command_line_texts);

        let error = command_line.err();
        let error_kind = error.as_ref().map(clap::Error::kind);
        let error_kind_expectation = ParseErrorKind::MissingRequiredArgument;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_command_line_of_paths_yields_every_path_it_names() {
        let command_line_texts = ["language", "harbor/mod.rs", "lighthouse/mod.rs"];

        let command_line = CommandLine::try_parse_from(command_line_texts).unwrap();

        let first_path_text = "harbor/mod.rs";
        let first_path = PathBuf::from(first_path_text);
        let last_path_text = "lighthouse/mod.rs";
        let last_path = PathBuf::from(last_path_text);
        let paths = vec![first_path, last_path];
        let book = false;
        let terms = false;
        let tokens = false;
        let expectation = CommandLine {
            paths,
            book,
            terms,
            tokens,
        };
        assert_eq!(command_line, expectation);
    }

    #[test]
    fn a_command_line_of_the_books_alone_names_no_path() {
        let command_line_texts = ["language", "--book"];

        let command_line = CommandLine::try_parse_from(command_line_texts).unwrap();

        let paths = Vec::new();
        let book = true;
        let terms = false;
        let tokens = false;
        let expectation = CommandLine {
            paths,
            book,
            terms,
            tokens,
        };
        assert_eq!(command_line, expectation);
    }

    #[test]
    fn a_command_line_of_the_terms_yields_every_path_it_names() {
        let command_line_texts = ["language", "--terms", "crates/harbor/src/lib.rs"];

        let command_line = CommandLine::try_parse_from(command_line_texts).unwrap();

        let source_path_text = "crates/harbor/src/lib.rs";
        let source_path = PathBuf::from(source_path_text);
        let paths = vec![source_path];
        let book = false;
        let terms = true;
        let tokens = false;
        let expectation = CommandLine {
            paths,
            book,
            terms,
            tokens,
        };
        assert_eq!(command_line, expectation);
    }

    #[test]
    fn a_command_line_of_the_books_and_the_terms_is_an_error() {
        let command_line_texts = ["language", "--book", "--terms", "crates/harbor/src/lib.rs"];

        let command_line = CommandLine::try_parse_from(command_line_texts);

        let error = command_line.err();
        let error_kind = error.as_ref().map(clap::Error::kind);
        let error_kind_expectation = ParseErrorKind::ArgumentConflict;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_source_in_the_words_of_rust_leaves_the_command_line_a_success() {
        let source_text = "fn main() {}\n";
        let source_path = fixture::source_factory("language_command_line_rust_source", source_text);
        let source_path_text = source_path.to_str().unwrap();
        let command_line_texts = ["language", source_path_text];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let report_text = String::from_utf8(report).unwrap();
        let error_text = String::from_utf8(errors).unwrap();

        let exit_code_expectation = ExitCode::SUCCESS;
        assert_eq!(exit_code, exit_code_expectation);
        assert_eq!(report_text, "");
        assert_eq!(error_text, "");
    }

    #[test]
    fn a_command_line_of_sources_that_cannot_be_read_is_an_error() {
        let command_line_texts = ["language", "nowhere/treasure.rs", "nowhere/chest.rs"];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();

        let error_expectation = r#"[LANGUAGE] cannot read `nowhere/treasure.rs`: No such file or directory (os error 2).
[LANGUAGE] cannot read `nowhere/chest.rs`: No such file or directory (os error 2).
"#;
        let exit_code_expectation = ExitCode::FAILURE;
        assert_eq!(error_text, error_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn the_terms_of_a_source_of_no_context_leave_the_command_line_a_success() {
        let source_text = "fn main() {}\n";
        let source_path = fixture::source_factory("language_command_line_terms", source_text);
        let source_path_text = source_path.to_str().unwrap();
        let command_line_texts = ["language", "--terms", source_path_text];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();

        let exit_code_expectation = ExitCode::SUCCESS;
        assert_eq!(exit_code, exit_code_expectation);
        assert_eq!(error_text, "");
    }

    #[test]
    fn a_command_line_of_the_terms_of_sources_that_cannot_be_read_is_an_error() {
        let command_line_texts = ["language", "--terms", "nowhere/treasure.rs"];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();

        let error_expectation = r#"[LANGUAGE] cannot read `nowhere/treasure.rs`: No such file or directory (os error 2).
"#;
        let exit_code_expectation = ExitCode::FAILURE;
        assert_eq!(error_text, error_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn a_command_line_of_tokens_reports_the_tokens_of_its_sources() {
        let source_text = "fn main() {}\n";
        let source_path = fixture::source_factory("language_command_line_tokens", source_text);
        let source_path_text = source_path.to_str().unwrap();
        let command_line_texts = ["language", "--tokens", source_path_text];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let report_text = String::from_utf8(report).unwrap();
        let error_text = String::from_utf8(errors).unwrap();

        let report_expectation = format!(
            "{source_path_text}\t1\t1\tidentifier\tfn\n{source_path_text}\t1\t4\tidentifier\tmain\n{source_path_text}\t1\t8\tpunctuation\t(\n{source_path_text}\t1\t9\tpunctuation\t)\n{source_path_text}\t1\t11\tpunctuation\t{{\n{source_path_text}\t1\t12\tpunctuation\t}}\n"
        );
        let exit_code_expectation = ExitCode::SUCCESS;
        assert_eq!(report_text, report_expectation);
        assert_eq!(error_text, "");
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn a_command_line_of_the_books_of_no_context_is_a_success() {
        let command_line_texts = ["language", "--book"];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let report_text = String::from_utf8(report).unwrap();
        let error_text = String::from_utf8(errors).unwrap();

        let exit_code_expectation = ExitCode::SUCCESS;
        assert_eq!(exit_code, exit_code_expectation);
        assert_eq!(report_text, "");
        assert_eq!(error_text, "");
    }

    #[test]
    fn a_command_line_of_the_book_of_a_directory_no_context_admits_is_an_error() {
        let command_line_texts = ["language", "--book", "crates/nowhere"];

        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();

        let exit_code = helper::respond_exit_code(&command_line_texts, &mut report, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();

        let error_expectation = r#"[LANGUAGE] `crates/nowhere` is not the directory of a context.
"#;
        let exit_code_expectation = ExitCode::FAILURE;
        assert_eq!(error_text, error_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn a_command_line_of_books_whose_error_reaches_a_closed_stream_is_an_error() {
        let command_line_texts = ["language", "--book", "crates/nowhere"];
        let command_line = CommandLine::try_parse_from(command_line_texts).unwrap();
        let vocabulary_adapter = fixture::VocabularyAdapter::Absence;
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors: &mut [u8] = &mut writer_bytes;
        let session = Session::from((vocabulary_adapter, report, errors));

        let exit_code = command_line.try_respond_exit_code(session);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_command_line_of_paths_whose_error_reaches_a_closed_stream_is_an_error() {
        let command_line_texts = ["language", "nowhere/treasure.rs"];
        let command_line = CommandLine::try_parse_from(command_line_texts).unwrap();
        let vocabulary_adapter = fixture::VocabularyAdapter::Absence;
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors: &mut [u8] = &mut writer_bytes;
        let session = Session::from((vocabulary_adapter, report, errors));

        let exit_code = command_line.try_respond_exit_code(session);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }
}
