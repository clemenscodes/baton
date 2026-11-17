use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreMappingPresenceContext;

impl Layer for GatesCoverageCoreMappingPresenceContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreMappingPresenceContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/mapping_presence";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Mapping", "Presence"];
            let definition = "The crate gates_coverage_core_mapping_presence, the crate of the domain of the coverage that holds the mapping presence, whether an executable carries the mapping section of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string as it was read or as it is written: the text of a source, the success text and the text of a path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "The standard output of a program: the stream the gate writes the success text to, and what a program a measurement starts writes there, as the measurement captures it, such as the messages of a compilation, the library directory of the compiler or an export.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Word"];
            let definition = "A word of the command line of a measurement, the path of its report, a word of the command that compiles the tests, the separator or a word of the command that runs them, and a word of the arguments of an invocation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence"];
            let definition = "The lack of what a measurement needs, named by what is lacking.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inspection"];
            let definition = "The listing of the section headers of the executables of a test build, which shows whether they carry a coverage mapping.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mapping", "Section"];
            let definition = "The section of an executable that maps the counters of its profile to the regions of its sources, named __llvm_covmap in ELF, Mach-O and WebAssembly and .lcovmap in COFF.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mapping", "Presence"];
            let definition = "Whether the executables of a test build carry a mapping section: its presence, or its absence, where they hold no region at all.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presence"];
            let definition = "The mapping presence of executables of which at least one carries a mapping section.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "The name of a variable of the environment, of a profile directory or of a tool.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Criterion"];
            let definition = "A closure that decides whether a value is the one a function looks for, named for that value, such as the criterion of the separator word or of the text of a message.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreMappingPresenceContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreMappingPresenceContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreMappingPresenceContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreMappingPresenceContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_mapping_presence_context_is_a_term() {
        let gates_coverage_core_mapping_presence_context = GatesCoverageCoreMappingPresenceContext;

        let context = Context::try_from(gates_coverage_core_mapping_presence_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
