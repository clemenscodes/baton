use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Gilrs;

impl Layer for Gilrs {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Gilrs {}

impl Gilrs {
    const NAME: &'static str = "gilrs";

    const NAMES: &'static [&'static str] = &[
        "Axis",
        "AxisChanged",
        "build",
        "Button",
        "ButtonChanged",
        "ButtonPressed",
        "ButtonReleased",
        "C",
        "Code",
        "Connected",
        "Disconnected",
        "DPadDown",
        "DPadLeft",
        "DPadRight",
        "DPadUp",
        "DPadX",
        "DPadY",
        "Dropped",
        "East",
        "Error",
        "ev",
        "Event",
        "event",
        "EventType",
        "GamepadId",
        "Gilrs",
        "gilrs",
        "GilrsBuilder",
        "id",
        "insert_event",
        "InvalidAxisToBtn",
        "LeftStickX",
        "LeftStickY",
        "LeftThumb",
        "LeftTrigger",
        "LeftTrigger2",
        "LeftZ",
        "Mode",
        "new",
        "next_event",
        "next_event_blocking",
        "North",
        "RightStickX",
        "RightStickY",
        "RightThumb",
        "RightTrigger",
        "RightTrigger2",
        "RightZ",
        "Select",
        "set_update_state",
        "South",
        "Start",
        "Unknown",
        "West",
        "with_default_filters",
        "Z",
    ];
}

impl TryFrom<Gilrs> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Gilrs) -> Result<Self, Self::Error> {
        let foreign_name_text = Gilrs::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Gilrs::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_gilrs_is_a_name() {
        let gilrs = Gilrs;

        let foreign_language = ForeignLanguage::try_from(gilrs);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
