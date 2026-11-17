mod judgment;
mod report_judgment;

pub use judgment::Judgment;
pub use report_judgment::ReportJudgment;

use crate::Error;
use gates_coverage_core_gaps::Gaps;
use gates_coverage_core_report::Decoder;
use gates_coverage_core_report::Report;
use gates_coverage_core_source::Reader;
use gates_coverage_core_test_boundaries::Parser;
use gates_coverage_core_test_boundaries::TestBoundaries;
use stack_domain_driven_design_core::application_service::ApplicationService;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use std::io::{self, Read};
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Gate<DecoderType, ReaderType, ParserType> {
    decoder: DecoderType,
    reader: ReaderType,
    parser: ParserType,
}

impl<DecoderType, ReaderType, ParserType> Layer for Gate<DecoderType, ReaderType, ParserType> {
    type Layer = ApplicationLayer;
}

impl<DecoderType, ReaderType, ParserType> ApplicationService
    for Gate<DecoderType, ReaderType, ParserType>
{
}

impl<DecoderType, ReaderType, ParserType> From<(DecoderType, ReaderType, ParserType)>
    for Gate<DecoderType, ReaderType, ParserType>
{
    fn from((decoder, reader, parser): (DecoderType, ReaderType, ParserType)) -> Self {
        Self {
            decoder,
            reader,
            parser,
        }
    }
}

impl<DecoderType: Decoder, ReaderType: Reader, ParserType: Parser, InputType: Read>
    SynchronousQueryHandler<ReportJudgment<InputType>>
    for Gate<DecoderType, ReaderType, ParserType>
{
    fn resolve_query(&self, report_judgment: ReportJudgment<InputType>) -> Result<Judgment, Error> {
        let report = match report_judgment {
            ReportJudgment::File { path } => self.try_decode_report_file(&path)?,
            ReportJudgment::Input { input } => self.try_decode_report(input)?,
        };
        let test_boundaries = self.try_read_test_boundaries(&report)?;
        let gaps = Gaps::from((&report, &test_boundaries));
        let judgment = Judgment::from(gaps);
        Ok(judgment)
    }
}

