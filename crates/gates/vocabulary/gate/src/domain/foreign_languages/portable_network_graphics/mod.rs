use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct PortableNetworkGraphics;

impl Layer for PortableNetworkGraphics {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for PortableNetworkGraphics {}

impl PortableNetworkGraphics {
    const NAME: &'static str = "png";

    const NAMES: &'static [&'static str] = &[
        "BitDepth",
        "ColorType",
        "Decoder",
        "Eight",
        "Encoder",
        "finish",
        "LimitsExceeded",
        "next_frame",
        "output_buffer_size",
        "png",
        "read_info",
        "Rgba",
        "set_color",
        "set_depth",
        "write_header",
        "write_image_data",
        "Writer",
    ];
}

impl TryFrom<PortableNetworkGraphics> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: PortableNetworkGraphics) -> Result<Self, Self::Error> {
        let foreign_name_text = PortableNetworkGraphics::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = PortableNetworkGraphics::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_portable_network_graphics_is_a_name() {
        let portable_network_graphics = PortableNetworkGraphics;

        let foreign_language = ForeignLanguage::try_from(portable_network_graphics);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
