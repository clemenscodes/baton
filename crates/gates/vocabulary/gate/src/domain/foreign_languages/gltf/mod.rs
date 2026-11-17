use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Gltf;

impl Layer for Gltf {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Gltf {}

impl Gltf {
    const NAME: &'static str = "gltf";

    const NAMES: &'static [&'static str] = &[
        "Accessor",
        "accessors",
        "alpha_cutoff",
        "alpha_mode",
        "AlphaMode",
        "animation",
        "animations",
        "base_color_factor",
        "base_color_texture",
        "Blend",
        "blob",
        "buffer",
        "buffers",
        "Channel",
        "channels",
        "children",
        "decomposed",
        "document",
        "double_sided",
        "Error",
        "extras",
        "from_slice",
        "gltf",
        "image",
        "images",
        "index",
        "interpolation",
        "into_f32",
        "into_u16",
        "into_u32",
        "joints",
        "json",
        "length",
        "Linear",
        "Mask",
        "material",
        "materials",
        "mesh",
        "meshes",
        "mime_type",
        "Mode",
        "morph_targets",
        "MorphTargetWeights",
        "name",
        "node",
        "nodes",
        "offset",
        "Opaque",
        "pbr_metallic_roughness",
        "Primitive",
        "primitives",
        "reader",
        "read_indices",
        "read_inputs",
        "read_joints",
        "read_normals",
        "read_outputs",
        "ReadOutputs",
        "read_positions",
        "read_tangents",
        "read_tex_coords",
        "read_weights",
        "Rotations",
        "sampler",
        "Scales",
        "scene",
        "scenes",
        "skin",
        "skins",
        "source",
        "target",
        "tex_coord",
        "texture",
        "transform",
        "Translations",
        "Triangles",
        "util",
        "View",
    ];
}

impl TryFrom<Gltf> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Gltf) -> Result<Self, Self::Error> {
        let foreign_name_text = Gltf::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Gltf::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_gltf_is_a_name() {
        let gltf = Gltf;

        let foreign_language = ForeignLanguage::try_from(gltf);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
