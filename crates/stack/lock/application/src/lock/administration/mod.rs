mod step;

use crate::Error;
use crate::acquisition::Acquisition;
use crate::admission::Admission;
use crate::control_group::ControlGroup;
use crate::device::program::DeviceProgram;
use crate::device::tree::DeviceTree;
use crate::dismissal::Dismissal;
use crate::enclosure::Enclosure;
use crate::lock::administration::step::AcquisitionStep;
use crate::lock::ports::LockPorts;
use crate::lock::record::LockRecord;
use crate::lock::setting::LockSetting;
use crate::pause::Pause;
use crate::refresh::Refresh;
use crate::release::Release;
use crate::removal::Removal;
use crate::status_query::StatusQuery;
use crate::task_table::TaskTable;
use crate::user_query::UserQuery;
use stack_domain_driven_design_core::application_service::ApplicationService;
use stack_domain_driven_design_core::command::SynchronousCommandHandler;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_domain_driven_design_core::query::SynchronousQueryHandler;
use stack_lock_core_demand::GraphicsDemand;
use stack_lock_core_device_file::DeviceFile;
use stack_lock_core_graphics_lock::GraphicsLock;
use stack_lock_core_graphics_user::GraphicsUser;
use stack_lock_core_name::{ApplicationName, LockName};
use std::fmt;
use std::time::Duration;

type RecordLease<PortsType> = <<PortsType as LockPorts>::LockRecord as LockRecord>::RecordLease;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LockAdministration<PortsType> {
    lock_ports: PortsType,
    lock_setting: LockSetting,
}

impl<PortsType> Layer for LockAdministration<PortsType> {
    type Layer = ApplicationLayer;
}

impl<PortsType> ApplicationService for LockAdministration<PortsType> {}

