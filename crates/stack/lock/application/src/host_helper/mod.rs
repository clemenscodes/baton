use crate::acquisition::Acquisition;
use crate::release::Release;
use stack_domain_driven_design_core::port::DrivenPort;
use std::error;

pub trait HostHelper: DrivenPort {
    type Error: error::Error;

    fn try_acquire_lock(&self, acquisition: &Acquisition) -> Result<(), Self::Error>;

    fn try_release_lock(&self, release: &Release) -> Result<(), Self::Error>;

    fn try_read_status(&self) -> Result<Vec<String>, Self::Error>;
}
