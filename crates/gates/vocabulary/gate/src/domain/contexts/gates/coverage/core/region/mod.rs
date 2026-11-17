use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreRegionContext;

impl Layer for GatesCoverageCoreRegionContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreRegionContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/region";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Region"];
            let definition = "The crate gates_coverage_core_region, the crate of the domain of the coverage that holds the region, a span with the count of its executions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Region"];
            let definition = "A span of a source with its execution count, as llvm-cov exports it for one copy of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Execution", "Count"];
            let definition = "How often the tests ran a region.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreRegionContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreRegionContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreRegionContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreRegionContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_region_context_is_a_term() {
        let gates_coverage_core_region_context = GatesCoverageCoreRegionContext;

        let context = Context::try_from(gates_coverage_core_region_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
