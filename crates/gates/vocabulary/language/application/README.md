# gates_vocabulary_language_application

The application of the vocabulary gate, one library: the gate that turns away an identifier made of
a word the context it is in does not have, and the press that issues the ubiquitous language books
of the contexts. It is a crate of the layer `application`.

Its `lib.rs` publishes its port `vocabulary::Vocabulary`, its services `gate::Gate` and
`press::Press`, their queries and their command, and its `Error`. The gate answers the queries
`gate::PathJudgment`, `gate::TermJudgment` and `gate::token::TokenSelection`; the press runs the
command `press::BookEdition`. Each service takes the driven ports of the core it needs, the reader,
the decoder and the parser for the gate and the writer for the press, as type parameters, so the
command line composes the adapters and names no crate of the core.

```
tool_vocabulary crates/harbor/src/lib.rs
[LANGUAGE] crates/harbor/src/lib.rs:1:8: `Zonk` is not made of words of this crate's vocabulary.
```

It writes nothing but the books and touches the world only through its ports, so its tests fill
them with fixed adapters of their own.

```
moon run gates_vocabulary_language_application:ci
```

## The book

- [The vocabulary](docs/vocabulary/README.md)
- [The gate](docs/gate/README.md)
- [The press](docs/press/README.md)
- [The errors](docs/error/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
