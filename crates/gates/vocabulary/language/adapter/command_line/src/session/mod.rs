mod edition;
mod judgment;
mod token_report;

use crate::outcome::Outcome;
use crate::session::edition::Edition;
use crate::session::judgment::Judgment;
use crate::session::token_report::TokenReport;
use gates_vocabulary_language_adapter_file_system::FileSystem;
use gates_vocabulary_language_adapter_syntax::Syntax;
use gates_vocabulary_language_adapter_toms_obvious_minimal_language::TomsObviousMinimalLanguage;
use gates_vocabulary_language_application::Error;
use gates_vocabulary_language_application::gate::token::TokenSelection;
use gates_vocabulary_language_application::gate::{Errors, Gate, PathJudgment, TermJudgment};
use gates_vocabulary_language_application::press::{BookEdition, Press};
use gates_vocabulary_language_application::vocabulary::Vocabulary;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use std::io::{self, Write};
use std::path::PathBuf;

type SessionGate = Gate<FileSystem, TomsObviousMinimalLanguage, Syntax>;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Session<VocabularyType, Report, ErrorStream> {
    vocabulary: VocabularyType,
    report: Report,
    errors: ErrorStream,
}

impl<VocabularyType, Report, ErrorStream> Layer for Session<VocabularyType, Report, ErrorStream> {
    type Layer = InfrastructureLayer;
}

impl<VocabularyType: Vocabulary, Report: Write, ErrorStream: Write>
    Session<VocabularyType, Report, ErrorStream>
{
    pub(crate) fn try_publish_books(self, directories: Vec<PathBuf>) -> io::Result<Outcome> {
        let Self {
            vocabulary,
            report,
            errors,
        } = self;
        let file_system = FileSystem;
        let book_edition = BookEdition::from(directories);
        let book_publication = |press: Press<FileSystem>| press.dispatch_command(book_edition);
        let press_outcome = Press::try_from((vocabulary, file_system));
        let edition_outcome = press_outcome.and_then(book_publication);
        let edition = Edition::from((report, errors));
        edition.try_report_edition(edition_outcome)
    }

    pub(crate) fn try_judge_paths(self, paths: Vec<PathBuf>) -> io::Result<Outcome> {
        let Self {
            vocabulary, errors, ..
        } = self;
        let path_judgment = PathJudgment::from(paths);
        let path_resolution = |gate: SessionGate| gate.resolve_query(path_judgment);
        let gate_outcome = Self::try_compose_gate(vocabulary);
        let judgment_outcome = gate_outcome.and_then(path_resolution);
        let judgment = Judgment::from(errors);
        judgment.try_report_judgment(judgment_outcome)
    }

    pub(crate) fn try_judge_terms(self, paths: Vec<PathBuf>) -> io::Result<Outcome> {
        let Self {
            vocabulary, errors, ..
        } = self;
        let term_judgment = TermJudgment::from(paths);
        let term_resolution = |gate: SessionGate| gate.resolve_query(term_judgment);
        let gate_outcome = Self::try_compose_gate(vocabulary);
        let judgment_outcome = gate_outcome.and_then(term_resolution);
        let judgment = Judgment::from(errors);
        judgment.try_report_judgment(judgment_outcome)
    }

    pub(crate) fn try_report_production_tokens(self, paths: Vec<PathBuf>) -> io::Result<Outcome> {
        let Self {
            vocabulary,
            report,
            errors,
        } = self;
        let token_selection = TokenSelection::from(paths);
        let token_resolution = |gate: SessionGate| gate.resolve_query(token_selection);
        let gate_outcome = Self::try_compose_gate(vocabulary);
        let token_outcome = gate_outcome.and_then(token_resolution);
        let token_report = TokenReport::from((report, errors));
        token_report.try_report_production_tokens(token_outcome)
    }

    fn try_compose_gate(vocabulary: VocabularyType) -> Result<SessionGate, Errors> {
        let file_system = FileSystem;
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let syntax = Syntax;
        let gate_outcome: Result<SessionGate, Error> = Gate::try_from((
            vocabulary,
            file_system,
            toms_obvious_minimal_language,
            syntax,
        ));
        gate_outcome.map_err(Errors::from)
    }
}

impl<VocabularyType, Report, ErrorStream> From<(VocabularyType, Report, ErrorStream)>
    for Session<VocabularyType, Report, ErrorStream>
{
    fn from((vocabulary, report, errors): (VocabularyType, Report, ErrorStream)) -> Self {
        Self {
            vocabulary,
            report,
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture;
    use std::io::BufWriter;

    #[test]
    fn a_vocabulary_whose_contexts_cannot_be_listed_fails_the_judgment_with_its_error() {
        let vocabulary_adapter = fixture::VocabularyAdapter::ContextError;
        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let session = Session::from((vocabulary_adapter, &mut report, &mut errors));
        let paths = Vec::new();

        let outcome = session.try_judge_paths(paths).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[LANGUAGE] there is no Rust source under `contexts/treasure`.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(error_text, error_expectation);
        assert_eq!(outcome, outcome_expectation);
    }

    #[test]
    fn a_vocabulary_whose_foreign_languages_cannot_be_listed_fails_the_edition_with_its_error() {
        let vocabulary_adapter = fixture::VocabularyAdapter::ForeignLanguageError;
        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let session = Session::from((vocabulary_adapter, &mut report, &mut errors));
        let directories = Vec::new();

        let outcome = session.try_publish_books(directories).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[LANGUAGE] there is no Rust source under `foreign_languages/anchor`.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(error_text, error_expectation);
        assert_eq!(outcome, outcome_expectation);
    }

    #[test]
    fn production_tokens_that_reach_a_closed_report_are_an_error() {
        let source_text = "fn main() {}
";
        let source_path = fixture::source_factory("language_session_closed_report", source_text);
        let vocabulary_adapter = fixture::VocabularyAdapter::Absence;
        let mut writer_bytes: [u8; 0] = [];
        let report: &mut [u8] = &mut writer_bytes;
        let errors: Vec<u8> = Vec::new();
        let session = Session::from((vocabulary_adapter, report, errors));
        let paths = vec![source_path];

        let outcome = session.try_report_production_tokens(paths);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn production_tokens_whose_report_cannot_be_flushed_are_an_error() {
        let source_text = "fn main() {}\n";
        let source_path = fixture::source_factory("language_session_report_flush", source_text);
        let vocabulary_adapter = fixture::VocabularyAdapter::Absence;
        let mut writer_bytes: [u8; 0] = [];
        let report = BufWriter::new(&mut writer_bytes[..]);
        let errors: Vec<u8> = Vec::new();
        let session = Session::from((vocabulary_adapter, report, errors));
        let paths = vec![source_path];

        let outcome = session.try_report_production_tokens(paths);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }
}
