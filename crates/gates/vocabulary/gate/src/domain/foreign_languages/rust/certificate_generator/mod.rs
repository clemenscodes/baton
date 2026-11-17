use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct RustCertificateGenerator;

impl Layer for RustCertificateGenerator {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for RustCertificateGenerator {}

impl RustCertificateGenerator {
    const NAME: &'static str = "rcgen";

    const NAMES: &'static [&'static str] = &[
        "cert",
        "der",
        "generate_simple_self_signed",
        "rcgen",
        "serialize_der",
        "signing_key",
    ];
}

impl TryFrom<RustCertificateGenerator> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: RustCertificateGenerator) -> Result<Self, Self::Error> {
        let foreign_name_text = RustCertificateGenerator::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = RustCertificateGenerator::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_rcgen_is_a_name() {
        let foreign_crate = RustCertificateGenerator;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