impl<DecoderType: Decoder, ReaderType: Reader, ParserType: Parser>
    Gate<DecoderType, ReaderType, ParserType>
{
    fn try_decode_report_file(&self, report_path: &Path) -> Result<Report, Error> {
        let Self { reader, .. } = self;
        let report_export_error = |error: io::Error| {
            let path = report_path.to_path_buf();
            Error::ReportExport { path, error }
        };
        let report_file = reader
            .try_open_report(report_path)
            .map_err(report_export_error)?;
        self.try_decode_report(report_file)
    }

    fn try_decode_report(&self, input: impl Read) -> Result<Report, Error> {
        let Self { decoder, .. } = self;
        let report_error = |error: DecoderType::Error| {
            let text = error.to_string();
            Error::Report { text }
        };
        decoder.try_decode_report(input).map_err(report_error)
    }

    fn try_read_test_boundaries(&self, report: &Report) -> Result<TestBoundaries, Error> {
        let paths = report.list_paths();
        let mut test_boundary_sequence = Vec::new();
        for path in paths {
            let source_test_boundaries = self.try_read_source_test_boundaries(path)?;
            test_boundary_sequence.extend(source_test_boundaries);
        }
        let test_boundaries = TestBoundaries::from_iter(test_boundary_sequence);
        Ok(test_boundaries)
    }

    fn try_read_source_test_boundaries(&self, path: PathBuf) -> Result<TestBoundaries, Error> {
        let Self { reader, parser, .. } = self;
        let source = match reader.try_read_source(&path) {
            Ok(source) => source,
            Err(error) => {
                let source_error = Error::Source { path, error };
                return Err(source_error);
            }
        };
        let syntax_error = |parser_error: ParserType::Error| {
            let text = parser_error.to_string();
            Error::Syntax { text }
        };
        parser
            .try_parse_test_boundaries(&source)
            .map_err(syntax_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Outcome;
    use gates_coverage_core_region::Region;
    use gates_coverage_core_source::Source;
    use gates_coverage_core_test_build::TestBuild;
    use stack_domain_driven_design_core::layer::InfrastructureLayer;
    use stack_domain_driven_design_core::port::{DrivenPort, Port};
    use std::io::Write;
    use std::path::Path;

    mod fixture {
        use super::*;
        use stack_domain_driven_design_core::adapter::Adapter;

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

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(super) struct DecoderAdapter {
            report: Result<Report, io::ErrorKind>,
        }

        impl Layer for DecoderAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for DecoderAdapter {}

        impl Port for DecoderAdapter {}

        impl DrivenPort for DecoderAdapter {}

        impl Decoder for DecoderAdapter {
            type Error = io::Error;

            fn try_decode_report(&self, _: impl Read) -> Result<Report, Self::Error> {
                let Self { report } = self;
                report.clone().map_err(io::Error::from)
            }

            fn try_decode_test_build(&self, _: &[u8]) -> Result<TestBuild, Self::Error> {
                let error_kind = io::ErrorKind::Unsupported;
                Err(io::Error::from(error_kind))
            }
        }

        impl From<Result<Report, io::ErrorKind>> for DecoderAdapter {
            fn from(report: Result<Report, io::ErrorKind>) -> Self {
                Self { report }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(super) struct ReaderAdapter {
            text: Result<String, io::ErrorKind>,
        }

        impl Layer for ReaderAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for ReaderAdapter {}

        impl Port for ReaderAdapter {}

        impl DrivenPort for ReaderAdapter {}

        impl Reader for ReaderAdapter {
            fn try_read_source(&self, path: &Path) -> io::Result<Source> {
                let Self { text } = self;
                let source_path = path.to_path_buf();
                let source_text = text.clone().map_err(io::Error::from)?;
                let source = Source::from((source_path, source_text));
                Ok(source)
            }

            fn try_open_report(&self, path: &Path) -> io::Result<impl Read + use<>> {
                let absence_path = Path::new(REPORT_ABSENCE_PATH_TEXT);
                match path == absence_path {
                    true => {
                        let absence_kind = io::ErrorKind::NotFound;
                        let absence_error = io::Error::from(absence_kind);
                        Err(absence_error)
                    }
                    false => Ok(io::empty()),
                }
            }
        }

        pub(super) const REPORT_ABSENCE_PATH_TEXT: &str = "nowhere/harbor.json";

        pub(super) const REPORT_PATH_TEXT: &str = "target/moon/coverage/harbor.json";

        impl From<Result<String, io::ErrorKind>> for ReaderAdapter {
            fn from(text: Result<String, io::ErrorKind>) -> Self {
                Self { text }
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
        pub(super) struct ParserAdapter {
            test_boundaries: Result<TestBoundaries, io::ErrorKind>,
        }

        impl Layer for ParserAdapter {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for ParserAdapter {}

        impl Port for ParserAdapter {}

        impl DrivenPort for ParserAdapter {}

        impl Parser for ParserAdapter {
            type Error = io::Error;

            fn try_parse_test_boundaries(&self, _: &Source) -> Result<TestBoundaries, Self::Error> {
                let Self { test_boundaries } = self;
                test_boundaries.clone().map_err(io::Error::from)
            }
        }

        impl From<Result<TestBoundaries, io::ErrorKind>> for ParserAdapter {
            fn from(test_boundaries: Result<TestBoundaries, io::ErrorKind>) -> Self {
                Self { test_boundaries }
            }
        }

        pub(super) fn judgment_outcome<InputType: Read>(
            gate: &Gate<DecoderAdapter, ReaderAdapter, ParserAdapter>,
            report_judgment: ReportJudgment<InputType>,
            output: &mut Vec<u8>,
            errors: &mut Vec<u8>,
        ) -> Outcome {
            match gate.resolve_query(report_judgment) {
                Ok(judgment) => {
                    let outcome = judgment.decide_outcome();
                    match outcome {
                        Outcome::Success => writeln!(output, "{judgment}").unwrap(),
                        Outcome::Error => writeln!(errors, "{judgment}").unwrap(),
                    }
                    outcome
                }
                Err(error) => {
                    writeln!(errors, "{error}").unwrap();
                    Outcome::Error
                }
            }
        }

        pub(super) fn report_factory(execution_count: usize) -> Report {
            let start_line: usize = 1;
            let span = fixture::span_factory(start_line);
            let region = Region::from((span, execution_count));
            let regions = [region];
            Report::from_iter(regions)
        }

        pub(super) fn text_factory() -> String {
            let source_line_text = r#"struct Treasure;
"#;
            String::from(source_line_text)
        }
    }

    #[test]
    fn a_report_whose_every_region_is_reached_is_a_success() {
        let execution_count: usize = 1;
        let report = fixture::report_factory(execution_count);
        let text = fixture::text_factory();
        let decoder_report = Ok(report);
        let reader_text = Ok(text);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let input = io::empty();
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_judgment = ReportJudgment::Input { input };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let output_text = String::from_utf8(output).unwrap();
        let output_expectation = r#"[COVERAGE] every region outside the test modules is covered by a test.
"#;
        let errors_expectation: Vec<u8> = Vec::new();
        let outcome_expectation = Outcome::Success;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(output_text, output_expectation);
        assert_eq!(errors, errors_expectation);
    }

    #[test]
    fn a_report_at_a_path_is_read_from_its_file_and_judged() {
        let execution_count: usize = 1;
        let report = fixture::report_factory(execution_count);
        let text = fixture::text_factory();
        let decoder_report = Ok(report);
        let reader_text = Ok(text);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_path_text = fixture::REPORT_PATH_TEXT;
        let path = PathBuf::from(report_path_text);
        let report_judgment: ReportJudgment<io::Empty> = ReportJudgment::File { path };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let output_text = String::from_utf8(output).unwrap();
        let output_expectation = r#"[COVERAGE] every region outside the test modules is covered by a test.
"#;
        let errors_expectation: Vec<u8> = Vec::new();
        let outcome_expectation = Outcome::Success;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(output_text, output_expectation);
        assert_eq!(errors, errors_expectation);
    }

    #[test]
    fn a_report_at_a_path_that_cannot_be_opened_is_named_and_is_an_error() {
        let execution_count: usize = 1;
        let report = fixture::report_factory(execution_count);
        let text = fixture::text_factory();
        let decoder_report = Ok(report);
        let reader_text = Ok(text);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_path_text = fixture::REPORT_ABSENCE_PATH_TEXT;
        let path = PathBuf::from(report_path_text);
        let report_judgment: ReportJudgment<io::Empty> = ReportJudgment::File { path };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[COVERAGE] cannot read the report `nowhere/harbor.json`: entity not found.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(error_text, error_expectation);
    }

    #[test]
    fn a_report_with_a_gap_names_it_and_is_an_error() {
        let execution_count: usize = 0;
        let report = fixture::report_factory(execution_count);
        let text = fixture::text_factory();
        let decoder_report = Ok(report);
        let reader_text = Ok(text);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let input = io::empty();
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_judgment = ReportJudgment::Input { input };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[COVERAGE] crates/harbor/src/lib.rs:1:1: no test covers this region.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(error_text, error_expectation);
    }

    #[test]
    fn a_report_that_cannot_be_decoded_is_named_and_is_an_error() {
        let decoder_error_kind = io::ErrorKind::InvalidData;
        let text = fixture::text_factory();
        let decoder_report = Err(decoder_error_kind);
        let reader_text = Ok(text);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let input = io::empty();
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_judgment = ReportJudgment::Input { input };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"invalid data
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(error_text, error_expectation);
    }

    #[test]
    fn a_report_without_a_region_is_a_success() {
        let report = Report::default();
        let reader_text = Err(io::ErrorKind::NotFound);
        let decoder_report = Ok(report);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let input = io::empty();
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_judgment = ReportJudgment::Input { input };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let output_text = String::from_utf8(output).unwrap();
        let output_expectation = r#"[COVERAGE] every region outside the test modules is covered by a test.
"#;
        let errors_expectation: Vec<u8> = Vec::new();
        let outcome_expectation = Outcome::Success;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(output_text, output_expectation);
        assert_eq!(errors, errors_expectation);
    }

    #[test]
    fn a_source_that_cannot_be_read_is_named_and_is_an_error() {
        let execution_count: usize = 0;
        let report = fixture::report_factory(execution_count);
        let reader_error_kind = io::ErrorKind::NotFound;
        let decoder_report = Ok(report);
        let reader_text = Err(reader_error_kind);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let test_boundaries = TestBoundaries::default();
        let parser_test_boundaries = Ok(test_boundaries);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let input = io::empty();
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_judgment = ReportJudgment::Input { input };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"[COVERAGE] cannot read `crates/harbor/src/lib.rs`: entity not found.
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(error_text, error_expectation);
    }

    #[test]
    fn a_source_that_is_no_rust_is_named_and_is_an_error() {
        let execution_count: usize = 0;
        let report = fixture::report_factory(execution_count);
        let text = fixture::text_factory();
        let parser_error_kind = io::ErrorKind::InvalidData;
        let decoder_report = Ok(report);
        let reader_text = Ok(text);
        let parser_test_boundaries = Err(parser_error_kind);
        let decoder = fixture::DecoderAdapter::from(decoder_report);
        let reader = fixture::ReaderAdapter::from(reader_text);
        let parser = fixture::ParserAdapter::from(parser_test_boundaries);
        let input = io::empty();
        let mut output = Vec::new();
        let mut errors = Vec::new();
        let gate = Gate::from((decoder, reader, parser));
        let report_judgment = ReportJudgment::Input { input };

        let outcome = fixture::judgment_outcome(&gate, report_judgment, &mut output, &mut errors);

        let error_text = String::from_utf8(errors).unwrap();
        let error_expectation = r#"invalid data
"#;
        let outcome_expectation = Outcome::Error;
        assert_eq!(outcome, outcome_expectation);
        assert_eq!(error_text, error_expectation);
    }
}
