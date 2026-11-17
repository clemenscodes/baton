# gates_vocabulary_language_adapter_toms_obvious_minimal_language

The adapter of the port `Decoder` over TOML. It decodes the package name, the dependencies, the
development dependencies and the workspace dependencies of a `Cargo.toml`, the tables of every
platform under `[target]` included, and refuses a manifest whose dependencies or whose workspace
are no table and a path of a dependency that is no text.

It is an adapter of the infrastructure of the vocabulary gate, its concept
`TomsObviousMinimalLanguage` in its `lib.rs`. Its tests decode manifests they hold as text.

```
moon run gates_vocabulary_language_adapter_toms_obvious_minimal_language:ci
```

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
