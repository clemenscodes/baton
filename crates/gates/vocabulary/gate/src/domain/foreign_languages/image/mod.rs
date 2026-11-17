use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Image;

impl Layer for Image {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Image {}

impl Image {
    const NAME: &'static str = "image";

    const NAMES: &'static [&'static str] = &[
        "codecs",
        "Decoding",
        "DecodingError",
        "dimensions",
        "DynamicImage",
        "FilterType",
        "from_fn",
        "from_mime_type",
        "from_pixel",
        "guess_format",
        "height",
        "image",
        "ImageError",
        "ImageFormat",
        "ImageFormatHint",
        "imageops",
        "ImageResult",
        "ImageRgb8",
        "ImageRgba8",
        "into_raw",
        "jpeg",
        "JpegEncoder",
        "Lanczos3",
        "load_from_memory",
        "load_from_memory_with_format",
        "new_with_quality",
        "Png",
        "resize",
        "Rgb",
        "Rgba",
        "RgbaImage",
        "RgbImage",
        "to_rgb8",
        "to_rgba8",
        "Unknown",
        "width",
        "write_to",
        "write_with_encoder",
    ];
}

impl TryFrom<Image> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Image) -> Result<Self, Self::Error> {
        let foreign_name_text = Image::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Image::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_image_is_a_name() {
        let image = Image;

        let foreign_language = ForeignLanguage::try_from(image);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
