use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCommandLineContext;

impl Layer for GatesCoverageCommandLineContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCommandLineContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/command_line";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Command", "Line"];
            let definition = "The crate gates_coverage_command_line, the program tool_coverage, which chooses between the gate and the measurement from its arguments.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mode"];
            let definition = "What the program does with its arguments: the judgment of a report, or a measurement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judgment"];
            let definition = "The mode that judges a report at a path, or on the standard input where no path is given.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measurement", "Word"];
            let definition = "The word measure, which starts a measurement where it is the first argument.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["First", "Argument"];
            let definition = "The first argument the program is given, which chooses its mode.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Respond", "Exit", "Code"];
            let definition = "To respond with the exit code of the outcome of the service the mode chooses.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCommandLineContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCommandLineContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCommandLineContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCommandLineContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_command_line_context_is_a_term() {
        let gates_coverage_command_line_context = GatesCoverageCommandLineContext;

        let context = Context::try_from(gates_coverage_command_line_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
