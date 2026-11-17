use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Postcard;

impl Layer for Postcard {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Postcard {}

impl Postcard {
    const NAME: &'static str = "postcard";

    const NAMES: &'static [&'static str] = &[
        "DeserializeUnexpectedEnd",
        "Error",
        "from_bytes",
        "postcard",
        "to_stdvec",
    ];
}

impl TryFrom<Postcard> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Postcard) -> Result<Self, Self::Error> {
        let foreign_name_text = Postcard::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Postcard::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_postcard_is_a_name() {
        let foreign_crate = Postcard;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
