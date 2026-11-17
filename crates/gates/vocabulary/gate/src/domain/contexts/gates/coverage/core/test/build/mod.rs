use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreTestBuildContext;

impl Layer for GatesCoverageCoreTestBuildContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreTestBuildContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/test/build";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Test", "Build"];
            let definition = "The crate gates_coverage_core_test_build, the crate of the domain of the coverage that holds the test build, the executables of the tests and the manifest of their package.";
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
            let spelling: &[&str] = &["Test", "Build"];
            let definition = "The executables the build of the tests made, with the manifest of their package.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Executable"];
            let definition = "A program the build of the tests made.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest", "Path"];
            let definition = "The path of the manifest of the package whose tests a build made.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Directory"];
            let definition = "The directory src beside the manifest, whose sources a measurement reports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Directory", "Name"];
            let definition = "The name src of the source directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Source", "Directory"];
            let definition = "To compose the source directory beside the manifest of a test build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "The name of a variable of the environment, of a profile directory or of a tool.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Package", "Directory"];
            let definition = "The directory of the package whose manifest a test build names, with the source directory under it.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreTestBuildContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreTestBuildContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreTestBuildContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreTestBuildContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_test_build_context_is_a_term() {
        let gates_coverage_core_test_build_context = GatesCoverageCoreTestBuildContext;

        let context = Context::try_from(gates_coverage_core_test_build_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
