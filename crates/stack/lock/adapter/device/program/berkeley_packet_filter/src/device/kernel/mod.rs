use std::ffi::CStr;
use std::io;
use std::os::fd::BorrowedFd;
use std::os::fd::OwnedFd;
use std::path::Path;

pub trait DeviceKernel {
    fn try_create_hash_map(
        &self,
        key_size: u32,
        value_size: u32,
        entry_limit: u32,
    ) -> io::Result<OwnedFd>;

    fn try_update_element(&self, map: BorrowedFd<'_>, key: &[u8], value: &[u8]) -> io::Result<()>;

    fn try_delete_element(&self, map: BorrowedFd<'_>, key: &[u8]) -> io::Result<()>;

    fn try_load_device_program(&self, instructions: &[u8], license: &CStr) -> io::Result<OwnedFd>;

    fn try_fasten_object(&self, object: BorrowedFd<'_>, path: &Path) -> io::Result<()>;

    fn try_open_object(&self, path: &Path) -> io::Result<OwnedFd>;

    fn try_attach_device_program(
        &self,
        cgroup: BorrowedFd<'_>,
        program: BorrowedFd<'_>,
    ) -> io::Result<()>;

    fn try_detach_device_program(
        &self,
        cgroup: BorrowedFd<'_>,
        program: BorrowedFd<'_>,
    ) -> io::Result<()>;
}
