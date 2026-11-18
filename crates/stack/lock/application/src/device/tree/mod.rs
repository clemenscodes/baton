use stack_domain_driven_design_core::port::DrivenPort;
use stack_lock_core_device_address::DeviceAddress;
use stack_lock_core_device_file::DeviceFile;
use stack_lock_core_graphics_user::GraphicsUser;
use std::error;

pub trait DeviceTree: DrivenPort {
    type Error: error::Error;

    fn try_read_device_files(
        &self,
        device_address: &DeviceAddress,
    ) -> Result<Vec<DeviceFile>, Self::Error>;

    fn try_read_console_users(
        &self,
        device_address: &DeviceAddress,
    ) -> Result<Vec<GraphicsUser>, Self::Error>;

    fn try_unbind_console_user(&self, console_user: &GraphicsUser) -> Result<(), Self::Error>;

    fn try_bind_console_user(&self, console_user: &GraphicsUser) -> Result<(), Self::Error>;

    fn check_identity_presence(&self, device_file: &DeviceFile) -> bool;

    fn try_bind_passthrough(&self, device_address: &DeviceAddress) -> Result<bool, Self::Error>;

    fn try_release_passthrough(
        &self,
        device_address: &DeviceAddress,
        persistence_presence: bool,
    ) -> Result<(), Self::Error>;
}
