use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct GetRandom;

impl Layer for GetRandom {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for GetRandom {}

impl GetRandom {
    const NAME: &'static str = "getrandom";

    const NAMES: &'static [&'static str] = &["fill", "getrandom", "UNSUPPORTED"];
}

impl TryFrom<GetRandom> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: GetRandom) -> Result<Self, Self::Error> {
        let foreign_name_text = GetRandom::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = GetRandom::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_getrandom_is_a_name() {
        let foreign_crate = GetRandom;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
