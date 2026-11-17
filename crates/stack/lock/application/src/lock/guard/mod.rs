pub mod ports;

use crate::Error;
use crate::acquisition::Acquisition;
use crate::guard_run::GuardRun;
use crate::host_helper::HostHelper;
use crate::lock::guard::ports::GuardPorts;
use crate::program_runner::ProgramRunner;
use crate::release::Release;
use crate::status_query::StatusQuery;
use stack_domain_driven_design_core::application_service::ApplicationService;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LockGuard<PortsType> {
    guard_ports: PortsType,
}

impl<PortsType> Layer for LockGuard<PortsType> {
    type Layer = ApplicationLayer;
}

impl<PortsType> ApplicationService for LockGuard<PortsType> {}

impl<PortsType> From<PortsType> for LockGuard<PortsType> {
    fn from(guard_ports: PortsType) -> Self {
        Self { guard_ports }
    }
}

impl<PortsType: GuardPorts> SynchronousCommandHandler<GuardRun> for LockGuard<PortsType> {
    type Outcome = Result<i32, Error>;

    fn dispatch_command(&self, guard_run: GuardRun) -> Self::Outcome {
        let program_runner = self.guard_ports.program_runner();
        let host_helper = self.guard_ports.host_helper();
        let lock_text = String::from(guard_run.lock_text());
        let application_text = String::from(guard_run.application_text());
        let waiter_identifier = program_runner.guard_identifier();
        let acquisition = Acquisition::try_from((
            lock_text,
            application_text,
            waiter_identifier,
            guard_run.acquisition_mode(),
        ))?;
        let lock_name = acquisition.lock_name().clone();
        let application_name = acquisition.application_name().clone();
        let release = Release::from((lock_name, application_name));
        host_helper
            .try_acquire_lock(&acquisition)
            .map_err(Self::compose_helper_error)?;
        let run_outcome = program_runner
            .try_spawn_program(guard_run.program_texts())
            .map_err(Self::compose_runner_error);
        let release_outcome = host_helper
            .try_release_lock(&release)
            .map_err(Self::compose_helper_error);
        let exit_code = run_outcome?;
        release_outcome?;
        Ok(exit_code)
    }
}

impl<PortsType: GuardPorts> SynchronousQueryHandler<StatusQuery> for LockGuard<PortsType> {
    fn resolve_query(&self, _: StatusQuery) -> Result<Vec<String>, Error> {
        self.guard_ports
            .host_helper()
            .try_read_status()
            .map_err(Self::compose_helper_error)
    }
}

impl<PortsType: GuardPorts> LockGuard<PortsType> {
    fn compose_helper_error(error: <PortsType::HostHelper as HostHelper>::Error) -> Error {
        let text = error.to_string();
        Error::HostHelper { text }
    }

