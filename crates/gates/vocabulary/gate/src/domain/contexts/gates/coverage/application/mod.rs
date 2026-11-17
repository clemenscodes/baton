use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageApplicationContext;

impl Layer for GatesCoverageApplicationContext {
    type Layer = DomainLayer;
}

impl GatesCoverageApplicationContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/application";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Decide", "Outcome"];
            let definition = "To decide the outcome of a judgment from its gaps: a success where no gap remains, and an error otherwise.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Application"];
            let definition = "The crate gates_coverage_application, the application of the coverage, one library: the gate that judges a report and the measurement that writes one.";
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
            let spelling: &[&str] = &["Syntax"];
            let definition = "The adapter of the parser over syn, which lexes the text of a source into its tokens with proc-macro2, parses those tokens into its file and visits every module of it with the test module visitor; the syntax tree of a source as syn parses it, which the test module visitor visits for the lifetime syntax; the error of the adapter for a text that is no Rust, which names the path of its source and the line and the column syn refuses it at and carries the error of syn, and the error of the gate that carries that error of the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judge", "Report"];
            let definition = "To decode a report out of the input, read the test boundaries of its sources, form its gaps outside those test boundaries and write the outcome of them, to write the error to the errors where the report cannot be decoded or a source cannot be read or is no Rust, and to flush the output and the errors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Test", "Boundaries"];
            let definition = "To read the source of every path a report names, to parse the test boundaries of each of those sources with the parser and to form the test boundaries of them all, and to refuse a source that cannot be read and a source that is no Rust.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Success", "Text"];
            let definition = "The line the gate writes to the output where no gap remains.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Errors"];
            let definition = "The stream the command line writes every error and every gap of a judgment to, and the error of a measurement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "File"];
            let definition = "The report a measurement writes, with its path and its export.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Path"];
            let definition = "The path of the report of a measurement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Service"];
            let definition = "A service of the program that its mode chooses: the judgment of a report or a measurement, as main composes it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measurement"];
            let definition = "The measurement of the coverage of the tests of one package: it builds the tests, runs them instrumented with a profile directory of their own, merges and exports their profiles and writes the report. Also the application service that runs a report measurement and the mode of the program that starts it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measure"];
            let definition = "To measure the coverage of the tests of a package and report whether the measurement failed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measure", "Report"];
            let definition = "To measure the coverage of a package from the words of a measurement and write its report, removing the profile directory whatever happens.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measure", "Assignment"];
            let definition = "To build, run, merge and export the tests of an assignment into the report file it names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compilation", "Word"];
            let definition = "A word of the command that compiles the tests and states their executables in messages of Cargo.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Word"];
            let definition = "A word of the command that runs the tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Separator", "Word"];
            let definition = "The word -- between the command of the build and the command of the tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command"];
            let definition = "A command a measurement starts, as its words: the command that compiles the tests or the command that runs them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Assignment"];
            let definition = "The commands of one measurement with the profile directory it keeps its profiles in.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Compilation", "Invocation"];
            let definition = "To compose the invocation of the compilation of the tests, which captures its output and compiles them instrumented, with the profile variable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Test", "Invocation"];
            let definition = "To compose the invocation of the tests, which shows its output and runs instrumented with the profile variable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Variables"];
            let definition = "To compose the variables every invocation of a measurement gets: the profile variable and the instrumentation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Instrumentation"];
            let definition = "The flags with which Cargo builds the tests so that they count every region they reach.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Instrumentation", "Flag"];
            let definition = "The flag -Cinstrument-coverage, with which rustc counts every region a program reaches.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Instrumentation", "Variable", "Name"];
            let definition = "The name CARGO_BUILD_RUSTFLAGS of the variable that carries the instrumentation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compile", "Tests"];
            let definition = "To build the tests of an assignment and decode their test build from the messages of the build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Locate", "Profile", "Tools"];
            let definition = "To ask the compiler for the directory of its library and locate the profile tools beside it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Start"];
            let definition = "The start of a program that failed, with its cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Root", "Name"];
            let definition = "The name profiles of the profile root.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Profile", "Root"];
            let definition = "To compose the profile root beside the report of a measurement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Absence"];
            let definition = "Tests that left no profile, so their build was not instrumented.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Removal"];
            let definition = "The removal of the profile directory of a measurement, whose error the measurement reports after the error of its assignment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compilation"];
            let definition = "The compilation of the tests of a package, the first step of a measurement, whose output states the executables of the tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Word"];
            let definition = "The first word of a measurement, the path of its report.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Words", "Error"];
            let definition = "To write the error of words that are no measurement, naming what a measurement takes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Program", "Start", "Error"];
            let definition = "To write the error of a program that cannot start, naming the program and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Failure"];
            let definition = "To write the failure of a program, naming the program and its exit status.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Profile", "Absence"];
            let definition = "To write the error of tests that left no profile, because their build was not instrumented.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Profile", "Directory", "Error"];
            let definition = "To write the error of a profile directory that cannot be created, listed or removed, naming the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Coverage"];
            let definition = "The crate that judges whether the tests of a crate reach every region of its sources outside their test modules, and the context of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gap", "Count"];
            let definition = "How many gaps the gaps of a report hold, 0 for the outcome success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gate"];
            let definition = "The service of the application over the decoder, the reader and the parser that answers a report judgment with the judgment of its report.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Success"];
            let definition = "The outcome of a gate whose count of gaps is zero, a report without a region with no case of its own, which ends the program with the exit code of a success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit", "Code"];
            let definition = "The status the program of coverage ends with, a success for the outcome of a success and a failure for the outcome of an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Outcome"];
            let definition = "What a judgment or a measurement comes to, a success where no gap remains and the measurement wrote its report, and an error otherwise, and the exit code it leaves.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application"];
            let definition = "The layer of the coverage that holds its use cases, the judgment of a report by the gate and the measurement of a package, and their outcome.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Failure"];
            let definition = "A completion of a program that ended with a status other than success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process"];
            let definition = "The adapter of the spawner over the processes of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Judgment"];
            let definition = "The query of the gate that judges the report in a file at a path or on an input, and answers with its judgment or the error that refused it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judgment"];
            let definition = "The outcome of a report judgment: the gaps of the report, read as the success text where there is none and as a line for every gap otherwise.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Measurement"];
            let definition = "The command of a measurement, read from its words: the path of its report, the command that compiles the tests and the command that runs them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Export"];
            let definition = "The error of a report whose file cannot be read, naming its path and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Report", "Export", "Error"];
            let definition = "To write the error of a report whose file cannot be read, naming its path and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Source", "Error"];
            let definition = "To write the error of a source that cannot be read, naming its path and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Report", "File"];
            let definition = "To open the report at a path through the reader and decode it, or the error that its file cannot be read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Report"];
            let definition = "To decode a report out of an input through the decoder, or the error of the decoder as text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line", "Separator"];
            let definition = "The line break between the lines of the gaps of a judgment.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageApplicationContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageApplicationContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageApplicationContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageApplicationContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_application_context_is_a_term() {
        let gates_coverage_application_context = GatesCoverageApplicationContext;

        let context = Context::try_from(gates_coverage_application_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
