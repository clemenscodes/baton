use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackProcessAdapterSpawnerStandardLibraryContext;

impl Layer for StackProcessAdapterSpawnerStandardLibraryContext {
    type Layer = DomainLayer;
}

impl StackProcessAdapterSpawnerStandardLibraryContext {
    const DIRECTORY_NAME: &'static str = "stack/process/adapter/spawner/standard_library";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack", "Process", "Adapter", "Spawner", "Standard", "Library",
            ];
            let definition = "The crate stack_process_adapter_spawner_standard_library, the one adapter of the spawner, which starts programs through the processes of the standard library.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The adapter of the port Spawner of stack_process_core_invocation over std::process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Standard", "Library"];
            let definition = "The standard library of Rust, whose process module starts the programs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Child"];
            let definition = "The process of the operating system a started program runs in, as the standard library hands it back.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Child", "Program"];
            let definition = "The running program of this adapter, the child process with the stream its invocation named.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Command"];
            let definition = "To turn an invocation into the command of the standard library, its program, its arguments, its variables and its three streams.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Completion"];
            let definition = "To read the captured output of a child program to its end, to wait for its exit and to turn its status into a completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Completion"];
            let definition = "To turn the exit status of a program and its output into a completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status", "Text"];
            let definition = "The exit status of a program as the text a failure carries.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Status", "Completion"];
            let definition = "To make the completion of a program from its exit status and its output.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit", "Status"];
            let definition = "The status a program of the host ended with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Completion"];
            let definition = "The completion of a program, where it has ended.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Status"];
            let definition = "The exit status of a program, where it has ended.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Send", "Termination"];
            let definition = "To send the signal of termination to a child, on a host that has signals.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackProcessAdapterSpawnerStandardLibraryContext> for Context {
    type Error = Error;

    fn try_from(_: StackProcessAdapterSpawnerStandardLibraryContext) -> Result<Self, Self::Error> {
        let directory_name = StackProcessAdapterSpawnerStandardLibraryContext::DIRECTORY_NAME;
        let term_texts = StackProcessAdapterSpawnerStandardLibraryContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_process_adapter_spawner_standard_library_context_is_a_term() {
        let context_type = StackProcessAdapterSpawnerStandardLibraryContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
