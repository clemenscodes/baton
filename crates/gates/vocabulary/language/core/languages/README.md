# gates_vocabulary_language_core_languages

Every language of a repository: Rust, the contexts and the foreign languages, and the language a
source is judged against.

It is a crate of the domain of the vocabulary gate, its concept `Languages` in its `lib.rs`. It
does no IO and keeps no state, and its tests are pure unit tests.

There is one language per bounded context, and a context is a directory under a root, `crates`
unless the context names another. The contexts belong to the repository that uses this library and
live in that repository's own crate.

The library knows no third party crate. A repository hands over one foreign language per third
party crate it uses, each a flat list of the words that crate brought along. A source is judged
against the language of a foreign crate when its own `Cargo.toml` names that crate. A crate that
loses a dependency loses its words in the same act. The contexts of the crates a crate depends on
by path are its languages, and so are the contexts of the crates those depend on; the development
dependencies are languages of its test module alone. A context may also name a foreign language it
uses without a manifest.

Rust is shared by every context. A context's own language and its foreign languages are not.
`Languages` adds Rust's own words and names and the test words, and its `Error` is either the error
of a language or a path that is not the directory of a context.

```
moon run gates_vocabulary_language_core_languages:ci
```

## The book

- [Rust](docs/rust/README.md)
- [The directory of a context](docs/contexts/context/directory/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
