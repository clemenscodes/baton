# gates_vocabulary_language_core_dependencies

The dependencies of the crate of a source: its package identifier and the foreign names and paths
of the crates its production code and its tests depend on, read from its manifest and from the
manifest of the workspace. It declares the port `Decoder`, through which the text of a manifest is
decoded.

It is a crate of the domain of the vocabulary gate, its concept `Dependencies` in its `lib.rs`. It
does no IO and keeps no state, and its tests are pure unit tests. A table of dependencies that
names one foreign name twice is refused with its `Error`.

```
moon run gates_vocabulary_language_core_dependencies:ci
```

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