    fn compose_runner_error(error: <PortsType::ProgramRunner as ProgramRunner>::Error) -> Error {
        let text = error.to_string();
        Error::ProgramRunner { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acquisition::mode::AcquisitionMode;
    use stack_domain_driven_design_core::adapter::Adapter;
    use stack_domain_driven_design_core::layer::InfrastructureLayer;
    use stack_domain_driven_design_core::port::{DrivenPort, Port};
    use std::error;
    use std::fmt;
    use std::sync::Mutex;

    mod fixture {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        pub(crate) struct FixtureError {
            pub(crate) text: String,
        }

        impl fmt::Display for FixtureError {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.text)
            }
        }

        impl error::Error for FixtureError {}

        #[derive(Debug, Default)]
        pub(crate) struct FixtureGuard {
            pub(crate) program_texts: Mutex<Vec<String>>,
            pub(crate) rejection_port: Option<&'static str>,
        }

        impl Layer for FixtureGuard {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for FixtureGuard {}

        impl Port for FixtureGuard {}

        impl DrivenPort for FixtureGuard {}

        impl FixtureGuard {
            fn outcome_factory(
                &self,
                port_name: &'static str,
                user_text: String,
            ) -> Result<(), FixtureError> {
                self.program_texts.lock().unwrap().push(user_text);
                match self.rejection_port == Some(port_name) {
                    true => Err(FixtureError {
                        text: format!("the {port_name} broke"),
                    }),
                    false => Ok(()),
                }
            }
        }

        impl HostHelper for FixtureGuard {
            type Error = FixtureError;

            fn try_acquire_lock(&self, acquisition: &Acquisition) -> Result<(), FixtureError> {
                let user_text = format!(
                    "acquire {} {} {}",
                    acquisition.lock_name(),
                    acquisition.application_name(),
                    acquisition.waiter_identifier()
                );
                self.outcome_factory("acquisition", user_text)
            }

            fn try_release_lock(&self, release: &Release) -> Result<(), FixtureError> {
                let user_text = format!(
                    "release {} {}",
                    release.lock_name(),
                    release.application_name()
                );
                self.outcome_factory("release", user_text)
            }

            fn try_read_status(&self) -> Result<Vec<String>, FixtureError> {
                self.outcome_factory("status", String::from("status"))?;
                Ok(vec![String::from("lock rtx3080")])
            }
        }

        impl ProgramRunner for FixtureGuard {
            type Error = FixtureError;

            fn guard_identifier(&self) -> u32 {
                4242
            }

            fn try_spawn_program(&self, program_texts: &[String]) -> Result<i32, FixtureError> {
                self.outcome_factory("program", program_texts.join(" "))?;
                Ok(3)
            }
        }

        impl<'guard> GuardPorts for &'guard FixtureGuard {
            type HostHelper = FixtureGuard;
            type ProgramRunner = FixtureGuard;

            fn host_helper(&self) -> &FixtureGuard {
                self
            }

            fn program_runner(&self) -> &FixtureGuard {
                self
            }
        }

        pub(crate) fn guard_run_factory(application_text: &str) -> GuardRun {
            GuardRun::from((
                String::from("rtx3080"),
                String::from(application_text),
                AcquisitionMode::default(),
                vec![String::from("davinci-resolve"), String::from("--render")],
            ))
        }
    }

    #[test]
    fn a_run_acquires_the_lock_runs_the_program_and_releases_the_lock() {
        let fixture_guard = fixture::FixtureGuard::default();
        let lock_guard = LockGuard::from(&fixture_guard);

        let exit_code = lock_guard
            .dispatch_command(fixture::guard_run_factory("resolve"))
            .unwrap();

        assert_eq!(exit_code, 3);
        assert_eq!(
            *fixture_guard.program_texts.lock().unwrap(),
            [
                "acquire rtx3080 resolve 4242",
                "davinci-resolve --render",
                "release rtx3080 resolve",
            ]
        );
    }

    #[test]
    fn a_run_whose_program_fails_still_releases_the_lock() {
        let fixture_guard = fixture::FixtureGuard {
            rejection_port: Some("program"),
            ..fixture::FixtureGuard::default()
        };
        let lock_guard = LockGuard::from(&fixture_guard);

        let outcome = lock_guard.dispatch_command(fixture::guard_run_factory("resolve"));

        assert_eq!(
            outcome,
            Err(Error::ProgramRunner {
                text: String::from("the program broke")
            })
        );
        assert_eq!(fixture_guard.program_texts.lock().unwrap().len(), 3);
    }

    #[test]
    fn a_run_whose_acquisition_fails_runs_nothing() {
        let fixture_guard = fixture::FixtureGuard {
            rejection_port: Some("acquisition"),
            ..fixture::FixtureGuard::default()
        };
        let lock_guard = LockGuard::from(&fixture_guard);

        let outcome = lock_guard.dispatch_command(fixture::guard_run_factory("resolve"));
        let name_outcome = lock_guard.dispatch_command(fixture::guard_run_factory("--help"));

        assert_eq!(
            outcome,
            Err(Error::HostHelper {
                text: String::from("the acquisition broke")
            })
        );
        assert!(matches!(name_outcome, Err(Error::Name { .. })));
        assert_eq!(fixture_guard.program_texts.lock().unwrap().len(), 1);
    }

    #[test]
    fn a_run_whose_release_fails_answers_the_error_of_the_release() {
        let fixture_guard = fixture::FixtureGuard {
            rejection_port: Some("release"),
            ..fixture::FixtureGuard::default()
        };
        let lock_guard = LockGuard::from(&fixture_guard);

        let outcome = lock_guard.dispatch_command(fixture::guard_run_factory("resolve"));

        assert_eq!(
            outcome,
            Err(Error::HostHelper {
                text: String::from("the release broke")
            })
        );
    }

    #[test]
    fn the_status_comes_from_the_helper() {
        let fixture_guard = fixture::FixtureGuard::default();
        let guard_ports = fixture::FixtureGuard {
            rejection_port: Some("status"),
            ..fixture::FixtureGuard::default()
        };

        let status_texts = LockGuard::from(&fixture_guard)
            .resolve_query(StatusQuery)
            .unwrap();
        let outcome = LockGuard::from(&guard_ports).resolve_query(StatusQuery);

        assert_eq!(status_texts, ["lock rtx3080"]);
        assert!(matches!(outcome, Err(Error::HostHelper { .. })));
    }
}
