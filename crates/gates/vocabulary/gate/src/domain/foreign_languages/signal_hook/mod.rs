use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct SignalHook;

impl Layer for SignalHook {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for SignalHook {}

impl SignalHook {
    const NAME: &'static str = "signal-hook";

    const NAMES: &'static [&'static str] = &[
        "consts",
        "flag",
        "low_level",
        "raise",
        "register",
        "register_conditional_default",
        "SigId",
        "SIGHUP",
        "SIGINT",
        "signal",
        "signal_hook",
        "SIGTERM",
    ];
}

impl TryFrom<SignalHook> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: SignalHook) -> Result<Self, Self::Error> {
        let foreign_name_text = SignalHook::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = SignalHook::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_signal_hook_is_a_name() {
        let signal_hook = SignalHook;

        let foreign_language = ForeignLanguage::try_from(signal_hook);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
