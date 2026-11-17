mod allocation_counter;
mod arboard;
mod argon2;
mod ash;
mod asynchronous_channel;
mod atomic_waker;
pub(super) mod berkeley_packet_filter;
mod c_library;
mod central_processing_unit_time;
mod command_line_argument_parser;
mod cpal;
mod foreign_crates;
mod get_random;
mod gilrs;
mod gloo_timers;
mod gltf;
mod graphics_processing_unit_allocator;
mod image;
mod international_components_for_unicode;
mod java_script_system;
mod khronos_texture2;
pub(super) mod naga;
mod percent_encoding;
mod portable_network_graphics;
mod postcard;
mod procedural_macro2;
mod quinn;
mod raw_window_handle;
mod rayon;
mod realtime_ring_buffer;
mod rust;
mod rustix;
mod rusty_object_notation;
mod secure_hash_algorithm2;
pub(super) mod serde;
mod softbuffer;
pub(super) mod standard_library;
mod structured_query_language_extension;
mod strum;
mod swash;
mod symphonia;
mod syntax;
mod tokio;
mod toms_obvious_minimal_language;
mod unicode_bidirectional_algorithm;
pub(super) mod universally_unique_identifier;
pub(super) mod ureq;
mod web;
mod winit;
mod x11rb;

use crate::domain::foreign_languages::foreign_crates::ForeignCrates;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct ForeignLanguages {
    foreign_languages: Vec<ForeignLanguage>,
}

impl Layer for ForeignLanguages {
    type Layer = DomainLayer;
}

impl FromIterator<ForeignLanguage> for ForeignLanguages {
    fn from_iter<ForeignLanguagesType: IntoIterator<Item = ForeignLanguage>>(
        foreign_language_sequence: ForeignLanguagesType,
    ) -> Self {
        let foreign_languages = foreign_language_sequence.into_iter().collect();
        Self { foreign_languages }
    }
}

impl TryFrom<()> for ForeignLanguages {
    type Error = Error;

    fn try_from(_: ()) -> Result<Self, Self::Error> {
        let foreign_crates = ForeignCrates;
        let foreign_language_outcomes = foreign_crates.compose_foreign_language_outcomes();
        foreign_language_outcomes.into_iter().collect()
    }
}

impl IntoIterator for ForeignLanguages {
    type Item = ForeignLanguage;
    type IntoIter = IntoIter<ForeignLanguage>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { foreign_languages } = self;
        foreign_languages.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_foreign_language_comes_about() {
        let foreign_languages = ForeignLanguages::try_from(());

        let error = foreign_languages.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
