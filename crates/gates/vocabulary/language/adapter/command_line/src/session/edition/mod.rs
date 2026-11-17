use crate::outcome::Outcome;
use gates_vocabulary_language_application::Error;
use gates_vocabulary_language_application::press::edition::{BookEntry, Edition as PressEdition};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io::{self, Write};

pub(crate) struct Edition<Report, ErrorStream> {
    report: Report,
    errors: ErrorStream,
}

impl<Report, ErrorStream> Layer for Edition<Report, ErrorStream> {
    type Layer = InfrastructureLayer;
}

impl<Report: Write, ErrorStream: Write> Edition<Report, ErrorStream> {
    pub(crate) fn try_report_edition(
        mut self,
        edition_outcome: Result<PressEdition, Error>,
    ) -> io::Result<Outcome> {
        let outcome = match edition_outcome {
            Ok(press_edition) => self.try_report_book_entries(press_edition)?,
            Err(error) => {
                let Self { errors, .. } = &mut self;
                writeln!(errors, "{error}")?;
                Outcome::Error
            }
        };
        self.try_flush_streams()?;
        Ok(outcome)
    }

    fn try_report_book_entries(&mut self, press_edition: PressEdition) -> io::Result<Outcome> {
        let mut outcome = Outcome::Success;
        for book_entry in &press_edition {
            let book_outcome = self.try_report_book_entry(book_entry)?;
            outcome = outcome | book_outcome;
        }
        Ok(outcome)
    }

    fn try_report_book_entry(&mut self, book_entry: &BookEntry) -> io::Result<Outcome> {
        let Self { report, errors } = self;
        match book_entry {
            BookEntry::Publication { .. } | BookEntry::Omission { .. } => {
                writeln!(report, "{book_entry}")?;
                let outcome = Outcome::Success;
                Ok(outcome)
            }
            BookEntry::Error { .. } => {
                writeln!(errors, "{book_entry}")?;
                let outcome = Outcome::Error;
                Ok(outcome)
            }
        }
    }

    fn try_flush_streams(&mut self) -> io::Result<()> {
        let Self { report, errors } = self;
        report.flush()?;
        errors.flush()
    }
}

impl<Report, ErrorStream> From<(Report, ErrorStream)> for Edition<Report, ErrorStream> {
    fn from((report, errors): (Report, ErrorStream)) -> Self {
        Self { report, errors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufWriter;
    use std::path::PathBuf;

    mod helper {
        use super::*;

        pub(super) fn press_edition() -> PressEdition {
            let publication_path_text = "crates/harbor/docs/UBIQUITOUS_LANGUAGE.md";
            let publication_path = PathBuf::from(publication_path_text);
            let term_count: usize = 2;
            let publication = BookEntry::Publication {
                path: publication_path,
                term_count,
            };
            let omission_path_text = "crates/ship/docs/UBIQUITOUS_LANGUAGE.md";
            let omission_path = PathBuf::from(omission_path_text);
            let omission = BookEntry::Omission {
                path: omission_path,
            };
            let book_entry_list = [publication, omission];
            book_entry_list.into_iter().collect()
        }

        pub(super) fn error_edition() -> PressEdition {
            let path_text = "crates/harbor";
            let path = PathBuf::from(path_text);
            let error = Error::SourceAbsence { path };
            let error_entry = BookEntry::Error { error };
            let book_entry_list = [error_entry];
            book_entry_list.into_iter().collect()
        }
    }

    #[test]
    fn every_publication_and_omission_reaches_the_report_and_is_a_success() {
        let press_edition = helper::press_edition();
        let edition_outcome = Ok(press_edition);
        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let edition = Edition::from((&mut report, &mut errors));

        let outcome = edition.try_report_edition(edition_outcome).unwrap();

        let report_text = String::from_utf8(report).unwrap();
        let report_expectation = r#"[UBIQUITOUS] `crates/harbor/docs/UBIQUITOUS_LANGUAGE.md` now contains 2 term(s).
[UBIQUITOUS] skipped `crates/ship/docs/UBIQUITOUS_LANGUAGE.md`: its directory does not exist.
"#;
        let outcome_expectation = Outcome::Success;
        assert_eq!(report_text, report_expectation);
        assert_eq!(outcome, outcome_expectation);
    }

    #[test]
    fn an_error_of_a_book_reaches_the_errors_and_is_an_error() {
        let press_edition = helper::error_edition();
        let edition_outcome = Ok(press_edition);
        let mut report: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let edition = Edition::from((&mut report, &mut errors));

        let outcome = edition.try_report_edition(edition_outcome).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[LANGUAGE] there is no Rust source under `crates/harbor`.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(error_text, error_expectation);
        assert_eq!(outcome, outcome_expectation);
    }

    #[test]
    fn an_edition_whose_report_of_a_book_reaches_a_closed_stream_is_an_error() {
        let press_edition = helper::press_edition();
        let edition_outcome = Ok(press_edition);
        let mut writer_bytes: [u8; 0] = [];
        let report: &mut [u8] = &mut writer_bytes;
        let errors: Vec<u8> = Vec::new();
        let edition = Edition::from((report, errors));

        let outcome = edition.try_report_edition(edition_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_edition_whose_error_of_a_book_reaches_a_closed_stream_is_an_error() {
        let press_edition = helper::error_edition();
        let edition_outcome = Ok(press_edition);
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors: &mut [u8] = &mut writer_bytes;
        let edition = Edition::from((report, errors));

        let outcome = edition.try_report_edition(edition_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_edition_whose_error_of_the_assembly_reaches_a_closed_stream_is_an_error() {
        let path_text = "crates/harbor";
        let path = PathBuf::from(path_text);
        let error = Error::SourceAbsence { path };
        let edition_outcome = Err(error);
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors: &mut [u8] = &mut writer_bytes;
        let edition = Edition::from((report, errors));

        let outcome = edition.try_report_edition(edition_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_edition_whose_report_cannot_be_flushed_is_an_error() {
        let press_edition = helper::press_edition();
        let edition_outcome = Ok(press_edition);
        let mut writer_bytes: [u8; 0] = [];
        let report = BufWriter::new(&mut writer_bytes[..]);
        let errors: Vec<u8> = Vec::new();
        let edition = Edition::from((report, errors));

        let outcome = edition.try_report_edition(edition_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_edition_whose_error_stream_cannot_be_flushed_is_an_error() {
        let press_edition = helper::error_edition();
        let edition_outcome = Ok(press_edition);
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors = BufWriter::new(&mut writer_bytes[..]);
        let edition = Edition::from((report, errors));

        let outcome = edition.try_report_edition(edition_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }
}
