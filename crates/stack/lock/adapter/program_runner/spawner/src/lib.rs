pub mod signal_trap;

mod configuration;
mod error;
mod system_trap;

pub use configuration::RunnerConfiguration;
pub use error::Error;
pub use system_trap::SystemTrap;

use crate::signal_trap::{SignalPresence, SignalTrap};
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::program_runner::ProgramRunner as ProgramRunnerPort;
use stack_process_core_invocation::{Completion, Invocation, RunningProgram, Spawner, Stream};
use std::ffi::OsString;
use std::io;
use std::process;
use std::thread;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ProgramRunner<SpawnerType, SignalTrapType> {
    runner_configuration: RunnerConfiguration,
    spawner: SpawnerType,
    signal_trap: SignalTrapType,
}

impl<SpawnerType, SignalTrapType> Layer for ProgramRunner<SpawnerType, SignalTrapType> {
    type Layer = InfrastructureLayer;
}

impl<SpawnerType, SignalTrapType> Adapter for ProgramRunner<SpawnerType, SignalTrapType> {}

impl<SpawnerType, SignalTrapType> Port for ProgramRunner<SpawnerType, SignalTrapType> {}

impl<SpawnerType, SignalTrapType> DrivenPort for ProgramRunner<SpawnerType, SignalTrapType> {}

impl<SpawnerType, SignalTrapType> From<(RunnerConfiguration, SpawnerType, SignalTrapType)>
    for ProgramRunner<SpawnerType, SignalTrapType>
{
    fn from(
        (runner_configuration, spawner, signal_trap): (
            RunnerConfiguration,
            SpawnerType,
            SignalTrapType,
        ),
    ) -> Self {
        Self {
            runner_configuration,
            spawner,
            signal_trap,
        }
    }
}

impl<SpawnerType: Spawner, SignalTrapType: SignalTrap> ProgramRunnerPort
    for ProgramRunner<SpawnerType, SignalTrapType>
{
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
        self.signal_trap.try_arm_trap().map_err(|error| {
            let text = error.to_string();
            Error::TrapFailure { text }
        })?;
        let completion_outcome = self.try_supervise_program(program_text, &invocation);
        self.signal_trap.lift_trap();
        let completion = completion_outcome?;
        Self::try_compose_exit_code(program_text, completion)
    }
}

impl<SpawnerType: Spawner, SignalTrapType: SignalTrap> ProgramRunner<SpawnerType, SignalTrapType> {
    fn try_supervise_program(
        &self,
        program_text: &str,
        invocation: &Invocation,
    ) -> Result<Completion, Error> {
        let mut running_program = self.spawner.try_start_invocation(invocation).map_err(
            |error| {
                let program_text = String::from(program_text);
                let text = error.to_string();
                Error::StartFailure { program_text, text }
            },
        )?;
        let check_interval = self.runner_configuration.check_interval();
        loop {
            let maybe_completion = running_program
                .try_poll_completion()
                .map_err(|error| Self::compose_wait_failure(program_text, error))?;
            match maybe_completion {
                Some(completion) => return Ok(completion),
                None => {
                    let signal_presence = self.signal_trap.poll_signal_presence();
                    match signal_presence {
                        SignalPresence::Presence => running_program
                            .try_request_end()
                            .map_err(|error| Self::compose_wait_failure(program_text, error))?,
                        SignalPresence::Absence => {}
                    }
                    thread::sleep(check_interval);
                }
            }
        }
    }

    fn compose_wait_failure(program_text: &str, error: io::Error) -> Error {
        let program_text = String::from(program_text);
        let text = error.to_string();
        Error::WaitFailure { program_text, text }
    }

