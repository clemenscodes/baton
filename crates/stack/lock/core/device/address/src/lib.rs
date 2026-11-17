mod error;

pub use error::Error;

use stack_domain_driven_design_core::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct DeviceAddress {
    domain: u16,
    bus: u8,
    slot: u8,
    function: u8,
}

impl Layer for DeviceAddress {
    type Layer = DomainLayer;
}

impl ValueObject for DeviceAddress {}

impl Identifier for DeviceAddress {}

impl DeviceAddress {
    const HEXADECIMAL_RADIX: u32 = 16;
    const FUNCTION_LIMIT: u8 = 7;
    const SLOT_LIMIT: u8 = 0x1F;

    pub fn domain(&self) -> u16 {
        self.domain
    }

    pub fn bus(&self) -> u8 {
        self.bus
    }

    pub fn slot(&self) -> u8 {
        self.slot
    }

    pub fn function(&self) -> u8 {
        self.function
    }

    pub fn slot_text(&self) -> String {
        format!("{:04x}:{:02x}:{:02x}", self.domain, self.bus, self.slot)
    }

    pub fn share_slot(&self, device_address: &DeviceAddress) -> bool {
        (self.domain, self.bus, self.slot)
            == (
                device_address.domain,
                device_address.bus,
                device_address.slot,
            )
    }

    fn maybe_parts(text: &str) -> Option<(u16, u8, u8, u8)> {
        let (domain_text, remainder_text) = text.split_once(':')?;
        let (bus_text, remainder_text) = remainder_text.split_once(':')?;
        let (slot_text, function_text) = remainder_text.split_once('.')?;
        let width_presence = domain_text.len() == 4
            && bus_text.len() == 2
            && slot_text.len() == 2
            && function_text.len() == 1;
        if !width_presence {
            return None;
        }
        let domain = u16::from_str_radix(domain_text, Self::HEXADECIMAL_RADIX).ok()?;
        let bus = u8::from_str_radix(bus_text, Self::HEXADECIMAL_RADIX).ok()?;
        let slot = u8::from_str_radix(slot_text, Self::HEXADECIMAL_RADIX).ok()?;
        let function = u8::from_str_radix(function_text, Self::HEXADECIMAL_RADIX).ok()?;
        match slot <= Self::SLOT_LIMIT && function <= Self::FUNCTION_LIMIT {
            true => Some((domain, bus, slot, function)),
            false => None,
        }
    }
}

impl TryFrom<String> for DeviceAddress {
    type Error = Error;

    fn try_from(text: String) -> Result<Self, Self::Error> {
        match Self::maybe_parts(&text) {
            Some((domain, bus, slot, function)) => Ok(Self {
                domain,
                bus,
                slot,
                function,
            }),
            None => Err(Error::AddressText { text }),
        }
    }
}

impl fmt::Display for DeviceAddress {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}.{:x}", self.slot_text(), self.function)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_address_of_sysfs_is_a_device_address_with_its_parts() {
        let device_address = DeviceAddress::try_from(String::from("0000:03:1f.1")).unwrap();

        let slot_text = device_address.slot_text();

        assert_eq!(device_address.domain(), 0);
        assert_eq!(device_address.bus(), 3);
        assert_eq!(device_address.slot(), 0x1F);
        assert_eq!(device_address.function(), 1);
        assert_eq!(slot_text, "0000:03:1f");
        assert_eq!(device_address.to_string(), "0000:03:1f.1");
    }

    #[test]
    fn two_functions_of_one_card_share_a_slot_and_another_card_does_not() {
        let device_address = ["0000:03:00.0", "0000:03:00.1", "0000:03:01.0"]
            .map(|text| DeviceAddress::try_from(String::from(text)).unwrap());

        let share_slot = device_address[0].share_slot(&device_address[1]);

        assert!(share_slot);
        assert!(!device_address[0].share_slot(&device_address[2]));
    }

    #[test]
    fn a_text_of_another_shape_is_no_device_address() {
        let text = [
            "",
            "03:00.0",
            "0000:03:00",
            "0000:03:00.8",
            "0000:03:20.0",
            "0000:0g:00.0",
            "000g:03:00.0",
            "0000:03:0g.0",
            "0000:03:00.g",
            "00000:03:00.0",
        ];

        let outcomes = text.map(|text| DeviceAddress::try_from(String::from(text)));

        let expectations = text.map(|text| {
            Err(Error::AddressText {
                text: String::from(text),
            })
        });
        assert_eq!(outcomes, expectations);
    }
}
