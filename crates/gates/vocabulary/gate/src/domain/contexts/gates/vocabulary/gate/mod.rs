use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyGateContext;

impl Layer for GatesVocabularyGateContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyGateContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/gate";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack"];
            let definition = "The module of the contexts of the crates of the stack, the technology every product is built from, the crates under crates/stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gates"];
            let definition = "The module of the contexts of the crates of the gates, the vocabulary and the coverage, the crates under crates/gates.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gate"];
            let definition = "The module of the context of the crate gates_vocabulary_gate, the gate of the vocabulary, beneath the module of the vocabulary.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rust"];
            let definition = "The module of the context of the crate products_editor_adapter_generation_content_factory_adapter_generator_rust, named for the language Rust, beneath the module of the generator of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context", "Outcomes"];
            let definition = "The outcomes of the contexts of one family of crates, each a context or the error of its definitions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vocabulary"];
            let definition = "The crate that defines the contexts of Dusk Requiem and judges every source against them, and the module of the context of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory"];
            let definition = "The path, relative to crates, of the directory a context covers, as the constant DIRECTORY_NAME of the context states it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "An identifier of a foreign language, admitted in exactly its spelling and only where the code uses it, the text that identifies a foreign language, equal to the name of the crate its names come from, and the name of the directory a context covers.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context"];
            let definition = "The terms of one crate of Dusk Requiem with their definitions, against which the vocabulary judges every source of that crate; the contexts of Dusk Requiem join the foreign languages into the languages the vocabulary judges with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Line", "Argument", "Parser"];
            let definition = "The foreign language of the crate clap, the command line argument parser, which parses the command lines of the library crate gates_vocabulary_language_adapter_command_line, of this crate, of the viewer of the client and of the program of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain", "Driven", "Design"];
            let definition = "The module of the context of the crate stack_domain_driven_design_core, whose building blocks of domain-driven design the crates of Dusk Requiem implement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Coverage"];
            let definition = "The module of the contexts of the crates of the coverage under crates/gates/coverage, the gate that refuses every region a test does not reach.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Serde"];
            let definition = "The foreign language of the crate serde, whose derive reads a structure out of a text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Serde", "Java", "Script", "Object", "Notation"];
            let definition = "The foreign language of the crate serde_json, which reads the JavaScript Object Notation into a structure of serde.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Universally", "Unique", "Identifier"];
            let definition = "The foreign language of the crate uuid, which parses a universally unique identifier and states its version and its variant.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Standard", "Library"];
            let definition = "The foreign language of the standard library std of Rust, the names the crates of Dusk Requiem use from it, which every context names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Foreign", "Language"];
            let definition = "A foreign language, the names the crates of Dusk Requiem use from a crate outside the workspace, as the library crate gates_vocabulary_language_core_languages receives it; every one of them is an anti-corruption layer of this crate, and the foreign languages of Dusk Requiem come about together.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Gate", "Context"];
            let definition = "The context of this crate, the contexts of Dusk Requiem and the program that judges every source against them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Java", "Script", "Object", "Notation"];
            let definition = "The module of the foreign language of the crate serde_json, beneath the foreign language of serde.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rayon"];
            let definition = "The foreign language of the crate rayon, whose parallel iterators spread the work of an import over every core.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Strum"];
            let definition = "The foreign language of the crate strum, whose derive lists the variants of an enum.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tokio"];
            let definition = "The foreign language of the crate tokio, whose channels and runtime the tests of the actors use through the development dependencies of their crates, and whose runtime and timer run the world in memory natively.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Asynchronous", "Channel"];
            let definition = "The foreign language of the crate async-channel, whose bounded channels connect the handle of the world in memory with its actor, received from and sent into without a mutable reference.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Percent", "Encoding"];
            let definition = "The foreign language of the crate percent-encoding, which percent encodes the segments of the path of a file in the addresses of the crate stack_engine_adapter_assets_source_http, as RFC 3986 encodes them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Atomic", "Waker"];
            let definition = "The foreign language of the crate atomic-waker, whose slot for a waker without a lock wakes the task that awaits the zone packages of the channel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gloo", "Timers"];
            let definition = "The foreign language of the crate gloo-timers, whose sleep on a timer of the browser ticks the world in memory in the browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Ash", "Window"];
            let definition = "The foreign language of the crate ash-window, whose function makes a surface of Vulkan from the raw handles of a window.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Raw", "Window", "Handle"];
            let definition = "The foreign language of the crate raw-window-handle, whose traits yield the handles of a window and of its display, which the adapter of Vulkan makes its surface of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Allocation", "Counter"];
            let definition = "The foreign language of the crate allocation-counter, whose measure counts the allocations of a closure, so a test of the mixing of stack_engine_core proves that it allocates nothing.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Web", "Time"];
            let definition = "The foreign language of the crate web-time, whose instant reads the clock of the browser, which the instant of std cannot.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Central", "Processing", "Unit", "Time"];
            let definition = "The foreign language of the crate cpu-time, whose process time reads the processor time a process has used on every core.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Realtime", "Ring", "Buffer"];
            let definition = "The foreign language of the crate rtrb, a ring buffer of one producer and one consumer whose push and pop never wait, which carries the queues between the thread of the sound card and the output stream of stack_engine_adapter_sound_cpal.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Toms", "Obvious", "Minimal", "Language"];
            let definition = "The foreign language of the crate toml, which decodes the manifests of the crates in Tom's Obvious, Minimal Language for the adapter gates_vocabulary_language_adapter_toms_obvious_minimal_language.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of this crate, the contexts of Dusk Requiem and the foreign languages of the crates outside the workspace.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gates", "Coverage"];
            let definition = "The family of the crates of the coverage under crates/gates/coverage, the gate that refuses every region a test does not reach, whose name begins the name of each of its crates and of their contexts.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Span"];
            let definition = "The module of the context of the crate gates_coverage_core_span, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Build"];
            let definition = "The module of the context of the crate gates_coverage_core_test_build, beneath the module of the test of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Region"];
            let definition = "The module of the context of the crate gates_coverage_core_region, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report"];
            let definition = "The module of the context of the crate gates_coverage_core_report, beneath the module of the core of the coverage, and of the context of the crate products_editor_adapter_generation_content_factory_core_report.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Boundaries"];
            let definition = "The module of the context of the crate gates_coverage_core_test_boundaries, beneath the module of the test of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Invocation"];
            let definition = "The module of the context of the crate gates_coverage_core_invocation, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Profile"];
            let definition = "The module of the contexts of the crates of the profile of the coverage, gates_coverage_core_profile_directory and gates_coverage_core_profile_tools, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gaps"];
            let definition = "The module of the context of the crate gates_coverage_core_gaps, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compiler"];
            let definition = "The module of the context of the crate gates_coverage_core_compiler, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mapping", "Presence"];
            let definition = "The module of the context of the crate gates_coverage_core_mapping_presence, beneath the module of the core of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measurement"];
            let definition = "The module of the context of the crate gates_coverage_application_measurement, beneath the module of the application of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "System"];
            let definition = "The module of the context of the crate gates_coverage_adapter_file_system, beneath the module of the adapters of the coverage.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Coverage", "Context", "Outcomes"];
            let definition = "To compose the outcome of every context of the crates of the coverage: its application, its program and, through the functions below, its adapters and its core.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Coverage", "Adapter", "Context", "Outcomes"];
            let definition = "To compose the outcome of every context of the adapters of the coverage: the file system, the JavaScript Object Notation, the process and the syntax.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Coverage", "Core", "Context", "Outcomes"];
            let definition = "To compose the outcome of every context of the crates of the core of the coverage about the measurement: the compiler, the gaps, the invocation, the mapping presence, the profile directory and the profile tools.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Coverage", "Report", "Context", "Outcomes"];
            let definition = "To compose the outcome of every context of the crates of the core of the coverage about the report: the region, the report, the report file, the source, the span, the test boundaries and the test build.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Domain", "Driven", "Design", "Core", "Context"];
            let definition = "The context of the crate stack_domain_driven_design_core, the building blocks of domain-driven design as traits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit"];
            let definition = "The module of the context of the crate of the client exit, beneath the module of the domain of the client.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line"];
            let definition = "The module of the context of the crate stack_engine_adapter_typesetter_line, beneath the module of the typesetter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Adapter"];
            let definition = "The module of the contexts of the adapters of one part of the workspace, each beneath the directory named for the port it fills.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Platform"];
            let definition = "The module of the contexts of the programs of the client, one for every operating system or page they run on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tool"];
            let definition = "The module of the contexts of the tools of the workspace and of the game, programs beside the game such as the vocabulary, the coverage and the world survey.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Shared", "Kernel"];
            let definition = "The module of the contexts of the crates of the game that every bounded context shares, such as the physics, the key and the content.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Web"];
            let definition = "The module of the context of a crate for the web, named for the web that crate runs on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Core"];
            let definition = "The module of the contexts of the crates of the core of the engine, beneath the module of the engine, of the contexts of the crates of the domain of the client, beneath the module of the client, and of the contexts of the crates of the domain of the content factory, beneath the module of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Input"];
            let definition = "The module of the context of the crate stack_engine_core_input, beneath the module of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Ash"];
            let definition = "The foreign language of the crate ash, the bindings of Rust to Vulkan.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Processing", "Unit", "Allocator"];
            let definition = "The foreign language of the crate gpu-allocator, which allocates the memory of a device of Vulkan for the crate stack_engine_adapter_gpu_vulkan.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Naga"];
            let definition = "The foreign language of the crate naga, with which the build of the crate stack_engine_adapter_gpu_vulkan compiles the shaders from WGSL into SPIR-V and the build of the crate stack_engine_adapter_gpu_webgpu validates its shaders, and which the contexts of those crates name themselves, because the gate reads no build dependencies of a manifest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Swash"];
            let definition = "The module of the context of the crate stack_engine_adapter_typeface_swash, beneath the module of the typeface, and the foreign language of the crate swash, which shapes text and scales the outlines of glyphs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gltf"];
            let definition = "The module of the context of the crate stack_engine_adapter_assets_gltf, beneath the module of the assets, and the foreign language of the crate gltf, through which that crate reads files of glTF 2.0.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gilrs"];
            let definition = "The foreign language of gilrs, the library of the gamepads.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Cpal"];
            let definition = "The module of the context of the crate stack_engine_adapter_sound_cpal, beneath the module of the sound, and the foreign language of the crate cpal, through which that crate plays the sound of the system, natively and in the browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Symphonia"];
            let definition = "The foreign language of the crate symphonia, through which the crate stack_engine_adapter_assets_serious_engine decodes the sounds of the reference assets.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unicode", "Bidirectional", "Algorithm"];
            let definition = "The foreign language of the crate unicode-bidi, which resolves the levels of a text by the bidirectional algorithm of Unicode.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "International",
                "Components",
                "For",
                "Unicode",
                "Locale",
                "Core",
            ];
            let definition = "The foreign language of the crate icu_locale_core of the International Components for Unicode, which parses and checks a language tag of BCP 47.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["International", "Components", "For", "Unicode", "Locale"];
            let definition = "The foreign language of the crate icu_locale of the International Components for Unicode, which yields the script a language is likely written in and the direction of that script.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["International", "Components", "For", "Unicode"];
            let definition = "The module of the foreign languages of the crates of the International Components for Unicode.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Locale"];
            let definition = "The module of the foreign language of the crate icu_locale, with the foreign language of its core, the crate icu_locale_core, beneath it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Window"];
            let definition = "The module of the contexts of the adapters of the port of the window of the engine, the crates under crates/stack/engine/adapter/window.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Winit"];
            let definition = "The module of the context of the crate stack_engine_adapter_window_winit, beneath the module of the window, and the foreign language of the crate winit, whose windows, events and event loop that crate drives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Arboard"];
            let definition = "The foreign language of the crate arboard, the clipboard of the Linux client under the X Window System, under Wayland and under Windows.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Softbuffer"];
            let definition = "The foreign language of the crate softbuffer, with which the live tests of the crate stack_engine_adapter_window_winit fill the window with pixels, so a Wayland compositor shows it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["X11rb"];
            let definition = "The foreign language of the crate x11rb, with which the live tests of the crate stack_engine_adapter_window_winit focus windows and press keys and buttons on an X server through its extension XTEST.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application"];
            let definition = "The module of the context of the crate stack_game_application, beneath the module of the game, of the contexts of the crates of the application of the engine, beneath the module of the engine, of the context of the crate stack_server_application, beneath the module of the server, of the context of the crate products_dusk_requiem_tool_world_survey_application, beneath the module of the world survey, of the contexts of the crates of the application of the client, beneath the module of the client, and of the contexts of the crates of the application of the content factory, beneath the module of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Item"];
            let definition = "The module of the context of the crate stack_game_core_item, beneath the module of the game.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reason"];
            let definition = "The module of the context of the crate stack_game_core_reason, beneath the module of the game.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Session"];
            let definition = "The module of the context of the crate stack_game_core_session, beneath the module of the game.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "The module of the context of the crate stack_game_core_text, beneath the module of the game, and the module of the context of the crate stack_game_adapter_typesetter_text, beneath the module of the typesetter, named for the texts of the content that typesetter translates its labels through.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Image"];
            let definition = "The foreign language of the crate image, which decodes the pictures of the media of the content factory and encodes their previews.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Secure", "Hash", "Algorithm2"];
            let definition = "The foreign language of the crate sha2, which computes the SHA-256 digest of the media of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Procedural", "Macro2"];
            let definition = "The foreign language of the crate proc-macro2, whose tokens of Rust syn parses for the adapter gates_vocabulary_language_adapter_syntax and the adapter gates_coverage_adapter_syntax.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rusty", "Object", "Notation"];
            let definition = "The foreign language of the crate ron, the notation in which the files of a package of the content are written, which the crate stack_game_adapter_content_ron reads.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Portable", "Network", "Graphics"];
            let definition = "The foreign language of the crate png, the format of the images in which the viewer writes the contact sheets of the reference assets.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Khronos", "Texture2"];
            let definition = "The foreign language of the crate ktx2, which reads the KTX2 containers of the textures of the content.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Java", "Script", "System"];
            let definition = "The foreign language of the crate js-sys, the objects of JavaScript in the browser, such as the arrays of bytes the crate stack_engine_adapter_assets_source_http reads, which the crates that run in the browser exchange with it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Web", "Assembly", "Binding", "Generator"];
            let definition = "The foreign language of the crate wasm-bindgen, the values of JavaScript and their conversions in WebAssembly, which the crates stack_engine_adapter_assets_source_http, stack_engine_adapter_gpu_webgpu and stack_engine_adapter_window_winit use in the browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Futures"];
            let definition = "The name of the directory of the foreign language of the crate wasm-bindgen-futures, beneath the foreign language of wasm-bindgen.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Web", "Assembly", "Binding", "Generator", "Futures"];
            let definition = "The foreign language of the crate wasm-bindgen-futures, which turns a promise of JavaScript into a future of Rust and spawns a future in the browser, so the crates that run in the browser await a fetch, its storage, its clipboard and its GPU, and the actor of the crate stack_game_adapter_world_memory runs there.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test"];
            let definition = "The name of the directory of the foreign language of the crate wasm-bindgen-test, beneath the foreign language of wasm-bindgen.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Web", "Assembly", "Binding", "Generator", "Test"];
            let definition = "The foreign language of the crate wasm-bindgen-test, whose attribute runs a test of the crates stack_engine_adapter_assets_source_http and stack_engine_adapter_gpu_webgpu in a headless browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Web", "System"];
            let definition = "The foreign language of the crate web-sys, the interfaces of the browser: its window, fetch and cache storage, through which the crate stack_engine_adapter_assets_source_http reads what it fetches, its canvas and its GPU, which the crate stack_engine_adapter_gpu_webgpu draws through, and its clipboard and its clock, which the crate stack_engine_adapter_window_winit reaches.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Assembly", "Binding", "Generator"];
            let definition = "The module of the foreign language of the crate wasm-bindgen of the web, with the foreign languages of its futures and its test beneath it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["System"];
            let definition = "The module of the foreign language of the crate web-sys of the web.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Time"];
            let definition = "The module of the foreign language of the crate web-time of the web.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source"];
            let definition = "The module of the contexts of the crates under crates/stack/game/adapter/content/source, which fill the port of stack_game_adapter_content_ron where a package of the content comes from, and of the crates under crates/stack/engine/adapter/assets/source, which fill the port of stack_engine_adapter_assets_serious_engine where the files of the reference assets come from.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File"];
            let definition = "The module of the context of the crate stack_engine_adapter_assets_source_file and of the crate products_editor_adapter_generation_content_factory_adapter_storage_file, named for the file system the first reads the package of the content and the files of the reference assets from and the second the definitions and the media of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Ureq"];
            let definition = "The foreign language of the crate ureq, the HTTP client that the crates products_editor_adapter_generation_content_factory_adapter_generator_comfyui and products_editor_adapter_generation_content_factory_adapter_station_process call synchronously: its agent, its configuration, its requests and its responses.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process"];
            let definition = "The module of the context of the crate products_editor_adapter_generation_content_factory_adapter_station_process, named for the processes the station starts, beneath the module of the station of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Line"];
            let definition = "The module of the context of the crate products_editor_adapter_generation_content_factory_command_line, named for the program of the owner and the agents, beneath the module of the content factory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Syntax"];
            let definition = "The foreign language of the crate syn, which parses the syntax of the sources of Rust for the adapter gates_vocabulary_language_adapter_syntax and the adapter gates_coverage_adapter_syntax.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Browser"];
            let definition = "The module of the context of the settings of the game in the storage of the browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics"];
            let definition = "What a program draws, the kind of work a group of foreign languages serves, and the module of the contexts of the crates of the graphics of the core of the engine, beneath the module of the core.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Media"];
            let definition = "The sound and the type a program plays and sets, the kind of work a group of foreign languages serves.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Notation"];
            let definition = "A written form of data, the kind of work a group of foreign languages serves.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program"];
            let definition = "The work every program does whatever it shows, the kind of work a group of foreign languages serves.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Language", "Context", "Outcomes"];
            let definition = "To compose the outcome of every context of the crates of the language of the vocabulary gate: its core, its application, its adapters and its command line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Context", "Outcomes"];
            let definition = "To compose the outcome of every context of this repository.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Foreign", "Crate"];
            let definition = "A crate outside the workspace that the crates of Dusk Requiem use, whose names its foreign language admits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Window", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates that open a window and read its input.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Graphics", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates that draw.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Media", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates that play sound, read models and set type.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Browser", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates that run a program in the browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Notation", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates that read and write data in a written form.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Program", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates whose work every program does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Concurrency", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of the crates that run the work of a program side by side: its channels, its wakers, its pool of threads and its runtime.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Concurrency"];
            let definition = "The work of a program that runs side by side, whose foreign languages are the crates of its channels, its wakers, its pool of threads and its runtime.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the outcome of every foreign language of every foreign crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library"];
            let definition = "The module of the context of the crate products_dusk_requiem_game_library, beneath the module of the game of Dusk Requiem, named for the library of the rules of the game it is.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The module of the context of the crate stack_engine_core_navigation_path, beneath the module of the navigation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mode"];
            let definition = "The directory of the contexts of the adapters of the mode of the client, under client/adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presentation"];
            let definition = "The module of the contexts of the presentations of the client, a library for the operating systems of a desktop and one for a page of the browser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Transport"];
            let definition = "The module of the contexts of the adapters of the transport of the protocol.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Quinn"];
            let definition = "The foreign language of the crate quinn, the implementation of QUIC.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rust", "Transport", "Layer", "Security"];
            let definition = "The foreign language of the crate rustls, the implementation of TLS.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rust", "Certificate", "Generator"];
            let definition = "The foreign language of the crate rcgen, which generates certificates of X.509.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Network", "Foreign", "Language", "Outcomes"];
            let definition = "To compose the foreign languages of the crates of the network.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Network", "Foreign", "Language", "Outcomes"];
            let definition = "The outcomes of composing the foreign languages of the crates of the network.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Certificate", "Generator"];
            let definition = "The module of the foreign language of the crate rcgen, which generates certificates of X.509.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Transport", "Layer", "Security"];
            let definition = "The module of the foreign language of the crate rustls, the implementation of TLS.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Argon2"];
            let definition = "The module of the context of the hasher of passwords with Argon2; also the foreign language of the crate argon2.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Get", "Random"];
            let definition = "The foreign language of the crate getrandom, the randomness of the operating system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Postcard"];
            let definition = "The module of the contexts of the adapters over postcard; also the foreign language of the crate postcard.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Structured", "Query", "Language", "Extension"];
            let definition = "The foreign language of the crate sqlx.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Process", "Core", "Invocation"];
            let definition = "The context of the crate stack_process_core_invocation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Process", "Adapter", "Spawner", "Transcript"];
            let definition = "The context of the crate stack_process_adapter_spawner_transcript.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Spawner"];
            let definition = "The module of the contexts under crates/stack/process/adapter/spawner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Transcript"];
            let definition = "The module of the contexts under crates/stack/process/adapter/spawner/transcript.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack", "Process", "Adapter", "Spawner", "Standard", "Library",
            ];
            let definition = "The context of the crate stack_process_adapter_spawner_standard_library.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Name"];
            let definition = "The context of the crate stack_lock_core_name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock"];
            let definition = "The module of the contexts under crates/stack/lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Device", "Address"];
            let definition = "The context of the crate stack_lock_core_device_address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device"];
            let definition = "The module of the contexts under crates/stack/lock/core/device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Address"];
            let definition = "The module of the contexts under crates/stack/lock/core/device/address.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Device", "File"];
            let definition = "The context of the crate stack_lock_core_device_file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Demand"];
            let definition = "The context of the crate stack_lock_core_demand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Demand"];
            let definition = "The module of the contexts under crates/stack/lock/core/demand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Graphics", "Lock"];
            let definition = "The context of the crate stack_lock_core_graphics_lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Graphics", "User"];
            let definition = "The context of the crate stack_lock_core_graphics_user.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["User"];
            let definition = "The module of the contexts under crates/stack/lock/core/graphics/user.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Application"];
            let definition = "The context of the crate stack_lock_application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["C", "Library"];
            let definition = "The foreign language of the crate libc, whose system call bpf the adapter of the device program makes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rustix"];
            let definition = "The foreign language of the crate rustix, which sends the signals to a process and holds the lock of a file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signal", "Hook"];
            let definition = "The foreign language of the crate signal-hook, which traps the signals of an end for the guard of a run.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Adapter", "Pause", "Standard", "Library"];
            let definition = "The context of the crate stack_lock_adapter_pause_standard_library.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pause"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/pause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Lock", "Adapter", "Program", "Runner", "Spawner"];
            let definition = "The context of the crate stack_lock_adapter_program_runner_spawner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Runner"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/program_runner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack",
                "Lock",
                "Adapter",
                "Host",
                "Helper",
                "Elevation",
                "Program",
            ];
            let definition = "The context of the crate stack_lock_adapter_host_helper_elevation_program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host", "Helper"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/host_helper.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Elevation", "Program"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/host_helper/elevation_program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Lock", "Record", "File", "System",
            ];
            let definition = "The context of the crate stack_lock_adapter_lock_record_file_system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Record"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/lock_record.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack",
                "Lock",
                "Adapter",
                "Control",
                "Group",
                "Unified",
                "Hierarchy",
            ];
            let definition = "The context of the crate stack_lock_adapter_control_group_unified_hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Control", "Group"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/control_group.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Unified", "Hierarchy"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/control_group/unified_hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Task", "Table", "Process", "File", "System",
            ];
            let definition = "The context of the crate stack_lock_adapter_task_table_process_file_system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Task", "Table"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/task_table.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process", "File", "System"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/task_table/process_file_system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Device", "Tree", "System", "File", "System",
            ];
            let definition = "The context of the crate stack_lock_adapter_device_tree_system_file_system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Tree"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/device_tree.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["System", "File", "System"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/device_tree/system_file_system.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Device", "Program", "Berkeley", "Packet", "Filter",
            ];
            let definition = "The context of the crate stack_lock_adapter_device_program_berkeley_packet_filter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Program"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/device_program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Berkeley", "Packet", "Filter"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/device_program/berkeley_packet_filter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Products",
                "Baton",
                "Presentation",
                "Command",
                "Line",
                "Interface",
            ];
            let definition = "The context of the crate products_baton_presentation_command_line_interface.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Products"];
            let definition = "The module of the contexts under crates/products.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Baton"];
            let definition = "The module of the contexts under crates/products/baton.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Line", "Interface"];
            let definition = "The module of the contexts under crates/products/baton/presentation/command_line_interface.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tree"];
            let definition = "The module of the contexts under crates/stack/lock/adapter/device/tree.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Products", "Baton", "Presentation", "Host"];
            let definition = "The context of the crate products_baton_presentation_host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Host"];
            let definition = "The module of the contexts under crates/products/baton/presentation/host.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyGateContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyGateContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyGateContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyGateContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_tool_vocabulary_context_is_a_term() {
        let gates_vocabulary_gate_context = GatesVocabularyGateContext;

        let context = Context::try_from(gates_vocabulary_gate_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
