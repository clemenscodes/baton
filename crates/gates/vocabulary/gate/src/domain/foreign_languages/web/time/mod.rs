use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(in crate::domain::foreign_languages) struct WebTime;

impl Layer for WebTime {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for WebTime {}

impl WebTime {
    const NAME: &'static str = "web-time";

    const NAMES: &'static [&'static str] = &[
        "Instant",
        "now",
        "saturating_duration_since",
        "web_time",
    ];
}

impl TryFrom<WebTime> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: WebTime) -> Result<Self, Self::Error> {
        let foreign_name_text = WebTime::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = WebTime::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_web_time_is_a_name() {
        let web_time = WebTime;

        let foreign_language = ForeignLanguage::try_from(web_time);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
