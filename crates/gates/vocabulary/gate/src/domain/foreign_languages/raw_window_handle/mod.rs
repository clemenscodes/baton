use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct RawWindowHandle;

impl Layer for RawWindowHandle {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for RawWindowHandle {}

impl RawWindowHandle {
    const NAME: &'static str = "raw-window-handle";

    const NAMES: &'static [&'static str] = &[
        "as_raw",
        "borrow_raw",
        "display_handle",
        "DisplayHandle",
        "HandleError",
        "HasDisplayHandle",
        "HasWindowHandle",
        "raw_window_handle",
        "RawDisplayHandle",
        "RawWindowHandle",
        "Unavailable",
        "web",
        "Web",
        "WebWindowHandle",
        "window_handle",
        "WindowHandle",
    ];
}

impl TryFrom<RawWindowHandle> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: RawWindowHandle) -> Result<Self, Self::Error> {
        let foreign_name_text = RawWindowHandle::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = RawWindowHandle::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_raw_window_handle_is_a_name() {
        let raw_window_handle = RawWindowHandle;

        let foreign_language = ForeignLanguage::try_from(raw_window_handle);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
