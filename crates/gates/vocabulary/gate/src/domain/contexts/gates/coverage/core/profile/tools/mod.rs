use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreProfileToolsContext;

impl Layer for GatesCoverageCoreProfileToolsContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreProfileToolsContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/profile/tools";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Profile", "Tools"];
            let definition = "The crate gates_coverage_core_profile_tools, the crate of the domain of the coverage that holds the profile tools, which compose the invocations of the tools of LLVM beside rustc.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Paths"];
            let definition = "The path of every source a report keeps a span of, each path once and in order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string as it was read or as it is written: the text of a source, the success text and the text of a path.";
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
            let spelling: &[&str] = &["File"];
            let definition = "A file of an export, named by its filename, whose regions the data keeps, and the error of a code region whose file index its function lacks, which names that file index and the file count of that function; and the file of syn, the items of a source as syn parses them, whose modules the test module visitor visits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library", "Directory"];
            let definition = "The directory of the library of the compiler for its host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library", "Directory", "Absence"];
            let definition = "An output of the compiler that names no library directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library", "Directory", "Encoding"];
            let definition = "An output of the compiler that is no text of UTF-8.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile"];
            let definition = "A raw profile that an instrumented test writes, with the counts of the regions it reached.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Data"];
            let definition = "The profiles of a measurement merged into one file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Data", "Path"];
            let definition = "The path of the profile data in the profile directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Tool"];
            let definition = "A tool of LLVM beside the library of the compiler that merges the profiles of a measurement or exports their coverage; the profile tools share the path of the profile data.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tool", "Directory"];
            let definition = "The directory of the profile tools, beside the library directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tool", "Directory", "Name"];
            let definition = "The name bin of the tool directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Merge", "Tool", "Name"];
            let definition = "The name llvm-profdata of the tool that merges profiles.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Export", "Tool", "Name"];
            let definition = "The name llvm-cov of the tool that exports coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Object", "Flag"];
            let definition = "The flag -object, which names every executable of a test after the first to the tool that exports coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Tool", "Path"];
            let definition = "To compose the path of a profile tool in the tool directory, with the suffix of executables of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Merge", "Invocation"];
            let definition = "To compose the invocation that merges the profiles of a measurement into its profile data.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Export", "Invocation"];
            let definition = "To compose the invocation that exports the coverage of the executables of a test build for the sources of its package.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inspection"];
            let definition = "The listing of the section headers of the executables of a test build, which shows whether they carry a coverage mapping.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inspection", "Tool", "Name"];
            let definition = "The name llvm-objdump of the tool that lists the section headers of an executable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Section", "Header", "Flag"];
            let definition = "The flag --section-headers, which asks the inspection tool for the section headers of every executable it names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Inspection", "Invocation"];
            let definition = "To compose the invocation that lists the section headers of every executable of a test build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Empty", "Export"];
            let definition = "The export without data, the report of tests whose executables carry no mapping section and so no region.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Index"];
            let definition = "The position of a value in a list, counted from zero, such as the index of the separator word or of an executable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library"];
            let definition = "The library of the compiler for its host, whose directory the compiler names on request.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Merge"];
            let definition = "The merge of the profiles of a measurement into its profile data.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tool"];
            let definition = "A program of LLVM beside the library of the compiler, the one that merges profiles or the one that exports coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tool", "File", "Name"];
            let definition = "The name of the file of a tool: its name and the suffix of the executables of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Data", "Flag"];
            let definition = "The flag -o, in front of the path of the profile data that the merge writes.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreProfileToolsContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreProfileToolsContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreProfileToolsContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreProfileToolsContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_profile_tools_context_is_a_term() {
        let gates_coverage_core_profile_tools_context = GatesCoverageCoreProfileToolsContext;

        let context = Context::try_from(gates_coverage_core_profile_tools_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
