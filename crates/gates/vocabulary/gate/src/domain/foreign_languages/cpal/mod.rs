use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Cpal;

impl Layer for Cpal {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Cpal {}

impl Cpal {
    const NAME: &'static str = "cpal";

    const NAMES: &'static [&'static str] = &[
        "build_output_stream",
        "channels",
        "config",
        "cpal",
        "default_host",
        "default_output_config",
        "default_output_device",
        "Device",
        "DeviceNotAvailable",
        "DeviceTrait",
        "Error",
        "ErrorKind",
        "HostTrait",
        "OutputCallbackInfo",
        "pause",
        "play",
        "sample_rate",
        "Stream",
        "StreamConfig",
        "StreamTrait",
        "SupportedStreamConfig",
        "traits",
        "Xrun",
    ];
}

impl TryFrom<Cpal> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Cpal) -> Result<Self, Self::Error> {
        let foreign_name_text = Cpal::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Cpal::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_cpal_is_a_name() {
        let cpal = Cpal;

        let foreign_language = ForeignLanguage::try_from(cpal);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
