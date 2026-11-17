pub(crate) mod java_script_object_notation;

use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Serde;

impl Layer for Serde {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Serde {}

impl Serde {
    const NAME: &'static str = "serde";

    const NAMES: &'static [&'static str] = &[
        "custom",
        "de",
        "deny_unknown_fields",
        "deserialize",
        "Deserialize",
        "deserialize_enum",
        "deserialize_identifier",
        "deserialize_with",
        "DeserializeOwned",
        "DeserializeSeed",
        "Deserializer",
        "EnumAccess",
        "EnumAccessDeserializer",
        "Expected",
        "expecting",
        "IgnoredAny",
        "into_deserializer",
        "IntoDeserializer",
        "rename",
        "rename_all",
        "serde",
        "Serialize",
        "serialize",
        "serialize_none",
        "serialize_with",
        "Serializer",
        "skip_serializing_if",
        "tag",
        "StringDeserializer",
        "transparent",
        "untagged",
        "value",
        "VariantAccess",
        "variant_seed",
        "visit_enum",
        "visit_str",
        "Visitor",
    ];
}

impl TryFrom<Serde> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Serde) -> Result<Self, Self::Error> {
        let foreign_name_text = Serde::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Serde::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_serde_is_a_name() {
        let serde = Serde;

        let foreign_language = ForeignLanguage::try_from(serde);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