    fn try_compose_exit_code(program_text: &str, completion: Completion) -> Result<i32, Error> {
        match completion {
            Completion::Success { .. } => Ok(0),
            Completion::Failure { maybe_code, status } => match maybe_code {
                Some(exit_code) => Ok(exit_code),
                None => {
                    let program_text = String::from(program_text);
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
    use std::collections::VecDeque;
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    mod fixture {
        use super::*;

        #[derive(Debug)]
        pub(crate) struct FixtureSignalTrap {
            pub(crate) registration_outcome: Result<(), io::ErrorKind>,
            pub(crate) signal_presences: Mutex<VecDeque<SignalPresence>>,
            pub(crate) lift_flag: AtomicBool,
        }

        impl SignalTrap for FixtureSignalTrap {
            fn try_arm_trap(&self) -> io::Result<()> {
                self.registration_outcome.map_err(io::Error::from)
            }

            fn poll_signal_presence(&self) -> SignalPresence {
                self.signal_presences
                    .lock()
                    .unwrap()
                    .pop_front()
                    .unwrap_or(SignalPresence::Absence)
            }

            fn lift_trap(&self) {
                self.lift_flag.store(true, Ordering::SeqCst);
            }
        }

        #[derive(Debug)]
        pub(crate) struct FixtureSpawner {
            pub(crate) completion_queue: VecDeque<Result<Option<Completion>, io::ErrorKind>>,
            pub(crate) end_request: Result<(), io::ErrorKind>,
            pub(crate) end_requests: Arc<AtomicUsize>,
        }

        impl Layer for FixtureSpawner {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for FixtureSpawner {}

        impl Port for FixtureSpawner {}

        impl DrivenPort for FixtureSpawner {}

        impl Spawner for FixtureSpawner {
            type RunningProgram = FixtureProgram;

            fn try_spawn_invocation(&self, _: &Invocation) -> io::Result<Completion> {
                Err(io::Error::from(io::ErrorKind::Unsupported))
            }

            fn try_start_invocation(&self, _: &Invocation) -> io::Result<FixtureProgram> {
                let completion_queue = self.completion_queue.clone();
                let end_requests = Arc::clone(&self.end_requests);
                Ok(FixtureProgram {
                    completion_queue,
                    end_request: self.end_request,
                    end_requests,
                })
            }
        }

        #[derive(Debug)]
        pub(crate) struct FixtureProgram {
            completion_queue: VecDeque<Result<Option<Completion>, io::ErrorKind>>,
            end_request: Result<(), io::ErrorKind>,
            end_requests: Arc<AtomicUsize>,
        }

        impl RunningProgram for FixtureProgram {
            fn try_wait_completion(self) -> io::Result<Completion> {
                Err(io::Error::from(io::ErrorKind::Unsupported))
            }

            fn try_request_end(&mut self) -> io::Result<()> {
                self.end_requests.fetch_add(1, Ordering::SeqCst);
                self.end_request.map_err(io::Error::from)
            }

            fn try_poll_completion(&mut self) -> io::Result<Option<Completion>> {
                let maybe_completion = self.completion_queue.pop_front().unwrap_or(Ok(None));
                maybe_completion.map_err(io::Error::from)
            }
        }

        pub(crate) fn spawner_factory(
            completion_sequence: Vec<Result<Option<Completion>, io::ErrorKind>>,
            end_request: Result<(), io::ErrorKind>,
        ) -> FixtureSpawner {
            let completion_queue = VecDeque::from(completion_sequence);
            let end_requests = Arc::new(AtomicUsize::new(0));
            FixtureSpawner {
                completion_queue,
                end_request,
                end_requests,
            }
        }

        pub(crate) fn signal_trap_factory(
            registration_outcome: Result<(), io::ErrorKind>,
            signal_presence_sequence: &[SignalPresence],
        ) -> FixtureSignalTrap {
            let signal_presences = Mutex::new(VecDeque::from_iter(
                signal_presence_sequence.iter().copied(),
            ));
            let lift_flag = AtomicBool::new(false);
            FixtureSignalTrap {
                registration_outcome,
                signal_presences,
                lift_flag,
            }
        }

        pub(crate) fn program_runner_factory<SpawnerType>(
            spawner: SpawnerType,
            signal_trap: FixtureSignalTrap,
        ) -> ProgramRunner<SpawnerType, FixtureSignalTrap> {
            let check_interval = Duration::from_millis(1);
            let runner_configuration = RunnerConfiguration::from(check_interval);
            ProgramRunner::from((runner_configuration, spawner, signal_trap))
        }

        pub(crate) fn completion_factory(maybe_code: Option<i32>) -> Completion {
            Completion::Failure {
                status: String::from("exit status: 143"),
                maybe_code,
            }
        }
    }

    #[test]
    fn a_program_runs_with_its_arguments_and_answers_its_exit_code() {
        let completion = [
            Some(Completion::Success { output: Vec::new() }),
            Some(fixture::completion_factory(Some(3))),
        ];
        let program_runner = fixture::program_runner_factory(
            stack_process_adapter_spawner_transcript::Spawner::from(Vec::from(completion)),
            fixture::signal_trap_factory(Ok(()), &[]),
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
        assert!(program_runner.spawner.end_requests().is_empty());
        assert!(program_runner.signal_trap.lift_flag.load(Ordering::SeqCst));
        assert_eq!(program_runner.guard_identifier(), process::id());
    }

    #[test]
    fn a_signal_while_the_program_runs_requests_its_end_and_the_runner_waits_for_its_code() {
        let completion = fixture::completion_factory(Some(143));
        let spawner = fixture::spawner_factory(
            vec![Ok(None), Ok(None), Ok(None), Ok(Some(completion))],
            Ok(()),
        );
        let signal_trap = fixture::signal_trap_factory(
            Ok(()),
            &[SignalPresence::Absence, SignalPresence::Presence],
        );
        let program_runner = fixture::program_runner_factory(spawner, signal_trap);
        let program_texts = [String::from("openrgb"), String::from("--server")];

        let exit_code = program_runner.try_spawn_program(&program_texts);

        assert_eq!(exit_code, Ok(143));
        assert_eq!(
            program_runner.spawner.end_requests.load(Ordering::SeqCst),
            1
        );
        assert!(program_runner.signal_trap.lift_flag.load(Ordering::SeqCst));
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
        let program_runner = fixture::program_runner_factory(
            stack_process_adapter_spawner_transcript::Spawner::from(Vec::from(completion)),
            fixture::signal_trap_factory(Ok(()), &[]),
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
        assert!(program_runner.signal_trap.lift_flag.load(Ordering::SeqCst));
    }

    #[test]
    fn a_trap_that_cannot_be_armed_starts_nothing() {
        let program_runner = fixture::program_runner_factory(
            stack_process_adapter_spawner_transcript::Spawner::from(()),
            fixture::signal_trap_factory(Err(io::ErrorKind::InvalidInput), &[]),
        );
        let program_texts = [String::from("openrgb")];

        let error = program_runner.try_spawn_program(&program_texts);

        assert!(matches!(error, Err(Error::TrapFailure { .. })));
        assert!(program_runner.spawner.invocations().is_empty());
        assert!(!program_runner.signal_trap.lift_flag.load(Ordering::SeqCst));
    }

    #[test]
    fn a_poll_or_a_request_to_end_that_fails_is_an_error_and_still_lifts_the_trap() {
        let program_runner = [
            fixture::program_runner_factory(
                fixture::spawner_factory(vec![Err(io::ErrorKind::Other)], Ok(())),
                fixture::signal_trap_factory(Ok(()), &[]),
            ),
            fixture::program_runner_factory(
                fixture::spawner_factory(vec![Ok(None)], Err(io::ErrorKind::PermissionDenied)),
                fixture::signal_trap_factory(Ok(()), &[SignalPresence::Presence]),
            ),
        ];
        let program_texts = [String::from("openrgb")];

        let error = program_runner
            .each_ref()
            .map(|program_runner| program_runner.try_spawn_program(&program_texts));

        assert!(matches!(error[0], Err(Error::WaitFailure { .. })));
        assert!(matches!(error[1], Err(Error::WaitFailure { .. })));
        assert!(
            program_runner.iter().all(|program_runner| {
                program_runner.signal_trap.lift_flag.load(Ordering::SeqCst)
            })
        );
    }
}
