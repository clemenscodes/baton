# gates_vocabulary_language_core_foreign_name

The name of a foreign language, the name of a crate outside the workspace whose names a source may
use. The dependencies, the languages and the decoder of the manifests all name a foreign language
by it.

It is a crate of the domain of the vocabulary gate, its concept `ForeignName` in its `lib.rs`. It
declares no port, does no IO and keeps no state, and its tests are pure unit tests.

```
moon run gates_vocabulary_language_core_foreign_name:ci
```

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
