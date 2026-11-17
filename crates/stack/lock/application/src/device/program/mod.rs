use stack_domain_driven_design_core::port::DrivenPort;
use stack_lock_core_device_file::DeviceFile;
use std::error;

pub trait DeviceProgram: DrivenPort {
    type Error: error::Error;

    fn try_install_program(&self) -> Result<(), Self::Error>;

    fn try_insert_device_files(
        &self,
        map_index: u32,
        device_files: &[DeviceFile],
    ) -> Result<(), Self::Error>;

    fn try_delete_device_files(&self, device_files: &[DeviceFile]) -> Result<(), Self::Error>;

    fn try_permit_group(&self, map_index: u32, group_identifier: u64) -> Result<(), Self::Error>;

    fn try_revoke_group(&self, map_index: u32, group_identifier: u64) -> Result<(), Self::Error>;

    fn try_remove_program(&self) -> Result<(), Self::Error>;
}
