use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct RunnerConfiguration {
    check_interval: Duration,
}

impl Layer for RunnerConfiguration {
    type Layer = InfrastructureLayer;
}

impl RunnerConfiguration {
    pub fn check_interval(&self) -> Duration {
        self.check_interval
    }
}

impl From<Duration> for RunnerConfiguration {
    fn from(check_interval: Duration) -> Self {
        Self { check_interval }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_interval_between_two_checks() {
        let check_interval = Duration::from_millis(200);

        let runner_configuration = RunnerConfiguration::from(check_interval);

        assert_eq!(runner_configuration.check_interval(), check_interval);
    }
}
