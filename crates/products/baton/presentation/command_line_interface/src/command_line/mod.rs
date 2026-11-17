use crate::arguments::Arguments;
use crate::arguments::request::Request;
use crate::guard_adapters::GuardAdapters;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use stack_lock_adapter_host_helper_elevation_program::{HelperConfiguration, HostHelper};
use stack_lock_adapter_program_runner_spawner::{ProgramRunner, RunnerConfiguration};
use stack_lock_application::acquisition::mode::AcquisitionMode;
use stack_lock_application::guard_run::GuardRun;
use stack_lock_application::host_helper::HostHelper as HostHelperPort;
use stack_lock_application::lock::guard::LockGuard;
use stack_lock_application::program_runner::ProgramRunner as ProgramRunnerPort;
use stack_lock_application::status_query::StatusQuery;
use std::ffi::OsString;
use std::io::{self, Write};
use std::process::ExitCode;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct CommandLine {
    arguments: Arguments,
}

impl Layer for CommandLine {
    type Layer = PresentationLayer;
}

impl From<Arguments> for CommandLine {
    fn from(arguments: Arguments) -> Self {
        Self { arguments }
    }
}

impl CommandLine {
    const FAILURE_CODE: u8 = 1;

    pub(crate) fn try_respond_exit_code<SpawnerType, SignalTrapType>(
        &self,
        (helper_spawner, runner_spawner): (SpawnerType, SpawnerType),
        signal_trap: SignalTrapType,
        mut output: impl Write,
        mut errors: impl Write,
    ) -> io::Result<ExitCode>
    where
        HostHelper<SpawnerType>: HostHelperPort,
        ProgramRunner<SpawnerType, SignalTrapType>: ProgramRunnerPort,
    {
        let elevation_program = OsString::from(self.arguments.elevation_program());
        let elevation_arguments = Vec::from_iter(
            self.arguments
                .elevation_argument()
                .iter()
                .map(OsString::from),
        );
        let host_program = OsString::from(self.arguments.host_program());
        let helper_configuration = HelperConfiguration::from((
            elevation_program,
            elevation_arguments,
            host_program,
        ));
        let host_helper = HostHelper::from((helper_configuration, helper_spawner));
        let check_interval = Duration::from_millis(self.arguments.check_interval());
        let runner_configuration = RunnerConfiguration::from(check_interval);
        let program_runner = ProgramRunner::from((
            runner_configuration,
            runner_spawner,
            signal_trap,
        ));
        let guard_adapters = GuardAdapters::from((host_helper, program_runner));
        let lock_guard = LockGuard::from(guard_adapters);
        match self.arguments.request() {
            Request::Run {
                lock,
                application,
                deadline,
                force,
                passthrough,
                program,
            } => {
                let maybe_deadline = deadline.map(Duration::from_millis);
                let acquisition_mode = AcquisitionMode::from((
                    maybe_deadline,
                    *force,
                    *passthrough,
                ));
                let guard_run = GuardRun::from((
                    lock.clone(),
                    application.clone(),
                    acquisition_mode,
                    program.clone(),
                ));
                match lock_guard.dispatch_command(guard_run) {
                    Ok(exit_code) => {
                        let exit_byte = u8::try_from(exit_code).unwrap_or(Self::FAILURE_CODE);
                        Ok(ExitCode::from(exit_byte))
                    }
                    Err(error) => {
                        writeln!(errors, "baton failed, {error}")?;
                        Ok(ExitCode::FAILURE)
                    }
                }
            }
            Request::Status => match lock_guard.resolve_query(StatusQuery) {
                Ok(status_texts) => {
                    for status_text in status_texts {
                        writeln!(output, "{status_text}")?;
                    }
                    Ok(ExitCode::SUCCESS)
                }
                Err(error) => {
                    writeln!(errors, "baton failed, {error}")?;
                    Ok(ExitCode::FAILURE)
                }
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use stack_lock_adapter_program_runner_spawner::SystemTrap;
    use stack_process_core_invocation::Completion;

    mod fixture {
        use super::*;

        pub(crate) type FixtureSpawner = stack_process_adapter_spawner_transcript::Spawner;

        pub(crate) struct FixtureOutput;

        impl Write for FixtureOutput {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::from(io::ErrorKind::BrokenPipe))
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        pub(crate) fn command_line_factory(argument_texts: &[&str]) -> CommandLine {
            let mut texts = vec!["baton", "--host-program", "/run/wrappers/bin/baton_host"];
            texts.extend_from_slice(argument_texts);
            CommandLine::from(Arguments::try_parse_from(texts).unwrap())
        }

        pub(crate) fn spawner_factory(completion: Vec<Option<Completion>>) -> FixtureSpawner {
            FixtureSpawner::from(completion)
        }

        pub(crate) fn failure_factory(maybe_code: Option<i32>) -> Option<Completion> {
            Some(Completion::Failure {
                status: String::from("exit status: 3"),
                maybe_code,
            })
        }
    }

    #[test]
    fn a_run_acquires_the_lock_runs_the_program_releases_the_lock_and_ends_with_its_code() {
        let command_line = fixture::command_line_factory(&[
            "run",
            "--lock",
            "rtx3080",
            "--application",
            "winix",
            "--deadline",
            "500",
            "--force",
            "--passthrough",
            "--",
            "qemu-system-x86_64",
        ]);
        let helper_spawner = fixture::spawner_factory(Vec::new());
        let runner_spawner = fixture::spawner_factory(vec![fixture::failure_factory(Some(3))]);
        let mut output = Vec::new();
        let mut errors = Vec::new();

        let exit_code = command_line
            .try_respond_exit_code(
                (helper_spawner, runner_spawner),
                SystemTrap::from(()),
                &mut output,
                &mut errors,
            )
            .unwrap();

        assert_eq!(exit_code, ExitCode::from(3));
        assert!(errors.is_empty());
    }

    #[test]
    fn a_run_whose_code_is_no_byte_and_a_run_whose_lock_fails_end_with_a_failure() {
        let command_line = fixture::command_line_factory(&[
            "run",
            "--lock",
            "rtx3080",
            "--application",
            "resolve",
            "--",
            "resolve",
        ]);
        let mut output = Vec::new();
        let mut errors = Vec::new();

        let exit_code = [
            command_line.try_respond_exit_code(
                (
                    fixture::spawner_factory(Vec::new()),
                    fixture::spawner_factory(vec![fixture::failure_factory(Some(300))]),
                ),
                SystemTrap::from(()),
                &mut output,
                &mut errors,
            ),
            command_line.try_respond_exit_code(
                (
                    fixture::spawner_factory(vec![fixture::failure_factory(Some(1))]),
                    fixture::spawner_factory(Vec::new()),
                ),
                SystemTrap::from(()),
                &mut output,
                &mut errors,
            ),
        ];

        assert_eq!(exit_code[0].as_ref().unwrap(), &ExitCode::from(1));
        assert_eq!(exit_code[1].as_ref().unwrap(), &ExitCode::FAILURE);
        assert!(
            String::from_utf8(errors)
                .unwrap()
                .starts_with("baton failed, ")
        );
    }

    #[test]
    fn a_status_prints_the_lines_of_the_helper_and_a_failed_status_its_error() {
        let command_line = fixture::command_line_factory(&["status"]);
        let status_output = Some(Completion::Success {
            output: b"lock rtx3080\n  owner nobody\n".to_vec(),
        });
        let mut output = Vec::new();
        let mut errors = Vec::new();

        let exit_code = [
            command_line.try_respond_exit_code(
                (
                    fixture::spawner_factory(vec![status_output]),
                    fixture::spawner_factory(Vec::new()),
                ),
                SystemTrap::from(()),
                &mut output,
                &mut errors,
            ),
            command_line.try_respond_exit_code(
                (
                    fixture::spawner_factory(vec![None]),
                    fixture::spawner_factory(Vec::new()),
                ),
                SystemTrap::from(()),
                &mut output,
                &mut errors,
            ),
        ];

        assert_eq!(exit_code[0].as_ref().unwrap(), &ExitCode::SUCCESS);
        assert_eq!(exit_code[1].as_ref().unwrap(), &ExitCode::FAILURE);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "lock rtx3080\n  owner nobody\n"
        );
        assert!(!errors.is_empty());
    }

    #[test]
    fn a_broken_output_or_a_broken_error_stream_is_an_error_of_the_program() {
        let command_line = [
            fixture::command_line_factory(&[
                "run",
                "--lock",
                "rtx3080",
                "--application",
                "resolve",
                "--",
                "resolve",
            ]),
            fixture::command_line_factory(&["status"]),
        ];
        let status_output = Some(Completion::Success {
            output: b"lock rtx3080\n".to_vec(),
        });

        let outcome = [
            command_line[0]
                .try_respond_exit_code(
                    (
                        fixture::spawner_factory(vec![fixture::failure_factory(Some(1))]),
                        fixture::spawner_factory(Vec::new()),
                    ),
                    SystemTrap::from(()),
                    Vec::new(),
                    fixture::FixtureOutput,
                )
                .is_err(),
            command_line[1]
                .try_respond_exit_code(
                    (
                        fixture::spawner_factory(vec![status_output]),
                        fixture::spawner_factory(Vec::new()),
                    ),
                    SystemTrap::from(()),
                    fixture::FixtureOutput,
                    Vec::new(),
                )
                .is_err(),
            command_line[1]
                .try_respond_exit_code(
                    (
                        fixture::spawner_factory(vec![None]),
                        fixture::spawner_factory(Vec::new()),
                    ),
                    SystemTrap::from(()),
                    Vec::new(),
                    fixture::FixtureOutput,
                )
                .is_err(),
        ];

        assert_eq!(outcome, [true, true, true]);
    }
}
