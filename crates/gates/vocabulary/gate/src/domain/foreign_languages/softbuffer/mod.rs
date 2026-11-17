use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Softbuffer;

impl Layer for Softbuffer {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Softbuffer {}

impl Softbuffer {
    const NAME: &'static str = "softbuffer";

    const NAMES: &'static [&'static str] = &[
        "Buffer",
        "buffer_mut",
        "Context",
        "present",
        "resize",
        "softbuffer",
        "Surface",
    ];
}

impl TryFrom<Softbuffer> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Softbuffer) -> Result<Self, Self::Error> {
        let foreign_name_text = Softbuffer::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Softbuffer::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_softbuffer_is_a_name() {
        let softbuffer = Softbuffer;

        let foreign_language = ForeignLanguage::try_from(softbuffer);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
