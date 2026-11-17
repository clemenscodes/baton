mod device_kind;

pub use device_kind::DeviceKind;

use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct DeviceFile {
    device_kind: DeviceKind,
    major: u32,
    minor: u32,
}

impl Layer for DeviceFile {
    type Layer = DomainLayer;
}

impl ValueObject for DeviceFile {}

impl DeviceFile {
    pub fn device_kind(&self) -> DeviceKind {
        self.device_kind
    }

    pub fn major(&self) -> u32 {
        self.major
    }

    pub fn minor(&self) -> u32 {
        self.minor
    }
}

impl From<(DeviceKind, u32, u32)> for DeviceFile {
    fn from((device_kind, major, minor): (DeviceKind, u32, u32)) -> Self {
        Self {
            device_kind,
            major,
            minor,
        }
    }
}

impl fmt::Display for DeviceFile {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} {}:{}",
            self.device_kind, self.major, self.minor
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_device_file_keeps_its_kind_its_major_and_its_minor() {
        let device_file = DeviceFile::from((DeviceKind::CharacterDevice, 195, 1));

        let text = device_file.to_string();

        assert_eq!(device_file.device_kind(), DeviceKind::CharacterDevice);
        assert_eq!(device_file.major(), 195);
        assert_eq!(device_file.minor(), 1);
        assert_eq!(text, "character 195:1");
    }
}
