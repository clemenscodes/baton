mod completion;
mod error;
mod spawner;
mod stream;
mod variable;

pub use completion::Completion;
pub use error::Error;
pub use spawner::Spawner;
pub use stream::Stream;
pub use variable::Variable;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::ffi::{OsStr, OsString};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Invocation {
    program: OsString,
    arguments: Vec<OsString>,
    variables: Vec<Variable>,
    stream: Stream,
}

impl Layer for Invocation {
    type Layer = DomainLayer;
}

impl ValueObject for Invocation {}

impl Invocation {
    pub fn program(&self) -> &OsStr {
        &self.program
    }

    pub fn arguments(&self) -> &[OsString] {
        &self.arguments
    }

    pub fn variables(&self) -> &[Variable] {
        &self.variables
    }

    pub fn stream(&self) -> Stream {
        self.stream
    }
}

impl From<(OsString, Vec<OsString>, Vec<Variable>, Stream)> for Invocation {
    fn from(
        (program, arguments, variables, stream): (OsString, Vec<OsString>, Vec<Variable>, Stream),
    ) -> Self {
        Self {
            program,
            arguments,
            variables,
            stream,
        }
    }
}

impl TryFrom<(Vec<OsString>, Vec<Variable>, Stream)> for Invocation {
    type Error = Error;

    fn try_from(
        (words, variables, stream): (Vec<OsString>, Vec<Variable>, Stream),
    ) -> Result<Self, Self::Error> {
        let mut word_sequence = words.into_iter();
        let program = word_sequence.next().ok_or(Error::ProgramAbsence)?;
        let arguments = Vec::from_iter(word_sequence);
        let invocation = Self::from((program, arguments, variables, stream));
        Ok(invocation)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_invocation_keeps_its_program_its_arguments_and_its_variables() {
        let program = OsString::from("cargo");
        let argument = OsString::from("nextest");
        let arguments = vec![argument];
        let name = OsString::from("LLVM_PROFILE_FILE");
        let value = OsString::from("/tmp/profiles/%p-%m.profraw");
        let variable = Variable::from((name, value));
        let variables = vec![variable];
        let stream = Stream::Capture;

        let invocation = Invocation::from((program, arguments, variables, stream));

        let program_expectation = OsStr::new("cargo");
        let argument_expectation = OsString::from("nextest");
        let name_expectation = OsString::from("LLVM_PROFILE_FILE");
        let value_expectation = OsString::from("/tmp/profiles/%p-%m.profraw");
        let variable_expectation = Variable::from((name_expectation, value_expectation));
        assert_eq!(invocation.program(), program_expectation);
        assert_eq!(invocation.arguments(), [argument_expectation]);
        assert_eq!(invocation.variables(), [variable_expectation]);
        assert_eq!(invocation.stream(), Stream::Capture);
    }

    #[test]
    fn the_words_of_a_command_are_its_program_and_its_arguments_with_the_variable() {
        let program = OsString::from("cargo");
        let first_argument = OsString::from("nextest");
        let last_argument = OsString::from("run");
        let words = vec![program, first_argument, last_argument];
        let name = OsString::from("LLVM_PROFILE_FILE");
        let value = OsString::from("/tmp/profiles/%p-%m.profraw");
        let variable = Variable::from((name, value));
        let variables = vec![variable];
        let stream = Stream::Inheritance;

        let invocation = Invocation::try_from((words, variables, stream)).unwrap();

        let program_expectation = OsString::from("cargo");
        let first_expectation = OsString::from("nextest");
        let last_expectation = OsString::from("run");
        let arguments_expectation = vec![first_expectation, last_expectation];
        let name_expectation = OsString::from("LLVM_PROFILE_FILE");
        let value_expectation = OsString::from("/tmp/profiles/%p-%m.profraw");
        let variable_expectation = Variable::from((name_expectation, value_expectation));
        let variables_expectation = vec![variable_expectation];
        let stream_expectation = Stream::Inheritance;
        let expectation = Invocation::from((
            program_expectation,
            arguments_expectation,
            variables_expectation,
            stream_expectation,
        ));
        assert_eq!(invocation, expectation);
    }

    #[test]
    fn a_command_without_words_is_an_error() {
        let words = Vec::new();
        let name = OsString::from("LLVM_PROFILE_FILE");
        let value = OsString::from("/tmp/profiles/%p-%m.profraw");
        let variable = Variable::from((name, value));
        let variables = vec![variable];
        let stream = Stream::Capture;

        let invocation = Invocation::try_from((words, variables, stream));

        let expectation = Err(Error::ProgramAbsence);
        assert_eq!(invocation, expectation);
    }
}
