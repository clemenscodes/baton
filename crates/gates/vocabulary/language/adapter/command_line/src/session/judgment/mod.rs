use crate::outcome::Outcome;
use gates_vocabulary_language_application::gate::Errors;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io::{self, Write};

pub(crate) struct Judgment<ErrorStream> {
    errors: ErrorStream,
}

impl<ErrorStream> Layer for Judgment<ErrorStream> {
    type Layer = InfrastructureLayer;
}

impl<ErrorStream: Write> Judgment<ErrorStream> {
    pub(crate) fn try_report_judgment(self, judgment: Result<(), Errors>) -> io::Result<Outcome> {
        let Self { mut errors } = self;
        let outcome = match judgment {
            Ok(()) => Outcome::Success,
            Err(gate_errors) => {
                for error in gate_errors {
                    writeln!(errors, "{error}")?;
                }
                Outcome::Error
            }
        };
        errors.flush()?;
        Ok(outcome)
    }
}

impl<ErrorStream> From<ErrorStream> for Judgment<ErrorStream> {
    fn from(errors: ErrorStream) -> Self {
        Self { errors }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_application::Error;
    use std::io::BufWriter;
    use std::path::PathBuf;

    mod helper {
        use super::*;

        pub(super) fn judgment_errors() -> Result<(), Errors> {
            let path_text = "crates/harbor";
            let path = PathBuf::from(path_text);
            let error = Error::SourceAbsence { path };
            let errors = Errors::from(error);
            Err(errors)
        }
    }

    #[test]
    fn every_error_of_a_judgment_reaches_the_errors_and_is_an_error() {
        let judgment_outcome = helper::judgment_errors();
        let mut errors: Vec<u8> = Vec::new();
        let judgment = Judgment::from(&mut errors);

        let outcome = judgment.try_report_judgment(judgment_outcome).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[LANGUAGE] there is no Rust source under `crates/harbor`.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(error_text, error_expectation);
        assert_eq!(outcome, outcome_expectation);
    }

    #[test]
    fn a_judgment_whose_error_reaches_a_closed_stream_is_an_error() {
        let judgment_outcome = helper::judgment_errors();
        let mut writer_bytes: [u8; 0] = [];
        let errors: &mut [u8] = &mut writer_bytes;
        let judgment = Judgment::from(errors);

        let outcome = judgment.try_report_judgment(judgment_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_judgment_whose_error_stream_cannot_be_flushed_is_an_error() {
        let judgment_outcome = helper::judgment_errors();
        let mut writer_bytes: [u8; 0] = [];
        let errors = BufWriter::new(&mut writer_bytes[..]);
        let judgment = Judgment::from(errors);

        let outcome = judgment.try_report_judgment(judgment_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }
}
