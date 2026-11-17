use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_lock_core_device_address::DeviceAddress;
use stack_lock_core_name::LockName;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Enclosure {
    lock_name: LockName,
    device_address: DeviceAddress,
}

impl Layer for Enclosure {
    type Layer = ApplicationLayer;
}

impl Enclosure {
    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }

    pub fn device_address(&self) -> DeviceAddress {
        self.device_address
    }
}

impl TryFrom<(String, String)> for Enclosure {
    type Error = Error;

    fn try_from((lock_text, address_text): (String, String)) -> Result<Self, Self::Error> {
        let lock_name = LockName::try_from(lock_text)?;
        let device_address = DeviceAddress::try_from(address_text)?;
        Ok(Self {
            lock_name,
            device_address,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_enclosure_keeps_its_lock_and_its_device() {
        let enclosure = Enclosure::try_from((
            String::from("rtx3080"),
            String::from("0000:03:00.0"),
        ))
        .unwrap();

        assert_eq!(enclosure.lock_name().text(), "rtx3080");
        assert_eq!(enclosure.device_address().to_string(), "0000:03:00.0");
    }

    #[test]
    fn an_enclosure_of_a_wrong_address_is_refused() {
        let outcome = Enclosure::try_from((String::from("rtx3080"), String::from("03:00.0")));

        assert!(matches!(outcome, Err(Error::DeviceAddress { .. })));
    }

    #[test]
    fn the_command_of_a_wrong_lock_is_refused() {
        let outcome = Enclosure::try_from((String::from("--lock"), String::from("0000:03:00.0")));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }
}
