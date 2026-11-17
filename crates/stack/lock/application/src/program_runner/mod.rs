use stack_domain_driven_design_core::port::DrivenPort;
use std::error;

pub trait ProgramRunner: DrivenPort {
    type Error: error::Error;

    fn guard_identifier(&self) -> u32;

    fn try_spawn_program(&self, program_texts: &[String]) -> Result<i32, Self::Error>;
}
