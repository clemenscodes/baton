use serde::{Deserialize, Serialize};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_lock_core_demand::GraphicsDemand;
use stack_lock_core_device_address::DeviceAddress;
use stack_lock_core_device_file::{DeviceFile, DeviceKind};
use stack_lock_core_graphics_lock::GraphicsLock;
use stack_lock_core_graphics_user::GraphicsUser;
use stack_lock_core_name::{ApplicationName, LockName};
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct LockDocument {
    lock_name: String,
    map_index: u32,
    device_addresses: Vec<String>,
    device_files: Vec<FileDocument>,
    application_names: Vec<String>,
    maybe_default_owner: Option<String>,
    #[serde(default)]
    yield_presence: bool,
    maybe_owner: Option<String>,
    graphics_demands: Vec<DemandDocument>,
    console_users: Vec<UserDocument>,
    passthrough_presence: bool,
    persistence_presence: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct FileDocument {
    character_presence: bool,
    major: u32,
    minor: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct DemandDocument {
    application_name: String,
    waiter_identifier: u32,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) enum UserDocument {
    ProcessUser {
        process_identifier: u32,
        command_text: String,
        control_group_text: String,
    },
    ConsoleUser {
        console_index: u32,
    },
    FramebufferUser {
        device_text: String,
        driver_text: String,
    },
}

impl Layer for LockDocument {
    type Layer = InfrastructureLayer;
}

impl Layer for FileDocument {
    type Layer = InfrastructureLayer;
}

impl Layer for DemandDocument {
    type Layer = InfrastructureLayer;
}

impl Layer for UserDocument {
    type Layer = InfrastructureLayer;
}

impl From<&GraphicsLock> for LockDocument {
    fn from(graphics_lock: &GraphicsLock) -> Self {
        let name_text = |application_name: &ApplicationName| application_name.to_string();
        Self {
            lock_name: graphics_lock.lock_name().to_string(),
            map_index: graphics_lock.map_index(),
            device_addresses: Vec::from_iter(
                graphics_lock
                    .device_addresses()
                    .iter()
                    .map(DeviceAddress::to_string),
            ),
            device_files: Vec::from_iter(
                graphics_lock.device_files().iter().map(FileDocument::from),
            ),
            application_names: Vec::from_iter(
                graphics_lock.application_names().iter().map(name_text),
            ),
            maybe_default_owner: graphics_lock.maybe_default_owner().map(name_text),
            yield_presence: graphics_lock.yield_presence(),
            maybe_owner: graphics_lock.maybe_owner().map(name_text),
            graphics_demands: Vec::from_iter(
                graphics_lock
                    .graphics_demands()
                    .iter()
                    .map(DemandDocument::from),
            ),
            console_users: Vec::from_iter(
                graphics_lock.console_users().iter().map(UserDocument::from),
            ),
            passthrough_presence: graphics_lock.passthrough_presence(),
            persistence_presence: graphics_lock.persistence_presence(),
        }
    }
}

impl From<&DeviceFile> for FileDocument {
    fn from(device_file: &DeviceFile) -> Self {
        Self {
            character_presence: device_file.device_kind() == DeviceKind::CharacterDevice,
            major: device_file.major(),
            minor: device_file.minor(),
        }
    }
}

impl From<&GraphicsDemand> for DemandDocument {
    fn from(graphics_demand: &GraphicsDemand) -> Self {
        Self {
            application_name: graphics_demand.application_name().to_string(),
            waiter_identifier: graphics_demand.waiter_identifier(),
        }
    }
}

impl From<&GraphicsUser> for UserDocument {
    fn from(graphics_user: &GraphicsUser) -> Self {
        match graphics_user.clone() {
            GraphicsUser::ProcessUser {
                process_identifier,
                command_text,
                control_group_text,
            } => Self::ProcessUser {
                process_identifier,
                command_text,
                control_group_text,
            },
            GraphicsUser::ConsoleUser { console_index } => Self::ConsoleUser { console_index },
            GraphicsUser::FramebufferUser {
                device_text,
                driver_text,
            } => Self::FramebufferUser {
                device_text,
                driver_text,
            },
        }
    }
}

impl From<UserDocument> for GraphicsUser {
    fn from(user_document: UserDocument) -> Self {
        match user_document {
            UserDocument::ProcessUser {
                process_identifier,
                command_text,
                control_group_text,
            } => Self::ProcessUser {
                process_identifier,
                command_text,
                control_group_text,
            },
            UserDocument::ConsoleUser { console_index } => Self::ConsoleUser { console_index },
            UserDocument::FramebufferUser {
                device_text,
                driver_text,
            } => Self::FramebufferUser {
                device_text,
                driver_text,
            },
        }
    }
}

impl LockDocument {
    fn compose_text(error: impl fmt::Display) -> String {
        error.to_string()
    }

    fn compose_device_file(file_document: FileDocument) -> DeviceFile {
        let device_kind = match file_document.character_presence {
            true => DeviceKind::CharacterDevice,
            false => DeviceKind::BlockDevice,
        };
        DeviceFile::from((device_kind, file_document.major, file_document.minor))
    }
}

impl TryFrom<LockDocument> for GraphicsLock {
    type Error = String;

    fn try_from(lock_document: LockDocument) -> Result<Self, Self::Error> {
        let LockDocument {
            lock_name,
            map_index,
            device_addresses,
            device_files,
            application_names,
            maybe_default_owner,
            yield_presence,
            maybe_owner,
            graphics_demands,
            console_users,
            passthrough_presence,
            persistence_presence,
        } = lock_document;
        let lock_name = LockName::try_from(lock_name).map_err(LockDocument::compose_text)?;
        let mut graphics_lock = GraphicsLock::from((lock_name, map_index));
        device_addresses.into_iter().try_for_each(|address_text| {
            DeviceAddress::try_from(address_text)
                .map_err(LockDocument::compose_text)
                .and_then(|device_address| {
                    graphics_lock
                        .try_add_device(device_address)
                        .map_err(LockDocument::compose_text)
                })
        })?;
        graphics_lock.replace_device_files(Vec::from_iter(
            device_files
                .into_iter()
                .map(LockDocument::compose_device_file),
        ));
        application_names
            .into_iter()
            .try_for_each(|application_text| {
                let default_presence = maybe_default_owner.as_ref() == Some(&application_text);
                ApplicationName::try_from(application_text)
                    .map_err(LockDocument::compose_text)
                    .and_then(|application_name| {
                        graphics_lock
                            .try_admit(application_name, default_presence)
                            .map_err(LockDocument::compose_text)
                    })
            })?;
        graphics_lock.record_yield_presence(yield_presence);
        graphics_demands
            .into_iter()
            .try_for_each(|demand_document| {
                ApplicationName::try_from(demand_document.application_name)
                    .map_err(LockDocument::compose_text)
                    .and_then(|application_name| {
                        let graphics_demand = GraphicsDemand::from((
                            application_name,
                            demand_document.waiter_identifier,
                        ));
                        graphics_lock
                            .try_record_demand(graphics_demand)
                            .map_err(LockDocument::compose_text)
                    })
            })?;
        maybe_owner.map_or(Ok(()), |owner_text| {
            ApplicationName::try_from(owner_text)
                .map_err(LockDocument::compose_text)
                .and_then(|owner_name| {
                    graphics_lock
                        .try_recover_owner(owner_name)
                        .map_err(LockDocument::compose_text)
                })
        })?;
        graphics_lock.record_console_users(Vec::from_iter(
            console_users.into_iter().map(GraphicsUser::from),
        ));
        if passthrough_presence {
            graphics_lock.record_passthrough(persistence_presence);
        }
        Ok(graphics_lock)
    }
}
