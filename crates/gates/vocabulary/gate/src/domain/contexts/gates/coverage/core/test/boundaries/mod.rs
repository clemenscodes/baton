use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreTestBoundariesContext;

impl Layer for GatesCoverageCoreTestBoundariesContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreTestBoundariesContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/test/boundaries";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Test", "Boundaries"];
            let definition = "The crate gates_coverage_core_test_boundaries, the crate of the domain of the coverage that holds the test boundaries of a source, and the port of the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Line"];
            let definition = "The line an export region starts on, as llvm-cov writes it, and the line of the start of a span, which a test boundary locates.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Production"];
            let definition = "The part of a source outside its test module, whose spans the test boundaries accept and whose spans no test reaches are gaps.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Module"];
            let definition = "The part of a source from its opening line to its closing line: an inline module tests that the test attribute stands on, which holds the tests of that source and whose spans the test boundaries refuse, so they are never a gap.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Module"];
            let definition = "A module of the file of a source as syn parses it, which the test module visitor visits, and a test module where it is the inline module of the test module name that a test attribute stands on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Boundary"];
            let definition = "The path of a source and the opening line and the closing line of a test module of that source, a specification of a span that accepts a span outside it and refuses one whose start line lies from its opening line to its closing line in its source, and refuses an opening line below the first line or after its closing line; and the error of the adapter Syntax for a test module that is no test boundary, which carries the error of that test boundary.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Boundaries"];
            let definition = "The test boundary of every test module of the sources the gate reads, each once and in order, a specification of a span that accepts a span every test boundary accepts, and the test boundaries the parser parses out of a source, which the gate forms them of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Opening", "Line"];
            let definition = "The line a test boundary opens on: the line of the hash mark of the test attribute of its test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Closing", "Line"];
            let definition = "The line a test boundary closes on: the line of the end of the brace of its test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Parser"];
            let definition = "The port that parses the test boundaries of a source, and the parser of the gate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Parse", "Test", "Boundaries"];
            let definition = "To parse the text of a source into the test boundary of every test module of it, and to refuse a text that is no Rust with a syntax error and a test module that is no test boundary with the error of that test boundary.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sequence"];
            let definition = "The values of a list one after another, as an iterator yields them, such as the arguments of the program or the words of a command.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test"];
            let definition = "A test of the package a measurement measures, which counts the regions it reaches while it runs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decision"];
            let definition = "The decision of a test boundary or of the test boundaries about a span: its acceptance outside every test module and its rejection inside one, which decides whether an unreached region is a gap.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreTestBoundariesContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreTestBoundariesContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreTestBoundariesContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreTestBoundariesContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_test_boundaries_context_is_a_term() {
        let gates_coverage_core_test_boundaries_context = GatesCoverageCoreTestBoundariesContext;

        let context = Context::try_from(gates_coverage_core_test_boundaries_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
