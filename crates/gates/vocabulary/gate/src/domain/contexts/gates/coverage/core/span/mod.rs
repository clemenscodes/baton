use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreSpanContext;

impl Layer for GatesCoverageCoreSpanContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreSpanContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/span";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Span"];
            let definition = "The crate gates_coverage_core_span, the crate of the domain of the coverage that holds the span, a path with the position where it starts and the position where it ends.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Span"];
            let definition = "Where a region lies in a source: the path of the source, the start and the end, which a span refuses where it lies before the start, and the error of the adapter Java Script Object Notation for an export region that is no span, which carries the error of that span; and where a token lies in its source as proc-macro2 locates it, such as the span of a hash mark, whose start lies on the opening line of a test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a file or a directory: of a source or of a file of an export, as llvm-cov names it, of the source a test boundary lies in, of a report, a profile, a profile directory, a tool, a manifest or an executable; and the path of an attribute as syn parses it, cfg for the test attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Position"];
            let definition = "A line and a column of a source, where a span starts or ends; the error of a position whose line or column is less than 1, which names that line and that column; and the error of the adapter Java Script Object Notation for an export region with such a position, which names the path of its file and carries the error of that position.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line"];
            let definition = "A line of a source, counted from 1: the line of a position, the line of a syntax error, and the opening line and the closing line of a test boundary; and the error of a test boundary whose opening line is less than 1 or comes after its closing line, which names its path, its opening line and its closing line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Column"];
            let definition = "The column of a position and the column of a syntax error, counted from 1.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["First", "Line"];
            let definition = "The line 1, below which a position refuses its line and a test boundary its opening line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["First", "Column"];
            let definition = "The column 1, below which a position refuses its column.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start"];
            let definition = "The position a span starts at.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["End"];
            let definition = "The position a span ends at.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Order"];
            let definition = "The error of a span whose end lies before its start, which names the path, the start and the end of that span.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string as it was read or as it is written: the text of a source, the success text and the text of a path.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreSpanContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreSpanContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreSpanContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreSpanContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_span_context_is_a_term() {
        let gates_coverage_core_span_context = GatesCoverageCoreSpanContext;

        let context = Context::try_from(gates_coverage_core_span_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
