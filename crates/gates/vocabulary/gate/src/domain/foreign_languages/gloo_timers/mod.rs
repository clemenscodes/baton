use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct GlooTimers;

impl Layer for GlooTimers {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for GlooTimers {}

impl GlooTimers {
    const NAME: &'static str = "gloo-timers";

    const NAMES: &'static [&'static str] = &["future", "gloo_timers", "sleep"];
}

impl TryFrom<GlooTimers> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: GlooTimers) -> Result<Self, Self::Error> {
        let foreign_name_text = GlooTimers::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = GlooTimers::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_gloo_timers_is_a_name() {
        let gloo_timers = GlooTimers;

        let foreign_language = ForeignLanguage::try_from(gloo_timers);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
