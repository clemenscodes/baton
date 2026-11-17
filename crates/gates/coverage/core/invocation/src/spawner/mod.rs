use crate::Completion;
use crate::Invocation;
use stack_domain_driven_design_core::port::DrivenPort;
use std::io;

pub trait Spawner: DrivenPort {
    fn try_spawn_invocation(&self, invocation: &Invocation) -> io::Result<Completion>;
}
