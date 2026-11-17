mod running_program;

use crate::running_program::ChildProgram;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_process_core_invocation::{Completion, Invocation, Spawner as SpawnerPort, Stream};
use std::io;
use std::process::{Command, Stdio};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Spawner;

impl Layer for Spawner {
    type Layer = InfrastructureLayer;
}

impl Adapter for Spawner {}

impl Port for Spawner {}

impl DrivenPort for Spawner {}

impl From<()> for Spawner {
    fn from(_: ()) -> Self {
        Self
    }
}

impl SpawnerPort for Spawner {
    type RunningProgram = ChildProgram;

    fn try_spawn_invocation(&self, invocation: &Invocation) -> io::Result<Completion> {
        let child_program = self.try_start_invocation(invocation)?;
        child_program.try_read_completion()
    }

    fn try_start_invocation(&self, invocation: &Invocation) -> io::Result<ChildProgram> {
        let mut command = self.compose_command(invocation);
        let child = command.spawn()?;
        let child_program = ChildProgram::from(child);
        Ok(child_program)
    }
}

impl Spawner {
    fn compose_command(&self, invocation: &Invocation) -> Command {
        let mut command = Command::new(invocation.program());
        command.args(invocation.arguments());
        for variable in invocation.variables() {
            command.env(variable.name(), variable.value());
        }
        match invocation.stream() {
            Stream::Capture => {
                command.stdin(Stdio::null());
                command.stdout(Stdio::piped());
                command.stderr(Stdio::inherit());
            }
            Stream::Inheritance => {
                command.stdin(Stdio::inherit());
                command.stdout(Stdio::inherit());
                command.stderr(Stdio::inherit());
            }
        }
        command
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stack_process_core_invocation::{RunningProgram, Variable};
    use std::ffi::OsString;

    mod fixture {
        use super::*;

        pub(crate) fn invocation_factory(
            program: &str,
            arguments: &[&str],
            stream: Stream,
        ) -> Invocation {
            Invocation::from((
                OsString::from(program),
                Vec::from_iter(arguments.iter().map(OsString::from)),
                stream,
            ))
        }
    }

    #[test]
    fn a_captured_invocation_yields_the_output_of_its_program() {
        let invocation = fixture::invocation_factory("echo", &["baton"], Stream::Capture);
        let spawner = Spawner::from(());

        let completion = spawner.try_spawn_invocation(&invocation).unwrap();

        let output = b"baton\n".to_vec();
        let expectation = Completion::Success { output };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn an_inheriting_invocation_that_succeeds_is_a_success_without_an_output() {
        let invocation = fixture::invocation_factory("true", &[], Stream::Inheritance);
        let spawner = Spawner::from(());

        let completion = spawner.try_spawn_invocation(&invocation).unwrap();

        let output = Vec::new();
        let expectation = Completion::Success { output };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_program_that_fails_is_a_failure_with_its_status() {
        let invocation = fixture::invocation_factory("false", &[], Stream::Capture);
        let spawner = Spawner::from(());

        let completion = spawner.try_spawn_invocation(&invocation).unwrap();

        let status = String::from("exit status: 1");
        let expectation = Completion::Failure {
            status,
            maybe_code: Some(1),
        };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_started_program_completes_once_it_is_waited_for() {
        let invocation = fixture::invocation_factory("sleep", &["0"], Stream::Capture);
        let spawner = Spawner::from(());

        let child_program = spawner.try_start_invocation(&invocation).unwrap();
        let completion = child_program.try_wait_completion().unwrap();

        let output = Vec::new();
        let expectation = Completion::Success { output };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_request_to_end_sends_the_signal_of_termination_and_never_the_signal_of_a_kill() {
        let invocation = fixture::invocation_factory(
            "sh",
            &["-c", "trap 'exit 42' TERM; while :; do sleep 0.05; done"],
            Stream::Inheritance,
        );
        let spawner = Spawner::from(());

        let mut child_program = spawner.try_start_invocation(&invocation).unwrap();
        std::thread::sleep(std::time::Duration::from_millis(300));
        let maybe_completion = child_program.try_poll_completion().unwrap();
        child_program.try_request_end().unwrap();
        let completion = child_program.try_wait_completion().unwrap();

        let expectation = Completion::Failure {
            status: String::from("exit status: 42"),
            maybe_code: Some(42),
        };
        assert_eq!(maybe_completion, None);
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_request_to_end_a_program_that_already_ended_changes_nothing() {
        let invocation = fixture::invocation_factory("true", &[], Stream::Inheritance);
        let spawner = Spawner::from(());

        let mut child_program = spawner.try_start_invocation(&invocation).unwrap();
        let mut maybe_completion = None;
        while maybe_completion.is_none() {
            maybe_completion = child_program.try_poll_completion().unwrap();
        }
        let end_outcome = child_program.try_request_end();

        let output = Vec::new();
        assert_eq!(maybe_completion, Some(Completion::Success { output }));
        assert!(end_outcome.is_ok());
    }

    #[test]
    fn a_variable_reaches_the_environment_of_the_program() {
        let program = OsString::from("printenv");
        let argument = OsString::from("BATON_PROBE");
        let arguments = vec![argument];
        let name = OsString::from("BATON_PROBE");
        let value = OsString::from("probe");
        let variable = Variable::from((name, value));
        let variables = vec![variable];
        let invocation = Invocation::from((program, arguments, variables, Stream::Capture));
        let spawner = Spawner::from(());

        let completion = spawner.try_spawn_invocation(&invocation).unwrap();

        let output = b"probe\n".to_vec();
        let expectation = Completion::Success { output };
        assert_eq!(completion, expectation);
    }

    #[test]
    fn a_program_that_does_not_exist_is_an_error() {
        let invocation = fixture::invocation_factory(
            "baton-probe-program-that-does-not-exist",
            &[],
            Stream::Inheritance,
        );
        let spawner = Spawner::from(());

        let completion = spawner.try_spawn_invocation(&invocation);

        let error_kind = completion.err().map(|error| error.kind());
        let expectation = Some(io::ErrorKind::NotFound);
        assert_eq!(error_kind, expectation);
    }
}
