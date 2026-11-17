use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct WebAssemblyBindingGeneratorTest;

impl Layer for WebAssemblyBindingGeneratorTest {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for WebAssemblyBindingGeneratorTest {}

impl WebAssemblyBindingGeneratorTest {
    const NAME: &'static str = "wasm-bindgen-test";

    const NAMES: &'static [&'static str] = &[
        "run_in_browser",
        "wasm_bindgen_test",
        "wasm_bindgen_test_configure",
    ];
}

impl TryFrom<WebAssemblyBindingGeneratorTest> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: WebAssemblyBindingGeneratorTest) -> Result<Self, Self::Error> {
        let foreign_name_text = WebAssemblyBindingGeneratorTest::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = WebAssemblyBindingGeneratorTest::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_web_assembly_binding_generator_test_is_a_name() {
        let web_assembly_binding_generator_test = WebAssemblyBindingGeneratorTest;

        let foreign_language = ForeignLanguage::try_from(web_assembly_binding_generator_test);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
