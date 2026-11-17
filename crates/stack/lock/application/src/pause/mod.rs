use stack_domain_driven_design_core::port::DrivenPort;
use std::time::Duration;

pub trait Pause: DrivenPort {
    fn wait_duration(&self, duration: Duration);
}
