pub(in crate::domain::foreign_languages) mod core;

use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(in crate::domain::foreign_languages) struct InternationalComponentsForUnicodeLocale;

impl Layer for InternationalComponentsForUnicodeLocale {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for InternationalComponentsForUnicodeLocale {}

impl InternationalComponentsForUnicodeLocale {
    const NAME: &'static str = "icu_locale";

    const NAMES: &'static [&'static str] = &[
        "icu_locale",
        "is_right_to_left",
        "LocaleDirectionality",
        "LocaleExpander",
        "maximize",
        "new_common",
    ];
}

impl TryFrom<InternationalComponentsForUnicodeLocale> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: InternationalComponentsForUnicodeLocale) -> Result<Self, Self::Error> {
        let foreign_name_text = InternationalComponentsForUnicodeLocale::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = InternationalComponentsForUnicodeLocale::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_international_components_for_unicode_locale_is_a_name() {
        let international_components_for_unicode_locale = InternationalComponentsForUnicodeLocale;

        let foreign_language = ForeignLanguage::try_from(
            international_components_for_unicode_locale,
        );

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
