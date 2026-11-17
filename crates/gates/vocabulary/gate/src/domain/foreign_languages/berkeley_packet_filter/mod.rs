use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct BerkeleyPacketFilter;

impl Layer for BerkeleyPacketFilter {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for BerkeleyPacketFilter {}

impl BerkeleyPacketFilter {
    pub(crate) const NAME: &'static str = "berkeley_packet_filter";

    const NAMES: &'static [&'static str] = &[
        "berkeley_packet_filter",
        "create_hash_map",
        "update_element",
        "delete_element",
        "load_device_program",
        "pin_object",
        "open_object",
        "attach_device_program",
        "detach_device_program",
    ];
}

impl TryFrom<BerkeleyPacketFilter> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: BerkeleyPacketFilter) -> Result<Self, Self::Error> {
        let foreign_name_text = BerkeleyPacketFilter::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = BerkeleyPacketFilter::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_berkeley_packet_filter_is_a_name() {
        let berkeley_packet_filter = BerkeleyPacketFilter;

        let foreign_language = ForeignLanguage::try_from(berkeley_packet_filter);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
