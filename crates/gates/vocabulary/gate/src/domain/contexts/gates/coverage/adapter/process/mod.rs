use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageAdapterProcessContext;

impl Layer for GatesCoverageAdapterProcessContext {
    type Layer = DomainLayer;
}

impl GatesCoverageAdapterProcessContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/adapter/process";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Adapter", "Process"];
            let definition = "The crate gates_coverage_adapter_process, the crate of the adapter of the coverage that fills the spawner over the processes of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Coverage"];
            let definition = "The crate that judges whether the tests of a crate reach every region of its sources outside their test modules, and the context of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gate"];
            let definition = "The service of the application that holds the decoder, the reader, the parser, the input, the output and the errors, judges a report and names every gap of it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Infrastructure"];
            let definition = "The layer of the adapters Java Script Object Notation, File System and Syntax.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command"];
            let definition = "A command a measurement starts, as its words: the command that compiles the tests or the command that runs them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Argument"];
            let definition = "An argument of an invocation, after its program, and an argument the program is given on its command line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process"];
            let definition = "The adapter of the spawner over the processes of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Command"];
            let definition = "To compose the command of the operating system that starts an invocation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Completion"];
            let definition = "To compose the completion of a program from its status and the output it captured.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compiler"];
            let definition = "The compiler of Rust, rustc, which names the directory of its library, beside which the profile tools lie.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageAdapterProcessContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageAdapterProcessContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageAdapterProcessContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageAdapterProcessContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_adapter_process_context_is_a_term() {
        let gates_coverage_adapter_process_context = GatesCoverageAdapterProcessContext;

        let context = Context::try_from(gates_coverage_adapter_process_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
