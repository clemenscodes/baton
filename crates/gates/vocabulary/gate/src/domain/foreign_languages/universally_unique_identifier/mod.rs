use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct UniversallyUniqueIdentifier;

impl Layer for UniversallyUniqueIdentifier {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for UniversallyUniqueIdentifier {}

impl UniversallyUniqueIdentifier {
    pub(crate) const NAME: &'static str = "uuid";

    const NAMES: &'static [&'static str] = &[
        "as_u128",
        "as_u64_pair",
        "Builder",
        "from_u128",
        "from_u64_pair",
        "get_variant",
        "get_version",
        "Hyphenated",
        "hyphenated",
        "into_uuid",
        "NAMESPACE_URL",
        "new_v5",
        "now_v7",
        "RFC4122",
        "SortRand",
        "Uuid",
        "uuid",
        "Variant",
        "Version",
        "with_variant",
        "with_version",
    ];
}

impl TryFrom<UniversallyUniqueIdentifier> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: UniversallyUniqueIdentifier) -> Result<Self, Self::Error> {
        let foreign_name_text = UniversallyUniqueIdentifier::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = UniversallyUniqueIdentifier::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_universally_unique_identifier_is_a_name() {
        let universally_unique_identifier = UniversallyUniqueIdentifier;

        let foreign_language = ForeignLanguage::try_from(universally_unique_identifier);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
