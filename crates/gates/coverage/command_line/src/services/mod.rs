use crate::mode::Mode;
use gates_coverage_adapter_file_system::FileSystem;
use gates_coverage_adapter_java_script_object_notation::JavaScriptObjectNotation;
use gates_coverage_adapter_process::Process;
use gates_coverage_adapter_syntax::Syntax;
use gates_coverage_application::gate::{Gate, Judgment, ReportJudgment};
use gates_coverage_application::measurement::{Measurement, ReportMeasurement};
use gates_coverage_application::{Error, Outcome};
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use std::ffi::OsString;
use std::io::{self, Read, Write};
use std::path::PathBuf;
use std::process::ExitCode;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(crate) struct Services<InputType, OutputType, ErrorsType> {
    input: InputType,
    output: OutputType,
    errors: ErrorsType,
}

impl<InputType, OutputType, ErrorsType> Layer for Services<InputType, OutputType, ErrorsType> {
    type Layer = PresentationLayer;
}

impl<InputType: Read, OutputType: Write, ErrorsType: Write>
    Services<InputType, OutputType, ErrorsType>
{
    pub(crate) fn try_respond_exit_code(self, mode: Mode) -> io::Result<ExitCode> {
        let outcome = match mode {
            Mode::Judgment { report_path } => self.try_judge_report(report_path),
            Mode::Measurement { words } => self.try_measure_report(words),
        };
        outcome.map(ExitCode::from)
    }

    fn try_judge_report(self, report_path: Option<PathBuf>) -> io::Result<Outcome> {
        let Self {
            input,
            mut output,
            mut errors,
        } = self;
        let java_script_object_notation = JavaScriptObjectNotation;
        let file_system = FileSystem;
        let syntax = Syntax;
        let gate = Gate::from((java_script_object_notation, file_system, syntax));
        let report_judgment = match report_path {
            Some(path) => ReportJudgment::File { path },
            None => ReportJudgment::Input { input },
        };
        let judgment = gate.resolve_query(report_judgment);
        let outcome = Self::try_report_judgment(judgment, &mut output, &mut errors)?;
        output.flush()?;
        errors.flush()?;
        Ok(outcome)
    }

    fn try_report_judgment(
        judgment: Result<Judgment, Error>,
        output: &mut OutputType,
        errors: &mut ErrorsType,
    ) -> io::Result<Outcome> {
        match judgment {
            Ok(judgment) => {
                let outcome = judgment.decide_outcome();
                match outcome {
                    Outcome::Success => writeln!(output, "{judgment}")?,
                    Outcome::Error => writeln!(errors, "{judgment}")?,
                }
                Ok(outcome)
            }
            Err(error) => {
                writeln!(errors, "{error}")?;
                let outcome = Outcome::Error;
                Ok(outcome)
            }
        }
    }

    fn try_measure_report(self, words: Vec<OsString>) -> io::Result<Outcome> {
        let Self { mut errors, .. } = self;
        let process = Process;
        let file_system = FileSystem;
        let java_script_object_notation = JavaScriptObjectNotation;
        let measurement = Measurement::from((process, file_system, java_script_object_notation));
        let report_measurement = ReportMeasurement::try_from(words.as_slice());
        let report_dispatch = |command: ReportMeasurement| measurement.dispatch_command(command);
        let measurement_outcome = report_measurement.and_then(report_dispatch);
        let outcome = Outcome::from(&measurement_outcome);
        let measurement_errors: Vec<Error> = measurement_outcome.err().into_iter().collect();
        for error in measurement_errors {
            writeln!(errors, "{error}")?;
        }
        errors.flush()?;
        Ok(outcome)
    }
}

