use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct CentralProcessingUnitTime;

impl Layer for CentralProcessingUnitTime {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for CentralProcessingUnitTime {}

impl CentralProcessingUnitTime {
    const NAME: &'static str = "cpu-time";

    const NAMES: &'static [&'static str] = &["ProcessTime", "as_duration", "cpu_time", "try_now"];
}

impl TryFrom<CentralProcessingUnitTime> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: CentralProcessingUnitTime) -> Result<Self, Self::Error> {
        let foreign_name_text = CentralProcessingUnitTime::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = CentralProcessingUnitTime::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_cpu_time_is_a_name() {
        let central_processing_unit_time = CentralProcessingUnitTime;

        let foreign_language = ForeignLanguage::try_from(central_processing_unit_time);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
