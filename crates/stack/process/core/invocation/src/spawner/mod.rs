use crate::Completion;
use crate::Invocation;
use crate::RunningProgram;
use stack_domain_driven_design_core::port::DrivenPort;
use std::io;

pub trait Spawner: DrivenPort + Send + Sync {
    type RunningProgram: RunningProgram;

    fn try_spawn_invocation(&self, invocation: &Invocation) -> io::Result<Completion>;

    fn try_start_invocation(&self, invocation: &Invocation) -> io::Result<Self::RunningProgram>;
}
