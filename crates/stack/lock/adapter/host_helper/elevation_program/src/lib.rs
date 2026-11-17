mod configuration;
mod error;

pub use configuration::HelperConfiguration;
pub use error::Error;

use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::acquisition::Acquisition;
use stack_lock_application::host_helper::HostHelper as HostHelperPort;
use stack_lock_application::release::Release;
use stack_process_core_invocation::{Completion, Invocation, Spawner, Stream};
use std::ffi::OsString;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HostHelper<SpawnerType> {
    helper_configuration: HelperConfiguration,
    spawner: SpawnerType,
}

impl<SpawnerType> Layer for HostHelper<SpawnerType> {
    type Layer = InfrastructureLayer;
}

impl<SpawnerType> Adapter for HostHelper<SpawnerType> {}

impl<SpawnerType> Port for HostHelper<SpawnerType> {}

impl<SpawnerType> DrivenPort for HostHelper<SpawnerType> {}

impl<SpawnerType> From<(HelperConfiguration, SpawnerType)> for HostHelper<SpawnerType> {
    fn from((helper_configuration, spawner): (HelperConfiguration, SpawnerType)) -> Self {
        Self {
            helper_configuration,
            spawner,
        }
    }
}

impl<SpawnerType: Spawner> HostHelperPort for HostHelper<SpawnerType> {
    type Error = Error;

    fn try_acquire_lock(&self, acquisition: &Acquisition) -> Result<(), Error> {
        let acquisition_mode = acquisition.acquisition_mode();
        let mut request_arguments = vec![
            String::from("acquire"),
            String::from("--lock"),
            acquisition.lock_name().to_string(),
            String::from("--application"),
            acquisition.application_name().to_string(),
            String::from("--process"),
            acquisition.waiter_identifier().to_string(),
        ];
        if let Some(deadline) = acquisition_mode.maybe_deadline() {
            request_arguments.push(String::from("--deadline"));
            request_arguments.push(deadline.as_millis().to_string());
        }
        if acquisition_mode.force_presence() {
            request_arguments.push(String::from("--force"));
        }
        if acquisition_mode.passthrough_presence() {
            request_arguments.push(String::from("--passthrough"));
        }
        self.try_spawn_request(&request_arguments, Stream::Inheritance)
            .map(|_| ())
    }

    fn try_release_lock(&self, release: &Release) -> Result<(), Error> {
        let request_arguments = [
            String::from("release"),
            String::from("--lock"),
            release.lock_name().to_string(),
            String::from("--application"),
            release.application_name().to_string(),
        ];
        self.try_spawn_request(&request_arguments, Stream::Inheritance)
            .map(|_| ())
    }

    fn try_read_status(&self) -> Result<Vec<String>, Error> {
        let request_arguments = [String::from("status")];
        let output = self.try_spawn_request(&request_arguments, Stream::Capture)?;
        let output_text = String::from_utf8_lossy(&output);
        Ok(Vec::from_iter(output_text.lines().map(String::from)))
    }
}

