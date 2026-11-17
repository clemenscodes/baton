use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct SecureHashAlgorithm2;

impl Layer for SecureHashAlgorithm2 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for SecureHashAlgorithm2 {}

impl SecureHashAlgorithm2 {
    const NAME: &'static str = "sha2";

    const NAMES: &'static [&'static str] = &[
        "Digest", "digest", "finalize", "sha2", "Sha256", "update",
    ];
}

impl TryFrom<SecureHashAlgorithm2> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: SecureHashAlgorithm2) -> Result<Self, Self::Error> {
        let foreign_name_text = SecureHashAlgorithm2::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = SecureHashAlgorithm2::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_secure_hash_algorithm_2_is_a_name() {
        let secure_hash_algorithm2 = SecureHashAlgorithm2;

        let foreign_language = ForeignLanguage::try_from(secure_hash_algorithm2);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
