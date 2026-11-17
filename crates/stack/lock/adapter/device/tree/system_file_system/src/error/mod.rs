use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::error;
use std::fmt;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum Error {
    DeviceAbsence {
        address_text: String,
    },
    GroupAbsence {
        address_text: String,
    },
    GroupBridge {
        address_text: String,
    },
    GroupMember {
        address_text: String,
    },
    OutputFailure {
        path: PathBuf,
        text: String,
    },
    BindingDeadline {
        path: PathBuf,
    },
    StartFailure {
        text: String,
    },
    CompletionFailure {
        command_text: String,
        status: String,
    },
}

impl Layer for Error {
    type Layer = InfrastructureLayer;
}

impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::DeviceAbsence { address_text } => {
                write!(formatter, "there is no PCI device {address_text}")
            }
            Self::GroupAbsence { address_text } => write!(
                formatter,
                "{address_text} has no IOMMU group, so the IOMMU is off and nothing is passed through"
            ),
            Self::GroupBridge { address_text } => write!(
                formatter,
                "the IOMMU group holds the bridge {address_text}, so it is never passed through"
            ),
            Self::GroupMember { address_text } => write!(
                formatter,
                "the IOMMU group holds {address_text} of another card, so it is never passed through"
            ),
            Self::OutputFailure { path, text } => {
                write!(formatter, "{} could not be written, {text}", path.display())
            }
            Self::BindingDeadline { path } => write!(
                formatter,
                "{} did not change within the deadline of the binding",
                path.display()
            ),
            Self::StartFailure { text } => {
                write!(formatter, "a program could not be started, {text}")
            }
            Self::CompletionFailure {
                command_text,
                status,
            } => write!(formatter, "{command_text} ended with {status}"),
        }
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_error_of_the_tree_reads_as_a_phrase() {
        let address_text = String::from("0000:03:00.0");
        let path = PathBuf::from("/sys/bus/pci/drivers/vfio-pci/bind");
        let text = String::from("it broke");
        let error = [
            Error::DeviceAbsence {
                address_text: address_text.clone(),
            },
            Error::GroupAbsence {
                address_text: address_text.clone(),
            },
            Error::GroupBridge {
                address_text: address_text.clone(),
            },
            Error::GroupMember { address_text },
            Error::OutputFailure {
                path: path.clone(),
                text: text.clone(),
            },
            Error::BindingDeadline { path },
            Error::StartFailure { text: text.clone() },
            Error::CompletionFailure {
                command_text: String::from("nvidia-smi -pm 0"),
                status: String::from("exit status: 1"),
            },
        ];

        let text = error.map(|error| error.to_string());

        assert_eq!(
            text,
            [
                "there is no PCI device 0000:03:00.0",
                "0000:03:00.0 has no IOMMU group, so the IOMMU is off and nothing is passed through",
                "the IOMMU group holds the bridge 0000:03:00.0, so it is never passed through",
                "the IOMMU group holds 0000:03:00.0 of another card, so it is never passed through",
                "/sys/bus/pci/drivers/vfio-pci/bind could not be written, it broke",
                "/sys/bus/pci/drivers/vfio-pci/bind did not change within the deadline of the binding",
                "a program could not be started, it broke",
                "nvidia-smi -pm 0 ended with exit status: 1",
            ]
        );
    }
}
