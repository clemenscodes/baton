mod error;

pub use error::Error;

use stack_domain_driven_design_core::aggregate_root::AggregateRoot;
use stack_domain_driven_design_core::entity::Entity;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_lock_core_demand::GraphicsDemand;
use stack_lock_core_device_address::DeviceAddress;
use stack_lock_core_device_file::DeviceFile;
use stack_lock_core_graphics_user::GraphicsUser;
use stack_lock_core_name::{ApplicationName, LockName};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphicsLock {
    lock_name: LockName,
    map_index: u32,
    device_addresses: Vec<DeviceAddress>,
    device_files: Vec<DeviceFile>,
    application_names: Vec<ApplicationName>,
    maybe_default_owner: Option<ApplicationName>,
    maybe_owner: Option<ApplicationName>,
    graphics_demands: Vec<GraphicsDemand>,
    console_users: Vec<GraphicsUser>,
    passthrough_presence: bool,
    persistence_presence: bool,
}

impl Layer for GraphicsLock {
    type Layer = DomainLayer;
}

impl Entity for GraphicsLock {
    type Identity = LockName;

    fn identity(&self) -> &Self::Identity {
        &self.lock_name
    }
}

impl AggregateRoot for GraphicsLock {}

impl From<(LockName, u32)> for GraphicsLock {
    fn from((lock_name, map_index): (LockName, u32)) -> Self {
        Self {
            lock_name,
            map_index,
            device_addresses: Vec::new(),
            device_files: Vec::new(),
            application_names: Vec::new(),
            maybe_default_owner: None,
            maybe_owner: None,
            graphics_demands: Vec::new(),
            console_users: Vec::new(),
            passthrough_presence: false,
            persistence_presence: false,
        }
    }
}

