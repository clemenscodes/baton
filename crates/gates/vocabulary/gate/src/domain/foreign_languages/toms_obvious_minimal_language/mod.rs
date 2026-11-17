use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct TomsObviousMinimalLanguage;

impl Layer for TomsObviousMinimalLanguage {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for TomsObviousMinimalLanguage {}

impl TomsObviousMinimalLanguage {
    const NAME: &'static str = "toml";

    const NAMES: &'static [&'static str] = &["Table", "toml", "Value"];
}

impl TryFrom<TomsObviousMinimalLanguage> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: TomsObviousMinimalLanguage) -> Result<Self, Self::Error> {
        let foreign_name_text = TomsObviousMinimalLanguage::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = TomsObviousMinimalLanguage::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_toms_obvious_minimal_language_is_a_name() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;

        let foreign_language = ForeignLanguage::try_from(toms_obvious_minimal_language);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
