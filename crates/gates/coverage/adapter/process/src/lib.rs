use gates_coverage_core_invocation::Completion;
use gates_coverage_core_invocation::Spawner;
use gates_coverage_core_invocation::{Invocation, Stream};
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::io;
use std::process::{Command, Output, Stdio};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Process;

impl Layer for Process {
    type Layer = InfrastructureLayer;
}

impl Adapter for Process {}

impl Port for Process {}

impl DrivenPort for Process {}

impl Spawner for Process {
    fn try_spawn_invocation(&self, invocation: &Invocation) -> io::Result<Completion> {
        let mut command = self.compose_command(invocation);
        let stream_output = match invocation.stream() {
            Stream::Capture => command.stderr(Stdio::inherit()).output()?,
            Stream::Inheritance => {
                let status = command.status()?;
                Output {
                    status,
                    stdout: Vec::new(),
                    stderr: Vec::new(),
                }
            }
        };
        let completion = self.compose_completion(stream_output);
        Ok(completion)
    }
}

impl Process {
    fn compose_command(&self, invocation: &Invocation) -> Command {
        let mut command = Command::new(invocation.program());
        command.args(invocation.arguments());
        for variable in invocation.variables() {
            command.env(variable.name(), variable.value());
        }
        command.stdin(Stdio::null());
        command
    }

    fn compose_completion(&self, stream_output: Output) -> Completion {
        let Output {
            status,
            stdout: output,
            ..
        } = stream_output;
        match status.success() {
            true => Completion::Success { output },
            false => {
                let status = status.to_string();
                Completion::Failure { status }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_coverage_core_invocation::Variable;
    use std::ffi::OsString;

    mod fixture {
        use super::*;

        pub(crate) fn compiler_invocation_factory(
            argument_texts: &[&str],
            stream: Stream,
        ) -> Invocation {
            let program = OsString::from("rustc");
            let arguments = Vec::from_iter(argument_texts.iter().map(OsString::from));
            let name = OsString::from("TOOL_COVERAGE_PROBE");
            let value = OsString::from("probe");
            let variable = Variable::from((name, value));
            let variables = vec![variable];
            Invocation::from((program, arguments, variables, stream))
        }
    }

    #[test]
    fn a_captured_invocation_yields_the_output_of_its_program() {
        let argument_texts = ["--print", "crate-name", "src/lib.rs"];
        let stream = Stream::Capture;
        let invocation = fixture::compiler_invocation_factory(&argument_texts, stream);
        let process = Process;

        let completion = process.try_spawn_invocation(&invocation).unwrap();

        let output = b"lib\n".to_vec();
        let expectation = Completion::Success { output };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn an_inheriting_invocation_that_succeeds_is_a_success_without_an_output() {
        let stream = Stream::Inheritance;
        let invocation = fixture::compiler_invocation_factory(&["--version"], stream);
        let process = Process;

        let completion = process.try_spawn_invocation(&invocation).unwrap();

        let output = Vec::new();
        let expectation = Completion::Success { output };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_program_that_fails_is_a_failure_with_its_status() {
        let stream = Stream::Capture;
        let invocation = fixture::compiler_invocation_factory(&["--coverage-probe-flag"], stream);
        let process = Process;

        let completion = process.try_spawn_invocation(&invocation).unwrap();

        let status = String::from("exit status: 1");
        let expectation = Completion::Failure { status };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_program_that_does_not_exist_is_an_error() {
        let program = OsString::from("coverage-probe-program-that-does-not-exist");
        let arguments = Vec::new();
        let variables = Vec::new();
        let stream = Stream::Inheritance;
        let invocation = Invocation::from((program, arguments, variables, stream));
        let process = Process;

        let completion = process.try_spawn_invocation(&invocation);

        let error_kind = completion.unwrap_err().kind();
        assert_eq!(error_kind, io::ErrorKind::NotFound);
    }

    #[test]
    fn a_captured_program_that_does_not_exist_is_an_error() {
        let program = OsString::from("coverage-probe-program-that-does-not-exist");
        let arguments = Vec::new();
        let variables = Vec::new();
        let stream = Stream::Capture;
        let invocation = Invocation::from((program, arguments, variables, stream));
        let process = Process;

        let completion = process.try_spawn_invocation(&invocation);

        let error_kind = completion.unwrap_err().kind();
        assert_eq!(error_kind, io::ErrorKind::NotFound);
    }
}
