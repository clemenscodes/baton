use crate::device::kernel::DeviceKernel;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::ffi::CStr;
use std::io;
use std::os::fd::BorrowedFd;
use std::os::fd::OwnedFd;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct SystemKernel;

impl Layer for SystemKernel {
    type Layer = InfrastructureLayer;
}

impl DeviceKernel for SystemKernel {
    fn try_create_hash_map(
        &self,
        key_size: u32,
        value_size: u32,
        entry_limit: u32,
    ) -> io::Result<OwnedFd> {
        berkeley_packet_filter::create_hash_map(key_size, value_size, entry_limit)
    }

    fn try_update_element(&self, map: BorrowedFd<'_>, key: &[u8], value: &[u8]) -> io::Result<()> {
        berkeley_packet_filter::update_element(map, key, value)
    }

    fn try_delete_element(&self, map: BorrowedFd<'_>, key: &[u8]) -> io::Result<()> {
        berkeley_packet_filter::delete_element(map, key)
    }

    fn try_load_device_program(&self, instructions: &[u8], license: &CStr) -> io::Result<OwnedFd> {
        berkeley_packet_filter::load_device_program(instructions, license)
    }

    fn try_fasten_object(&self, object: BorrowedFd<'_>, path: &Path) -> io::Result<()> {
        berkeley_packet_filter::pin_object(object, path)
    }

    fn try_open_object(&self, path: &Path) -> io::Result<OwnedFd> {
        berkeley_packet_filter::open_object(path)
    }

    fn try_attach_device_program(
        &self,
        cgroup: BorrowedFd<'_>,
        program: BorrowedFd<'_>,
    ) -> io::Result<()> {
        berkeley_packet_filter::attach_device_program(cgroup, program, true)
    }

    fn try_detach_device_program(
        &self,
        cgroup: BorrowedFd<'_>,
        program: BorrowedFd<'_>,
    ) -> io::Result<()> {
        berkeley_packet_filter::detach_device_program(cgroup, program)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::fd::AsFd;

    #[test]
    fn a_creation_of_a_map_of_zero_sizes_is_an_error() {
        let system_kernel = SystemKernel;

        let outcome = system_kernel.try_create_hash_map(0, 0, 0);

        assert!(outcome.is_err());
    }

    #[test]
    fn an_update_of_a_descriptor_that_is_no_map_is_an_error() {
        let system_kernel = SystemKernel;
        let object = io::stdin();

        let outcome = system_kernel.try_update_element(object.as_fd(), &[], &[]);

        assert!(outcome.is_err());
    }

    #[test]
    fn a_deletion_of_a_descriptor_that_is_no_map_is_an_error() {
        let system_kernel = SystemKernel;
        let object = io::stdin();

        let outcome = system_kernel.try_delete_element(object.as_fd(), &[]);

        assert!(outcome.is_err());
    }

    #[test]
    fn a_load_of_an_empty_program_is_an_error() {
        let system_kernel = SystemKernel;
        let instructions = [0; 8];

        let outcome = system_kernel.try_load_device_program(&instructions, c"GPL");

        assert!(outcome.is_err());
    }

    #[test]
    fn a_fastening_of_a_descriptor_that_is_no_object_is_an_error() {
        let system_kernel = SystemKernel;
        let object = io::stdin();
        let path = Path::new("/sys/fs/bpf/berkeley_packet_filter_adapter_probe");

        let outcome = system_kernel.try_fasten_object(object.as_fd(), path);

        assert!(outcome.is_err());
    }

    #[test]
    fn an_opening_of_an_absent_object_is_an_error() {
        let system_kernel = SystemKernel;
        let path = Path::new("/sys/fs/bpf/berkeley_packet_filter_adapter_absent");

        let outcome = system_kernel.try_open_object(path);

        assert!(outcome.is_err());
    }

    #[test]
    fn an_attachment_of_descriptors_that_are_no_cgroup_is_an_error() {
        let system_kernel = SystemKernel;
        let object = io::stdin();

        let outcome = system_kernel.try_attach_device_program(object.as_fd(), object.as_fd());

        assert!(outcome.is_err());
    }

    #[test]
    fn a_detachment_of_descriptors_that_are_no_cgroup_is_an_error() {
        let system_kernel = SystemKernel;
        let object = io::stdin();

        let outcome = system_kernel.try_detach_device_program(object.as_fd(), object.as_fd());

        assert!(outcome.is_err());
    }
}
