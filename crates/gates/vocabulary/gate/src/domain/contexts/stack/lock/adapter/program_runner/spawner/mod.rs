use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterProgramRunnerSpawnerContext;

impl Layer for StackLockAdapterProgramRunnerSpawnerContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterProgramRunnerSpawnerContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/program_runner/spawner";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Adapter", "Program", "Runner", "Spawner"];
            let definition = "The crate stack_lock_adapter_program_runner_spawner, the adapter of the port ProgramRunner of stack_lock_application over the spawner of stack_process_core_invocation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Runner"];
            let definition = "The adapter of the port ProgramRunner, which runs the program of a guard run through a spawner and answers its exit code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Runner", "Port"];
            let definition = "The port ProgramRunner as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The spawner of stack_process_core_invocation that starts the program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner", "Type"];
            let definition = "The type of the spawner a program runner holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Absence"];
            let definition = "The error of a guard run that names no program after --.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Failure"];
            let definition = "The error of a program that could not be started.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal"];
            let definition = "The error of a program that a signal ended, so it has no exit code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Text"];
            let definition = "The program of a guard run as a text in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Argument", "Texts"];
            let definition = "The arguments of the program of a guard run.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status"];
            let definition = "The exit status of a program as the operating system reports it.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterProgramRunnerSpawnerContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockAdapterProgramRunnerSpawnerContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterProgramRunnerSpawnerContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterProgramRunnerSpawnerContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_adapter_program_runner_spawner_context_is_a_term() {
        let context_type = StackLockAdapterProgramRunnerSpawnerContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
