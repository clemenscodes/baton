use crate::domain::foreign_languages::allocation_counter::AllocationCounter;
use crate::domain::foreign_languages::arboard::Arboard;
use crate::domain::foreign_languages::argon2::Argon2;
use crate::domain::foreign_languages::ash::Ash;
use crate::domain::foreign_languages::ash::AshWindow;
use crate::domain::foreign_languages::asynchronous_channel::AsynchronousChannel;
use crate::domain::foreign_languages::atomic_waker::AtomicWaker;
use crate::domain::foreign_languages::berkeley_packet_filter::BerkeleyPacketFilter;
use crate::domain::foreign_languages::c_library::CLibrary;
use crate::domain::foreign_languages::central_processing_unit_time::CentralProcessingUnitTime;
use crate::domain::foreign_languages::command_line_argument_parser::CommandLineArgumentParser;
use crate::domain::foreign_languages::cpal::Cpal;
use crate::domain::foreign_languages::get_random::GetRandom;
use crate::domain::foreign_languages::gilrs::Gilrs;
use crate::domain::foreign_languages::gloo_timers::GlooTimers;
use crate::domain::foreign_languages::gltf::Gltf;
use crate::domain::foreign_languages::graphics_processing_unit_allocator::GraphicsProcessingUnitAllocator;
use crate::domain::foreign_languages::image::Image;
use crate::domain::foreign_languages::international_components_for_unicode::locale::InternationalComponentsForUnicodeLocale;
use crate::domain::foreign_languages::international_components_for_unicode::locale::core::InternationalComponentsForUnicodeLocaleCore;
use crate::domain::foreign_languages::java_script_system::JavaScriptSystem;
use crate::domain::foreign_languages::khronos_texture2::KhronosTexture2;
use crate::domain::foreign_languages::naga::Naga;
use crate::domain::foreign_languages::percent_encoding::PercentEncoding;
use crate::domain::foreign_languages::portable_network_graphics::PortableNetworkGraphics;
use crate::domain::foreign_languages::postcard::Postcard;
use crate::domain::foreign_languages::procedural_macro2::ProceduralMacro2;
use crate::domain::foreign_languages::quinn::Quinn;
use crate::domain::foreign_languages::raw_window_handle::RawWindowHandle;
use crate::domain::foreign_languages::rayon::Rayon;
use crate::domain::foreign_languages::realtime_ring_buffer::RealtimeRingBuffer;
use crate::domain::foreign_languages::rust::certificate_generator::RustCertificateGenerator;
use crate::domain::foreign_languages::rust::transport_layer_security::RustTransportLayerSecurity;
use crate::domain::foreign_languages::rustix::Rustix;
use crate::domain::foreign_languages::rusty_object_notation::RustyObjectNotation;
use crate::domain::foreign_languages::secure_hash_algorithm2::SecureHashAlgorithm2;
use crate::domain::foreign_languages::serde::Serde;
use crate::domain::foreign_languages::serde::java_script_object_notation::SerdeJavaScriptObjectNotation;
use crate::domain::foreign_languages::softbuffer::Softbuffer;
use crate::domain::foreign_languages::standard_library::StandardLibrary;
use crate::domain::foreign_languages::structured_query_language_extension::StructuredQueryLanguageExtension;
use crate::domain::foreign_languages::strum::Strum;
use crate::domain::foreign_languages::swash::Swash;
use crate::domain::foreign_languages::symphonia::Symphonia;
use crate::domain::foreign_languages::syntax::Syntax;
use crate::domain::foreign_languages::tokio::Tokio;
use crate::domain::foreign_languages::toms_obvious_minimal_language::TomsObviousMinimalLanguage;
use crate::domain::foreign_languages::unicode_bidirectional_algorithm::UnicodeBidirectionalAlgorithm;
use crate::domain::foreign_languages::universally_unique_identifier::UniversallyUniqueIdentifier;
use crate::domain::foreign_languages::ureq::Ureq;
use crate::domain::foreign_languages::web::assembly_binding_generator::WebAssemblyBindingGenerator;
use crate::domain::foreign_languages::web::assembly_binding_generator::futures::WebAssemblyBindingGeneratorFutures;
use crate::domain::foreign_languages::web::assembly_binding_generator::test::WebAssemblyBindingGeneratorTest;
use crate::domain::foreign_languages::web::system::WebSystem;
use crate::domain::foreign_languages::web::time::WebTime;
use crate::domain::foreign_languages::web::transport::WebTransport;
use crate::domain::foreign_languages::winit::Winit;
use crate::domain::foreign_languages::x11rb::X11rb;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct ForeignCrates;

