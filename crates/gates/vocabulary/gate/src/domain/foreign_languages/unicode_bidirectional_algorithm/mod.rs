use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct UnicodeBidirectionalAlgorithm;

impl Layer for UnicodeBidirectionalAlgorithm {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for UnicodeBidirectionalAlgorithm {}

impl UnicodeBidirectionalAlgorithm {
    const NAME: &'static str = "unicode-bidi";

    const NAMES: &'static [&'static str] = &[
        "BidiInfo",
        "is_rtl",
        "Level",
        "ltr",
        "paragraphs",
        "range",
        "rtl",
        "unicode_bidi",
        "visual_runs",
    ];
}

impl TryFrom<UnicodeBidirectionalAlgorithm> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: UnicodeBidirectionalAlgorithm) -> Result<Self, Self::Error> {
        let foreign_name_text = UnicodeBidirectionalAlgorithm::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = UnicodeBidirectionalAlgorithm::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_unicode_bidirectional_algorithm_is_a_name() {
        let unicode_bidirectional_algorithm = UnicodeBidirectionalAlgorithm;

        let foreign_language = ForeignLanguage::try_from(unicode_bidirectional_algorithm);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
