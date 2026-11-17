use crate::arguments::Arguments;
use crate::arguments::request::Request;
use crate::host_adapters::HostAdapters;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use stack_lock_application::Error;
use stack_lock_application::acquisition::Acquisition;
use stack_lock_application::acquisition::mode::AcquisitionMode;
use stack_lock_application::admission::Admission;
use stack_lock_application::dismissal::Dismissal;
use stack_lock_application::enclosure::Enclosure;
use stack_lock_application::lock::administration::LockAdministration;
use stack_lock_application::lock::setting::LockSetting;
use stack_lock_application::refresh::Refresh;
use stack_lock_application::release::Release;
use stack_lock_application::removal::Removal;
use stack_lock_application::status_query::StatusQuery;
use stack_lock_application::user_query::UserQuery;
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
    pub(crate) fn compose_administration(&self) -> LockAdministration<HostAdapters> {
        let host_adapters = HostAdapters::from(&self.arguments);
        let check_interval = Duration::from_millis(self.arguments.check_interval());
        let end_deadline = Duration::from_millis(self.arguments.end_deadline());
        let lock_setting = LockSetting::from((check_interval, end_deadline));
        LockAdministration::from((host_adapters, lock_setting))
    }

    pub(crate) fn try_respond_exit_code<AdministrationType>(
        &self,
        lock_administration: &AdministrationType,
        mut output: impl Write,
        mut errors: impl Write,
    ) -> io::Result<ExitCode>
    where
        AdministrationType: SynchronousCommandHandler<Enclosure, Outcome = Result<Vec<String>, Error>>
            + SynchronousCommandHandler<Admission, Outcome = Result<(), Error>>
            + SynchronousCommandHandler<Dismissal, Outcome = Result<(), Error>>
            + SynchronousCommandHandler<Acquisition, Outcome = Result<(), Error>>
            + SynchronousCommandHandler<Release, Outcome = Result<(), Error>>
            + SynchronousCommandHandler<Refresh, Outcome = Result<(), Error>>
            + SynchronousCommandHandler<Removal, Outcome = Result<(), Error>>
            + SynchronousQueryHandler<UserQuery>
            + SynchronousQueryHandler<StatusQuery>,
    {
        let outcome = match self.arguments.request() {
            Request::Enclosure { lock, device } => {
                Enclosure::try_from((lock.clone(), device.clone()))
                    .and_then(|enclosure| lock_administration.dispatch_command(enclosure))
            }
            Request::Admission {
                lock,
                application,
                default_owner,
            } => Admission::try_from((lock.clone(), application.clone(), *default_owner))
                .and_then(|admission| lock_administration.dispatch_command(admission))
                .map(|()| Vec::new()),
            Request::Dismissal { lock, application } => {
                Dismissal::try_from((lock.clone(), application.clone()))
                    .and_then(|dismissal| lock_administration.dispatch_command(dismissal))
                    .map(|()| Vec::new())
            }
            Request::Acquisition {
                lock,
                application,
                process,
                deadline,
                force,
                passthrough,
            } => {
                let maybe_deadline = deadline.map(Duration::from_millis);
                let acquisition_mode = AcquisitionMode::from((
                    maybe_deadline,
                    *force,
                    *passthrough,
                ));
                Acquisition::try_from((
                    lock.clone(),
                    application.clone(),
                    *process,
                    acquisition_mode,
                ))
                .and_then(|acquisition| lock_administration.dispatch_command(acquisition))
                .map(|()| Vec::new())
            }
            Request::Release { lock, application } => {
                Release::try_from((lock.clone(), application.clone()))
                    .and_then(|release| lock_administration.dispatch_command(release))
                    .map(|()| Vec::new())
            }
            Request::Refresh => lock_administration
                .dispatch_command(Refresh)
                .map(|()| Vec::new()),
            Request::Removal { lock } => Removal::try_from(lock.clone())
                .and_then(|removal| lock_administration.dispatch_command(removal))
                .map(|()| Vec::new()),
            Request::Users { lock } => UserQuery::try_from(lock.clone())
                .and_then(|user_query| lock_administration.resolve_query(user_query)),
            Request::Status => lock_administration.resolve_query(StatusQuery),
        };
        match outcome {
            Ok(texts) => {
                for text in texts {
                    writeln!(output, "{text}")?;
                }
                Ok(ExitCode::SUCCESS)
            }
            Err(error) => {
                writeln!(errors, "baton_host failed, {error}")?;
                Ok(ExitCode::FAILURE)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;
    use stack_domain_driven_design_core::layer::ApplicationLayer;

    mod fixture {
        use super::*;

        pub(crate) struct FixtureAdministration {
            pub(crate) outcome: Result<(), Error>,
        }

        impl Layer for FixtureAdministration {
            type Layer = ApplicationLayer;
        }

        impl SynchronousCommandHandler<Enclosure> for FixtureAdministration {
            type Outcome = Result<Vec<String>, Error>;

            fn dispatch_command(&self, enclosure: Enclosure) -> Self::Outcome {
                self.outcome
                    .clone()
                    .map(|()| vec![enclosure.lock_name().to_string()])
            }
        }

        impl SynchronousCommandHandler<Admission> for FixtureAdministration {
            type Outcome = Result<(), Error>;

            fn dispatch_command(&self, _: Admission) -> Self::Outcome {
                self.outcome.clone()
            }
        }

        impl SynchronousCommandHandler<Dismissal> for FixtureAdministration {
            type Outcome = Result<(), Error>;

            fn dispatch_command(&self, _: Dismissal) -> Self::Outcome {
                self.outcome.clone()
            }
        }

        impl SynchronousCommandHandler<Acquisition> for FixtureAdministration {
            type Outcome = Result<(), Error>;

            fn dispatch_command(&self, _: Acquisition) -> Self::Outcome {
                self.outcome.clone()
            }
        }

        impl SynchronousCommandHandler<Release> for FixtureAdministration {
            type Outcome = Result<(), Error>;

            fn dispatch_command(&self, _: Release) -> Self::Outcome {
                self.outcome.clone()
            }
        }

        impl SynchronousCommandHandler<Refresh> for FixtureAdministration {
            type Outcome = Result<(), Error>;

            fn dispatch_command(&self, _: Refresh) -> Self::Outcome {
                self.outcome.clone()
            }
        }

        impl SynchronousCommandHandler<Removal> for FixtureAdministration {
            type Outcome = Result<(), Error>;

            fn dispatch_command(&self, _: Removal) -> Self::Outcome {
                self.outcome.clone()
            }
        }

        impl SynchronousQueryHandler<UserQuery> for FixtureAdministration {
            fn resolve_query(&self, user_query: UserQuery) -> Result<Vec<String>, Error> {
                self.outcome
                    .clone()
                    .map(|()| vec![user_query.lock_name().to_string()])
            }
        }

        impl SynchronousQueryHandler<StatusQuery> for FixtureAdministration {
            fn resolve_query(&self, _: StatusQuery) -> Result<Vec<String>, Error> {
                self.outcome
                    .clone()
                    .map(|()| vec![String::from("lock rtx3080")])
            }
        }

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
            let mut texts = vec!["baton_host"];
            texts.extend_from_slice(argument_texts);
            CommandLine::from(Arguments::try_parse_from(texts).unwrap())
        }

        pub(crate) fn request_factory() -> Vec<&'static [&'static str]> {
            vec![
                &["enclose", "--lock", "rtx3080", "--device", "0000:01:00.0"],
                &[
                    "admit",
                    "--lock",
                    "rtx3080",
                    "--application",
                    "desktop",
                    "--default-owner",
                ],
                &["dismiss", "--lock", "rtx3080", "--application", "desktop"],
                &[
                    "acquire",
                    "--lock",
                    "rtx3080",
                    "--application",
                    "winix",
                    "--process",
                    "7",
                    "--deadline",
                    "500",
                    "--force",
                    "--passthrough",
                ],
                &["release", "--lock", "rtx3080", "--application", "winix"],
                &["refresh"],
                &["remove", "--lock", "rtx3080"],
                &["users", "--lock", "rtx3080"],
                &["status"],
            ]
        }
    }

    #[test]
    fn every_request_reaches_the_administration_and_prints_its_answer() {
        let lock_administration = fixture::FixtureAdministration { outcome: Ok(()) };
        let mut output = Vec::new();
        let mut errors = Vec::new();

        let exit_code = Vec::from_iter(fixture::request_factory().into_iter().map(|arguments| {
            fixture::command_line_factory(arguments)
                .try_respond_exit_code(&lock_administration, &mut output, &mut errors)
                .unwrap()
        }));

        assert_eq!(exit_code, vec![ExitCode::SUCCESS; 9]);
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "rtx3080\nrtx3080\nlock rtx3080\n"
        );
        assert!(errors.is_empty());
    }

    #[test]
    fn every_failed_request_prints_its_error_and_ends_with_a_failure() {
        let lock_text = String::from("rtx3080");
        let error = Error::LockAbsence { lock_text };
        let lock_administration = fixture::FixtureAdministration {
            outcome: Err(error),
        };
        let mut output = Vec::new();
        let mut errors = Vec::new();

        let exit_code = Vec::from_iter(fixture::request_factory().into_iter().map(|arguments| {
            fixture::command_line_factory(arguments)
                .try_respond_exit_code(&lock_administration, &mut output, &mut errors)
                .unwrap()
        }));

        assert_eq!(exit_code, vec![ExitCode::FAILURE; 9]);
        assert!(output.is_empty());
        assert_eq!(
            String::from_utf8(errors)
                .unwrap()
                .matches("baton_host failed, ")
                .count(),
            9
        );
    }

    #[test]
    fn a_request_with_an_empty_name_fails_before_the_administration() {
        let lock_administration = fixture::FixtureAdministration { outcome: Ok(()) };
        let mut output = Vec::new();
        let mut errors = Vec::new();

        let exit_code = fixture::command_line_factory(&["users", "--lock", ""])
            .try_respond_exit_code(&lock_administration, &mut output, &mut errors)
            .unwrap();

        assert_eq!(exit_code, ExitCode::FAILURE);
        assert!(output.is_empty());
        assert!(!errors.is_empty());
    }

    #[test]
    fn a_broken_output_or_a_broken_error_stream_is_an_error_of_the_program() {
        let lock_text = String::from("rtx3080");
        let error = Error::LockAbsence { lock_text };
        let lock_administration = [
            fixture::FixtureAdministration { outcome: Ok(()) },
            fixture::FixtureAdministration {
                outcome: Err(error),
            },
        ];
        let command_line = fixture::command_line_factory(&["status"]);

        let outcome = [
            command_line
                .try_respond_exit_code(&lock_administration[0], fixture::FixtureOutput, Vec::new())
                .is_err(),
            command_line
                .try_respond_exit_code(&lock_administration[1], Vec::new(), fixture::FixtureOutput)
                .is_err(),
        ];

        assert_eq!(outcome, [true, true]);
    }

    #[test]
    fn the_administration_holds_the_adapters_and_the_setting_of_the_arguments() {
        let command_line = fixture::command_line_factory(&[
            "--check-interval",
            "50",
            "--end-deadline",
            "3000",
            "status",
        ]);

        let lock_administration = command_line.compose_administration();

        let arguments = Arguments::try_parse_from([
            "baton_host",
            "--check-interval",
            "50",
            "--end-deadline",
            "3000",
            "status",
        ])
        .unwrap();
        let host_adapters = HostAdapters::from(&arguments);
        let check_interval = Duration::from_millis(50);
        let end_deadline = Duration::from_millis(3000);
        let lock_setting = LockSetting::from((check_interval, end_deadline));
        assert_eq!(
            lock_administration,
            LockAdministration::from((host_adapters, lock_setting))
        );
    }
}