impl Layer for ForeignCrates {
    type Layer = DomainLayer;
}

impl ForeignCrates {
    fn compose_window_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let arboard = Arboard;
        let gilrs = Gilrs;
        let raw_window_handle = RawWindowHandle;
        let softbuffer = Softbuffer;
        let winit = Winit;
        let x11rb = X11rb;
        vec![
            ForeignLanguage::try_from(arboard),
            ForeignLanguage::try_from(gilrs),
            ForeignLanguage::try_from(raw_window_handle),
            ForeignLanguage::try_from(softbuffer),
            ForeignLanguage::try_from(winit),
            ForeignLanguage::try_from(x11rb),
        ]
    }

    fn compose_graphics_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let khronos_texture2 = KhronosTexture2;
        let ash = Ash;
        let ash_window = AshWindow;
        let graphics_processing_unit_allocator = GraphicsProcessingUnitAllocator;
        let naga = Naga;
        let portable_network_graphics = PortableNetworkGraphics;
        vec![
            ForeignLanguage::try_from(khronos_texture2),
            ForeignLanguage::try_from(ash),
            ForeignLanguage::try_from(ash_window),
            ForeignLanguage::try_from(graphics_processing_unit_allocator),
            ForeignLanguage::try_from(naga),
            ForeignLanguage::try_from(portable_network_graphics),
        ]
    }

    fn compose_media_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let allocation_counter = AllocationCounter;
        let cpal = Cpal;
        let realtime_ring_buffer = RealtimeRingBuffer;
        let gltf = Gltf;
        let international_components_for_unicode_locale = InternationalComponentsForUnicodeLocale;
        let international_components_for_unicode_locale_core = InternationalComponentsForUnicodeLocaleCore;
        let swash = Swash;
        let symphonia = Symphonia;
        let unicode_bidirectional_algorithm = UnicodeBidirectionalAlgorithm;
        vec![
            ForeignLanguage::try_from(allocation_counter),
            ForeignLanguage::try_from(cpal),
            ForeignLanguage::try_from(realtime_ring_buffer),
            ForeignLanguage::try_from(gltf),
            ForeignLanguage::try_from(international_components_for_unicode_locale),
            ForeignLanguage::try_from(international_components_for_unicode_locale_core),
            ForeignLanguage::try_from(swash),
            ForeignLanguage::try_from(symphonia),
            ForeignLanguage::try_from(unicode_bidirectional_algorithm),
        ]
    }

    fn compose_browser_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let gloo_timers = GlooTimers;
        let java_script_system = JavaScriptSystem;
        let percent_encoding = PercentEncoding;
        let web_assembly_binding_generator = WebAssemblyBindingGenerator;
        let web_assembly_binding_generator_futures = WebAssemblyBindingGeneratorFutures;
        let web_assembly_binding_generator_test = WebAssemblyBindingGeneratorTest;
        let web_system = WebSystem;
        let web_time = WebTime;
        vec![
            ForeignLanguage::try_from(gloo_timers),
            ForeignLanguage::try_from(java_script_system),
            ForeignLanguage::try_from(percent_encoding),
            ForeignLanguage::try_from(web_assembly_binding_generator),
            ForeignLanguage::try_from(web_assembly_binding_generator_futures),
            ForeignLanguage::try_from(web_assembly_binding_generator_test),
            ForeignLanguage::try_from(web_system),
            ForeignLanguage::try_from(web_time),
        ]
    }

    fn compose_notation_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let rusty_object_notation = RustyObjectNotation;
        let serde = Serde;
        let serde_java_script_object_notation = SerdeJavaScriptObjectNotation;
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let image = Image;
        let rustix = Rustix;
        let berkeley_packet_filter = BerkeleyPacketFilter;
        let secure_hash_algorithm2 = SecureHashAlgorithm2;
        let ureq = Ureq;
        let c_library = CLibrary;
        vec![
            ForeignLanguage::try_from(rusty_object_notation),
            ForeignLanguage::try_from(serde),
            ForeignLanguage::try_from(serde_java_script_object_notation),
            ForeignLanguage::try_from(toms_obvious_minimal_language),
            ForeignLanguage::try_from(image),
            ForeignLanguage::try_from(rustix),
            ForeignLanguage::try_from(berkeley_packet_filter),
            ForeignLanguage::try_from(secure_hash_algorithm2),
            ForeignLanguage::try_from(ureq),
            ForeignLanguage::try_from(c_library),
        ]
    }

    fn compose_concurrency_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let asynchronous_channel = AsynchronousChannel;
        let atomic_waker = AtomicWaker;
        let rayon = Rayon;
        let tokio = Tokio;
        vec![
            ForeignLanguage::try_from(asynchronous_channel),
            ForeignLanguage::try_from(atomic_waker),
            ForeignLanguage::try_from(rayon),
            ForeignLanguage::try_from(tokio),
        ]
    }

    fn compose_network_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let argon2 = Argon2;
        let get_random = GetRandom;
        let postcard = Postcard;
        let structured_query_language_extension = StructuredQueryLanguageExtension;
        let web_transport = WebTransport;
        let quinn = Quinn;
        let rust_certificate_generator = RustCertificateGenerator;
        let rust_transport_layer_security = RustTransportLayerSecurity;
        vec![
            ForeignLanguage::try_from(argon2),
            ForeignLanguage::try_from(get_random),
            ForeignLanguage::try_from(postcard),
            ForeignLanguage::try_from(structured_query_language_extension),
            ForeignLanguage::try_from(web_transport),
            ForeignLanguage::try_from(quinn),
            ForeignLanguage::try_from(rust_certificate_generator),
            ForeignLanguage::try_from(rust_transport_layer_security),
        ]
    }

    fn compose_program_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let central_processing_unit_time = CentralProcessingUnitTime;
        let command_line_argument_parser = CommandLineArgumentParser;
        let procedural_macro2 = ProceduralMacro2;
        let standard_library = StandardLibrary;
        let strum = Strum;
        let syntax = Syntax;
        let universally_unique_identifier = UniversallyUniqueIdentifier;
        vec![
            ForeignLanguage::try_from(central_processing_unit_time),
            ForeignLanguage::try_from(command_line_argument_parser),
            ForeignLanguage::try_from(procedural_macro2),
            ForeignLanguage::try_from(standard_library),
            ForeignLanguage::try_from(strum),
            ForeignLanguage::try_from(syntax),
            ForeignLanguage::try_from(universally_unique_identifier),
        ]
    }

    pub(super) fn compose_foreign_language_outcomes(
        &self,
    ) -> impl IntoIterator<Item = Result<ForeignLanguage, Error>> {
        let window_foreign_language_outcomes = self.compose_window_foreign_language_outcomes();
        let graphics_foreign_language_outcomes = self.compose_graphics_foreign_language_outcomes();
        let media_foreign_language_outcomes = self.compose_media_foreign_language_outcomes();
        let browser_foreign_language_outcomes = self.compose_browser_foreign_language_outcomes();
        let notation_foreign_language_outcomes = self.compose_notation_foreign_language_outcomes();
        let concurrency_foreign_language_outcomes = self
            .compose_concurrency_foreign_language_outcomes();
        let program_foreign_language_outcomes = self.compose_program_foreign_language_outcomes();
        let network_foreign_language_outcomes = self.compose_network_foreign_language_outcomes();
        let mut foreign_language_outcomes = Vec::new();
        foreign_language_outcomes.extend(window_foreign_language_outcomes);
        foreign_language_outcomes.extend(graphics_foreign_language_outcomes);
        foreign_language_outcomes.extend(media_foreign_language_outcomes);
        foreign_language_outcomes.extend(browser_foreign_language_outcomes);
        foreign_language_outcomes.extend(notation_foreign_language_outcomes);
        foreign_language_outcomes.extend(concurrency_foreign_language_outcomes);
        foreign_language_outcomes.extend(program_foreign_language_outcomes);
        foreign_language_outcomes.extend(network_foreign_language_outcomes);
        foreign_language_outcomes
    }
}
