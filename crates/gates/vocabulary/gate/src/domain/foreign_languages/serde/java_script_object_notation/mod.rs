use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct SerdeJavaScriptObjectNotation;

impl Layer for SerdeJavaScriptObjectNotation {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for SerdeJavaScriptObjectNotation {}

impl SerdeJavaScriptObjectNotation {
    pub(crate) const NAME: &'static str = "serde_json";

    const NAMES: &'static [&'static str] = &[
        "as_array",
        "as_array_mut",
        "as_object",
        "as_object_mut",
        "as_str",
        "from_slice",
        "from_str",
        "from_string",
        "get",
        "insert",
        "json",
        "Number",
        "RawValue",
        "remove",
        "serde_json",
        "to_string",
        "to_string_pretty",
        "pointer",
        "to_value",
        "to_vec_pretty",
        "Value",
        "values",
    ];
}

impl TryFrom<SerdeJavaScriptObjectNotation> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: SerdeJavaScriptObjectNotation) -> Result<Self, Self::Error> {
        let foreign_name_text = SerdeJavaScriptObjectNotation::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = SerdeJavaScriptObjectNotation::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_serde_java_script_object_notation_is_a_name() {
        let serde_java_script_object_notation = SerdeJavaScriptObjectNotation;

        let foreign_language = ForeignLanguage::try_from(serde_java_script_object_notation);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
