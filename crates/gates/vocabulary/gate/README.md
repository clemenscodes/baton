# gates_vocabulary_gate

The contexts of Dusk Requiem, and the program that judges every source against them.

The crate has one layer, `src/domain`, because every type in it is a type of the domain:
the contexts under `src/domain/contexts` and the foreign languages under
`src/domain/foreign_languages`, and every one of them implements `Layer` with the domain
layer.

`gates_vocabulary_gate crates/gates/vocabulary/gate` judges the sources passed to it, a directory standing for every
Rust file under it, and
`tool_vocabulary --book` prints the book of every context. A new term is added only after the owner has
approved it.

## The book

- [The contexts](docs/domain/contexts/README.md)
- [The forms of the program](docs/main/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
