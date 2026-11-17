use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageAdapterJavaScriptObjectNotationContext;

impl Layer for GatesCoverageAdapterJavaScriptObjectNotationContext {
    type Layer = DomainLayer;
}

impl GatesCoverageAdapterJavaScriptObjectNotationContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/adapter/java_script_object_notation";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Gates", "Coverage", "Adapter", "Java", "Script", "Object", "Notation",
            ];
            let definition = "The crate gates_coverage_adapter_java_script_object_notation, the crate of the adapter of the coverage that fills the decoder over serde_json.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Coverage"];
            let definition = "The crate that judges whether the tests of a crate reach every region of its sources outside their test modules, and the context of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Position", "Error", "Conversion"];
            let definition = "The conversion of the error of a position of an export region into the error of the adapter Java Script Object Notation, which names the path of the file of that region.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Line"];
            let definition = "The line an export region starts on, as llvm-cov writes it, and the line of the start of a span, which a test boundary locates.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Column"];
            let definition = "The column an export region starts at, as llvm-cov writes it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["End", "Line"];
            let definition = "The line an export region ends on, as llvm-cov writes it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["End", "Column"];
            let definition = "The column an export region ends at, as llvm-cov writes it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Export"];
            let definition = "The text that llvm-cov export writes in the Java Script Object Notation, holding the data of every binary of the tests, which a measurement writes into its report file, and the error of the adapter Java Script Object Notation for a text that is no export, which carries the error of serde_json.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Data"];
            let definition = "The files and the functions of one binary of an export, whose regions count only in the files it keeps.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Files"];
            let definition = "The files of a data, which select the regions of its functions that lie in one of them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File"];
            let definition = "A file of an export, named by its filename, whose regions the data keeps, and the error of a code region whose file index its function lacks, which names that file index and the file count of that function; and the file of syn, the items of a source as syn parses them, whose modules the test module visitor visits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Functions"];
            let definition = "The functions of a data, whose regions are the regions of every function in their order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function"];
            let definition = "A function of an export, with the filenames of the files it was made from and its export regions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Filename"];
            let definition = "The path of a file an export keeps, as llvm-cov names the field.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Filenames"];
            let definition = "The paths of the files a function of an export was made from, as llvm-cov names the field, which locate the filename of a file index.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Index"];
            let definition = "The index of the file of an export region among the filenames of its function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Count"];
            let definition = "How many filenames a function of an export holds, below which a file index has to lie.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Locate", "Filename"];
            let definition = "To locate the filename of a file index among the filenames of a function, and to refuse a file index past them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Export", "Regions"];
            let definition = "The export regions of a function, of which only those of the kind code are decoded, each into a region of the filename its file index locates.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Export", "Region"];
            let definition = "A region as an export of llvm-cov writes it, a list of its start line, its start column, its end line, its end column, its execution count, its file index, the index of its expansion, which the adapter Java Script Object Notation does not read, and its export kind, which decodes into a region of the filename its file index locates among the filenames of its function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Export", "Kind"];
            let definition = "The kind of an export region as an export of llvm-cov writes it, 0 for a region of code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Kind"];
            let definition = "The kind of an export region, code or annotation, which decides whether it is decoded.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Code"];
            let definition = "The kind of an export region of the export kind 0, which counts the runs of a piece of code, the one kind the adapter Java Script Object Notation decodes and the default kind.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Annotation"];
            let definition = "The kind of an export region of every export kind above 0, such as an expansion, a skipped range, the space between code or a branch, which the adapter Java Script Object Notation passes over.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Region"];
            let definition = "To decode a region out of an export region and the filenames of its function, and the regions of the kind code out of the export regions of a function, out of a function, out of the functions of a data, out of a data or out of an export.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Regions"];
            let definition = "To select the regions of the functions of a data that lie in one of its files.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Java", "Script", "Object", "Notation"];
            let definition = "The adapter of the decoder over serde_json, which decodes an export out of its text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Syntax"];
            let definition = "The adapter of the parser over syn, which lexes the text of a source into its tokens with proc-macro2, parses those tokens into its file and visits every module of it with the test module visitor; the syntax tree of a source as syn parses it, which the test module visitor visits for the lifetime syntax; the error of the adapter for a text that is no Rust, which names the path of its source and the line and the column syn refuses it at and carries the error of syn, and the error of the gate that carries that error of the parser.";
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
            let spelling: &[&str] = &["Absence"];
            let definition = "The lack of what a measurement needs, named by what is lacking.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Test", "Executables"];
            let definition = "To decode the test executables from every message of Cargo on the output of a build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Message"];
            let definition = "A message of Cargo in JSON on the output of a build. Also the error of a line of that output that is no such message.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reason"];
            let definition = "The reason of a message of Cargo, such as compiler-artifact.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Artifact", "Reason"];
            let definition = "The reason compiler-artifact of a message that names what the build made.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Executable"];
            let definition = "An executable of a test that a message of Cargo names, with the manifest of its package.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Executable", "Absence"];
            let definition = "A message or a build that names no test executable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Index"];
            let definition = "The position of a value in a list, counted from zero, such as the index of the separator word or of an executable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Criterion"];
            let definition = "A closure that decides whether a value is the one a function looks for, named for that value, such as the criterion of the separator word or of the text of a message.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test"];
            let definition = "A test of the package a measurement measures, which counts the regions it reaches while it runs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["First", "Test", "Executable"];
            let definition = "The test executable of the first message of a compilation that names one, whose manifest names the package of the tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Artifact", "Profile"];
            let definition = "The profile of Cargo that a message names for its artifact, which states whether the artifact is a test.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Flag"];
            let definition = "The flag test of an artifact profile, which states whether Cargo compiled the artifact as a test.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Input", "Error"];
            let definition = "To write the error of an input that cannot be read, naming the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Export", "Error"];
            let definition = "To write the error of a report that is no llvm-cov export, naming the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Message", "Error"];
            let definition = "To write the error of a line of the output of a build that is no message of Cargo, naming the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Test", "Executable", "Absence"];
            let definition = "To write the error of a build that made no executable of a test.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "File", "Error"];
            let definition = "To write the error of a region whose file index lies past the file indexes of its function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Position", "Error"];
            let definition = "To write the error of a position of a region that is no position, naming the path of its source.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageAdapterJavaScriptObjectNotationContext> for Context {
    type Error = Error;

    fn try_from(
        _: GatesCoverageAdapterJavaScriptObjectNotationContext,
    ) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageAdapterJavaScriptObjectNotationContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageAdapterJavaScriptObjectNotationContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_adapter_java_script_object_notation_context_is_a_term()
     {
        let gates_coverage_adapter_java_script_object_notation_context = GatesCoverageAdapterJavaScriptObjectNotationContext;

        let context = Context::try_from(gates_coverage_adapter_java_script_object_notation_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
