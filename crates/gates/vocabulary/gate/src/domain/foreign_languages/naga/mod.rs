use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct Naga;

impl Layer for Naga {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Naga {}

impl Naga {
    pub(crate) const NAME: &'static str = "naga";

    const NAMES: &'static [&'static str] = &[
        "all",
        "back",
        "Capabilities",
        "emit_to_string_with_path",
        "empty",
        "flags",
        "front",
        "IMMEDIATES",
        "lang_version",
        "ModuleInfo",
        "naga",
        "new",
        "Options",
        "parse_str",
        "spv",
        "TEXTURE_AND_SAMPLER_BINDING_ARRAY",
        "TEXTURE_AND_SAMPLER_BINDING_ARRAY_NON_UNIFORM_INDEXING",
        "valid",
        "validate",
        "ValidationFlags",
        "Validator",
        "wgsl",
        "WithSpan",
        "write_vec",
        "WriterFlags",
    ];
}

impl TryFrom<Naga> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Naga) -> Result<Self, Self::Error> {
        let foreign_name_text = Naga::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Naga::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_naga_is_a_name() {
        let naga = Naga;

        let foreign_language = ForeignLanguage::try_from(naga);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
