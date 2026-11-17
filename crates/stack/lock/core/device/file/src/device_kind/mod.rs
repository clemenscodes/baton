use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum DeviceKind {
    CharacterDevice,
    BlockDevice,
}

impl Layer for DeviceKind {
    type Layer = DomainLayer;
}

impl ValueObject for DeviceKind {}

impl fmt::Display for DeviceKind {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CharacterDevice => write!(formatter, "character"),
            Self::BlockDevice => write!(formatter, "block"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_device_kind_reads_as_a_word() {
        let text = [DeviceKind::CharacterDevice, DeviceKind::BlockDevice]
            .map(|device_kind| device_kind.to_string());

        assert_eq!(text, ["character", "block"]);
    }
}
