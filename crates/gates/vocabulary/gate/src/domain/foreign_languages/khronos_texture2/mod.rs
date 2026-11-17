use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct KhronosTexture2;

impl Layer for KhronosTexture2 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for KhronosTexture2 {}

impl KhronosTexture2 {
    const NAME: &'static str = "ktx2";

    const NAMES: &'static [&'static str] = &[
        "BC1_RGBA_SRGB_BLOCK",
        "BC1_RGBA_UNORM_BLOCK",
        "BC1_RGB_SRGB_BLOCK",
        "BC1_RGB_UNORM_BLOCK",
        "BC2_SRGB_BLOCK",
        "BC2_UNORM_BLOCK",
        "BC3_SRGB_BLOCK",
        "BC3_UNORM_BLOCK",
        "BC4_UNORM_BLOCK",
        "BC5_UNORM_BLOCK",
        "BC6H_UFLOAT_BLOCK",
        "BC7_SRGB_BLOCK",
        "BC7_UNORM_BLOCK",
        "Format",
        "Level",
        "ParseError",
        "R8G8B8A8_SRGB",
        "R8G8B8A8_UNORM",
        "Reader",
        "data",
        "format",
        "header",
        "ktx2",
        "levels",
        "new",
        "pixel_height",
        "pixel_width",
        "supercompression_scheme",
    ];
}

impl TryFrom<KhronosTexture2> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: KhronosTexture2) -> Result<Self, Self::Error> {
        let foreign_name_text = KhronosTexture2::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = KhronosTexture2::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_khronos_texture2_is_a_name() {
        let khronos_texture2 = KhronosTexture2;

        let foreign_language = ForeignLanguage::try_from(khronos_texture2);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
