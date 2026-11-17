use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Swash;

impl Layer for Swash {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Swash {}

impl Swash {
    const NAME: &'static str = "swash";

    const NAMES: &'static [&'static str] = &[
        "add_cluster",
        "advance",
        "analyze",
        "ascent",
        "Bengali",
        "Boundary",
        "Bounds",
        "bounds",
        "build",
        "builder",
        "CacheKey",
        "ch",
        "CharCluster",
        "CharInfo",
        "charmap",
        "Close",
        "cluster",
        "Command",
        "commands",
        "Complete",
        "CurveTo",
        "data",
        "descent",
        "Devanagari",
        "direction",
        "Direction",
        "Discard",
        "from_index",
        "from_opentype",
        "from_points",
        "FontRef",
        "Glyph",
        "glyph_count",
        "GlyphCluster",
        "glyphs",
        "Hebrew",
        "id",
        "info",
        "Inherited",
        "is_real",
        "Keep",
        "Language",
        "language",
        "key",
        "Latin",
        "leading",
        "LeftToRight",
        "len",
        "LineTo",
        "Mandatory",
        "map",
        "max",
        "Metrics",
        "metrics",
        "min",
        "MoveTo",
        "new",
        "next",
        "NormalizedCoord",
        "offset",
        "outline",
        "Outline",
        "Parser",
        "parse",
        "path",
        "PathData",
        "points",
        "properties",
        "QuadTo",
        "range",
        "RightToLeft",
        "scale",
        "scale_outline",
        "ScaleContext",
        "Script",
        "script",
        "shape",
        "shape_with",
        "ShapeContext",
        "size",
        "source",
        "SourceRange",
        "Status",
        "swash",
        "tag_from_str_lossy",
        "text",
        "to_opentype",
        "to_range",
        "Token",
        "transform",
        "Transform",
        "Unknown",
        "UserData",
        "Vector",
        "Word",
        "x",
        "y",
        "zeno",
    ];
}

impl TryFrom<Swash> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Swash) -> Result<Self, Self::Error> {
        let foreign_name_text = Swash::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Swash::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_swash_is_a_name() {
        let swash = Swash;

        let foreign_language = ForeignLanguage::try_from(swash);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
