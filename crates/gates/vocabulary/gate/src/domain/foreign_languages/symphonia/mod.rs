use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Symphonia;

impl Layer for Symphonia {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Symphonia {}

impl Symphonia {
    const NAME: &'static str = "symphonia";

    const NAMES: &'static [&'static str] = &[
        "audio",
        "Audio",
        "AudioCodecParameters",
        "AudioDecoderOptions",
        "channels",
        "Channels",
        "codec_params",
        "CodecParameters",
        "codecs",
        "copy_to_vec_interleaved",
        "core",
        "count",
        "decode",
        "DecodeError",
        "default",
        "default_track",
        "Error",
        "errors",
        "FormatOptions",
        "FormatReader",
        "GenericAudioBufferRef",
        "formats",
        "get_codecs",
        "get_probe",
        "Hint",
        "id",
        "io",
        "make_audio_decoder",
        "MediaSource",
        "MediaSourceStream",
        "MediaSourceStreamOptions",
        "meta",
        "MetadataOptions",
        "new",
        "next_packet",
        "probe",
        "sample_rate",
        "symphonia",
        "track_id",
        "TrackType",
        "with_extension",
    ];
}

impl TryFrom<Symphonia> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Symphonia) -> Result<Self, Self::Error> {
        let foreign_name_text = Symphonia::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Symphonia::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_symphonia_is_a_name() {
        let symphonia = Symphonia;

        let foreign_language = ForeignLanguage::try_from(symphonia);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
