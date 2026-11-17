use stack_domain_driven_design_core::port::DrivenPort;
use stack_lock_core_graphics_lock::GraphicsLock;
use stack_lock_core_name::LockName;
use std::error;

pub trait LockRecord: DrivenPort {
    type Error: error::Error;
    type RecordLease;

    fn try_lease_records(&self) -> Result<Self::RecordLease, Self::Error>;

    fn try_load_lock(&self, lock_name: &LockName) -> Result<Option<GraphicsLock>, Self::Error>;

    fn try_load_locks(&self) -> Result<Vec<GraphicsLock>, Self::Error>;

    fn try_store_lock(&self, graphics_lock: &GraphicsLock) -> Result<(), Self::Error>;

    fn try_delete_lock(&self, lock_name: &LockName) -> Result<(), Self::Error>;
}