impl<SpawnerType: Spawner> HostHelper<SpawnerType> {
    fn try_spawn_request(
        &self,
        request_arguments: &[String],
        stream: Stream,
    ) -> Result<Vec<u8>, Error> {
        let elevation_arguments = self.helper_configuration.elevation_arguments();
        let host_program = OsString::from(self.helper_configuration.host_program());
        let mut arguments = Vec::from(elevation_arguments);
        arguments.push(host_program);
        arguments.extend(request_arguments.iter().map(OsString::from));
        let program = OsString::from(self.helper_configuration.elevation_program());
        let invocation = Invocation::from((program, arguments, stream));
        let completion = self
            .spawner
            .try_spawn_invocation(&invocation)
            .map_err(|error| {
                let text = error.to_string();
                Error::StartFailure { text }
            })?;
        match completion {
            Completion::Success { output } => Ok(output),
            Completion::Failure { status, .. } => {
                let request_name = request_arguments.first().cloned().unwrap_or_default();
                Err(Error::Request {
                    request_name,
                    status,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stack_lock_application::acquisition::mode::AcquisitionMode;
    use std::time::Duration;

    mod fixture {
        use super::*;

        pub(crate) fn host_helper_factory(
            completion: Vec<Option<Completion>>,
        ) -> HostHelper<stack_process_adapter_spawner_transcript::Spawner> {
            let helper_configuration = HelperConfiguration::from((
                OsString::from("sudo"),
                vec![OsString::from("-n")],
                OsString::from("/run/current-system/sw/bin/baton_host"),
            ));
            HostHelper::from((
                helper_configuration,
                stack_process_adapter_spawner_transcript::Spawner::from(completion),
            ))
        }

        pub(crate) fn argument_factory(invocation: &Invocation) -> Vec<String> {
            Vec::from_iter(
                invocation
                    .arguments()
                    .iter()
                    .map(|argument| argument.to_string_lossy().into_owned()),
            )
        }
    }

    #[test]
    fn an_acquisition_runs_baton_host_through_the_elevation_program_with_every_option() {
        let host_helper = fixture::host_helper_factory(Vec::new());
        let acquisition_mode = AcquisitionMode::from((
            Some(Duration::from_millis(1500)),
            true,
            true,
        ));
        let acquisition = [
            Acquisition::try_from((
                String::from("rtx3080"),
                String::from("winix"),
                7,
                acquisition_mode,
            ))
            .unwrap(),
            Acquisition::try_from((
                String::from("rtx3080"),
                String::from("resolve"),
                8,
                AcquisitionMode::default(),
            ))
            .unwrap(),
        ];

        host_helper.try_acquire_lock(&acquisition[0]).unwrap();
        host_helper.try_acquire_lock(&acquisition[1]).unwrap();

        let invocations = host_helper.spawner.invocations();
        assert_eq!(invocations[0].program(), "sudo");
        assert_eq!(invocations[0].stream(), Stream::Inheritance);
        assert_eq!(
            fixture::argument_factory(&invocations[0]),
            [
                "-n",
                "/run/current-system/sw/bin/baton_host",
                "acquire",
                "--lock",
                "rtx3080",
                "--application",
                "winix",
                "--process",
                "7",
                "--deadline",
                "1500",
                "--force",
                "--passthrough",
            ]
        );
        assert_eq!(
            fixture::argument_factory(&invocations[1]),
            [
                "-n",
                "/run/current-system/sw/bin/baton_host",
                "acquire",
                "--lock",
                "rtx3080",
                "--application",
                "resolve",
                "--process",
                "8"
            ]
        );
    }

    #[test]
    fn a_release_and_a_status_run_their_requests_and_the_status_answers_its_lines() {
        let completion = vec![
            Some(Completion::Success { output: Vec::new() }),
            Some(Completion::Success {
                output: b"lock rtx3080\n  owner nobody\n".to_vec(),
            }),
        ];
        let host_helper = fixture::host_helper_factory(completion);
        let release = Release::try_from((String::from("rtx3080"), String::from("winix"))).unwrap();

        host_helper.try_release_lock(&release).unwrap();
        let status_texts = host_helper.try_read_status().unwrap();

        assert_eq!(status_texts, ["lock rtx3080", "  owner nobody"]);
        let invocations = host_helper.spawner.invocations();
        assert_eq!(
            fixture::argument_factory(&invocations[0]),
            [
                "-n",
                "/run/current-system/sw/bin/baton_host",
                "release",
                "--lock",
                "rtx3080",
                "--application",
                "winix"
            ]
        );
        assert_eq!(invocations[1].stream(), Stream::Capture);
    }

    #[test]
    fn a_failed_request_and_a_program_that_never_started_are_errors() {
        let completion = vec![
            Some(Completion::Failure {
                status: String::from("exit status: 1"),
                maybe_code: Some(1),
            }),
            None,
        ];
        let host_helper = fixture::host_helper_factory(completion);
        let release = Release::try_from((String::from("rtx3080"), String::from("winix"))).unwrap();

        let error = [
            host_helper.try_release_lock(&release),
            host_helper.try_release_lock(&release),
        ];
        let status_outcome = host_helper.try_read_status();

        assert_eq!(
            error[0],
            Err(Error::Request {
                request_name: String::from("release"),
                status: String::from("exit status: 1"),
            })
        );
        assert!(matches!(error[1], Err(Error::StartFailure { .. })));
        assert_eq!(status_outcome, Ok(Vec::new()));
    }

    #[test]
    fn a_failed_status_is_an_error() {
        let host_helper = fixture::host_helper_factory(vec![None]);

        let status_outcome = host_helper.try_read_status();

        assert!(matches!(status_outcome, Err(Error::StartFailure { .. })));
    }
}
