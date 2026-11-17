use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockCoreDeviceAddressContext;

impl Layer for StackLockCoreDeviceAddressContext {
    type Layer = DomainLayer;
}

impl StackLockCoreDeviceAddressContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/core/device/address";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Device", "Address"];
            let definition = "The crate stack_lock_core_device_address, the PCI address of a GPU, its domain, its bus, its slot and its function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Address"];
            let definition = "The PCI address of a function of a device, such as 0000:03:00.0, its domain, its bus, its slot and its function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The PCI domain of a device address, the first four hexadecimal digits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bus"];
            let definition = "The PCI bus of a device address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Slot"];
            let definition = "The slot of a device address, which every function of one card shares.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function"];
            let definition = "The function of a device address, from 0 to 7, such as the graphics and the audio of one card.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Slot", "Text"];
            let definition = "The text of the domain, the bus and the slot of a device address, such as 0000:03:00.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Share", "Slot"];
            let definition = "To tell whether two device addresses are functions of the same card.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Parts"];
            let definition = "The domain, the bus, the slot and the function of a text, where it is a device address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "The text of a device address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain", "Text"];
            let definition = "The text of the domain of a device address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bus", "Text"];
            let definition = "The text of the bus of a device address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function", "Text"];
            let definition = "The text of the function of a device address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Remainder", "Text"];
            let definition = "What of a text of a device address is left to read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Width", "Presence"];
            let definition = "Whether every part of a text of a device address has its width.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hexadecimal", "Radix"];
            let definition = "The radix 16 the parts of a device address are written in.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Slot", "Limit"];
            let definition = "The highest slot of a PCI bus, 31.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Function", "Limit"];
            let definition = "The highest function of a PCI slot, 7.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Address", "Text"];
            let definition = "The error of a text that is no device address.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockCoreDeviceAddressContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockCoreDeviceAddressContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockCoreDeviceAddressContext::DIRECTORY_NAME;
        let term_texts = StackLockCoreDeviceAddressContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_stack_lock_core_device_address_context_is_a_term() {
        let context_type = StackLockCoreDeviceAddressContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
