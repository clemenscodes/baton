# gates_coverage_adapter_file_system

The adapter of the file system of the coverage, which fills the driven ports `Reader` and `ProfileStore`.

It is a crate of the adapters of the coverage, its concept in its `lib.rs`. Its tests read and write files under a directory of their own and start no program, so nextest runs them.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
