use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(in crate::domain::foreign_languages) struct InternationalComponentsForUnicodeLocaleCore;

impl Layer for InternationalComponentsForUnicodeLocaleCore {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for InternationalComponentsForUnicodeLocaleCore {}

impl InternationalComponentsForUnicodeLocaleCore {
    const NAME: &'static str = "icu_locale_core";

    const NAMES: &'static [&'static str] = &[
        "icu_locale_core",
        "InvalidLanguage",
        "LanguageIdentifier",
        "ParseError",
        "try_from_str",
    ];
}

impl TryFrom<InternationalComponentsForUnicodeLocaleCore> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: InternationalComponentsForUnicodeLocaleCore) -> Result<Self, Self::Error> {
        let foreign_name_text = InternationalComponentsForUnicodeLocaleCore::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = InternationalComponentsForUnicodeLocaleCore::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_international_components_for_unicode_locale_core_is_a_name() {
        let international_components_for_unicode_locale_core = InternationalComponentsForUnicodeLocaleCore;

        let foreign_language = ForeignLanguage::try_from(
            international_components_for_unicode_locale_core,
        );

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
