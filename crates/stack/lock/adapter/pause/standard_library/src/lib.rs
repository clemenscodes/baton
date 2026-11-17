use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use stack_lock_application::pause::Pause as PausePort;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Pause;

impl Layer for Pause {
    type Layer = InfrastructureLayer;
}

impl Adapter for Pause {}

impl Port for Pause {}

impl DrivenPort for Pause {}

impl From<()> for Pause {
    fn from(_: ()) -> Self {
        Self
    }
}

impl PausePort for Pause {
    fn wait_duration(&self, duration: Duration) {
        thread::sleep(duration);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Instant;

    #[test]
    fn a_pause_waits_at_least_its_duration() {
        let pause = Pause::from(());
        let duration = Duration::from_millis(20);
        let start = Instant::now();

        pause.wait_duration(duration);

        assert!(start.elapsed() >= duration);
    }
}
