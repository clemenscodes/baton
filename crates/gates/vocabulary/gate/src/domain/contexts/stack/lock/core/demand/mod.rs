use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockCoreDemandContext;

impl Layer for StackLockCoreDemandContext {
    type Layer = DomainLayer;
}

impl StackLockCoreDemandContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/core/demand";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Demand"];
            let definition = "The crate stack_lock_core_demand, the demand of an eligible application for a GPU, with the process that waits for it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Demand"];
            let definition = "The demand of an eligible application for the GPU of a lock, with the process that waits for it, served in the order it was made.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Waiter", "Identifier"];
            let definition = "The identifier of the process that waits for a demand, which leaves the queue once that process ends.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockCoreDemandContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockCoreDemandContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockCoreDemandContext::DIRECTORY_NAME;
        let term_texts = StackLockCoreDemandContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_core_demand_context_is_a_term() {
        let context_type = StackLockCoreDemandContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
