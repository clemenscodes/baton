use crate::arguments::Arguments;
use stack_domain_driven_design_core::layer::{Layer, PresentationLayer};
use stack_lock_adapter_control_group_unified_hierarchy::{ControlGroup, GroupConfiguration};
use stack_lock_adapter_device_program_berkeley_packet_filter::{
    DeviceProgram, ProgramConfiguration, SystemKernel,
};
use stack_lock_adapter_device_tree_system_file_system::{DeviceTree, TreeConfiguration};
use stack_lock_adapter_lock_record_file_system::{LockRecord, RecordConfiguration};
use stack_lock_adapter_pause_standard_library::Pause;
use stack_lock_adapter_task_table_process_file_system::{
    SystemSignal, TaskConfiguration, TaskTable,
};
use stack_lock_application::lock::ports::LockPorts;
use stack_process_adapter_spawner_standard_library::Spawner;
use std::ffi::OsString;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) struct HostAdapters {
    device_program: DeviceProgram<SystemKernel>,
    device_tree: DeviceTree<Spawner>,
    control_group: ControlGroup,
    lock_record: LockRecord,
    task_table: TaskTable<SystemSignal>,
    pause: Pause,
}

impl Layer for HostAdapters {
    type Layer = PresentationLayer;
}

impl From<&Arguments> for HostAdapters {
    fn from(arguments: &Arguments) -> Self {
        let hierarchy_directory = PathBuf::from(arguments.hierarchy_directory());
        let system_directory = PathBuf::from(arguments.system_directory());
        let program_configuration = ProgramConfiguration::from((
            hierarchy_directory.clone(),
            PathBuf::from(arguments.pin_directory()),
            arguments.ancestry_depth(),
        ));
        let device_program = DeviceProgram::from((program_configuration, SystemKernel));
        let check_interval = Duration::from_millis(arguments.check_interval());
        let binding_deadline = Duration::from_millis(arguments.binding_deadline());
        let tree_configuration = TreeConfiguration::from((
            system_directory.clone(),
            (
                OsString::from(arguments.console_program()),
                OsString::from(arguments.persistence_program()),
            ),
            (binding_deadline, check_interval),
        ));
        let device_tree = DeviceTree::from((tree_configuration, Spawner::from(())));
        let control_group = ControlGroup::from(GroupConfiguration::from(hierarchy_directory));
        let record_configuration = RecordConfiguration::from(PathBuf::from(
            arguments.record_directory(),
        ));
        let lock_record = LockRecord::from(record_configuration);
        let task_configuration = TaskConfiguration::from(system_directory.join("proc"));
        let task_table = TaskTable::from((task_configuration, SystemSignal));
        let pause = Pause::from(());
        Self {
            device_program,
            device_tree,
            control_group,
            lock_record,
            task_table,
            pause,
        }
    }
}

impl LockPorts for HostAdapters {
    type DeviceProgram = DeviceProgram<SystemKernel>;
    type DeviceTree = DeviceTree<Spawner>;
    type ControlGroup = ControlGroup;
    type LockRecord = LockRecord;
    type TaskTable = TaskTable<SystemSignal>;
    type Pause = Pause;

    fn device_program(&self) -> &Self::DeviceProgram {
        &self.device_program
    }

    fn device_tree(&self) -> &Self::DeviceTree {
        &self.device_tree
    }

    fn control_group(&self) -> &Self::ControlGroup {
        &self.control_group
    }

    fn lock_record(&self) -> &Self::LockRecord {
        &self.lock_record
    }

    fn task_table(&self) -> &Self::TaskTable {
        &self.task_table
    }

    fn pause(&self) -> &Self::Pause {
        &self.pause
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::Parser;

    #[test]
    fn the_adapters_follow_the_directories_of_the_arguments() {
        let arguments = Arguments::try_parse_from([
            "baton_host",
            "--hierarchy-directory",
            "/tmp/hierarchy",
            "--system-directory",
            "/tmp/system",
            "status",
        ])
        .unwrap();

        let host_adapters = HostAdapters::from(&arguments);

        let group_configuration = GroupConfiguration::from(PathBuf::from("/tmp/hierarchy"));
        let task_configuration = TaskConfiguration::from(PathBuf::from("/tmp/system/proc"));
        let record_configuration = RecordConfiguration::from(PathBuf::from("/run/baton"));
        assert_eq!(
            host_adapters.control_group(),
            &ControlGroup::from(group_configuration)
        );
        assert_eq!(
            host_adapters.task_table(),
            &TaskTable::from((task_configuration, SystemSignal))
        );
        assert_eq!(
            host_adapters.lock_record(),
            &LockRecord::from(record_configuration)
        );
        assert_eq!(host_adapters.pause(), &Pause::from(()));
        assert_eq!(
            host_adapters.device_program(),
            &host_adapters.device_program
        );
        assert_eq!(host_adapters.device_tree(), &host_adapters.device_tree);
    }
}
