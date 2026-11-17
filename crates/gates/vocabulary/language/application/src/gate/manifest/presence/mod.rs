use gates_vocabulary_language_core_dependencies::manifest::Manifest;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(in crate::gate) enum ManifestPresence {
    Presence { manifest: Manifest },
    Absence,
}

impl Layer for ManifestPresence {
    type Layer = ApplicationLayer;
}
