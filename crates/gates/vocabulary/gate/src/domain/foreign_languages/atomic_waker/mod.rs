use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct AtomicWaker;

impl Layer for AtomicWaker {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for AtomicWaker {}

impl AtomicWaker {
    const NAME: &'static str = "atomic-waker";

    const NAMES: &'static [&'static str] = &["AtomicWaker", "atomic_waker", "register", "wake"];
}

impl TryFrom<AtomicWaker> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: AtomicWaker) -> Result<Self, Self::Error> {
        let foreign_name_text = AtomicWaker::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = AtomicWaker::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_atomic_waker_is_a_name() {
        let atomic_waker = AtomicWaker;

        let foreign_language = ForeignLanguage::try_from(atomic_waker);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
