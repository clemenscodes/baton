use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Arboard;

impl Layer for Arboard {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Arboard {}

impl Arboard {
    const NAME: &'static str = "arboard";

    const NAMES: &'static [&'static str] = &[
        "arboard",
        "Clipboard",
        "ClipboardNotSupported",
        "ClipboardOccupied",
        "ContentNotAvailable",
        "Error",
        "get_text",
        "new",
        "set_text",
    ];
}

impl TryFrom<Arboard> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Arboard) -> Result<Self, Self::Error> {
        let foreign_name_text = Arboard::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Arboard::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_arboard_is_a_name() {
        let arboard = Arboard;

        let foreign_language = ForeignLanguage::try_from(arboard);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
