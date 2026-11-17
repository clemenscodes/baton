use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct RustyObjectNotation;

impl Layer for RustyObjectNotation {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for RustyObjectNotation {}

impl RustyObjectNotation {
    const NAME: &'static str = "ron";

    const NAMES: &'static [&'static str] = &[
        "de",
        "error",
        "extensions",
        "Extensions",
        "from_bytes",
        "from_str",
        "get_ron",
        "IMPLICIT_SOME",
        "Message",
        "Options",
        "PrettyConfig",
        "RawValue",
        "ron",
        "ser",
        "SpannedError",
        "struct_names",
        "to_string_pretty",
        "UNWRAP_VARIANT_NEWTYPES",
        "value",
        "with_default_extension",
    ];
}

impl TryFrom<RustyObjectNotation> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: RustyObjectNotation) -> Result<Self, Self::Error> {
        let foreign_name_text = RustyObjectNotation::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = RustyObjectNotation::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_rusty_object_notation_is_a_name() {
        let rusty_object_notation = RustyObjectNotation;

        let foreign_language = ForeignLanguage::try_from(rusty_object_notation);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
