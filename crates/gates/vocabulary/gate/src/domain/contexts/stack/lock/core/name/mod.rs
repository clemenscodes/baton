use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockCoreNameContext;

impl Layer for StackLockCoreNameContext {
    type Layer = DomainLayer;
}

impl StackLockCoreNameContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/core/name";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Name"];
            let definition = "The crate stack_lock_core_name, the name of a lock and the name of an eligible application of the mutex around a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Name"];
            let definition = "The name of a lock of a GPU, such as rtx3080, of letters, digits, _ and -, never beginning with -, which names its records and its tree of cgroups.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Name"];
            let definition = "The name of an eligible application of a lock, such as render or guest, of letters, digits, _ and -, never beginning with -, which names its cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "The text of a name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Character"];
            let definition = "One character of the text of a name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Character"];
            let definition = "To tell whether a character may stand in a name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Character", "Presence"];
            let definition = "Whether every character of a text may stand in a name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Option", "Absence"];
            let definition = "Whether a text does not begin with -, so no program reads it as an option.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockCoreNameContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockCoreNameContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockCoreNameContext::DIRECTORY_NAME;
        let term_texts = StackLockCoreNameContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_core_name_context_is_a_term() {
        let context_type = StackLockCoreNameContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
