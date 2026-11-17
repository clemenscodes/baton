use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct Ureq;

impl Layer for Ureq {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Ureq {}

impl Ureq {
    pub(crate) const NAME: &'static str = "ureq";

    const NAMES: &'static [&'static str] = &[
        "Agent",
        "as_u16",
        "Body",
        "body_mut",
        "build",
        "call",
        "config",
        "config_builder",
        "content_type",
        "get",
        "http",
        "http_status_as_error",
        "Io",
        "limit",
        "new_with_config",
        "post",
        "proxy",
        "query",
        "read_to_vec",
        "Response",
        "send",
        "status",
        "timeout_global",
        "typestate",
        "ureq",
        "with_config",
        "WithoutBody",
    ];
}

impl TryFrom<Ureq> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Ureq) -> Result<Self, Self::Error> {
        let foreign_name_text = Ureq::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Ureq::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_ureq_is_a_name() {
        let ureq = Ureq;

        let foreign_language = ForeignLanguage::try_from(ureq);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
