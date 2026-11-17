use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreGapsContext;

impl Layer for GatesCoverageCoreGapsContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreGapsContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/gaps";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Gaps"];
            let definition = "The crate gates_coverage_core_gaps, the crate of the domain of the coverage that holds the gaps, the regions outside the test modules that no test reaches.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gap"];
            let definition = "A span of a source that no test reaches, that lies outside a main path and in the production of that source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gaps"];
            let definition = "The gaps of a report in the order of their spans, selected out of the regions of the report by the test boundaries of its sources, which the outcome counts and the gate writes to the errors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Main", "Path"];
            let definition = "The path src/main.rs, whose spans are never a gap, because a main parses, delegates and decides nothing.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreGapsContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreGapsContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreGapsContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreGapsContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_gaps_context_is_a_term() {
        let gates_coverage_core_gaps_context = GatesCoverageCoreGapsContext;

        let context = Context::try_from(gates_coverage_core_gaps_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
