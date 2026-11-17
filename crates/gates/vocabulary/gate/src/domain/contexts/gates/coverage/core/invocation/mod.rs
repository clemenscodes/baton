use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreInvocationContext;

impl Layer for GatesCoverageCoreInvocationContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreInvocationContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/invocation";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Invocation"];
            let definition = "The crate gates_coverage_core_invocation, the crate of the domain of the coverage that holds the invocation, a program with its arguments, its variables and its stream, its completion and the port of the spawner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string as it was read or as it is written: the text of a source, the success text and the text of a path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Success"];
            let definition = "The outcome of a gate whose count of gaps is zero, a report without a region with no case of its own, which ends the program with the exit code of a success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Output"];
            let definition = "The standard output of a program: the stream the gate writes the success text to, and what a program a measurement starts writes there, as the measurement captures it, such as the messages of a compilation, the library directory of the compiler or an export.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Word"];
            let definition = "A word of the command line of a measurement, the path of its report, a word of the command that compiles the tests, the separator or a word of the command that runs them, and a word of the arguments of an invocation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawn"];
            let definition = "To start an invocation, wait for its completion and yield its output or the error of its program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Invocation"];
            let definition = "A program with its arguments, the variables of its environment and the stream of its output, as a measurement starts it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program"];
            let definition = "The program an invocation starts.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Absence"];
            let definition = "A command that names no program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Argument"];
            let definition = "An argument of an invocation, after its program, and an argument the program is given on its command line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Variable"];
            let definition = "A variable of the environment of an invocation, with its name and its value.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stream"];
            let definition = "Whether a measurement captures the output of an invocation or the invocation writes it where the measurement writes its own.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Capture"];
            let definition = "The stream of an invocation whose output the measurement reads, such as the messages of the build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inheritance"];
            let definition = "The stream of an invocation that writes its output where the measurement writes its own, such as the run of the tests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Completion"];
            let definition = "How a program ended: a success with the output it captured, or a failure with its status.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Failure"];
            let definition = "A completion of a program that ended with a status other than success.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Status"];
            let definition = "The status a program ended with, as the operating system states it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The driven port that starts an invocation and waits for its completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawn", "Invocation"];
            let definition = "To start an invocation, capture or pass on its output as its stream says, and wait for its completion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence"];
            let definition = "The lack of what a measurement needs, named by what is lacking.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "The name of a variable of the environment, of a profile directory or of a tool.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value"];
            let definition = "The value of a variable of the environment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Sequence"];
            let definition = "The values of a list one after another, as an iterator yields them, such as the arguments of the program or the words of a command.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreInvocationContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreInvocationContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreInvocationContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreInvocationContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_invocation_context_is_a_term() {
        let gates_coverage_core_invocation_context = GatesCoverageCoreInvocationContext;

        let context = Context::try_from(gates_coverage_core_invocation_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
