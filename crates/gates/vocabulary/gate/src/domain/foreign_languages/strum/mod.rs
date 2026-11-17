use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Strum;

impl Layer for Strum {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Strum {}

impl Strum {
    const NAME: &'static str = "strum";

    const NAMES: &'static [&'static str] = &["strum", "VARIANTS", "VariantArray"];
}

impl TryFrom<Strum> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Strum) -> Result<Self, Self::Error> {
        let foreign_name_text = Strum::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Strum::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_strum_is_a_name() {
        let strum = Strum;

        let foreign_language = ForeignLanguage::try_from(strum);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
