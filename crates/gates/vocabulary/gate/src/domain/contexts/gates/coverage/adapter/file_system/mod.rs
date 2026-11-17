use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageAdapterFileSystemContext;

impl Layer for GatesCoverageAdapterFileSystemContext {
    type Layer = DomainLayer;
}

impl GatesCoverageAdapterFileSystemContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/adapter/file_system";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Adapter", "File", "System"];
            let definition = "The crate gates_coverage_adapter_file_system, the crate of the adapter of the file system of the coverage, which fills the reader and the profile store.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Coverage"];
            let definition = "The crate that judges whether the tests of a crate reach every region of its sources outside their test modules, and the context of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Paths"];
            let definition = "The path of every source a report keeps a span of, each path once and in order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Kind"];
            let definition = "The kind of an export region, code or annotation, which decides whether it is decoded.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "System"];
            let definition = "The adapter of the reader over the file system of the host.";
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
            let spelling: &[&str] = &["Process"];
            let definition = "The adapter of the spawner over the processes of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Extension"];
            let definition = "The extension profraw of a profile.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Path"];
            let definition = "The path of the report of a measurement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Directory"];
            let definition = "The directory of the report of a measurement, which the measurement creates where it is missing.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory", "Entry"];
            let definition = "An entry of a directory as the file system lists it, such as a profile in a profile directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Profiles"];
            let definition = "To select the path of every profile among the directory entries of a profile directory, in their order.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageAdapterFileSystemContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageAdapterFileSystemContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageAdapterFileSystemContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageAdapterFileSystemContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_adapter_file_system_context_is_a_term() {
        let gates_coverage_adapter_file_system_context = GatesCoverageAdapterFileSystemContext;

        let context = Context::try_from(gates_coverage_adapter_file_system_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
