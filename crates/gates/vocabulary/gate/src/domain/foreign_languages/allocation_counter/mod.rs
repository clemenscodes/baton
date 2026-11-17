use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct AllocationCounter;

impl Layer for AllocationCounter {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for AllocationCounter {}

impl AllocationCounter {
    const NAME: &'static str = "allocation-counter";

    const NAMES: &'static [&'static str] = &["allocation_counter", "count_total", "measure"];
}

impl TryFrom<AllocationCounter> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: AllocationCounter) -> Result<Self, Self::Error> {
        let foreign_name_text = AllocationCounter::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = AllocationCounter::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_allocation_counter_is_a_name() {
        let allocation_counter = AllocationCounter;

        let foreign_language = ForeignLanguage::try_from(allocation_counter);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
