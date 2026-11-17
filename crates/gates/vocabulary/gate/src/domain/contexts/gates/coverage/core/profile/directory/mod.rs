use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreProfileDirectoryContext;

impl Layer for GatesCoverageCoreProfileDirectoryContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreProfileDirectoryContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/profile/directory";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Profile", "Directory"];
            let definition = "The crate gates_coverage_core_profile_directory, the crate of the domain of the coverage that holds the profile directory and the port of the profile store.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Data"];
            let definition = "The files and the functions of one binary of an export, whose regions count only in the files it keeps.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile"];
            let definition = "A raw profile that an instrumented test writes, with the counts of the regions it reached.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Directory"];
            let definition = "The directory of one measurement into which its tests write their profiles; no other measurement writes there, and the measurement removes it when it ends.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Root"];
            let definition = "The directory beside the report under which every measurement makes its profile directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Variable"];
            let definition = "The variable LLVM_PROFILE_FILE, which tells an instrumented program where it writes its profile.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Variable", "Name"];
            let definition = "The name LLVM_PROFILE_FILE of the profile variable.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Name", "Pattern"];
            let definition = "The pattern %p-%m.profraw, which names the profile of every process and every binary apart.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Profile", "Variable"];
            let definition = "To compose the profile variable that names a profile in the profile directory for every process and binary.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Data"];
            let definition = "The profiles of a measurement merged into one file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Data", "Name"];
            let definition = "The name coverage.profdata of the profile data.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Profile", "Data", "Path"];
            let definition = "To compose the path of the profile data in the profile directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile", "Store"];
            let definition = "The driven port that keeps the profiles of a measurement in a directory of their own and writes its report.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Create", "Profile", "Directory"];
            let definition = "To create the profile directory of the process under the profile root, empty, replacing one an earlier process of the same number left.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List", "Profiles"];
            let definition = "To list the profiles of a profile directory in their order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Remove", "Profile", "Directory"];
            let definition = "To remove a profile directory with its profiles and its profile data.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Report", "File"];
            let definition = "To write the export of a report file at its path, creating its directory.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreProfileDirectoryContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreProfileDirectoryContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreProfileDirectoryContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreProfileDirectoryContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_profile_directory_context_is_a_term() {
        let gates_coverage_core_profile_directory_context = GatesCoverageCoreProfileDirectoryContext;

        let context = Context::try_from(gates_coverage_core_profile_directory_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
