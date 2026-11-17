use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackProcessCoreInvocationContext;

impl Layer for StackProcessCoreInvocationContext {
    type Layer = DomainLayer;
}

impl StackProcessCoreInvocationContext {
    const DIRECTORY_NAME: &'static str = "stack/process/core/invocation";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Process", "Core", "Invocation"];
            let definition = "The crate stack_process_core_invocation, the invocation of a program and the driven port that starts it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Invocation"];
            let definition = "A program with its arguments, its variables and the stream it writes its output to, everything a spawner needs to start it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program"];
            let definition = "The name or the path of the executable an invocation starts, such as qemu-system-x86_64 or ssh.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Argument"];
            let definition = "One word an invocation hands its program after the name of the program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Variable"];
            let definition = "One variable of the environment an invocation sets for its program, with its name and its value.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "The name of a variable of the environment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value"];
            let definition = "The value of a variable of the environment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stream"];
            let definition = "Where the output of a program goes, captured for the caller or inherited from the terminal of the caller.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Capture"];
            let definition = "The stream whose standard output the spawner collects and hands back in the completion, while the standard error stays inherited.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inheritance"];
            let definition = "The stream that hands the program the terminal of the caller, so a person sees and types into it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Completion"];
            let definition = "How a program ended, a success with its captured output or a failure with its status.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Success"];
            let definition = "The completion of a program that ended with the status zero, with the output it wrote where its stream was a capture.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Failure"];
            let definition = "The completion of a program that ended with a status other than zero.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status"];
            let definition = "The text of the exit status of a program that did not end with zero, as the operating system reports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "The bytes a program wrote to its standard output under a capture.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The driven port that starts the program of an invocation, the one seam between baton and the processes of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawn", "Invocation"];
            let definition = "To start the program of an invocation and to wait until it ends.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Start", "Invocation"];
            let definition = "To start the program of an invocation and to hand back the running program without waiting for it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Running", "Program"];
            let definition = "A program a spawner started and that may still run, which the caller waits for once its own work beside it is done.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Wait", "Completion"];
            let definition = "To wait until a running program ends and to hand back its completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Code"];
            let definition = "The exit code of a program that ended in a failure, if the system names one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Request", "End"];
            let definition = "To ask a running program to end, with the signal of termination where the host has one, and never with a kill.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Poll", "Completion"];
            let definition = "To look whether a running program has ended, without waiting for it.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackProcessCoreInvocationContext> for Context {
    type Error = Error;

    fn try_from(_: StackProcessCoreInvocationContext) -> Result<Self, Self::Error> {
        let directory_name = StackProcessCoreInvocationContext::DIRECTORY_NAME;
        let term_texts = StackProcessCoreInvocationContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_process_core_invocation_context_is_a_term() {
        let context_type = StackProcessCoreInvocationContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
