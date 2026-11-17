mod error;
mod export;
mod message;
mod regions;

use error::Error;
use message::{Message, TestExecutable};

use crate::export::Export;
use gates_coverage_core_report::Decoder;
use gates_coverage_core_report::Report;
use gates_coverage_core_test_build::TestBuild;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::io::{self, Read};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct JavaScriptObjectNotation;

impl Layer for JavaScriptObjectNotation {
    type Layer = InfrastructureLayer;
}

impl Adapter for JavaScriptObjectNotation {}

impl Port for JavaScriptObjectNotation {}

impl DrivenPort for JavaScriptObjectNotation {}

impl Decoder for JavaScriptObjectNotation {
    type Error = Error;

    fn try_decode_report(&self, input: impl Read) -> Result<Report, Self::Error> {
        let text = io::read_to_string(input)?;
        let export: Export = serde_json::from_str(&text)?;
        let regions = export.try_decode_regions()?;
        let report = Report::from_iter(regions);
        Ok(report)
    }

    fn try_decode_test_build(&self, output: &[u8]) -> Result<TestBuild, Self::Error> {
        let text = io::read_to_string(output)?;
        let test_executables = self.try_decode_test_executables(&text)?;
        let first_test_executable = test_executables
            .first()
            .ok_or(Error::TestExecutableAbsence)?;
        let manifest_path = first_test_executable.manifest_path().clone();
        let executable_sequence = test_executables
            .iter()
            .map(|test_executable| test_executable.executable().clone());
        let executables = Vec::from_iter(executable_sequence);
        let test_build = TestBuild::from((executables, manifest_path));
        Ok(test_build)
    }
}

impl JavaScriptObjectNotation {
    fn try_decode_test_executables(&self, text: &str) -> Result<Vec<TestExecutable>, Error> {
        let mut test_executables = Vec::new();
        let message_criterion = |message_text: &&str| !message_text.trim().is_empty();
        for message_text in text.lines().filter(message_criterion) {
            let message: Message = serde_json::from_str(message_text)
                .map_err(|error| Error::Message { error })?;
            if let Ok(test_executable) = TestExecutable::try_from(message) {
                test_executables.push(test_executable);
            }
        }
        Ok(test_executables)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_coverage_core_region::Region;
    use std::path::PathBuf;

    mod fixture {
        pub(super) fn span_factory(start_line: usize) -> gates_coverage_core_span::Span {
            let path_text = "crates/harbor/src/lib.rs";
            let path = std::path::PathBuf::from(path_text);
            let start_column: usize = 1;
            let end_column: usize = 2;
            let start_position = gates_coverage_core_span::position::Position::try_from((
                start_line,
                start_column,
            ))
            .unwrap();
            let end_position = gates_coverage_core_span::position::Position::try_from((
                start_line, end_column,
            ))
            .unwrap();
            gates_coverage_core_span::Span::try_from((path, start_position, end_position)).unwrap()
        }
    }

    #[test]
    fn an_export_of_llvm_cov_is_a_report_of_its_regions() {
        let text = r#"{"data": [{"files": [{"filename": "crates/harbor/src/lib.rs"}], "functions": [
            {"filenames": ["crates/harbor/src/lib.rs"], "regions": [[3, 1, 3, 2, 0, 0, 0, 0]]}
        ]}]}"#;
        let input = text.as_bytes();
        let java_script_object_notation = JavaScriptObjectNotation;

        let report = java_script_object_notation
            .try_decode_report(input)
            .unwrap();

        let start_line: usize = 3;
        let span = fixture::span_factory(start_line);
        let execution_count: usize = 0;
        let region = Region::from((span, execution_count));
        let regions = [region];
        let expectation = Report::from_iter(regions);
        assert_eq!(report, expectation);
    }

    #[test]
    fn an_input_that_cannot_be_read_is_an_error() {
        let input: &[u8] = &[255];
        let java_script_object_notation = JavaScriptObjectNotation;

        let report = java_script_object_notation.try_decode_report(input);

        let error_text = report.unwrap_err().to_string();
        let expectation = "[COVERAGE] cannot read the input: stream did not contain valid UTF-8.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_text_that_is_no_export_is_an_error() {
        let text = "no export";
        let input = text.as_bytes();
        let java_script_object_notation = JavaScriptObjectNotation;

        let report = java_script_object_notation.try_decode_report(input);

        let error_text = report.unwrap_err().to_string();
        let expectation = "[COVERAGE] the report is not an llvm-cov export: expected ident at line 1 column 2.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn an_export_whose_regions_are_an_error_is_an_error() {
        let text = r#"{"data": [{"files": [], "functions": [{"filenames": [], "regions": [[3, 1, 3, 2, 7, 0, 0, 0]]}]}]}"#;
        let input = text.as_bytes();
        let java_script_object_notation = JavaScriptObjectNotation;

        let report = java_script_object_notation.try_decode_report(input);

        let error_text = report.unwrap_err().to_string();
        let expectation = "[COVERAGE] a region refers to file index 0, but the file indexes of its function end before 0.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn the_test_executables_of_a_build_and_the_manifest_of_their_package_are_its_test_build() {
        let text = r#"{"reason":"compiler-artifact","manifest_path":"/workspace/crates/harbor/Cargo.toml","profile":{"test":false},"executable":null}
{"reason":"compiler-artifact","manifest_path":"/workspace/crates/harbor/Cargo.toml","profile":{"test":true},"executable":"/target/debug/deps/harbor-0a1b"}

{"reason":"compiler-artifact","manifest_path":"/workspace/crates/harbor/Cargo.toml","profile":{"test":true},"executable":"/target/debug/deps/harbor_tests-2c3d"}
{"reason":"build-finished","success":true}"#;
        let output = text.as_bytes();
        let java_script_object_notation = JavaScriptObjectNotation;

        let test_build = java_script_object_notation
            .try_decode_test_build(output)
            .unwrap();

        let first_executable = PathBuf::from("/target/debug/deps/harbor-0a1b");
        let last_executable = PathBuf::from("/target/debug/deps/harbor_tests-2c3d");
        let executables = vec![first_executable, last_executable];
        let manifest_path = PathBuf::from("/workspace/crates/harbor/Cargo.toml");
        let expectation = TestBuild::from((executables, manifest_path));
        assert_eq!(test_build, expectation);
    }

    #[test]
    fn a_build_without_a_test_executable_is_an_error() {
        let text = r#"{"reason":"build-finished","success":true}"#;
        let output = text.as_bytes();
        let java_script_object_notation = JavaScriptObjectNotation;

        let test_build = java_script_object_notation.try_decode_test_build(output);

        let error_text = test_build.unwrap_err().to_string();
        let expectation = "[COVERAGE] the build made no executable of a test, so no test can be measured.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_build_output_that_is_no_message_of_cargo_is_an_error() {
        let text = "Compiling harbor";
        let output = text.as_bytes();
        let java_script_object_notation = JavaScriptObjectNotation;

        let test_build = java_script_object_notation.try_decode_test_build(output);

        let error_text = test_build.unwrap_err().to_string();
        let expectation = "[COVERAGE] a line of the output of the build is no message of Cargo: expected value at line 1 column 1.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_build_output_that_cannot_be_read_is_an_error() {
        let output: &[u8] = &[255];
        let java_script_object_notation = JavaScriptObjectNotation;

        let test_build = java_script_object_notation.try_decode_test_build(output);

        let error_text = test_build.unwrap_err().to_string();
        let expectation = "[COVERAGE] cannot read the input: stream did not contain valid UTF-8.";
        assert_eq!(error_text, expectation);
    }
}
