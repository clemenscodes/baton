mod error;

pub use error::Error;

use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::program_runner::ProgramRunner as ProgramRunnerPort;
use stack_process_core_invocation::{Completion, Invocation, Spawner, Stream};
use std::ffi::OsString;
use std::process;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ProgramRunner<SpawnerType> {
    spawner: SpawnerType,
}

impl<SpawnerType> Layer for ProgramRunner<SpawnerType> {
    type Layer = InfrastructureLayer;
}

impl<SpawnerType> Adapter for ProgramRunner<SpawnerType> {}

impl<SpawnerType> Port for ProgramRunner<SpawnerType> {}

impl<SpawnerType> DrivenPort for ProgramRunner<SpawnerType> {}

impl<SpawnerType> From<SpawnerType> for ProgramRunner<SpawnerType> {
    fn from(spawner: SpawnerType) -> Self {
        Self { spawner }
    }
}

impl<SpawnerType: Spawner> ProgramRunnerPort for ProgramRunner<SpawnerType> {
    type Error = Error;

    fn guard_identifier(&self) -> u32 {
        process::id()
    }

    fn try_spawn_program(&self, program_texts: &[String]) -> Result<i32, Error> {
        let Some((program_text, argument_texts)) = program_texts.split_first() else {
            return Err(Error::ProgramAbsence);
        };
        let program = OsString::from(program_text);
        let arguments = Vec::from_iter(argument_texts.iter().map(OsString::from));
        let invocation = Invocation::from((program, arguments, Stream::Inheritance));
        let completion = self
            .spawner
            .try_spawn_invocation(&invocation)
            .map_err(|error| {
                let program_text = program_text.clone();
                let text = error.to_string();
                Error::StartFailure { program_text, text }
            })?;
        match completion {
            Completion::Success { .. } => Ok(0),
            Completion::Failure { maybe_code, status } => match maybe_code {
                Some(exit_code) => Ok(exit_code),
                None => {
                    let program_text = program_text.clone();
                    Err(Error::Signal {
                        program_text,
                        status,
                    })
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_program_runs_with_its_arguments_and_answers_its_exit_code() {
        let completion = [
            Some(Completion::Success { output: Vec::new() }),
            Some(Completion::Failure {
                status: String::from("exit status: 3"),
                maybe_code: Some(3),
            }),
        ];
        let program_runner = ProgramRunner::from(
            stack_process_adapter_spawner_transcript::Spawner::from(Vec::from(completion)),
        );
        let program_texts = [String::from("davinci-resolve"), String::from("--render")];

        let exit_code = [
            program_runner.try_spawn_program(&program_texts),
            program_runner.try_spawn_program(&program_texts),
        ];

        assert_eq!(exit_code, [Ok(0), Ok(3)]);
        let invocations = program_runner.spawner.invocations();
        assert_eq!(invocations[0].program(), "davinci-resolve");
        assert_eq!(invocations[0].arguments(), [OsString::from("--render")]);
        assert_eq!(invocations[0].stream(), Stream::Inheritance);
        assert_eq!(program_runner.guard_identifier(), process::id());
    }

    #[test]
    fn a_program_ended_by_a_signal_a_program_that_never_started_and_no_program_are_errors() {
        let completion = [
            Some(Completion::Failure {
                status: String::from("signal: 9 (SIGKILL)"),
                maybe_code: None,
            }),
            None,
        ];
        let program_runner = ProgramRunner::from(
            stack_process_adapter_spawner_transcript::Spawner::from(Vec::from(completion)),
        );
        let program_texts = [String::from("davinci-resolve")];

        let error = [
            program_runner.try_spawn_program(&program_texts),
            program_runner.try_spawn_program(&program_texts),
            program_runner.try_spawn_program(&[]),
        ];

        assert_eq!(
            error[0],
            Err(Error::Signal {
                program_text: String::from("davinci-resolve"),
                status: String::from("signal: 9 (SIGKILL)"),
            })
        );
        assert!(matches!(error[1], Err(Error::StartFailure { .. })));
        assert_eq!(error[2], Err(Error::ProgramAbsence));
    }
}
