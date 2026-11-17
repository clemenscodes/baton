use stack_domain_driven_design_core::port::DrivenPort;
use std::error;

pub trait ControlGroup: DrivenPort {
    type Error: error::Error;

    fn try_create_group(&self, path_text: &str) -> Result<u64, Self::Error>;

    fn try_read_group_identifier(&self, path_text: &str) -> Result<u64, Self::Error>;

    fn try_check_population(&self, path_text: &str) -> Result<bool, Self::Error>;

    fn try_move_process(&self, path_text: &str, process_identifier: u32)
    -> Result<(), Self::Error>;

    fn try_remove_group(&self, path_text: &str) -> Result<(), Self::Error>;
}