impl<InputType, OutputType, ErrorsType> From<(InputType, OutputType, ErrorsType)>
    for Services<InputType, OutputType, ErrorsType>
{
    fn from((input, output, errors): (InputType, OutputType, ErrorsType)) -> Self {
        Self {
            input,
            output,
            errors,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env::temp_dir;
    use std::fs::{create_dir_all, remove_dir_all, write};
    use std::io::BufWriter;

    mod fixture {
        use super::*;

        pub(super) const EMPTY_REPORT_TEXT: &str = r#"{"data": [{"files": [], "functions": []}]}"#;

        pub(super) fn source_factory(path_text: &str) -> PathBuf {
            let path = temp_dir().join(path_text);
            create_dir_all(&path).unwrap();
            remove_dir_all(&path).unwrap();
            create_dir_all(path.join("src")).unwrap();
            let source_path = path.join("src").join("lib.rs");
            let source_text = "struct Treasure;\n";
            write(&source_path, source_text).unwrap();
            source_path
        }

        pub(super) fn gap_report_factory(source_path: &std::path::Path) -> String {
            let source_path_text = source_path.display();
            format!(
                r#"{{"data": [{{"files": [{{"filename": "{source_path_text}"}}], "functions": [{{"filenames": ["{source_path_text}"], "regions": [[1, 1, 1, 2, 0, 0, 0, 0]]}}]}}]}}"#
            )
        }
    }

    #[test]
    fn the_mode_of_judgment_responds_with_the_judgment_of_its_report() {
        let path = PathBuf::from("nowhere/harbor.json");
        let report_path = Some(path);
        let mode = Mode::Judgment { report_path };
        let input = io::empty();
        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let services = Services::from((input, &mut output, &mut errors));

        let exit_code = services.try_respond_exit_code(mode).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[COVERAGE] cannot read the report `nowhere/harbor.json`: No such file or directory (os error 2).
"#;
        let exit_code_expectation = ExitCode::FAILURE;
        assert_eq!(error_text, error_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn a_report_of_no_region_on_the_input_is_a_success() {
        let report_path = None;
        let mode = Mode::Judgment { report_path };
        let input = fixture::EMPTY_REPORT_TEXT.as_bytes();
        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let services = Services::from((input, &mut output, &mut errors));

        let exit_code = services.try_respond_exit_code(mode).unwrap();

        let output_text = String::from_utf8(output).unwrap();
        let output_expectation = r#"[COVERAGE] every region outside the test modules is covered by a test.
"#;
        let exit_code_expectation = ExitCode::SUCCESS;
        assert_eq!(output_text, output_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn a_region_no_test_reaches_is_named_on_the_errors_and_is_an_error() {
        let source_path = fixture::source_factory("coverage_command_line_gap");
        let report_text = fixture::gap_report_factory(&source_path);
        let report_path = None;
        let mode = Mode::Judgment { report_path };
        let input = report_text.as_bytes();
        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let services = Services::from((input, &mut output, &mut errors));

        let exit_code = services.try_respond_exit_code(mode).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let source_path_text = source_path.display();
        let error_expectation = format!(
            "[COVERAGE] {source_path_text}:1:1: no test covers this region.\n"
        );
        let exit_code_expectation = ExitCode::FAILURE;
        assert_eq!(error_text, error_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn the_mode_of_measurement_responds_with_the_measurement_of_its_words() {
        let words = Vec::new();
        let mode = Mode::Measurement { words };
        let input = io::empty();
        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let services = Services::from((input, &mut output, &mut errors));

        let exit_code = services.try_respond_exit_code(mode).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[COVERAGE] a measurement takes the path of its report, the command of the build, `--` and the command of the tests.
"#;
        let exit_code_expectation = ExitCode::FAILURE;
        assert_eq!(error_text, error_expectation);
        assert_eq!(exit_code, exit_code_expectation);
    }

    #[test]
    fn a_success_that_reaches_a_closed_output_is_an_error() {
        let report_path = None;
        let mode = Mode::Judgment { report_path };
        let input = fixture::EMPTY_REPORT_TEXT.as_bytes();
        let mut stream: [u8; 0] = [];
        let output: &mut [u8] = &mut stream;
        let errors: Vec<u8> = Vec::new();
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_gap_that_reaches_closed_errors_is_an_error() {
        let source_path = fixture::source_factory("coverage_command_line_closed_gap");
        let report_text = fixture::gap_report_factory(&source_path);
        let report_path = None;
        let mode = Mode::Judgment { report_path };
        let input = report_text.as_bytes();
        let output: Vec<u8> = Vec::new();
        let mut stream: [u8; 0] = [];
        let errors: &mut [u8] = &mut stream;
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_of_a_judgment_that_reaches_closed_errors_is_an_error() {
        let path = PathBuf::from("nowhere/harbor.json");
        let report_path = Some(path);
        let mode = Mode::Judgment { report_path };
        let input = io::empty();
        let output: Vec<u8> = Vec::new();
        let mut stream: [u8; 0] = [];
        let errors: &mut [u8] = &mut stream;
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_success_whose_output_cannot_be_flushed_is_an_error() {
        let report_path = None;
        let mode = Mode::Judgment { report_path };
        let input = fixture::EMPTY_REPORT_TEXT.as_bytes();
        let mut stream: [u8; 0] = [];
        let output = BufWriter::new(&mut stream[..]);
        let errors: Vec<u8> = Vec::new();
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_of_a_judgment_whose_errors_cannot_be_flushed_is_an_error() {
        let path = PathBuf::from("nowhere/harbor.json");
        let report_path = Some(path);
        let mode = Mode::Judgment { report_path };
        let input = io::empty();
        let output: Vec<u8> = Vec::new();
        let mut stream: [u8; 0] = [];
        let errors = BufWriter::new(&mut stream[..]);
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_of_a_measurement_that_reaches_closed_errors_is_an_error() {
        let words = Vec::new();
        let mode = Mode::Measurement { words };
        let input = io::empty();
        let output: Vec<u8> = Vec::new();
        let mut stream: [u8; 0] = [];
        let errors: &mut [u8] = &mut stream;
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write whole buffer";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_error_of_a_measurement_whose_errors_cannot_be_flushed_is_an_error() {
        let words = Vec::new();
        let mode = Mode::Measurement { words };
        let input = io::empty();
        let output: Vec<u8> = Vec::new();
        let mut stream: [u8; 0] = [];
        let errors = BufWriter::new(&mut stream[..]);
        let services = Services::from((input, output, errors));

        let exit_code = services.try_respond_exit_code(mode);

        let error_text = exit_code.unwrap_err().to_string();
        let expectation = "failed to write the buffered data";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_measurement_whose_profiles_cannot_be_kept_is_an_error_before_any_program_starts() {
        let word_texts = ["/proc/coverage/harbor.json", "cargo", "--", "cargo"];
        let words = Vec::from_iter(word_texts.map(OsString::from));
        let mode = Mode::Measurement { words };
        let input = io::empty();
        let mut output: Vec<u8> = Vec::new();
        let mut errors: Vec<u8> = Vec::new();
        let services = Services::from((input, &mut output, &mut errors));

        let exit_code = services.try_respond_exit_code(mode).unwrap();

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = "[COVERAGE] cannot keep the profiles of the measurement in a directory of their own";
        let exit_code_expectation = ExitCode::FAILURE;
        assert!(error_text.starts_with(error_expectation));
        assert_eq!(exit_code, exit_code_expectation);
    }
}
