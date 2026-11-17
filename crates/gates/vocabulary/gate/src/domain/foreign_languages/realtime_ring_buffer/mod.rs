use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct RealtimeRingBuffer;

impl Layer for RealtimeRingBuffer {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for RealtimeRingBuffer {}

impl RealtimeRingBuffer {
    const NAME: &'static str = "rtrb";

    const NAMES: &'static [&'static str] = &[
        "Consumer",
        "Empty",
        "Full",
        "PopError",
        "Producer",
        "PushError",
        "RingBuffer",
        "new",
        "pop",
        "push",
        "rtrb",
        "slots",
    ];
}

impl TryFrom<RealtimeRingBuffer> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: RealtimeRingBuffer) -> Result<Self, Self::Error> {
        let foreign_name_text = RealtimeRingBuffer::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = RealtimeRingBuffer::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_rtrb_is_a_name() {
        let realtime_ring_buffer = RealtimeRingBuffer;

        let foreign_language = ForeignLanguage::try_from(realtime_ring_buffer);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