impl<PortsType> From<(PortsType, LockSetting)> for LockAdministration<PortsType> {
    fn from((lock_ports, lock_setting): (PortsType, LockSetting)) -> Self {
        Self {
            lock_ports,
            lock_setting,
        }
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Enclosure> for LockAdministration<PortsType> {
    type Outcome = Result<Vec<String>, Error>;

    fn dispatch_command(&self, enclosure: Enclosure) -> Self::Outcome {
        let _record_lease = self.try_lease_records()?;
        self.lock_ports
            .device_program()
            .try_install_program()
            .map_err(Self::compose_program_error)?;
        let graphics_locks = self
            .lock_ports
            .lock_record()
            .try_load_locks()
            .map_err(Self::compose_record_error)?;
        let maybe_graphics_lock = graphics_locks
            .iter()
            .find(|graphics_lock| graphics_lock.lock_name() == enclosure.lock_name())
            .cloned();
        let map_index = graphics_locks
            .iter()
            .map(GraphicsLock::map_index)
            .max()
            .map_or(0, |map_index| map_index + 1);
        let lock_name = enclosure.lock_name().clone();
        let mut graphics_lock = maybe_graphics_lock
            .unwrap_or_else(|| GraphicsLock::from((lock_name, map_index)));
        graphics_lock.try_add_device(enclosure.device_address())?;
        self.try_permit_host(&graphics_lock)?;
        self.try_update_device_files(&mut graphics_lock)?;
        self.try_store_lock(&graphics_lock)?;
        Ok(Self::compose_texts(graphics_lock.device_files()))
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Admission> for LockAdministration<PortsType> {
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, admission: Admission) -> Self::Outcome {
        let _record_lease = self.try_lease_records()?;
        let mut graphics_lock = self.try_load_lock(admission.lock_name())?;
        let application_name = admission.application_name();
        graphics_lock.try_admit(application_name.clone(), admission.default_presence())?;
        if admission.default_presence() {
            graphics_lock.record_yield_presence(admission.yield_presence());
        }
        let path_text = graphics_lock.application_path_text(application_name);
        self.lock_ports
            .control_group()
            .try_create_group(&path_text)
            .map_err(Self::compose_group_error)?;
        if graphics_lock.maybe_permitted_application() == Some(application_name) {
            self.try_permit_application(&graphics_lock, application_name)?;
        }
        self.try_store_lock(&graphics_lock)
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Dismissal> for LockAdministration<PortsType> {
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, dismissal: Dismissal) -> Self::Outcome {
        let _record_lease = self.try_lease_records()?;
        let mut graphics_lock = self.try_load_lock(dismissal.lock_name())?;
        let application_name = dismissal.application_name();
        let permission_presence = graphics_lock.maybe_permitted_application()
            == Some(application_name);
        graphics_lock.try_dismiss(application_name)?;
        if permission_presence {
            self.try_revoke_application(&graphics_lock, application_name)?;
        }
        let path_text = graphics_lock.application_path_text(application_name);
        self.lock_ports
            .control_group()
            .try_remove_group(&path_text)
            .map_err(Self::compose_group_error)?;
        self.try_store_lock(&graphics_lock)
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Acquisition>
    for LockAdministration<PortsType>
{
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, acquisition: Acquisition) -> Self::Outcome {
        let application_name = acquisition.application_name().clone();
        let waiter_identifier = acquisition.waiter_identifier();
        let graphics_demand = GraphicsDemand::from((application_name, waiter_identifier));
        {
            let _record_lease = self.try_lease_records()?;
            let mut graphics_lock = self.try_load_lock(acquisition.lock_name())?;
            graphics_lock.try_record_demand(graphics_demand.clone())?;
            self.try_store_lock(&graphics_lock)?;
        }
        let check_interval = self.lock_setting.check_interval();
        let acquisition_mode = acquisition.acquisition_mode();
        let mut wait_duration = Duration::ZERO;
        loop {
            let acquisition_step = self.try_advance_acquisition(&acquisition, &graphics_demand)?;
            let user_text = match acquisition_step {
                AcquisitionStep::Ownership => return Ok(()),
                AcquisitionStep::Displacement { process_users } => {
                    self.try_end_process_users(acquisition.lock_name(), &process_users)?;
                    continue;
                }
                AcquisitionStep::Wait { user_text } => user_text,
            };
            if let Some(deadline) = acquisition_mode.maybe_deadline()
                && wait_duration >= deadline
            {
                let _record_lease = self.try_lease_records()?;
                let mut graphics_lock = self.try_load_lock(acquisition.lock_name())?;
                graphics_lock.drop_demand(waiter_identifier);
                self.try_store_lock(&graphics_lock)?;
                let lock_text = acquisition.lock_name().to_string();
                return Err(Error::DeadlineExpiry {
                    lock_text,
                    user_text,
                });
            }
            self.lock_ports.pause().wait_duration(check_interval);
            wait_duration += check_interval;
        }
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Release> for LockAdministration<PortsType> {
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, release: Release) -> Self::Outcome {
        let _record_lease = self.try_lease_records()?;
        let mut graphics_lock = self.try_load_lock(release.lock_name())?;
        let application_name = release.application_name();
        graphics_lock.try_release(application_name)?;
        self.try_restore_lock(&mut graphics_lock, application_name)?;
        self.try_store_lock(&graphics_lock)
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Refresh> for LockAdministration<PortsType> {
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, _: Refresh) -> Self::Outcome {
        let _record_lease = self.try_lease_records()?;
        let graphics_locks = self
            .lock_ports
            .lock_record()
            .try_load_locks()
            .map_err(Self::compose_record_error)?;
        for mut graphics_lock in graphics_locks {
            self.try_update_device_files(&mut graphics_lock)?;
            self.try_store_lock(&graphics_lock)?;
        }
        Ok(())
    }
}

impl<PortsType: LockPorts> SynchronousCommandHandler<Removal> for LockAdministration<PortsType> {
    type Outcome = Result<(), Error>;

    fn dispatch_command(&self, removal: Removal) -> Self::Outcome {
        let _record_lease = self.try_lease_records()?;
        let mut graphics_lock = self.try_load_lock(removal.lock_name())?;
        self.try_settle_lock(&mut graphics_lock)?;
        if let Some(owner_name) = graphics_lock.maybe_owner() {
            let lock_text = graphics_lock.lock_name().to_string();
            let owner_text = owner_name.to_string();
            return Err(Error::OwnerPresence {
                lock_text,
                owner_text,
            });
        }
        if let Some(default_owner) = graphics_lock.maybe_default_owner() {
            self.try_revoke_application(&graphics_lock, default_owner)?;
        }
        self.try_revoke_host(&graphics_lock)?;
        let device_program = self.lock_ports.device_program();
        device_program
            .try_delete_device_files(graphics_lock.device_files())
            .map_err(Self::compose_program_error)?;
        let control_group = self.lock_ports.control_group();
        for application_name in graphics_lock.application_names() {
            let path_text = graphics_lock.application_path_text(application_name);
            control_group
                .try_remove_group(&path_text)
                .map_err(Self::compose_group_error)?;
        }
        control_group
            .try_remove_group(&graphics_lock.tree_path_text())
            .map_err(Self::compose_group_error)?;
        let lock_record = self.lock_ports.lock_record();
        lock_record
            .try_delete_lock(graphics_lock.lock_name())
            .map_err(Self::compose_record_error)?;
        let graphics_locks = lock_record
            .try_load_locks()
            .map_err(Self::compose_record_error)?;
        if graphics_locks.is_empty() {
            device_program
                .try_remove_program()
                .map_err(Self::compose_program_error)?;
        }
        Ok(())
    }
}

impl<PortsType: LockPorts> SynchronousQueryHandler<UserQuery> for LockAdministration<PortsType> {
    fn resolve_query(&self, user_query: UserQuery) -> Result<Vec<String>, Error> {
        let _record_lease = self.try_lease_records()?;
        let graphics_lock = self.try_load_lock(user_query.lock_name())?;
        let graphics_users = self.try_scan_users(&graphics_lock, None)?;
        Ok(Self::compose_texts(&graphics_users))
    }
}

impl<PortsType: LockPorts> SynchronousQueryHandler<StatusQuery> for LockAdministration<PortsType> {
    fn resolve_query(&self, _: StatusQuery) -> Result<Vec<String>, Error> {
        let _record_lease = self.try_lease_records()?;
        let graphics_locks = self
            .lock_ports
            .lock_record()
            .try_load_locks()
            .map_err(Self::compose_record_error)?;
        let mut status_texts = Vec::new();
        for graphics_lock in graphics_locks {
            status_texts.extend(Self::compose_status_texts(&graphics_lock));
        }
        Ok(status_texts)
    }
}

impl<PortsType: LockPorts> LockAdministration<PortsType> {
    fn try_advance_acquisition(
        &self,
        acquisition: &Acquisition,
        graphics_demand: &GraphicsDemand,
    ) -> Result<AcquisitionStep, Error> {
        let _record_lease = self.try_lease_records()?;
        let mut graphics_lock = self.try_load_lock(acquisition.lock_name())?;
        self.try_settle_lock(&mut graphics_lock)?;
        graphics_lock.try_record_demand(graphics_demand.clone())?;
        let acquisition_mode = acquisition.acquisition_mode();
        let force_presence = acquisition_mode.force_presence();
        let application_name = acquisition.application_name();
        let turn_presence = graphics_lock.maybe_first_demand() == Some(graphics_demand);
        if !force_presence && !turn_presence {
            self.try_store_lock(&graphics_lock)?;
            let user_text = Self::compose_demand_text(&graphics_lock);
            return Ok(AcquisitionStep::Wait { user_text });
        }
        if !force_presence
            && let Some(owner_name) = graphics_lock.maybe_owner()
            && owner_name != application_name
            && graphics_lock.maybe_yielding_owner() != Some(owner_name)
        {
            let user_text = owner_name.to_string();
            self.try_store_lock(&graphics_lock)?;
            return Ok(AcquisitionStep::Wait { user_text });
        }
        let graphics_users = self.try_scan_users(&graphics_lock, Some(application_name))?;
        let process_users = Vec::from_iter(
            graphics_users
                .iter()
                .filter(|graphics_user| graphics_user.maybe_process_identifier().is_some())
                .cloned(),
        );
        let yielding_users = match graphics_lock.maybe_yielding_owner() {
            Some(yielding_owner) if yielding_owner != application_name => {
                let path_text = graphics_lock.application_path_text(yielding_owner);
                Vec::from_iter(
                    process_users
                        .iter()
                        .filter(|process_user| process_user.check_membership(&path_text))
                        .cloned(),
                )
            }
            _ => Vec::new(),
        };
        let foreign_users = Vec::from_iter(
            process_users
                .iter()
                .filter(|process_user| !yielding_users.contains(process_user))
                .cloned(),
        );
        if !force_presence && !foreign_users.is_empty() {
            self.try_store_lock(&graphics_lock)?;
            let user_text = Self::compose_texts(&foreign_users).join(", ");
            return Ok(AcquisitionStep::Wait { user_text });
        }
        let process_users = match force_presence {
            true => process_users,
            false => yielding_users,
        };
        if !process_users.is_empty() {
            if let Some(permitted_application) = graphics_lock.maybe_permitted_application()
                && permitted_application != application_name
            {
                self.try_revoke_application(&graphics_lock, permitted_application)?;
            }
            self.try_store_lock(&graphics_lock)?;
            return Ok(AcquisitionStep::Displacement { process_users });
        }
        if let Some(permitted_application) = graphics_lock.maybe_permitted_application()
            && permitted_application != application_name
        {
            self.try_revoke_application(&graphics_lock, permitted_application)?;
        }
        let device_tree = self.lock_ports.device_tree();
        let console_users = Vec::from_iter(
            graphics_users
                .into_iter()
                .filter(|graphics_user| graphics_user.maybe_process_identifier().is_none()),
        );
        for console_user in &console_users {
            device_tree
                .try_unbind_console_user(console_user)
                .map_err(Self::compose_tree_error)?;
        }
        graphics_lock.record_console_users(console_users);
        graphics_lock.assign_demand_owner(graphics_demand);
        self.try_permit_application(&graphics_lock, application_name)?;
        let path_text = graphics_lock.application_path_text(application_name);
        self.lock_ports
            .control_group()
            .try_move_process(&path_text, graphics_demand.waiter_identifier())
            .map_err(Self::compose_group_error)?;
        if acquisition_mode.passthrough_presence() && !graphics_lock.passthrough_presence() {
            self.try_enter_host()?;
            let mut persistence_presence = false;
            for device_address in graphics_lock.device_addresses() {
                let device_persistence = device_tree
                    .try_bind_passthrough(device_address)
                    .map_err(Self::compose_tree_error)?;
                persistence_presence = persistence_presence || device_persistence;
            }
            graphics_lock.record_passthrough(persistence_presence);
            self.try_extend_device_files(&mut graphics_lock)?;
        }
        self.try_store_lock(&graphics_lock)?;
        Ok(AcquisitionStep::Ownership)
    }

    fn try_end_process_users(
        &self,
        lock_name: &LockName,
        process_users: &[GraphicsUser],
    ) -> Result<(), Error> {
        let task_table = self.lock_ports.task_table();
        let process_identifiers = Vec::from_iter(
            process_users
                .iter()
                .filter_map(GraphicsUser::maybe_process_identifier),
        );
        for process_identifier in &process_identifiers {
            task_table
                .try_request_end(*process_identifier)
                .map_err(Self::compose_task_error)?;
        }
        let check_interval = self.lock_setting.check_interval();
        let end_deadline = self.lock_setting.end_deadline();
        let mut wait_duration = Duration::ZERO;
        let check_survival = |process_identifier: &&u32| {
            task_table.check_process_presence(**process_identifier)
        };
        while wait_duration < end_deadline
            && process_identifiers
                .iter()
                .any(|process_identifier| check_survival(&process_identifier))
        {
            self.lock_ports.pause().wait_duration(check_interval);
            wait_duration += check_interval;
        }
        for process_identifier in process_identifiers.iter().filter(check_survival) {
            task_table
                .try_force_end(*process_identifier)
                .map_err(Self::compose_task_error)?;
        }
        self.lock_ports.pause().wait_duration(check_interval);
        let survivor_users = Vec::from_iter(process_users.iter().filter(|process_user| {
            process_user
                .maybe_process_identifier()
                .is_some_and(|process_identifier| check_survival(&&process_identifier))
        }));
        if survivor_users.is_empty() {
            return Ok(());
        }
        let lock_text = lock_name.to_string();
        let user_text = Self::compose_texts(&survivor_users).join(", ");
        Err(Error::UserSurvival {
            lock_text,
            user_text,
        })
    }

    fn try_settle_lock(&self, graphics_lock: &mut GraphicsLock) -> Result<(), Error> {
        let task_table = self.lock_ports.task_table();
        graphics_lock.retain_demands(|waiter_identifier| {
            task_table.check_process_presence(waiter_identifier)
        });
        let Some(owner_name) = graphics_lock.maybe_owner().cloned() else {
            return Ok(());
        };
        let path_text = graphics_lock.application_path_text(&owner_name);
        let population_presence = self
            .lock_ports
            .control_group()
            .try_check_population(&path_text)
            .map_err(Self::compose_group_error)?;
        if population_presence {
            return Ok(());
        }
        graphics_lock.clear_owner();
        self.try_restore_lock(graphics_lock, &owner_name)
    }

    fn try_restore_lock(
        &self,
        graphics_lock: &mut GraphicsLock,
        former_owner: &ApplicationName,
    ) -> Result<(), Error> {
        let device_tree = self.lock_ports.device_tree();
        if graphics_lock.passthrough_presence() {
            self.try_enter_host()?;
            let persistence_presence = graphics_lock.clear_passthrough();
            for device_address in graphics_lock.device_addresses() {
                device_tree
                    .try_release_passthrough(device_address, persistence_presence)
                    .map_err(Self::compose_tree_error)?;
            }
            self.try_extend_device_files(graphics_lock)?;
        }
        for console_user in graphics_lock.remove_console_users() {
            device_tree
                .try_bind_console_user(&console_user)
                .map_err(Self::compose_tree_error)?;
        }
        self.try_revoke_application(graphics_lock, former_owner)?;
        if let Some(default_owner) = graphics_lock.maybe_default_owner() {
            self.try_permit_application(graphics_lock, default_owner)?;
        }
        Ok(())
    }

    fn try_scan_users(
        &self,
        graphics_lock: &GraphicsLock,
        maybe_application_name: Option<&ApplicationName>,
    ) -> Result<Vec<GraphicsUser>, Error> {
        let mut graphics_users = self
            .lock_ports
            .task_table()
            .try_scan_process_users(graphics_lock.device_files())
            .map_err(Self::compose_task_error)?;
        if let Some(application_name) = maybe_application_name {
            let path_text = graphics_lock.application_path_text(application_name);
            graphics_users.retain(|graphics_user| !graphics_user.check_membership(&path_text));
        }
        let device_tree = self.lock_ports.device_tree();
        for device_address in graphics_lock.device_addresses() {
            let console_users = device_tree
                .try_read_console_users(device_address)
                .map_err(Self::compose_tree_error)?;
            graphics_users.extend(console_users);
        }
        Ok(graphics_users)
    }

    fn try_update_device_files(&self, graphics_lock: &mut GraphicsLock) -> Result<(), Error> {
        let mut device_files = self.try_scan_device_files(graphics_lock)?;
        let device_tree = self.lock_ports.device_tree();
        let mut stale_files = Vec::new();
        for device_file in graphics_lock.device_files() {
            if device_files.contains(device_file) {
                continue;
            }
            match device_tree.check_identity_presence(device_file) {
                true => stale_files.push(*device_file),
                false => device_files.push(*device_file),
            }
        }
        let device_program = self.lock_ports.device_program();
        device_program
            .try_insert_device_files(graphics_lock.map_index(), &device_files)
            .map_err(Self::compose_program_error)?;
        graphics_lock.replace_device_files(device_files);
        device_program
            .try_delete_device_files(&stale_files)
            .map_err(Self::compose_program_error)
    }

    fn try_extend_device_files(&self, graphics_lock: &mut GraphicsLock) -> Result<(), Error> {
        let mut device_files = Vec::from(graphics_lock.device_files());
        for device_file in self.try_scan_device_files(graphics_lock)? {
            if !device_files.contains(&device_file) {
                device_files.push(device_file);
            }
        }
        self.lock_ports
            .device_program()
            .try_insert_device_files(graphics_lock.map_index(), &device_files)
            .map_err(Self::compose_program_error)?;
        graphics_lock.replace_device_files(device_files);
        Ok(())
    }

    fn try_scan_device_files(
        &self,
        graphics_lock: &GraphicsLock,
    ) -> Result<Vec<DeviceFile>, Error> {
        let device_tree = self.lock_ports.device_tree();
        let mut device_files: Vec<DeviceFile> = Vec::new();
        for device_address in graphics_lock.device_addresses() {
            let address_files = device_tree
                .try_read_device_files(device_address)
                .map_err(Self::compose_tree_error)?;
            device_files.extend(address_files);
        }
        Ok(device_files)
    }

    fn try_permit_application(
        &self,
        graphics_lock: &GraphicsLock,
        application_name: &ApplicationName,
    ) -> Result<(), Error> {
        let group_identifier = self.try_read_group_identifier(graphics_lock, application_name)?;
        self.lock_ports
            .device_program()
            .try_permit_group(graphics_lock.map_index(), group_identifier)
            .map_err(Self::compose_program_error)
    }

    fn try_revoke_application(
        &self,
        graphics_lock: &GraphicsLock,
        application_name: &ApplicationName,
    ) -> Result<(), Error> {
        let group_identifier = self.try_read_group_identifier(graphics_lock, application_name)?;
        self.lock_ports
            .device_program()
            .try_revoke_group(graphics_lock.map_index(), group_identifier)
            .map_err(Self::compose_program_error)
    }

    fn try_permit_host(&self, graphics_lock: &GraphicsLock) -> Result<(), Error> {
        let host_identifier = self
            .lock_ports
            .control_group()
            .try_create_group(&GraphicsLock::host_path_text())
            .map_err(Self::compose_group_error)?;
        self.lock_ports
            .device_program()
            .try_permit_group(graphics_lock.map_index(), host_identifier)
            .map_err(Self::compose_program_error)
    }

    fn try_revoke_host(&self, graphics_lock: &GraphicsLock) -> Result<(), Error> {
        let host_identifier = self
            .lock_ports
            .control_group()
            .try_create_group(&GraphicsLock::host_path_text())
            .map_err(Self::compose_group_error)?;
        self.lock_ports
            .device_program()
            .try_revoke_group(graphics_lock.map_index(), host_identifier)
            .map_err(Self::compose_program_error)
    }

    fn try_enter_host(&self) -> Result<(), Error> {
        self.lock_ports
            .control_group()
            .try_enter_group(&GraphicsLock::host_path_text())
            .map_err(Self::compose_group_error)
    }

    fn try_read_group_identifier(
        &self,
        graphics_lock: &GraphicsLock,
        application_name: &ApplicationName,
    ) -> Result<u64, Error> {
        let path_text = graphics_lock.application_path_text(application_name);
        self.lock_ports
            .control_group()
            .try_read_group_identifier(&path_text)
            .map_err(Self::compose_group_error)
    }

    fn try_lease_records(&self) -> Result<RecordLease<PortsType>, Error> {
        self.lock_ports
            .lock_record()
            .try_lease_records()
            .map_err(Self::compose_record_error)
    }

    fn try_load_lock(&self, lock_name: &LockName) -> Result<GraphicsLock, Error> {
        let maybe_graphics_lock = self
            .lock_ports
            .lock_record()
            .try_load_lock(lock_name)
            .map_err(Self::compose_record_error)?;
        maybe_graphics_lock.ok_or_else(|| {
            let lock_text = lock_name.to_string();
            Error::LockAbsence { lock_text }
        })
    }

    fn try_store_lock(&self, graphics_lock: &GraphicsLock) -> Result<(), Error> {
        self.lock_ports
            .lock_record()
            .try_store_lock(graphics_lock)
            .map_err(Self::compose_record_error)
    }

    fn compose_demand_text(graphics_lock: &GraphicsLock) -> String {
        let owner_text = graphics_lock
            .maybe_owner()
            .map(|owner_name| format!("the owner {owner_name} and "))
            .unwrap_or_default();
        let demand_texts = Self::compose_texts(graphics_lock.graphics_demands());
        format!("{owner_text}the demands of {}", demand_texts.join(", "))
    }

    fn compose_status_texts(graphics_lock: &GraphicsLock) -> Vec<String> {
        let maybe_owner_text = |maybe_application_name: Option<&ApplicationName>| {
            maybe_application_name
                .map_or_else(|| String::from("nobody"), ApplicationName::to_string)
        };
        let passthrough_text = match graphics_lock.passthrough_presence() {
            true => "on",
            false => "off",
        };
        vec![
            format!("lock {}", graphics_lock.lock_name()),
            format!(
                "  devices {}",
                Self::compose_texts(graphics_lock.device_addresses()).join(" ")
            ),
            format!(
                "  device files {}",
                Self::compose_texts(graphics_lock.device_files()).join(", ")
            ),
            format!(
                "  eligible applications {}",
                Self::compose_texts(graphics_lock.application_names()).join(" ")
            ),
            format!("  owner {}", maybe_owner_text(graphics_lock.maybe_owner())),
            format!(
                "  default owner {}{}",
                maybe_owner_text(graphics_lock.maybe_default_owner()),
                match graphics_lock.yield_presence() {
                    true => ", yielding",
                    false => "",
                }
            ),
            format!(
                "  demands {}",
                Self::compose_texts(graphics_lock.graphics_demands()).join(", ")
            ),
            format!("  passthrough {passthrough_text}"),
        ]
    }

    fn compose_texts<ItemType: fmt::Display>(items: &[ItemType]) -> Vec<String> {
        Vec::from_iter(items.iter().map(ItemType::to_string))
    }

    fn compose_program_error(error: <PortsType::DeviceProgram as DeviceProgram>::Error) -> Error {
        let text = error.to_string();
        Error::DeviceProgram { text }
    }

    fn compose_tree_error(error: <PortsType::DeviceTree as DeviceTree>::Error) -> Error {
        let text = error.to_string();
        Error::DeviceTree { text }
    }

    fn compose_group_error(error: <PortsType::ControlGroup as ControlGroup>::Error) -> Error {
        let text = error.to_string();
        Error::ControlGroup { text }
    }

    fn compose_record_error(error: <PortsType::LockRecord as LockRecord>::Error) -> Error {
        let text = error.to_string();
        Error::LockRecord { text }
    }

    fn compose_task_error(error: <PortsType::TaskTable as TaskTable>::Error) -> Error {
        let text = error.to_string();
        Error::TaskTable { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::acquisition::mode::AcquisitionMode;
    use stack_domain_driven_design_core::adapter::Adapter;
    use stack_domain_driven_design_core::layer::InfrastructureLayer;
    use stack_domain_driven_design_core::port::{DrivenPort, Port};
    use stack_lock_core_device_address::DeviceAddress;
    use stack_lock_core_device_file::DeviceKind;
    use std::collections::{BTreeMap, BTreeSet};
    use std::error;
    use std::sync::Mutex;

    mod fixture {
        use super::*;

        #[derive(Debug, PartialEq, Eq)]
        pub(crate) struct FixtureError {
            pub(crate) text: String,
        }

        impl fmt::Display for FixtureError {
            fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                formatter.write_str(&self.text)
            }
        }

        impl error::Error for FixtureError {}

        #[derive(Debug, Default)]
        pub(crate) struct FixtureStore {
            pub(crate) program_presence: bool,
            pub(crate) device_files: BTreeMap<DeviceFile, u32>,
            pub(crate) permissions: BTreeSet<(u32, u64)>,
            pub(crate) control_group: BTreeMap<String, (u64, bool)>,
            pub(crate) moves: Vec<(String, u32)>,
            pub(crate) record: BTreeMap<String, GraphicsLock>,
            pub(crate) task_table: BTreeSet<u32>,
            pub(crate) process_users: Vec<GraphicsUser>,
            pub(crate) survivor_users: BTreeSet<u32>,
            pub(crate) user_survival: BTreeSet<u32>,
            pub(crate) displacement: Vec<u32>,
            pub(crate) console_users: Vec<GraphicsUser>,
            pub(crate) record_console_users: Vec<GraphicsUser>,
            pub(crate) address_files: Vec<DeviceFile>,
            pub(crate) stale_files: Vec<DeviceFile>,
            pub(crate) device_addresses: BTreeSet<DeviceAddress>,
            pub(crate) device_persistence: bool,
            pub(crate) persistence_presence: Vec<bool>,
            pub(crate) rejection_port: Option<&'static str>,
            pub(crate) pause_count: u32,
            pub(crate) former_owner: Option<(u32, String)>,
            pub(crate) sequence: u32,
            pub(crate) rejection: Option<u32>,
            pub(crate) dismissal_sequence: Option<(u32, &'static str)>,
        }

        #[derive(Debug, Default)]
        pub(crate) struct FixtureLockPorts {
            pub(crate) store: Mutex<FixtureStore>,
        }

        impl Layer for FixtureLockPorts {
            type Layer = InfrastructureLayer;
        }

        impl Adapter for FixtureLockPorts {}

        impl Port for FixtureLockPorts {}

        impl DrivenPort for FixtureLockPorts {}

        impl FixtureLockPorts {
            pub(crate) fn outcome_factory(
                &self,
                port_name: &'static str,
            ) -> Result<(), FixtureError> {
                let mut store = self.store.lock().unwrap();
                if store.rejection_port == Some(port_name) {
                    return Err(FixtureError {
                        text: format!("the {port_name} broke"),
                    });
                }
                let outcome_sequence = store.sequence;
                store.sequence += 1;
                match store.rejection == Some(outcome_sequence) {
                    true => Err(FixtureError {
                        text: format!("the call {outcome_sequence} broke"),
                    }),
                    false => Ok(()),
                }
            }

            pub(crate) fn end_process(store: &mut FixtureStore, process_identifier: u32) {
                store.task_table.remove(&process_identifier);
                store.process_users.retain(|graphics_user| {
                    graphics_user.maybe_process_identifier() != Some(process_identifier)
                });
            }
        }

        impl DeviceProgram for FixtureLockPorts {
            type Error = FixtureError;

            fn try_install_program(&self) -> Result<(), FixtureError> {
                self.outcome_factory("program")?;
                self.store.lock().unwrap().program_presence = true;
                Ok(())
            }

            fn try_insert_device_files(
                &self,
                map_index: u32,
                device_files: &[DeviceFile],
            ) -> Result<(), FixtureError> {
                self.outcome_factory("program")?;
                let mut store = self.store.lock().unwrap();
                for device_file in device_files {
                    store.device_files.insert(*device_file, map_index);
                }
                Ok(())
            }

            fn try_delete_device_files(
                &self,
                device_files: &[DeviceFile],
            ) -> Result<(), FixtureError> {
                self.outcome_factory("program")?;
                let mut store = self.store.lock().unwrap();
                for device_file in device_files {
                    store.device_files.remove(device_file);
                }
                Ok(())
            }

            fn try_permit_group(
                &self,
                map_index: u32,
                group_identifier: u64,
            ) -> Result<(), FixtureError> {
                self.outcome_factory("program")?;
                self.store
                    .lock()
                    .unwrap()
                    .permissions
                    .insert((map_index, group_identifier));
                Ok(())
            }

            fn try_revoke_group(
                &self,
                map_index: u32,
                group_identifier: u64,
            ) -> Result<(), FixtureError> {
                self.outcome_factory("program")?;
                self.store
                    .lock()
                    .unwrap()
                    .permissions
                    .remove(&(map_index, group_identifier));
                Ok(())
            }

            fn try_remove_program(&self) -> Result<(), FixtureError> {
                self.outcome_factory("program")?;
                self.store.lock().unwrap().program_presence = false;
                Ok(())
            }
        }

        impl DeviceTree for FixtureLockPorts {
            type Error = FixtureError;

            fn try_read_device_files(
                &self,
                device_address: &DeviceAddress,
            ) -> Result<Vec<DeviceFile>, FixtureError> {
                self.outcome_factory("tree")?;
                let store = self.store.lock().unwrap();
                match store.device_addresses.contains(device_address) {
                    true => Ok(vec![DeviceFile::from((
                        DeviceKind::CharacterDevice,
                        511,
                        16,
                    ))]),
                    false => Ok(store.address_files.clone()),
                }
            }

            fn try_read_console_users(
                &self,
                _: &DeviceAddress,
            ) -> Result<Vec<GraphicsUser>, FixtureError> {
                self.outcome_factory("tree")?;
                Ok(self.store.lock().unwrap().console_users.clone())
            }

            fn try_unbind_console_user(
                &self,
                console_user: &GraphicsUser,
            ) -> Result<(), FixtureError> {
                self.outcome_factory("tree")?;
                let mut store = self.store.lock().unwrap();
                store
                    .console_users
                    .retain(|graphics_user| graphics_user != console_user);
                store.record_console_users.push(console_user.clone());
                Ok(())
            }

            fn try_bind_console_user(
                &self,
                console_user: &GraphicsUser,
            ) -> Result<(), FixtureError> {
                self.outcome_factory("tree")?;
                let mut store = self.store.lock().unwrap();
                store
                    .record_console_users
                    .retain(|graphics_user| graphics_user != console_user);
                store.console_users.push(console_user.clone());
                Ok(())
            }

            fn try_bind_passthrough(
                &self,
                device_address: &DeviceAddress,
            ) -> Result<bool, FixtureError> {
                self.outcome_factory("tree")?;
                let mut store = self.store.lock().unwrap();
                store.device_addresses.insert(*device_address);
                Ok(store.device_persistence)
            }

            fn check_identity_presence(&self, device_file: &DeviceFile) -> bool {
                self.store.lock().unwrap().stale_files.contains(device_file)
            }

            fn try_release_passthrough(
                &self,
                device_address: &DeviceAddress,
                persistence_presence: bool,
            ) -> Result<(), FixtureError> {
                self.outcome_factory("tree")?;
                let mut store = self.store.lock().unwrap();
                store.device_addresses.remove(device_address);
                store.persistence_presence.push(persistence_presence);
                Ok(())
            }
        }

        impl ControlGroup for FixtureLockPorts {
            type Error = FixtureError;

            fn try_create_group(&self, path_text: &str) -> Result<u64, FixtureError> {
                self.outcome_factory("group")?;
                let mut store = self.store.lock().unwrap();
                let group_identifier = 100 + u64::try_from(store.control_group.len()).unwrap();
                let control_group_entry = store
                    .control_group
                    .entry(String::from(path_text))
                    .or_insert((group_identifier, false));
                Ok(control_group_entry.0)
            }

            fn try_read_group_identifier(&self, path_text: &str) -> Result<u64, FixtureError> {
                self.outcome_factory("group")?;
                let store = self.store.lock().unwrap();
                store
                    .control_group
                    .get(path_text)
                    .map(|control_group_entry| control_group_entry.0)
                    .ok_or_else(|| FixtureError {
                        text: format!("{path_text} is absent"),
                    })
            }

            fn try_check_population(&self, path_text: &str) -> Result<bool, FixtureError> {
                self.outcome_factory("group")?;
                let store = self.store.lock().unwrap();
                Ok(store
                    .control_group
                    .get(path_text)
                    .is_some_and(|control_group_entry| control_group_entry.1))
            }

            fn try_move_process(
                &self,
                path_text: &str,
                process_identifier: u32,
            ) -> Result<(), FixtureError> {
                self.outcome_factory("group")?;
                let mut store = self.store.lock().unwrap();
                if let Some(control_group_entry) = store.control_group.get_mut(path_text) {
                    control_group_entry.1 = true;
                }
                store
                    .moves
                    .push((String::from(path_text), process_identifier));
                store.task_table.insert(process_identifier);
                Ok(())
            }

            fn try_enter_group(&self, path_text: &str) -> Result<(), FixtureError> {
                self.outcome_factory("group")?;
                self.store
                    .lock()
                    .unwrap()
                    .moves
                    .push((String::from(path_text), 0));
                Ok(())
            }

            fn try_remove_group(&self, path_text: &str) -> Result<(), FixtureError> {
                self.outcome_factory("group")?;
                self.store.lock().unwrap().control_group.remove(path_text);
                Ok(())
            }
        }

        impl LockRecord for FixtureLockPorts {
            type Error = FixtureError;
            type RecordLease = ();

            fn try_lease_records(&self) -> Result<(), FixtureError> {
                self.outcome_factory("record")
            }

            fn try_load_lock(
                &self,
                lock_name: &LockName,
            ) -> Result<Option<GraphicsLock>, FixtureError> {
                self.outcome_factory("record")?;
                Ok(self
                    .store
                    .lock()
                    .unwrap()
                    .record
                    .get(lock_name.text())
                    .cloned())
            }

            fn try_load_locks(&self) -> Result<Vec<GraphicsLock>, FixtureError> {
                self.outcome_factory("record")?;
                Ok(Vec::from_iter(
                    self.store.lock().unwrap().record.values().cloned(),
                ))
            }

            fn try_store_lock(&self, graphics_lock: &GraphicsLock) -> Result<(), FixtureError> {
                self.outcome_factory("record")?;
                let lock_text = String::from(graphics_lock.lock_name().text());
                self.store
                    .lock()
                    .unwrap()
                    .record
                    .insert(lock_text, graphics_lock.clone());
                Ok(())
            }

            fn try_delete_lock(&self, lock_name: &LockName) -> Result<(), FixtureError> {
                self.outcome_factory("record")?;
                self.store.lock().unwrap().record.remove(lock_name.text());
                Ok(())
            }
        }

        impl TaskTable for FixtureLockPorts {
            type Error = FixtureError;

            fn try_scan_process_users(
                &self,
                _: &[DeviceFile],
            ) -> Result<Vec<GraphicsUser>, FixtureError> {
                self.outcome_factory("process")?;
                Ok(self.store.lock().unwrap().process_users.clone())
            }

            fn check_process_presence(&self, process_identifier: u32) -> bool {
                self.store
                    .lock()
                    .unwrap()
                    .task_table
                    .contains(&process_identifier)
            }

            fn try_request_end(&self, process_identifier: u32) -> Result<(), FixtureError> {
                self.outcome_factory("process")?;
                let mut store = self.store.lock().unwrap();
                store.displacement.push(process_identifier);
                if !store.survivor_users.contains(&process_identifier) {
                    Self::end_process(&mut store, process_identifier);
                }
                Ok(())
            }

            fn try_force_end(&self, process_identifier: u32) -> Result<(), FixtureError> {
                self.outcome_factory("process")?;
                let mut store = self.store.lock().unwrap();
                if !store.user_survival.contains(&process_identifier) {
                    Self::end_process(&mut store, process_identifier);
                }
                Ok(())
            }
        }

        impl Pause for FixtureLockPorts {
            fn wait_duration(&self, _: Duration) {
                let mut store = self.store.lock().unwrap();
                store.pause_count += 1;
                if let Some((pause_count, application_text)) = store.dismissal_sequence
                    && store.pause_count >= pause_count
                    && let Some(graphics_lock) = store.record.get_mut("rtx3080")
                {
                    let application_name = ApplicationName::try_from(String::from(
                        application_text,
                    ))
                    .unwrap();
                    drop(graphics_lock.try_dismiss(&application_name));
                }
                if let Some((pause_count, path_text)) = store.former_owner.clone()
                    && store.pause_count >= pause_count
                {
                    if let Some(control_group_entry) = store.control_group.get_mut(&path_text) {
                        control_group_entry.1 = false;
                    }
                    let survivor_users = Vec::from_iter(
                        store
                            .process_users
                            .iter()
                            .filter(|graphics_user| graphics_user.check_membership(&path_text))
                            .cloned(),
                    );
                    for graphics_user in survivor_users {
                        let process_identifier = graphics_user.maybe_process_identifier().unwrap();
                        Self::end_process(&mut store, process_identifier);
                    }
                }
            }
        }

        impl<'lock> LockPorts for &'lock FixtureLockPorts {
            type DeviceProgram = FixtureLockPorts;
            type DeviceTree = FixtureLockPorts;
            type ControlGroup = FixtureLockPorts;
            type LockRecord = FixtureLockPorts;
            type TaskTable = FixtureLockPorts;
            type Pause = FixtureLockPorts;

            fn device_program(&self) -> &FixtureLockPorts {
                self
            }

            fn device_tree(&self) -> &FixtureLockPorts {
                self
            }

            fn control_group(&self) -> &FixtureLockPorts {
                self
            }

            fn lock_record(&self) -> &FixtureLockPorts {
                self
            }

            fn task_table(&self) -> &FixtureLockPorts {
                self
            }

            fn pause(&self) -> &FixtureLockPorts {
                self
            }
        }

        pub(crate) type FixtureAdministration<'lock> = LockAdministration<&'lock FixtureLockPorts>;

        pub(crate) fn rejection_factory(
            blueprint: fn(&FixtureLockPorts, &FixtureAdministration),
            command: fn(&FixtureAdministration) -> Result<(), Error>,
        ) -> u32 {
            let fixture_lock_ports = lock_ports_factory();
            let lock_administration = enclosure_factory(&fixture_lock_ports);
            blueprint(&fixture_lock_ports, &lock_administration);
            fixture_lock_ports.store.lock().unwrap().sequence = 0;
            let outcome = command(&lock_administration);
            let sequence = fixture_lock_ports.store.lock().unwrap().sequence;
            for rejection in 0..sequence {
                let fixture_lock_ports = lock_ports_factory();
                let lock_administration = enclosure_factory(&fixture_lock_ports);
                blueprint(&fixture_lock_ports, &lock_administration);
                let mut store = fixture_lock_ports.store.lock().unwrap();
                store.sequence = 0;
                store.rejection = Some(rejection);
                drop(store);
                let rejection_outcome = command(&lock_administration);
                assert_ne!(
                    rejection_outcome, outcome,
                    "the failure of the call {rejection} went unnoticed"
                );
            }
            sequence
        }

        pub(crate) fn device_file_factory(minor: u32) -> DeviceFile {
            DeviceFile::from((DeviceKind::CharacterDevice, 195, minor))
        }

        pub(crate) fn lock_ports_factory() -> FixtureLockPorts {
            let fixture_lock_ports = FixtureLockPorts::default();
            fixture_lock_ports.store.lock().unwrap().address_files = vec![
                device_file_factory(1),
                device_file_factory(2),
            ];
            fixture_lock_ports
        }

        pub(crate) fn administration_factory(
            fixture_lock_ports: &FixtureLockPorts,
        ) -> LockAdministration<&FixtureLockPorts> {
            let lock_setting = LockSetting::from((
                Duration::from_millis(100),
                Duration::from_millis(300),
            ));
            LockAdministration::from((fixture_lock_ports, lock_setting))
        }

        pub(crate) fn enclosure_factory(
            fixture_lock_ports: &FixtureLockPorts,
        ) -> LockAdministration<&FixtureLockPorts> {
            let lock_administration = administration_factory(fixture_lock_ports);
            let enclosure = Enclosure::try_from((
                String::from("rtx3080"),
                String::from("0000:03:00.0"),
            ))
            .unwrap();
            lock_administration.dispatch_command(enclosure).unwrap();
            for (application_text, default_presence) in [
                ("resolve", false),
                ("winix", false),
                ("desktop", false),
            ] {
                let admission = Admission::try_from((
                    String::from("rtx3080"),
                    String::from(application_text),
                    default_presence,
                    false,
                ))
                .unwrap();
                lock_administration.dispatch_command(admission).unwrap();
            }
            lock_administration
        }

        pub(crate) fn acquisition_factory(
            application_text: &str,
            waiter_identifier: u32,
            acquisition_mode: AcquisitionMode,
        ) -> Acquisition {
            Acquisition::try_from((
                String::from("rtx3080"),
                String::from(application_text),
                waiter_identifier,
                acquisition_mode,
            ))
            .unwrap()
        }

        pub(crate) fn release_factory(application_text: &str) -> Release {
            Release::try_from((String::from("rtx3080"), String::from(application_text))).unwrap()
        }

        pub(crate) fn process_user_factory(
            process_identifier: u32,
            control_group_text: &str,
        ) -> GraphicsUser {
            GraphicsUser::ProcessUser {
                process_identifier,
                command_text: String::from("render"),
                control_group_text: String::from(control_group_text),
            }
        }

        pub(crate) fn graphics_lock_factory(fixture_lock_ports: &FixtureLockPorts) -> GraphicsLock {
            fixture_lock_ports
                .store
                .lock()
                .unwrap()
                .record
                .get("rtx3080")
                .cloned()
                .unwrap()
        }

        pub(crate) fn host_identifier_factory(fixture_lock_ports: &FixtureLockPorts) -> u64 {
            fixture_lock_ports
                .store
                .lock()
                .unwrap()
                .control_group
                .get("/baton/host")
                .unwrap()
                .0
        }

        pub(crate) fn group_identifier_factory(
            fixture_lock_ports: &FixtureLockPorts,
            application_text: &str,
        ) -> u64 {
            let path_text = format!("/baton/rtx3080/{application_text}");
            fixture_lock_ports
                .store
                .lock()
                .unwrap()
                .control_group
                .get(&path_text)
                .unwrap()
                .0
        }
    }

    #[test]
    fn an_enclosure_installs_the_program_and_locks_the_files_of_its_gpu() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::administration_factory(&fixture_lock_ports);
        let enclosure = Enclosure::try_from((
            String::from("rtx3080"),
            String::from("0000:03:00.0"),
        ))
        .unwrap();

        let device_texts = lock_administration
            .dispatch_command(enclosure.clone())
            .unwrap();

        assert_eq!(device_texts, ["character 195:1", "character 195:2"]);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert!(store.program_presence);
        assert_eq!(
            store.device_files.values().copied().collect::<Vec<_>>(),
            [0, 0]
        );
        drop(store);
        let outcome = lock_administration.dispatch_command(enclosure);
        assert!(matches!(outcome, Err(Error::GraphicsLock { .. })));
        let enclosure = Enclosure::try_from((String::from("quadro"), String::from("0000:04:00.0")))
            .unwrap();
        lock_administration.dispatch_command(enclosure).unwrap();
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(store.record.get("quadro").unwrap().map_index(), 1);
    }

    #[test]
    fn an_admission_creates_the_group_and_permits_a_default_owner_at_once() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            false,
        ))
        .unwrap();

        lock_administration.dispatch_command(admission).unwrap();

        let group_identifier = fixture::group_identifier_factory(&fixture_lock_ports, "desktop");
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(
            store.permissions,
            BTreeSet::from([(0, host_identifier), (0, group_identifier)])
        );
        assert_eq!(store.control_group.len(), 4);
    }

    #[test]
    fn a_dismissal_of_the_default_owner_revokes_it_and_removes_its_group() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            false,
        ))
        .unwrap();
        lock_administration.dispatch_command(admission).unwrap();
        let dismissal = Dismissal::try_from((String::from("rtx3080"), String::from("desktop")))
            .unwrap();

        lock_administration.dispatch_command(dismissal).unwrap();

        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(store.permissions, BTreeSet::from([(0, host_identifier)]));
        assert!(!store.control_group.contains_key("/baton/rtx3080/desktop"));
        drop(store);
        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(graphics_lock.maybe_default_owner(), None);
        let dismissal = Dismissal::try_from((String::from("rtx3080"), String::from("resolve")))
            .unwrap();
        lock_administration.dispatch_command(dismissal).unwrap();
        let dismissal = Dismissal::try_from((String::from("quadro"), String::from("resolve")))
            .unwrap();
        let outcome = lock_administration.dispatch_command(dismissal);
        assert_eq!(
            outcome,
            Err(Error::LockAbsence {
                lock_text: String::from("quadro")
            })
        );
    }

    #[test]
    fn an_acquisition_of_a_free_gpu_takes_it_at_once_and_its_release_gives_it_back() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let console_user = GraphicsUser::ConsoleUser { console_index: 1 };
        fixture_lock_ports.store.lock().unwrap().console_users = vec![console_user.clone()];
        let acquisition = fixture::acquisition_factory("resolve", 7, AcquisitionMode::default());

        lock_administration.dispatch_command(acquisition).unwrap();

        let group_identifier = fixture::group_identifier_factory(&fixture_lock_ports, "resolve");
        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(
            graphics_lock.maybe_owner().map(ApplicationName::text),
            Some("resolve")
        );
        assert!(graphics_lock.graphics_demands().is_empty());
        assert_eq!(graphics_lock.console_users(), &[console_user.clone()]);
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(
            store.permissions,
            BTreeSet::from([(0, host_identifier), (0, group_identifier)])
        );
        assert_eq!(store.moves, [(String::from("/baton/rtx3080/resolve"), 7)]);
        assert_eq!(store.record_console_users, [console_user.clone()]);
        drop(store);
        lock_administration
            .dispatch_command(fixture::release_factory("resolve"))
            .unwrap();
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(store.permissions, BTreeSet::from([(0, host_identifier)]));
        assert_eq!(store.console_users, [console_user]);
        drop(store);
        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(graphics_lock.maybe_owner(), None);
        let outcome = lock_administration.dispatch_command(fixture::release_factory("resolve"));
        assert!(matches!(outcome, Err(Error::GraphicsLock { .. })));
    }

    #[test]
    fn a_release_gives_the_gpu_to_the_default_owner() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            false,
        ))
        .unwrap();
        lock_administration.dispatch_command(admission).unwrap();
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "winix",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let group_identifier = fixture::group_identifier_factory(&fixture_lock_ports, "winix");
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        assert_eq!(
            fixture_lock_ports.store.lock().unwrap().permissions,
            BTreeSet::from([(0, host_identifier), (0, group_identifier)])
        );

        lock_administration
            .dispatch_command(fixture::release_factory("winix"))
            .unwrap();

        let group_identifier = fixture::group_identifier_factory(&fixture_lock_ports, "desktop");
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        assert_eq!(
            fixture_lock_ports.store.lock().unwrap().permissions,
            BTreeSet::from([(0, host_identifier), (0, group_identifier)])
        );
    }

    #[test]
    fn an_acquisition_waits_for_a_user_and_gives_up_at_its_deadline() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let process_user = fixture::process_user_factory(42, "/user.slice/session-1.scope");
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![process_user];
        store.task_table.insert(42);
        drop(store);
        let acquisition_mode = AcquisitionMode::from((
            Some(Duration::from_millis(250)),
            false,
            false,
        ));

        let outcome = lock_administration.dispatch_command(fixture::acquisition_factory(
            "resolve",
            7,
            acquisition_mode,
        ));

        assert_eq!(
            outcome,
            Err(Error::DeadlineExpiry {
                lock_text: String::from("rtx3080"),
                user_text: String::from("the process 42 render"),
            })
        );
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(store.pause_count, 3);
        assert!(store.displacement.is_empty());
        drop(store);
        assert!(
            fixture::graphics_lock_factory(&fixture_lock_ports)
                .graphics_demands()
                .is_empty()
        );
    }

    #[test]
    fn an_acquisition_waits_until_the_owner_ends_and_then_takes_the_gpu() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "resolve",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![fixture::process_user_factory(7, "/baton/rtx3080/resolve")];
        store.former_owner = Some((2, String::from("/baton/rtx3080/resolve")));
        store.task_table.insert(8);
        drop(store);

        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "winix",
                8,
                AcquisitionMode::default(),
            ))
            .unwrap();

        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(
            graphics_lock.maybe_owner().map(ApplicationName::text),
            Some("winix")
        );
        let group_identifier = fixture::group_identifier_factory(&fixture_lock_ports, "winix");
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(store.pause_count, 2);
        assert_eq!(
            store.permissions,
            BTreeSet::from([(0, host_identifier), (0, group_identifier)])
        );
        assert!(store.displacement.is_empty());
    }

    #[test]
    fn a_demand_behind_another_waits_its_turn() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "resolve",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let mut graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        graphics_lock
            .try_record_demand(GraphicsDemand::from((
                ApplicationName::try_from(String::from("desktop")).unwrap(),
                9,
            )))
            .unwrap();
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.record.insert(String::from("rtx3080"), graphics_lock);
        store.task_table.insert(9);
        drop(store);
        let acquisition_mode = AcquisitionMode::from((Some(Duration::ZERO), false, false));

        let outcome = lock_administration.dispatch_command(fixture::acquisition_factory(
            "winix",
            8,
            acquisition_mode,
        ));

        assert_eq!(
            outcome,
            Err(Error::DeadlineExpiry {
                lock_text: String::from("rtx3080"),
                user_text: String::from(
                    "the owner resolve and the demands of desktop for the process 9, winix for the process 8"
                ),
            })
        );
    }

    #[test]
    fn a_turn_against_another_owner_waits_for_the_owner() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "resolve",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let acquisition_mode = AcquisitionMode::from((Some(Duration::ZERO), false, false));

        let outcome = lock_administration.dispatch_command(fixture::acquisition_factory(
            "winix",
            8,
            acquisition_mode,
        ));

        assert_eq!(
            outcome,
            Err(Error::DeadlineExpiry {
                lock_text: String::from("rtx3080"),
                user_text: String::from("resolve"),
            })
        );
    }

    #[test]
    fn a_forced_acquisition_ends_the_users_and_takes_the_gpu() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "resolve",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![
            fixture::process_user_factory(7, "/baton/rtx3080/resolve"),
            fixture::process_user_factory(43, "/baton/rtx3080/resolve/render"),
        ];
        store.task_table.extend([43]);
        store.survivor_users.insert(43);
        drop(store);
        let acquisition_mode = AcquisitionMode::from((None, true, false));

        lock_administration
            .dispatch_command(fixture::acquisition_factory("winix", 8, acquisition_mode))
            .unwrap();

        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(
            graphics_lock.maybe_owner().map(ApplicationName::text),
            Some("winix")
        );
        let group_identifier = fixture::group_identifier_factory(&fixture_lock_ports, "winix");
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(store.displacement, [7, 43]);
        assert!(!store.task_table.contains(&43));
        assert_eq!(
            store.permissions,
            BTreeSet::from([(0, host_identifier), (0, group_identifier)])
        );
    }

    #[test]
    fn an_acquisition_ends_a_yielding_default_owner_and_waits_for_every_other_user() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            true,
        ))
        .unwrap();
        lock_administration.dispatch_command(admission).unwrap();
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![
            fixture::process_user_factory(7, "/baton/rtx3080/desktop"),
            fixture::process_user_factory(44, "/init.scope"),
        ];
        store.task_table.extend([7, 44]);
        drop(store);
        let acquisition_mode = AcquisitionMode::from((
            Some(Duration::from_millis(100)),
            false,
            false,
        ));

        let outcome = lock_administration.dispatch_command(fixture::acquisition_factory(
            "winix",
            8,
            acquisition_mode,
        ));

        assert!(matches!(outcome, Err(Error::DeadlineExpiry { .. })));
        assert!(
            fixture_lock_ports
                .store
                .lock()
                .unwrap()
                .displacement
                .is_empty()
        );
        let mut store = fixture_lock_ports.store.lock().unwrap();
        fixture::FixtureLockPorts::end_process(&mut store, 44);
        drop(store);

        lock_administration
            .dispatch_command(fixture::acquisition_factory("winix", 8, acquisition_mode))
            .unwrap();

        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(
            graphics_lock.maybe_owner().map(ApplicationName::text),
            Some("winix")
        );
        assert!(graphics_lock.yield_presence());
        assert_eq!(fixture_lock_ports.store.lock().unwrap().displacement, [7]);
        assert!(
            LockAdministration::<&fixture::FixtureLockPorts>::compose_status_texts(&graphics_lock)
                .contains(&String::from("  default owner desktop, yielding"))
        );
    }

    #[test]
    fn a_yielding_default_owner_that_acquired_the_lock_yields_it_as_well() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            true,
        ))
        .unwrap();
        lock_administration.dispatch_command(admission).unwrap();
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "desktop",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![fixture::process_user_factory(43, "/baton/rtx3080/desktop")];
        store.task_table.extend([43]);
        drop(store);

        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "winix",
                8,
                AcquisitionMode::default(),
            ))
            .unwrap();

        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(
            graphics_lock.maybe_owner().map(ApplicationName::text),
            Some("winix")
        );
        assert_eq!(fixture_lock_ports.store.lock().unwrap().displacement, [43]);
    }

    #[test]
    fn a_forced_acquisition_fails_on_a_user_that_survives_its_end() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![fixture::process_user_factory(44, "/init.scope")];
        store.task_table.insert(44);
        store.survivor_users.insert(44);
        store.user_survival.insert(44);
        drop(store);
        let acquisition_mode = AcquisitionMode::from((None, true, false));

        let outcome = lock_administration.dispatch_command(fixture::acquisition_factory(
            "winix",
            8,
            acquisition_mode,
        ));

        assert_eq!(
            outcome,
            Err(Error::UserSurvival {
                lock_text: String::from("rtx3080"),
                user_text: String::from("the process 44 render"),
            })
        );
    }

    #[test]
    fn a_passthrough_binds_the_gpu_to_vfio_and_its_release_gives_it_back() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        fixture_lock_ports.store.lock().unwrap().device_persistence = true;
        let acquisition_mode = AcquisitionMode::from((None, false, true));

        lock_administration
            .dispatch_command(fixture::acquisition_factory("winix", 8, acquisition_mode))
            .unwrap();

        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert!(graphics_lock.passthrough_presence());
        assert!(graphics_lock.persistence_presence());
        let device_file = DeviceFile::from((DeviceKind::CharacterDevice, 511, 16));
        let device_files = [
            fixture::device_file_factory(1),
            fixture::device_file_factory(2),
            device_file,
        ];
        assert_eq!(graphics_lock.device_files(), &device_files);
        assert_eq!(
            fixture_lock_ports
                .store
                .lock()
                .unwrap()
                .device_files
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            device_files
        );
        lock_administration
            .dispatch_command(fixture::release_factory("winix"))
            .unwrap();
        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert!(!graphics_lock.passthrough_presence());
        assert_eq!(graphics_lock.device_files(), &device_files);
        fixture_lock_ports.store.lock().unwrap().stale_files = vec![device_file];
        lock_administration.dispatch_command(Refresh).unwrap();
        let graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        assert_eq!(
            graphics_lock.device_files(),
            &[
                fixture::device_file_factory(1),
                fixture::device_file_factory(2)
            ]
        );
        assert_eq!(
            fixture_lock_ports
                .store
                .lock()
                .unwrap()
                .device_files
                .keys()
                .copied()
                .collect::<Vec<_>>(),
            [
                fixture::device_file_factory(1),
                fixture::device_file_factory(2)
            ]
        );
        let store = fixture_lock_ports.store.lock().unwrap();
        assert!(store.device_addresses.is_empty());
        assert_eq!(store.persistence_presence, [true]);
        assert_eq!(
            Vec::from_iter(
                store
                    .moves
                    .iter()
                    .filter(|(path_text, _)| path_text == "/baton/host")
            ),
            [
                &(String::from("/baton/host"), 0),
                &(String::from("/baton/host"), 0)
            ]
        );
    }

    #[test]
    fn an_owner_that_ended_without_a_release_gives_the_gpu_back_on_the_next_request() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let acquisition_mode = AcquisitionMode::from((None, false, true));
        lock_administration
            .dispatch_command(fixture::acquisition_factory("winix", 8, acquisition_mode))
            .unwrap();
        fixture_lock_ports
            .store
            .lock()
            .unwrap()
            .control_group
            .get_mut("/baton/rtx3080/winix")
            .unwrap()
            .1 = false;
        let mut graphics_lock = fixture::graphics_lock_factory(&fixture_lock_ports);
        graphics_lock
            .try_record_demand(GraphicsDemand::from((
                ApplicationName::try_from(String::from("desktop")).unwrap(),
                99,
            )))
            .unwrap();
        fixture_lock_ports
            .store
            .lock()
            .unwrap()
            .record
            .insert(String::from("rtx3080"), graphics_lock);

        lock_administration
            .dispatch_command(Removal::try_from(String::from("rtx3080")).unwrap())
            .unwrap();

        let store = fixture_lock_ports.store.lock().unwrap();
        assert!(store.device_addresses.is_empty());
        assert!(store.record.is_empty());
        assert!(store.device_files.is_empty());
        assert_eq!(
            Vec::from_iter(store.control_group.keys()),
            [&String::from("/baton/host")]
        );
        assert!(!store.program_presence);
    }

    #[test]
    fn a_removal_of_an_owned_lock_is_refused_and_one_of_two_locks_keeps_the_program() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            false,
        ))
        .unwrap();
        lock_administration.dispatch_command(admission).unwrap();
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "resolve",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();

        let outcome = lock_administration
            .dispatch_command(Removal::try_from(String::from("rtx3080")).unwrap());

        assert_eq!(
            outcome,
            Err(Error::OwnerPresence {
                lock_text: String::from("rtx3080"),
                owner_text: String::from("resolve"),
            })
        );
        lock_administration
            .dispatch_command(fixture::release_factory("resolve"))
            .unwrap();
        let enclosure = Enclosure::try_from((String::from("quadro"), String::from("0000:04:00.0")))
            .unwrap();
        lock_administration.dispatch_command(enclosure).unwrap();
        lock_administration
            .dispatch_command(Removal::try_from(String::from("rtx3080")).unwrap())
            .unwrap();
        let host_identifier = fixture::host_identifier_factory(&fixture_lock_ports);
        let store = fixture_lock_ports.store.lock().unwrap();
        assert!(store.program_presence);
        assert_eq!(store.permissions, BTreeSet::from([(1, host_identifier)]));
    }

    #[test]
    fn a_refresh_locks_the_files_a_gpu_has_now_and_keeps_an_identity_no_other_device_took() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.address_files = vec![fixture::device_file_factory(3)];
        store.stale_files = vec![fixture::device_file_factory(1)];
        drop(store);

        lock_administration.dispatch_command(Refresh).unwrap();

        let store = fixture_lock_ports.store.lock().unwrap();
        assert_eq!(
            store.device_files.keys().copied().collect::<Vec<_>>(),
            [
                fixture::device_file_factory(2),
                fixture::device_file_factory(3)
            ]
        );
    }

    #[test]
    fn the_queries_name_the_users_and_the_state_of_every_lock() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![fixture::process_user_factory(42, "/user.slice")];
        store.console_users = vec![GraphicsUser::ConsoleUser { console_index: 1 }];
        drop(store);

        let user_texts = lock_administration
            .resolve_query(UserQuery::try_from(String::from("rtx3080")).unwrap())
            .unwrap();
        let status_texts = lock_administration.resolve_query(StatusQuery).unwrap();

        assert_eq!(
            user_texts,
            ["the process 42 render", "the virtual console 1"]
        );
        assert_eq!(
            status_texts,
            [
                "lock rtx3080",
                "  devices 0000:03:00.0",
                "  device files character 195:1, character 195:2",
                "  eligible applications desktop resolve winix",
                "  owner nobody",
                "  default owner nobody",
                "  demands ",
                "  passthrough off",
            ]
        );
    }

    #[test]
    fn a_failure_of_a_port_becomes_an_error_of_the_application() {
        let port_names = ["program", "tree", "group", "record", "process"];

        let error = port_names.map(|port_name| {
            let fixture_lock_ports = fixture::lock_ports_factory();
            let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
            fixture_lock_ports.store.lock().unwrap().rejection_port = Some(port_name);
            match port_name {
                "program" => {
                    let enclosure = Enclosure::try_from((
                        String::from("quadro"),
                        String::from("0000:04:00.0"),
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(enclosure).map(|_| ())
                }
                "tree" => lock_administration.dispatch_command(Refresh),
                "group" => {
                    let admission = Admission::try_from((
                        String::from("rtx3080"),
                        String::from("strata"),
                        false,
                        false,
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(admission)
                }
                "record" => lock_administration.resolve_query(StatusQuery).map(|_| ()),
                _ => lock_administration
                    .resolve_query(UserQuery::try_from(String::from("rtx3080")).unwrap())
                    .map(|_| ()),
            }
            .unwrap_err()
        });

        assert_eq!(
            error,
            [
                Error::DeviceProgram {
                    text: String::from("the program broke")
                },
                Error::DeviceTree {
                    text: String::from("the tree broke")
                },
                Error::ControlGroup {
                    text: String::from("the group broke")
                },
                Error::LockRecord {
                    text: String::from("the record broke")
                },
                Error::TaskTable {
                    text: String::from("the process broke")
                },
            ]
        );
    }

    #[test]
    fn every_failure_of_a_port_fails_every_command_and_every_query() {
        let sequence = [
            fixture::rejection_factory(
                |_, _| {},
                |lock_administration| {
                    let enclosure = Enclosure::try_from((
                        String::from("quadro"),
                        String::from("0000:04:00.0"),
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(enclosure).map(|_| ())
                },
            ),
            fixture::rejection_factory(
                |_, _| {},
                |lock_administration| {
                    let admission = Admission::try_from((
                        String::from("rtx3080"),
                        String::from("desktop"),
                        true,
                        false,
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(admission)
                },
            ),
            fixture::rejection_factory(
                |_, lock_administration| {
                    let admission = Admission::try_from((
                        String::from("rtx3080"),
                        String::from("desktop"),
                        true,
                        false,
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(admission).unwrap();
                },
                |lock_administration| {
                    let dismissal = Dismissal::try_from((
                        String::from("rtx3080"),
                        String::from("desktop"),
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(dismissal)
                },
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, _| {
                    fixture_lock_ports.store.lock().unwrap().console_users = vec![
                        GraphicsUser::ConsoleUser { console_index: 1 },
                    ];
                },
                |lock_administration| {
                    let acquisition_mode = AcquisitionMode::from((None, false, true));
                    lock_administration.dispatch_command(fixture::acquisition_factory(
                        "winix",
                        8,
                        acquisition_mode,
                    ))
                },
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, lock_administration| {
                    fixture_lock_ports.store.lock().unwrap().console_users = vec![
                        GraphicsUser::ConsoleUser { console_index: 1 },
                    ];
                    let admission = Admission::try_from((
                        String::from("rtx3080"),
                        String::from("desktop"),
                        true,
                        false,
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(admission).unwrap();
                    let acquisition_mode = AcquisitionMode::from((None, false, true));
                    lock_administration
                        .dispatch_command(fixture::acquisition_factory(
                            "winix",
                            8,
                            acquisition_mode,
                        ))
                        .unwrap();
                },
                |lock_administration| {
                    lock_administration.dispatch_command(fixture::release_factory("winix"))
                },
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, lock_administration| {
                    lock_administration
                        .dispatch_command(fixture::acquisition_factory(
                            "resolve",
                            7,
                            AcquisitionMode::default(),
                        ))
                        .unwrap();
                    let mut store = fixture_lock_ports.store.lock().unwrap();
                    store.process_users = vec![fixture::process_user_factory(
                        7,
                        "/baton/rtx3080/resolve",
                    )];
                    store.survivor_users.insert(7);
                },
                |lock_administration| {
                    let acquisition_mode = AcquisitionMode::from((None, true, false));
                    lock_administration.dispatch_command(fixture::acquisition_factory(
                        "winix",
                        8,
                        acquisition_mode,
                    ))
                },
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, _| {
                    let mut store = fixture_lock_ports.store.lock().unwrap();
                    store.process_users = vec![fixture::process_user_factory(42, "/user.slice")];
                    store.task_table.insert(42);
                },
                |lock_administration| {
                    let acquisition_mode = AcquisitionMode::from((
                        Some(Duration::ZERO),
                        false,
                        false,
                    ));
                    lock_administration.dispatch_command(fixture::acquisition_factory(
                        "winix",
                        8,
                        acquisition_mode,
                    ))
                },
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, lock_administration| {
                    let acquisition_mode = AcquisitionMode::from((None, false, true));
                    lock_administration
                        .dispatch_command(fixture::acquisition_factory(
                            "winix",
                            8,
                            acquisition_mode,
                        ))
                        .unwrap();
                    fixture_lock_ports
                        .store
                        .lock()
                        .unwrap()
                        .control_group
                        .get_mut("/baton/rtx3080/winix")
                        .unwrap()
                        .1 = false;
                    let admission = Admission::try_from((
                        String::from("rtx3080"),
                        String::from("desktop"),
                        true,
                        false,
                    ))
                    .unwrap();
                    lock_administration.dispatch_command(admission).unwrap();
                },
                |lock_administration| {
                    lock_administration
                        .dispatch_command(Removal::try_from(String::from("rtx3080")).unwrap())
                },
            ),
            fixture::rejection_factory(
                |_, _| {},
                |lock_administration| lock_administration.dispatch_command(Refresh),
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, _| {
                    fixture_lock_ports.store.lock().unwrap().console_users = vec![
                        GraphicsUser::ConsoleUser { console_index: 1 },
                    ];
                },
                |lock_administration| {
                    lock_administration
                        .resolve_query(UserQuery::try_from(String::from("rtx3080")).unwrap())
                        .map(|_| ())
                },
            ),
            fixture::rejection_factory(
                |_, _| {},
                |lock_administration| lock_administration.resolve_query(StatusQuery).map(|_| ()),
            ),
            fixture::rejection_factory(
                |_, lock_administration| {
                    lock_administration
                        .dispatch_command(fixture::acquisition_factory(
                            "resolve",
                            7,
                            AcquisitionMode::default(),
                        ))
                        .unwrap();
                },
                |lock_administration| {
                    let acquisition_mode = AcquisitionMode::from((
                        Some(Duration::ZERO),
                        false,
                        false,
                    ));
                    lock_administration.dispatch_command(fixture::acquisition_factory(
                        "winix",
                        8,
                        acquisition_mode,
                    ))
                },
            ),
            fixture::rejection_factory(
                |fixture_lock_ports, _| {
                    let mut graphics_lock = fixture::graphics_lock_factory(fixture_lock_ports);
                    let application_name = ApplicationName::try_from(String::from("desktop"))
                        .unwrap();
                    graphics_lock
                        .try_record_demand(GraphicsDemand::from((application_name, 9)))
                        .unwrap();
                    let mut store = fixture_lock_ports.store.lock().unwrap();
                    store.record.insert(String::from("rtx3080"), graphics_lock);
                    store.task_table.insert(9);
                },
                |lock_administration| {
                    let acquisition_mode = AcquisitionMode::from((
                        Some(Duration::ZERO),
                        false,
                        false,
                    ));
                    lock_administration.dispatch_command(fixture::acquisition_factory(
                        "winix",
                        8,
                        acquisition_mode,
                    ))
                },
            ),
        ];

        assert!(sequence.iter().all(|sequence| *sequence > 0));
    }

    #[test]
    fn a_refusal_of_the_domain_ends_the_command() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            false,
        ))
        .unwrap();
        lock_administration.dispatch_command(admission).unwrap();
        lock_administration
            .dispatch_command(fixture::acquisition_factory(
                "resolve",
                7,
                AcquisitionMode::default(),
            ))
            .unwrap();
        let admission = Admission::try_from((
            String::from("rtx3080"),
            String::from("winix"),
            true,
            false,
        ))
        .unwrap();
        let dismissal = Dismissal::try_from((String::from("rtx3080"), String::from("resolve")))
            .unwrap();
        let acquisition = fixture::acquisition_factory("strata", 9, AcquisitionMode::default());

        let outcome = [
            lock_administration.dispatch_command(admission),
            lock_administration.dispatch_command(dismissal),
            lock_administration.dispatch_command(acquisition),
        ];

        assert!(
            outcome
                .iter()
                .all(|outcome| matches!(outcome, Err(Error::GraphicsLock { .. })))
        );
    }

    #[test]
    fn an_application_dismissed_while_it_waits_ends_its_acquisition() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let mut store = fixture_lock_ports.store.lock().unwrap();
        store.process_users = vec![fixture::process_user_factory(42, "/user.slice")];
        store.task_table.insert(42);
        store.dismissal_sequence = Some((1, "winix"));
        drop(store);

        let outcome = lock_administration.dispatch_command(fixture::acquisition_factory(
            "winix",
            8,
            AcquisitionMode::default(),
        ));

        assert!(matches!(outcome, Err(Error::GraphicsLock { .. })));
    }

    #[test]
    fn the_status_of_a_passthrough_names_it() {
        let fixture_lock_ports = fixture::lock_ports_factory();
        let lock_administration = fixture::enclosure_factory(&fixture_lock_ports);
        let acquisition_mode = AcquisitionMode::from((None, false, true));
        lock_administration
            .dispatch_command(fixture::acquisition_factory("winix", 8, acquisition_mode))
            .unwrap();

        let status_texts = lock_administration.resolve_query(StatusQuery).unwrap();

        assert_eq!(
            status_texts.last().map(String::as_str),
            Some("  passthrough on")
        );
        assert_eq!(status_texts[4], "  owner winix");
    }
}
