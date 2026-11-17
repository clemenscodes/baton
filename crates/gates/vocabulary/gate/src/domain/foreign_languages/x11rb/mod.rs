use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct X11rb;

impl Layer for X11rb {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for X11rb {}

impl X11rb {
    const NAME: &'static str = "x11rb";

    const NAMES: &'static [&'static str] = &[
        "BUTTON_PRESS_EVENT",
        "BUTTON_RELEASE_EVENT",
        "children",
        "connect",
        "Connection",
        "connection",
        "ConnectionExt",
        "CURRENT_TIME",
        "get_window_attributes",
        "InputFocus",
        "KEY_PRESS_EVENT",
        "KEY_RELEASE_EVENT",
        "map_state",
        "MapState",
        "MOTION_NOTIFY_EVENT",
        "NONE",
        "PARENT",
        "protocol",
        "query_tree",
        "reply",
        "root",
        "roots",
        "rust_connection",
        "RustConnection",
        "set_input_focus",
        "setup",
        "sync",
        "VIEWABLE",
        "Window",
        "wrapper",
        "x11rb",
        "xproto",
        "xtest",
        "xtest_fake_input",
    ];
}

impl TryFrom<X11rb> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: X11rb) -> Result<Self, Self::Error> {
        let foreign_name_text = X11rb::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = X11rb::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_x11rb_is_a_name() {
        let x11rb = X11rb;

        let foreign_language = ForeignLanguage::try_from(x11rb);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
