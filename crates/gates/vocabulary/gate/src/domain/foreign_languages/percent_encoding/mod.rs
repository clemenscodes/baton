use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct PercentEncoding;

impl Layer for PercentEncoding {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for PercentEncoding {}

impl PercentEncoding {
    const NAME: &'static str = "percent-encoding";

    const NAMES: &'static [&'static str] = &[
        "AsciiSet",
        "NON_ALPHANUMERIC",
        "percent_encoding",
        "remove",
        "utf8_percent_encode",
    ];
}

impl TryFrom<PercentEncoding> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: PercentEncoding) -> Result<Self, Self::Error> {
        let foreign_name_text = PercentEncoding::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = PercentEncoding::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_percent_encoding_is_a_name() {
        let percent_encoding = PercentEncoding;

        let foreign_language = ForeignLanguage::try_from(percent_encoding);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
