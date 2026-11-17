use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::error;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    DeviceRepetition {
        lock_text: String,
        address_text: String,
    },
    ApplicationAbsence {
        lock_text: String,
        application_text: String,
    },
    DefaultOwnerConflict {
        lock_text: String,
        application_text: String,
    },
    OwnerConflict {
        lock_text: String,
        application_text: String,
    },
    OwnerMismatch {
        lock_text: String,
        application_text: String,
        owner_text: String,
    },
}

impl Layer for Error {
    type Layer = DomainLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceRepetition {
                lock_text,
                address_text,
            } => write!(
                formatter,
                "The lock {lock_text} already holds the device {address_text}."
            ),
            Self::ApplicationAbsence {
                lock_text,
                application_text,
            } => write!(
                formatter,
                "{application_text} is no eligible application of the lock {lock_text}, admit it first."
            ),
            Self::DefaultOwnerConflict {
                lock_text,
                application_text,
            } => write!(
                formatter,
                "The lock {lock_text} already has the default owner {application_text}."
            ),
            Self::OwnerConflict {
                lock_text,
                application_text,
            } => write!(
                formatter,
                "{application_text} owns the lock {lock_text} and is dismissed only after its release."
            ),
            Self::OwnerMismatch {
                lock_text,
                application_text,
                owner_text,
            } => write!(
                formatter,
                "{application_text} does not own the lock {lock_text}, {owner_text} does."
            ),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_a_lock_reads_as_a_sentence() {
        let lock_text = String::from("rtx3080");
        let application_text = String::from("winix");
        let error = [
            Error::DeviceRepetition {
                lock_text: lock_text.clone(),
                address_text: String::from("0000:03:00.0"),
            },
            Error::ApplicationAbsence {
                lock_text: lock_text.clone(),
                application_text: application_text.clone(),
            },
            Error::DefaultOwnerConflict {
                lock_text: lock_text.clone(),
                application_text: application_text.clone(),
            },
            Error::OwnerConflict {
                lock_text: lock_text.clone(),
                application_text: application_text.clone(),
            },
            Error::OwnerMismatch {
                lock_text,
                application_text,
                owner_text: String::from("resolve"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "The lock rtx3080 already holds the device 0000:03:00.0.",
                "winix is no eligible application of the lock rtx3080, admit it first.",
                "The lock rtx3080 already has the default owner winix.",
                "winix owns the lock rtx3080 and is dismissed only after its release.",
                "winix does not own the lock rtx3080, resolve does.",
            ]
        );
    }
}