impl GraphicsLock {
    const TREE_NAME: &'static str = "baton";

    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }

    pub fn map_index(&self) -> u32 {
        self.map_index
    }

    pub fn device_addresses(&self) -> &[DeviceAddress] {
        &self.device_addresses
    }

    pub fn device_files(&self) -> &[DeviceFile] {
        &self.device_files
    }

    pub fn application_names(&self) -> &[ApplicationName] {
        &self.application_names
    }

    pub fn maybe_default_owner(&self) -> Option<&ApplicationName> {
        self.maybe_default_owner.as_ref()
    }

    pub fn maybe_owner(&self) -> Option<&ApplicationName> {
        self.maybe_owner.as_ref()
    }

    pub fn graphics_demands(&self) -> &[GraphicsDemand] {
        &self.graphics_demands
    }

    pub fn console_users(&self) -> &[GraphicsUser] {
        &self.console_users
    }

    pub fn passthrough_presence(&self) -> bool {
        self.passthrough_presence
    }

    pub fn persistence_presence(&self) -> bool {
        self.persistence_presence
    }

    pub fn maybe_permitted_application(&self) -> Option<&ApplicationName> {
        self.maybe_owner
            .as_ref()
            .or(self.maybe_default_owner.as_ref())
    }

    pub fn maybe_first_demand(&self) -> Option<&GraphicsDemand> {
        self.graphics_demands.first()
    }

    pub fn application_path_text(&self, application_name: &ApplicationName) -> String {
        format!("/{}/{}/{application_name}", Self::TREE_NAME, self.lock_name)
    }

    pub fn tree_path_text(&self) -> String {
        format!("/{}/{}", Self::TREE_NAME, self.lock_name)
    }

    pub fn check_eligibility(&self, application_name: &ApplicationName) -> bool {
        self.application_names.contains(application_name)
    }

    pub fn try_add_device(&mut self, device_address: DeviceAddress) -> Result<(), Error> {
        if self.device_addresses.contains(&device_address) {
            let lock_text = self.lock_name.to_string();
            let address_text = device_address.to_string();
            return Err(Error::DeviceRepetition {
                lock_text,
                address_text,
            });
        }
        self.device_addresses.push(device_address);
        self.device_addresses.sort();
        Ok(())
    }

    pub fn replace_device_files(&mut self, device_files: Vec<DeviceFile>) -> Vec<DeviceFile> {
        let mut device_files = device_files;
        device_files.sort();
        device_files.dedup();
        let previous_files = std::mem::replace(&mut self.device_files, device_files);
        Vec::from_iter(
            previous_files
                .into_iter()
                .filter(|device_file| !self.device_files.contains(device_file)),
        )
    }

    pub fn try_admit(
        &mut self,
        application_name: ApplicationName,
        default_presence: bool,
    ) -> Result<(), Error> {
        if default_presence
            && let Some(default_owner) = &self.maybe_default_owner
            && *default_owner != application_name
        {
            let lock_text = self.lock_name.to_string();
            let application_text = default_owner.to_string();
            return Err(Error::DefaultOwnerConflict {
                lock_text,
                application_text,
            });
        }
        if default_presence {
            self.maybe_default_owner = Some(application_name.clone());
        }
        if !self.application_names.contains(&application_name) {
            self.application_names.push(application_name);
            self.application_names.sort();
        }
        Ok(())
    }

    pub fn try_dismiss(&mut self, application_name: &ApplicationName) -> Result<(), Error> {
        self.try_check_application(application_name)?;
        if self.maybe_owner.as_ref() == Some(application_name) {
            let lock_text = self.lock_name.to_string();
            let application_text = application_name.to_string();
            return Err(Error::OwnerConflict {
                lock_text,
                application_text,
            });
        }
        if self.maybe_default_owner.as_ref() == Some(application_name) {
            self.maybe_default_owner = None;
        }
        self.application_names
            .retain(|eligible_name| eligible_name != application_name);
        self.graphics_demands
            .retain(|graphics_demand| graphics_demand.application_name() != application_name);
        Ok(())
    }

    pub fn try_record_demand(&mut self, graphics_demand: GraphicsDemand) -> Result<(), Error> {
        self.try_check_application(graphics_demand.application_name())?;
        let process_identifier = graphics_demand.waiter_identifier();
        let repetition_presence = self
            .graphics_demands
            .iter()
            .any(|recorded_demand| recorded_demand.waiter_identifier() == process_identifier);
        if !repetition_presence {
            self.graphics_demands.push(graphics_demand);
        }
        Ok(())
    }

    pub fn drop_demand(&mut self, process_identifier: u32) {
        self.graphics_demands
            .retain(|graphics_demand| graphics_demand.waiter_identifier() != process_identifier);
    }

    pub fn retain_demands(&mut self, check_presence: impl Fn(u32) -> bool) {
        self.graphics_demands
            .retain(|graphics_demand| check_presence(graphics_demand.waiter_identifier()));
    }

    pub fn assign_demand_owner(&mut self, graphics_demand: &GraphicsDemand) {
        if self.graphics_demands.contains(graphics_demand) {
            self.maybe_owner = Some(graphics_demand.application_name().clone());
            self.drop_demand(graphics_demand.waiter_identifier());
        }
    }

    pub fn try_recover_owner(&mut self, application_name: ApplicationName) -> Result<(), Error> {
        self.try_check_application(&application_name)?;
        self.maybe_owner = Some(application_name);
        Ok(())
    }

    pub fn try_release(&mut self, application_name: &ApplicationName) -> Result<(), Error> {
        if self.maybe_owner.as_ref() != Some(application_name) {
            let lock_text = self.lock_name.to_string();
            let application_text = application_name.to_string();
            let owner_text = self
                .maybe_owner
                .as_ref()
                .map_or_else(|| String::from("nobody"), ApplicationName::to_string);
            return Err(Error::OwnerMismatch {
                lock_text,
                application_text,
                owner_text,
            });
        }
        self.maybe_owner = None;
        Ok(())
    }

    pub fn clear_owner(&mut self) {
        self.maybe_owner = None;
    }

    pub fn record_console_users(&mut self, console_users: Vec<GraphicsUser>) {
        for console_user in console_users {
            if !self.console_users.contains(&console_user) {
                self.console_users.push(console_user);
            }
        }
    }

    pub fn remove_console_users(&mut self) -> Vec<GraphicsUser> {
        std::mem::take(&mut self.console_users)
    }

    pub fn record_passthrough(&mut self, persistence_presence: bool) {
        self.passthrough_presence = true;
        self.persistence_presence = persistence_presence;
    }

    pub fn clear_passthrough(&mut self) -> bool {
        let persistence_presence = self.persistence_presence;
        self.passthrough_presence = false;
        self.persistence_presence = false;
        persistence_presence
    }

    fn try_check_application(&self, application_name: &ApplicationName) -> Result<(), Error> {
        match self.check_eligibility(application_name) {
            true => Ok(()),
            false => {
                let lock_text = self.lock_name.to_string();
                let application_text = application_name.to_string();
                Err(Error::ApplicationAbsence {
                    lock_text,
                    application_text,
                })
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use stack_lock_core_device_file::DeviceKind;

    mod fixture {
        use super::*;

        pub(crate) fn application_name_factory(text: &str) -> ApplicationName {
            ApplicationName::try_from(String::from(text)).unwrap()
        }

        pub(crate) fn graphics_lock_factory() -> GraphicsLock {
            let lock_name = LockName::try_from(String::from("rtx3080")).unwrap();
            let mut graphics_lock = GraphicsLock::from((lock_name, 3));
            for text in ["resolve", "winix", "desktop"] {
                let application_name = application_name_factory(text);
                graphics_lock.try_admit(application_name, false).unwrap();
            }
            graphics_lock
        }

        pub(crate) fn graphics_demand_factory(
            text: &str,
            process_identifier: u32,
        ) -> GraphicsDemand {
            GraphicsDemand::from((application_name_factory(text), process_identifier))
        }
    }

    #[test]
    fn a_new_lock_holds_its_name_and_its_index_and_nothing_else() {
        let lock_name = LockName::try_from(String::from("rtx3080")).unwrap();

        let graphics_lock = GraphicsLock::from((lock_name.clone(), 3));

        assert_eq!(graphics_lock.identity(), &lock_name);
        assert_eq!(graphics_lock.lock_name(), &lock_name);
        assert_eq!(graphics_lock.map_index(), 3);
        assert!(graphics_lock.device_addresses().is_empty());
        assert!(graphics_lock.device_files().is_empty());
        assert!(graphics_lock.application_names().is_empty());
        assert_eq!(graphics_lock.maybe_permitted_application(), None);
        assert_eq!(graphics_lock.maybe_first_demand(), None);
        assert!(!graphics_lock.passthrough_presence());
        assert_eq!(graphics_lock.tree_path_text(), "/baton/rtx3080");
    }

    #[test]
    fn a_device_is_added_once_and_a_repetition_is_refused() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let device_address = DeviceAddress::try_from(String::from("0000:03:00.0")).unwrap();

        graphics_lock.try_add_device(device_address).unwrap();
        let outcome = graphics_lock.try_add_device(device_address);

        assert_eq!(graphics_lock.device_addresses(), &[device_address]);
        assert_eq!(
            outcome,
            Err(Error::DeviceRepetition {
                lock_text: String::from("rtx3080"),
                address_text: String::from("0000:03:00.0"),
            })
        );
    }

    #[test]
    fn a_replacement_of_the_device_files_answers_the_files_no_longer_locked() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let device_file = [195, 226, 226]
            .map(|major| DeviceFile::from((DeviceKind::CharacterDevice, major, 1)));
        graphics_lock.replace_device_files(vec![device_file[0], device_file[1]]);

        let previous_files = graphics_lock
            .replace_device_files(vec![device_file[2], device_file[0]]);

        assert!(previous_files.is_empty());
        let device_files = vec![DeviceFile::from((DeviceKind::CharacterDevice, 226, 2))];
        let previous_files = graphics_lock.replace_device_files(device_files.clone());
        assert_eq!(previous_files, vec![device_file[0], device_file[1]]);
        assert_eq!(graphics_lock.device_files(), device_files.as_slice());
    }

    #[test]
    fn the_default_owner_is_permitted_until_an_application_owns_the_lock() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let default_owner = fixture::application_name_factory("desktop");
        graphics_lock
            .try_admit(default_owner.clone(), true)
            .unwrap();
        let graphics_demand = fixture::graphics_demand_factory("winix", 7);

        let maybe_permitted_application = graphics_lock.maybe_permitted_application().cloned();
        graphics_lock
            .try_record_demand(graphics_demand.clone())
            .unwrap();
        graphics_lock.assign_demand_owner(&graphics_demand);

        assert_eq!(maybe_permitted_application, Some(default_owner.clone()));
        let application_name = fixture::application_name_factory("winix");
        assert_eq!(
            graphics_lock.maybe_permitted_application(),
            Some(&application_name)
        );
        assert_eq!(graphics_lock.maybe_owner(), Some(&application_name));
        graphics_lock.try_release(&application_name).unwrap();
        assert_eq!(
            graphics_lock.maybe_permitted_application(),
            Some(&default_owner)
        );
        assert_eq!(graphics_lock.maybe_default_owner(), Some(&default_owner));
    }

    #[test]
    fn a_second_default_owner_is_refused() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let default_owner = fixture::application_name_factory("desktop");
        graphics_lock
            .try_admit(default_owner.clone(), true)
            .unwrap();

        let repetition_presence = graphics_lock.try_admit(default_owner, true);
        let outcome = graphics_lock.try_admit(fixture::application_name_factory("resolve"), true);

        assert_eq!(repetition_presence, Ok(()));
        assert_eq!(
            outcome,
            Err(Error::DefaultOwnerConflict {
                lock_text: String::from("rtx3080"),
                application_text: String::from("desktop"),
            })
        );
    }

    #[test]
    fn a_demand_of_an_application_that_is_not_eligible_is_refused() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let graphics_demand = fixture::graphics_demand_factory("strata", 7);

        let outcome = graphics_lock.try_record_demand(graphics_demand.clone());
        graphics_lock.assign_demand_owner(&graphics_demand);

        let expectation = Err(Error::ApplicationAbsence {
            lock_text: String::from("rtx3080"),
            application_text: String::from("strata"),
        });
        assert_eq!(outcome, expectation);
        assert_eq!(graphics_lock.maybe_owner(), None);
        assert!(graphics_lock.graphics_demands().is_empty());
    }

    #[test]
    fn the_demands_are_served_in_their_order_and_once_for_every_process() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let graphics_demand = [
            fixture::graphics_demand_factory("winix", 7),
            fixture::graphics_demand_factory("resolve", 8),
        ];

        for recorded_demand in graphics_demand.clone() {
            graphics_lock
                .try_record_demand(recorded_demand.clone())
                .unwrap();
            graphics_lock.try_record_demand(recorded_demand).unwrap();
        }

        assert_eq!(graphics_lock.graphics_demands(), &graphics_demand);
        assert_eq!(
            graphics_lock.maybe_first_demand(),
            Some(&graphics_demand[0])
        );
        graphics_lock.assign_demand_owner(&graphics_demand[0]);
        assert_eq!(
            graphics_lock.maybe_first_demand(),
            Some(&graphics_demand[1])
        );
    }

    #[test]
    fn a_demand_whose_process_ended_leaves_the_queue() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        for (text, process_identifier) in [("winix", 7), ("resolve", 8), ("desktop", 9)] {
            let graphics_demand = fixture::graphics_demand_factory(text, process_identifier);
            graphics_lock.try_record_demand(graphics_demand).unwrap();
        }

        graphics_lock.retain_demands(|process_identifier| process_identifier != 8);
        graphics_lock.drop_demand(9);

        let graphics_demand = fixture::graphics_demand_factory("winix", 7);
        assert_eq!(graphics_lock.graphics_demands(), &[graphics_demand]);
    }

    #[test]
    fn only_the_owner_releases_the_lock() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let eligible_name = fixture::application_name_factory("resolve");

        let outcome = graphics_lock.try_release(&eligible_name);
        let graphics_demand = fixture::graphics_demand_factory("winix", 7);
        graphics_lock
            .try_record_demand(graphics_demand.clone())
            .unwrap();
        graphics_lock.assign_demand_owner(&graphics_demand);
        let owner_text = graphics_lock.try_release(&eligible_name);
        graphics_lock.clear_owner();

        assert_eq!(
            outcome,
            Err(Error::OwnerMismatch {
                lock_text: String::from("rtx3080"),
                application_text: String::from("resolve"),
                owner_text: String::from("nobody"),
            })
        );
        assert_eq!(
            owner_text,
            Err(Error::OwnerMismatch {
                lock_text: String::from("rtx3080"),
                application_text: String::from("resolve"),
                owner_text: String::from("winix"),
            })
        );
        assert_eq!(graphics_lock.maybe_owner(), None);
    }

    #[test]
    fn a_dismissal_removes_the_application_its_demands_and_its_default_but_never_the_owner() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let default_owner = fixture::application_name_factory("desktop");
        graphics_lock
            .try_admit(default_owner.clone(), true)
            .unwrap();
        graphics_lock
            .try_record_demand(fixture::graphics_demand_factory("desktop", 9))
            .unwrap();
        let graphics_demand = fixture::graphics_demand_factory("winix", 7);
        graphics_lock
            .try_record_demand(graphics_demand.clone())
            .unwrap();
        graphics_lock.assign_demand_owner(&graphics_demand);
        let application_name = fixture::application_name_factory("winix");

        graphics_lock.try_dismiss(&default_owner).unwrap();
        graphics_lock
            .try_dismiss(&fixture::application_name_factory("resolve"))
            .unwrap();
        let outcome = graphics_lock.try_dismiss(&application_name);
        let maybe_owner = graphics_lock.try_dismiss(&default_owner);

        assert!(!graphics_lock.check_eligibility(&default_owner));
        assert_eq!(graphics_lock.maybe_default_owner(), None);
        assert!(graphics_lock.graphics_demands().is_empty());
        assert_eq!(
            outcome,
            Err(Error::OwnerConflict {
                lock_text: String::from("rtx3080"),
                application_text: String::from("winix"),
            })
        );
        assert_eq!(
            maybe_owner,
            Err(Error::ApplicationAbsence {
                lock_text: String::from("rtx3080"),
                application_text: String::from("desktop"),
            })
        );
    }

    #[test]
    fn the_consoles_and_the_passthrough_are_recorded_until_they_are_taken_back() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let console_user = GraphicsUser::ConsoleUser { console_index: 1 };

        graphics_lock.record_console_users(vec![console_user.clone(), console_user.clone()]);
        graphics_lock.record_passthrough(true);

        assert_eq!(graphics_lock.console_users(), &[console_user.clone()]);
        assert!(graphics_lock.passthrough_presence());
        assert!(graphics_lock.persistence_presence());
        assert_eq!(graphics_lock.remove_console_users(), vec![console_user]);
        assert!(graphics_lock.console_users().is_empty());
        assert!(graphics_lock.clear_passthrough());
        assert!(!graphics_lock.passthrough_presence());
        assert!(!graphics_lock.clear_passthrough());
    }

    #[test]
    fn the_path_of_an_application_lies_in_the_tree_of_its_lock() {
        let graphics_lock = fixture::graphics_lock_factory();
        let eligible_name = fixture::application_name_factory("resolve");

        let application_path_text = graphics_lock.application_path_text(&eligible_name);

        assert_eq!(application_path_text, "/baton/rtx3080/resolve");
    }

    #[test]
    fn a_recorded_owner_is_recovered_only_when_it_is_eligible() {
        let mut graphics_lock = fixture::graphics_lock_factory();
        let application_name = fixture::application_name_factory("winix");

        graphics_lock
            .try_recover_owner(application_name.clone())
            .unwrap();
        let outcome = graphics_lock.try_recover_owner(fixture::application_name_factory("strata"));

        assert_eq!(graphics_lock.maybe_owner(), Some(&application_name));
        assert_eq!(
            outcome,
            Err(Error::ApplicationAbsence {
                lock_text: String::from("rtx3080"),
                application_text: String::from("strata"),
            })
        );
    }
}
