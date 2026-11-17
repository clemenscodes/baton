use crate::manifest::{Manifest, ManifestText};
use stack_domain_driven_design_core::port::DrivenPort;
use std::io;

pub trait Decoder: DrivenPort {
    fn try_decode_manifest(&self, manifest_text: &ManifestText) -> io::Result<Manifest>;
}
