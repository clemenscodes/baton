use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageAdapterSyntaxContext;

impl Layer for GatesCoverageAdapterSyntaxContext {
    type Layer = DomainLayer;
}

impl GatesCoverageAdapterSyntaxContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/adapter/syntax";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Adapter", "Syntax"];
            let definition = "The crate gates_coverage_adapter_syntax, the crate of the adapter of the coverage that fills the parser over syn and proc-macro2.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Push", "Test", "Boundaries"];
            let definition = "To push a test boundary for every test attribute of a module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Test", "Boundaries"];
            let definition = "To compose the test boundary of every test attribute that stands on a test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Coverage"];
            let definition = "The crate that judges whether the tests of a crate reach every region of its sources outside their test modules, and the context of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Module", "Name"];
            let definition = "The name tests, which the module of a test module bears.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identifier"];
            let definition = "The name of a module as syn parses it, the test module name for a test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Brace"];
            let definition = "The braces around the items of an inline module, the end of which lies on the closing line of a test module.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Attribute"];
            let definition = "An attribute of a module as syn parses it, and the test attribute, the attribute cfg test, which opens the test module it stands on where its path is the configuration name and its tokens are the test name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Attribute", "Presence"];
            let definition = "What an attribute of a test module is to the test module visitor: a presence that names the opening line where it is the test attribute, and an absence where it is any other attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration", "Name"];
            let definition = "The name cfg, the path of the test attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Name"];
            let definition = "The name test, the tokens of the test attribute inside its parentheses.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hash", "Mark"];
            let definition = "The mark # an attribute starts with, the line of which is the opening line of the test module the test attribute stands on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token"];
            let definition = "A token of the text of a source as proc-macro2 lexes it: the tokens of a source, which syn parses into its file, and the tokens of an attribute inside its parentheses, the test name for the test attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File"];
            let definition = "A file of an export, named by its filename, whose regions the data keeps, and the error of a code region whose file index its function lacks, which names that file index and the file count of that function; and the file of syn, the items of a source as syn parses them, whose modules the test module visitor visits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Module", "Visitor"];
            let definition = "The visitor of syn that visits every module of the file of a source, nested modules included, and lists the test boundary of every test module among them, or the error of that test boundary, from the line of the hash mark of its test attribute to the line of the end of its brace.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Syntax"];
            let definition = "The adapter of the parser over syn, which lexes the text of a source into its tokens with proc-macro2, parses those tokens into its file and visits every module of it with the test module visitor; the syntax tree of a source as syn parses it, which the test module visitor visits for the lifetime syntax; the error of the adapter for a text that is no Rust, which names the path of its source and the line and the column syn refuses it at and carries the error of syn, and the error of the gate that carries that error of the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Column", "Offset"];
            let definition = "The column a syntax error starts at as proc-macro2 counts it, from 0, so that the column of that syntax error lies 1 above it.";
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
            let spelling: &[&str] = &["Stream"];
            let definition = "Whether a measurement captures the output of an invocation or the invocation writes it where the measurement writes its own.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence"];
            let definition = "The lack of what a measurement needs, named by what is lacking.";
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
            let spelling: &[&str] = &["Compose", "Test", "Boundary"];
            let definition = "To compose the test boundary of a test module from the path of its source, the opening line of its test attribute and its closing line.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageAdapterSyntaxContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageAdapterSyntaxContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageAdapterSyntaxContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageAdapterSyntaxContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_adapter_syntax_context_is_a_term() {
        let gates_coverage_adapter_syntax_context = GatesCoverageAdapterSyntaxContext;

        let context = Context::try_from(gates_coverage_adapter_syntax_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
