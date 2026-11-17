use stack_domain_driven_design_core::port::DrivenPort;
use stack_lock_core_device_file::DeviceFile;
use stack_lock_core_graphics_user::GraphicsUser;
use std::error;

pub trait TaskTable: DrivenPort {
    type Error: error::Error;

    fn try_scan_process_users(
        &self,
        device_files: &[DeviceFile],
    ) -> Result<Vec<GraphicsUser>, Self::Error>;

    fn check_process_presence(&self, process_identifier: u32) -> bool;

    fn try_request_end(&self, process_identifier: u32) -> Result<(), Self::Error>;

    fn try_force_end(&self, process_identifier: u32) -> Result<(), Self::Error>;
}
