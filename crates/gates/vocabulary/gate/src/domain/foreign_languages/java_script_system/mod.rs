use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct JavaScriptSystem;

impl Layer for JavaScriptSystem {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for JavaScriptSystem {}

impl JavaScriptSystem {
    const NAME: &'static str = "js-sys";

    const NAMES: &'static [&'static str] = &[
        "Array",
        "Date",
        "define_property",
        "delete_property",
        "eval",
        "Function",
        "into_option",
        "js_sys",
        "JsNullable",
        "JsString",
        "new",
        "Number",
        "Object",
        "Promise",
        "push",
        "Reflect",
        "reject_typed",
        "resolve",
        "set",
        "subarray",
        "to_vec",
        "Uint8Array",
        "Undefined",
        "wrap",
        "global",
    ];
}

impl TryFrom<JavaScriptSystem> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: JavaScriptSystem) -> Result<Self, Self::Error> {
        let foreign_name_text = JavaScriptSystem::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = JavaScriptSystem::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_java_script_system_is_a_name() {
        let java_script_system = JavaScriptSystem;

        let foreign_language = ForeignLanguage::try_from(java_script_system);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
