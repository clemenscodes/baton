use crate::control_group::ControlGroup;
use crate::device::program::DeviceProgram;
use crate::device::tree::DeviceTree;
use crate::lock::record::LockRecord;
use crate::pause::Pause;
use crate::task_table::TaskTable;

pub trait LockPorts {
    type DeviceProgram: DeviceProgram;
    type DeviceTree: DeviceTree;
    type ControlGroup: ControlGroup;
    type LockRecord: LockRecord;
    type TaskTable: TaskTable;
    type Pause: Pause;

    fn device_program(&self) -> &Self::DeviceProgram;

    fn device_tree(&self) -> &Self::DeviceTree;

    fn control_group(&self) -> &Self::ControlGroup;

    fn lock_record(&self) -> &Self::LockRecord;

    fn task_table(&self) -> &Self::TaskTable;

    fn pause(&self) -> &Self::Pause;
}
