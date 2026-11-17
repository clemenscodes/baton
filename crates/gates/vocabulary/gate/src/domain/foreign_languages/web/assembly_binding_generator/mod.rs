pub(in crate::domain::foreign_languages) mod futures;
pub(in crate::domain::foreign_languages) mod test;

use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(in crate::domain::foreign_languages) struct WebAssemblyBindingGenerator;

impl Layer for WebAssemblyBindingGenerator {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for WebAssemblyBindingGenerator {}

impl WebAssemblyBindingGenerator {
    const NAME: &'static str = "wasm-bindgen";

    const NAMES: &'static [&'static str] = &[
        "as_string",
        "Closure",
        "closure",
        "forget",
        "from_str",
        "into_js_value",
        "is_undefined",
        "JsCast",
        "JsValue",
        "NULL",
        "throw_str",
        "throw_val",
        "TRUE",
        "unchecked_into",
        "unchecked_ref",
        "UNDEFINED",
        "wasm_bindgen",
    ];
}

impl TryFrom<WebAssemblyBindingGenerator> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: WebAssemblyBindingGenerator) -> Result<Self, Self::Error> {
        let foreign_name_text = WebAssemblyBindingGenerator::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = WebAssemblyBindingGenerator::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_web_assembly_binding_generator_is_a_name() {
        let web_assembly_binding_generator = WebAssemblyBindingGenerator;

        let foreign_language = ForeignLanguage::try_from(web_assembly_binding_generator);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
