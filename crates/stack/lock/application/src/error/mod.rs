use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    Name {
        text: String,
    },
    DeviceAddress {
        text: String,
    },
    GraphicsLock {
        text: String,
    },
    DeviceProgram {
        text: String,
    },
    DeviceTree {
        text: String,
    },
    ControlGroup {
        text: String,
    },
    LockRecord {
        text: String,
    },
    TaskTable {
        text: String,
    },
    HostHelper {
        text: String,
    },
    ProgramRunner {
        text: String,
    },
    LockAbsence {
        lock_text: String,
    },
    OwnerPresence {
        lock_text: String,
        owner_text: String,
    },
    DeadlineExpiry {
        lock_text: String,
        user_text: String,
    },
    UserSurvival {
        lock_text: String,
        user_text: String,
    },
}

impl Layer for Error {
    type Layer = ApplicationLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Name { text } | Self::DeviceAddress { text } | Self::GraphicsLock { text } => {
                write!(formatter, "{text}")
            }
            Self::DeviceProgram { text } => {
                write!(formatter, "the device program failed, {text}")
            }
            Self::DeviceTree { text } => write!(formatter, "the device failed, {text}"),
            Self::ControlGroup { text } => write!(formatter, "the cgroup failed, {text}"),
            Self::LockRecord { text } => {
                write!(formatter, "the record of the lock failed, {text}")
            }
            Self::TaskTable { text } => {
                write!(formatter, "the processes of the host failed, {text}")
            }
            Self::HostHelper { text } => write!(formatter, "baton_host failed, {text}"),
            Self::ProgramRunner { text } => write!(formatter, "the program failed, {text}"),
            Self::LockAbsence { lock_text } => write!(
                formatter,
                "There is no lock {lock_text}, enclose a GPU in it first."
            ),
            Self::OwnerPresence {
                lock_text,
                owner_text,
            } => write!(
                formatter,
                "{owner_text} owns the lock {lock_text}, so it is removed only after the release."
            ),
            Self::DeadlineExpiry {
                lock_text,
                user_text,
            } => write!(
                formatter,
                "The lock {lock_text} stayed in use by {user_text} until the deadline."
            ),
            Self::UserSurvival {
                lock_text,
                user_text,
            } => write!(
                formatter,
                "The lock {lock_text} could not be taken, {user_text} survived its end."
            ),
        }
    }
}

impl error::Error for Error {}

impl From<stack_lock_core_name::Error> for Error {
    fn from(error: stack_lock_core_name::Error) -> Self {
        let text = error.to_string();
        Self::Name { text }
    }
}

impl From<stack_lock_core_device_address::Error> for Error {
    fn from(error: stack_lock_core_device_address::Error) -> Self {
        let text = error.to_string();
        Self::DeviceAddress { text }
    }
}

impl From<stack_lock_core_graphics_lock::Error> for Error {
    fn from(error: stack_lock_core_graphics_lock::Error) -> Self {
        let text = error.to_string();
        Self::GraphicsLock { text }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_application_reads_as_a_sentence() {
        let text = String::from("it broke");
        let lock_text = String::from("rtx3080");
        let error = [
            Error::Name { text: text.clone() },
            Error::DeviceAddress { text: text.clone() },
            Error::GraphicsLock { text: text.clone() },
            Error::DeviceProgram { text: text.clone() },
            Error::DeviceTree { text: text.clone() },
            Error::ControlGroup { text: text.clone() },
            Error::LockRecord { text: text.clone() },
            Error::TaskTable { text: text.clone() },
            Error::HostHelper { text: text.clone() },
            Error::ProgramRunner { text },
            Error::LockAbsence {
                lock_text: lock_text.clone(),
            },
            Error::OwnerPresence {
                lock_text: lock_text.clone(),
                owner_text: String::from("winix"),
            },
            Error::DeadlineExpiry {
                lock_text: lock_text.clone(),
                user_text: String::from("the process 7 resolve"),
            },
            Error::UserSurvival {
                lock_text,
                user_text: String::from("the process 7 resolve"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "it broke",
                "it broke",
                "it broke",
                "the device program failed, it broke",
                "the device failed, it broke",
                "the cgroup failed, it broke",
                "the record of the lock failed, it broke",
                "the processes of the host failed, it broke",
                "baton_host failed, it broke",
                "the program failed, it broke",
                "There is no lock rtx3080, enclose a GPU in it first.",
                "winix owns the lock rtx3080, so it is removed only after the release.",
                "The lock rtx3080 stayed in use by the process 7 resolve until the deadline.",
                "The lock rtx3080 could not be taken, the process 7 resolve survived its end.",
            ]
        );
    }

    #[test]
    fn an_error_of_the_core_becomes_an_error_of_the_application() {
        let error = [
            Error::from(stack_lock_core_name::LockName::try_from(String::new()).unwrap_err()),
            Error::from(
                stack_lock_core_device_address::DeviceAddress::try_from(String::new()).unwrap_err(),
            ),
        ];

        assert!(matches!(error[0], Error::Name { .. }));
        assert!(matches!(error[1], Error::DeviceAddress { .. }));
    }
}
