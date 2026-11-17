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
            let definition = "The adapter of the port ProgramRunner, which runs the program of a guard run through a spawner, forwards a trapped signal of an end to it as a request to end, and answers its exit code.";
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
        {
            let spelling: &[&str] = &["Signal", "Trap"];
            let definition = "The seam of the signals of an end the guard traps while its program runs, so a test raises no signal in its own process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Trap", "Type"];
            let definition = "The type of the signal trap a program runner holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["System", "Trap"];
            let definition = "The signal trap over the crate signal-hook, which traps SIGTERM, SIGINT and SIGHUP.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Presence"];
            let definition = "Whether a signal of an end arrived since the last poll of the trap, its presence or its absence.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Arm", "Trap"];
            let definition = "To install the actions of the trap for every trap signal, before the program starts.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Poll", "Signal", "Presence"];
            let definition = "To look whether a signal of an end arrived since the last poll and to clear it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lift", "Trap"];
            let definition = "To hand every later trap signal back to its default action, once the program ended.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Trap", "Signal"];
            let definition = "A signal of an end the trap catches, SIGTERM, SIGINT or SIGHUP.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Flag"];
            let definition = "The flag the action of a trap signal sets and a poll of the trap clears.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lift", "Flag"];
            let definition = "The flag a lift of the trap sets, after which every trap signal runs its default action.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Registration"];
            let definition = "The registration of the two actions of the trap for one trap signal.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Registration", "Outcome"];
            let definition = "The outcome of every registration of the trap, the identifiers of its actions or the error of the first that failed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Runner", "Configuration"];
            let definition = "The configuration of a program runner, the interval between two checks of its running program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The module of the runner configuration.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Interval"];
            let definition = "The time between two checks of the running program, 200 ms unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Trap", "Failure"];
            let definition = "The error of a trap whose actions could not be installed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wait", "Failure"];
            let definition = "The error of a running program that could not be polled or asked to end.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Wait", "Failure"];
            let definition = "To compose the wait failure of a program from the error of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Compose", "Exit", "Code"];
            let definition = "To answer the exit code of a completion, or the error of a program a signal ended.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Supervise", "Program"];
            let definition = "To start the program, to poll it every check interval, to request its end once a trap signal arrived, and to answer its completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Completion"];
            let definition = "The completion of the running program at a poll, none while it still runs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Completion", "Outcome"];
            let definition = "The completion of the program, or the error of its start or of its wait.";
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
