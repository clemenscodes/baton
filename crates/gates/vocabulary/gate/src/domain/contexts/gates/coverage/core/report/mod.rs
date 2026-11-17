use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreReportContext;

impl Layer for GatesCoverageCoreReportContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreReportContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/report";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Report"];
            let definition = "The crate gates_coverage_core_report, the crate of the domain of the coverage that holds the report, its regions by span, and the port of the decoder.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Regions"];
            let definition = "The regions the adapter Java Script Object Notation decodes out of an export, out of its data, out of their files and functions and out of the export regions of a function, in their order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Paths"];
            let definition = "The path of every source a report keeps a span of, each path once and in order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report"];
            let definition = "The region of every span of every source, the copies of a region taken together into the copy that counts highest, so that a span counts as reached where any copy of it is.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stored", "Region"];
            let definition = "The region a report keeps for a span so far, the first copy of its region, which a copy that counts higher replaces.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["First", "Region"];
            let definition = "The region a report keeps for a span where it keeps none yet, a copy of the region it reads, because llvm-cov exports a region once for every binary of the tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List", "Paths"];
            let definition = "To list the path of every source a report keeps a span of, each path once.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decoder"];
            let definition = "The port that decodes an export of llvm-cov out of an input into a report, and the output of a compilation into its test build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Report"];
            let definition = "To decode a report out of an export.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Input"];
            let definition = "The stream the gate decodes the export out of, and the error of the adapter Java Script Object Notation for an input that cannot be read, which carries that error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "The standard output of a program: the stream the gate writes the success text to, and what a program a measurement starts writes there, as the measurement captures it, such as the messages of a compilation, the library directory of the compiler or an export.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Test", "Build"];
            let definition = "To decode the test build from the messages of Cargo on the output of a build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value"];
            let definition = "The value of a variable of the environment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sequence"];
            let definition = "The values of a list one after another, as an iterator yields them, such as the arguments of the program or the words of a command.";
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
            let spelling: &[&str] = &["Export"];
            let definition = "The text that llvm-cov export writes in the Java Script Object Notation, holding the data of every binary of the tests, which a measurement writes into its report file, and the error of the adapter Java Script Object Notation for a text that is no export, which carries the error of serde_json.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File"];
            let definition = "A file of an export, named by its filename, whose regions the data keeps, and the error of a code region whose file index its function lacks, which names that file index and the file count of that function; and the file of syn, the items of a source as syn parses them, whose modules the test module visitor visits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "File"];
            let definition = "The report a measurement writes, with its path and its export.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreReportContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreReportContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreReportContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreReportContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_report_context_is_a_term() {
        let gates_coverage_core_report_context = GatesCoverageCoreReportContext;

        let context = Context::try_from(gates_coverage_core_report_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
