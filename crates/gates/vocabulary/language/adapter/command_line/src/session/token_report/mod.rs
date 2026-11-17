use crate::outcome::Outcome;
use gates_vocabulary_language_application::gate::Errors;
use gates_vocabulary_language_application::gate::token::TokenReport as GateTokenReport;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io::{self, Write};

pub(crate) struct TokenReport<Report, ErrorStream> {
    report: Report,
    errors: ErrorStream,
}

impl<Report, ErrorStream> Layer for TokenReport<Report, ErrorStream> {
    type Layer = InfrastructureLayer;
}

impl<Report: Write, ErrorStream: Write> TokenReport<Report, ErrorStream> {
    pub(crate) fn try_report_production_tokens(
        self,
        token_outcome: Result<GateTokenReport, Errors>,
    ) -> io::Result<Outcome> {
        let Self {
            mut report,
            mut errors,
        } = self;
        let outcome = match token_outcome {
            Ok(production_tokens) => {
                write!(report, "{production_tokens}")?;
                Outcome::Success
            }
            Err(gate_errors) => {
                for error in &gate_errors {
                    writeln!(errors, "{error}")?;
                }
                Outcome::Error
            }
        };
        report.flush()?;
        errors.flush()?;
        Ok(outcome)
    }
}

impl<Report, ErrorStream> From<(Report, ErrorStream)> for TokenReport<Report, ErrorStream> {
    fn from((report, errors): (Report, ErrorStream)) -> Self {
        Self { report, errors }
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

        pub(super) fn token_errors() -> Result<GateTokenReport, Errors> {
            let path_text = "crates/harbor";
            let path = PathBuf::from(path_text);
            let error = Error::SourceAbsence { path };
            let errors = Errors::from(error);
            Err(errors)
        }
    }

    #[test]
    fn an_error_that_reaches_a_closed_stream_is_an_error() {
        let token_outcome = helper::token_errors();
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors: &mut [u8] = &mut writer_bytes;
        let token_report = TokenReport::from((report, errors));

        let outcome = token_report.try_report_production_tokens(token_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_stream_that_cannot_be_flushed_is_an_error() {
        let token_outcome = helper::token_errors();
        let report: Vec<u8> = Vec::new();
        let mut writer_bytes: [u8; 0] = [];
        let errors = BufWriter::new(&mut writer_bytes[..]);
        let token_report = TokenReport::from((report, errors));

        let outcome = token_report.try_report_production_tokens(token_outcome);

        let error_text = outcome.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }
}
