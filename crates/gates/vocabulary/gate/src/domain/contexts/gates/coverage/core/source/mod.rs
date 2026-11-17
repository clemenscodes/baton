use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreSourceContext;

impl Layer for GatesCoverageCoreSourceContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreSourceContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/source";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Source"];
            let definition = "The crate gates_coverage_core_source, the crate of the domain of the coverage that holds the source, a path with its text, and the port of the reader.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a file or a directory: of a source or of a file of an export, as llvm-cov names it, of the source a test boundary lies in, of a report, a profile, a profile directory, a tool, a manifest or an executable; and the path of an attribute as syn parses it, cfg for the test attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line"];
            let definition = "A line of a source, counted from 1: the line of a position, the line of a syntax error, and the opening line and the closing line of a test boundary; and the error of a test boundary whose opening line is less than 1 or comes after its closing line, which names its path, its opening line and its closing line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source"];
            let definition = "A file of Rust whose spans a report holds: its path and its text, as the reader reads it and the parser parses it, and the error of the gate for a source that cannot be read, which names its path and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string as it was read or as it is written: the text of a source, the success text and the text of a path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reader"];
            let definition = "The port that reads a source by its path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Source"];
            let definition = "To read the text of the file of a path into a source of that path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Open", "Report"];
            let definition = "To open the export of llvm-cov at the path the gate is given, which it then decodes instead of its standard input.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreSourceContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreSourceContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreSourceContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreSourceContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_gates_coverage_core_source_context_is_a_term() {
        let gates_coverage_core_source_context = GatesCoverageCoreSourceContext;

        let context = Context::try_from(gates_coverage_core_source_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
