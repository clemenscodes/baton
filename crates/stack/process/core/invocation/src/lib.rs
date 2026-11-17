mod completion;
mod running_program;
mod spawner;
mod stream;
mod variable;

pub use completion::Completion;
pub use running_program::RunningProgram;
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

impl From<(OsString, Vec<OsString>, Stream)> for Invocation {
    fn from((program, arguments, stream): (OsString, Vec<OsString>, Stream)) -> Self {
        let variables = Vec::new();
        Self::from((program, arguments, variables, stream))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_invocation_keeps_its_program_its_arguments_its_variables_and_its_stream() {
        let program = OsString::from("qemu-img");
        let argument = OsString::from("create");
        let arguments = vec![argument];
        let name = OsString::from("LANG");
        let value = OsString::from("C");
        let variable = Variable::from((name, value));
        let variables = vec![variable];
        let stream = Stream::Capture;

        let invocation = Invocation::from((program, arguments, variables, stream));

        let program_expectation = OsStr::new("qemu-img");
        let argument_expectation = OsString::from("create");
        let name_expectation = OsString::from("LANG");
        let value_expectation = OsString::from("C");
        let variable_expectation = Variable::from((name_expectation, value_expectation));
        assert_eq!(invocation.program(), program_expectation);
        assert_eq!(invocation.arguments(), [argument_expectation]);
        assert_eq!(invocation.variables(), [variable_expectation]);
        assert_eq!(invocation.stream(), Stream::Capture);
    }

    #[test]
    fn an_invocation_without_variables_keeps_an_empty_environment() {
        let program = OsString::from("ssh");
        let argument = OsString::from("-V");
        let arguments = vec![argument];
        let stream = Stream::Inheritance;

        let invocation = Invocation::from((program, arguments, stream));

        let variables: &[Variable] = &[];
        assert_eq!(invocation.variables(), variables);
        assert_eq!(invocation.stream(), Stream::Inheritance);
    }
}
