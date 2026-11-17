use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterPauseStandardLibraryContext;

impl Layer for StackLockAdapterPauseStandardLibraryContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterPauseStandardLibraryContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/pause/standard_library";

    const TERM_TEXTS: &'static [TermText] = &[{
        let spelling: &[&str] = &["Stack", "Lock", "Adapter", "Pause", "Standard", "Library"];
        let definition = "The crate stack_lock_adapter_pause_standard_library, the adapter of the port Pause of stack_lock_application over the sleep of the standard library.";
        TermText::new(spelling, definition)
    }];
}

impl TryFrom<StackLockAdapterPauseStandardLibraryContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockAdapterPauseStandardLibraryContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterPauseStandardLibraryContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterPauseStandardLibraryContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_adapter_pause_standard_library_context_is_a_term() {
        let context_type = StackLockAdapterPauseStandardLibraryContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
