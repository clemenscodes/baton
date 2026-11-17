use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackProcessAdapterSpawnerTranscriptContext;

impl Layer for StackProcessAdapterSpawnerTranscriptContext {
    type Layer = DomainLayer;
}

impl StackProcessAdapterSpawnerTranscriptContext {
    const DIRECTORY_NAME: &'static str = "stack/process/adapter/spawner/transcript";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Process", "Adapter", "Spawner", "Transcript"];
            let definition = "The crate stack_process_adapter_spawner_transcript, the adapter of the spawner that starts nothing and records every invocation, for the tests of the adapters that drive a program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The transcript adapter of the port Spawner, which hands out the completions it was made with in order and succeeds once they are spent.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Recorded", "Program"];
            let definition = "The running program of the recording spawner, which completes at once with its completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Completion", "Queue"];
            let definition = "The completions a recording spawner still hands out, in order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Completion"];
            let definition = "The next completion of a recording spawner, where one is left, and none where the program is to be missing.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Invocation"];
            let definition = "To record an invocation and to take the next completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Poison"];
            let definition = "The lock of a thread that panicked while it held it, whose record is still read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["End", "Request"];
            let definition = "A request to end a recorded program, kept with the invocation of that program.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackProcessAdapterSpawnerTranscriptContext> for Context {
    type Error = Error;

    fn try_from(_: StackProcessAdapterSpawnerTranscriptContext) -> Result<Self, Self::Error> {
        let directory_name = StackProcessAdapterSpawnerTranscriptContext::DIRECTORY_NAME;
        let term_texts = StackProcessAdapterSpawnerTranscriptContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_process_adapter_spawner_transcript_context_is_a_term() {
        let context_type = StackProcessAdapterSpawnerTranscriptContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
