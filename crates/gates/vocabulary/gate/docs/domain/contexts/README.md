# The contexts

A context lies under `src/domain/contexts` at the path of its crate under `crates`, so the
context of `crates/gates/coverage/core/span` is `src/domain/contexts/gates/coverage/core/span` and the context of
`crates/stack/game/core/zone` is `src/domain/contexts/stack/game/core/zone`, and defines the terms of that crate.
Its type is named after the crate, followed by `Context`, such as `GatesCoverageCoreSpanContext`,
`StackGameCoreZoneContext` and `GatesVocabularyGateContext`, and converts into the `Context` of the library.
Its constant `DIRECTORY_NAME` names the directory under `crates` that it covers, such as
`stack/game/core/zone` for `crates/stack/game/core/zone`, and decides where its book is printed, its constant
`TERM_TEXTS` holds its terms, and every context names the standard library of Rust as a foreign
language it uses. `src/domain/foreign_languages` holds one module per crate outside the
workspace that the crates of Dusk Requiem use, each a type whose `NAME` and `NAMES` convert into
the foreign language of that crate, because the library knows no crate outside the workspace; its own name
spells the crate out, as `TomsObviousMinimalLanguage` for `toml` and `SerdeJavaScriptObjectNotation`,
beneath `serde`, for `serde_json`, while its `NAME` is the name of the crate. A crate of the
workspace needs no foreign language, because the terms of its context spell its names for every
crate that depends on it. Every foreign language is an anti-corruption layer, and its names admit
only what the code uses, never what it declares. `ForeignLanguages` holds the foreign languages
of all of these crates, and `Contexts` stands for the contexts of Dusk Requiem: it fills the port
`Vocabulary` of `gates_vocabulary_language_application`, listing every context and every foreign
language. `main.rs` hands `Contexts` with the standard output and the standard error to the command
line as a `Session`. The application composes the languages from the two lists and reports the
error where they cannot be built, and the command line composes the file system, the decoder over
TOML and the parser over syn. The library does everything else: it judges a source and prints a
book.

## A new term

A new term is added, with its definition, in the module of the context it belongs to, and
only after the owner has approved both.
