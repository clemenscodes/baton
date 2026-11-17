use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Argon2;

impl Layer for Argon2 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Argon2 {}

impl Argon2 {
    const NAME: &'static str = "argon2";

    const NAMES: &'static [&'static str] = &[
        "argon2",
        "Argon2",
        "hash_password_with_salt",
        "password_hash",
        "PasswordHash",
        "PasswordHasher",
        "PasswordInvalid",
        "PasswordVerifier",
        "phc",
        "verify_password",
    ];
}

impl TryFrom<Argon2> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Argon2) -> Result<Self, Self::Error> {
        let foreign_name_text = Argon2::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Argon2::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_argon2_is_a_name() {
        let foreign_crate = Argon2;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
