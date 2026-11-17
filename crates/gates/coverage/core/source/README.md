# gates_coverage_core_source

The source of Rust a report names: its path and its text, and the port `Reader` through which the gate reads a source and opens a report.

It is a crate of the domain of the coverage, its concept in its `lib.rs`. It declares the driven port `Reader`, which `gates_coverage_adapter_file_system` fills. It does no IO itself, and its tests are pure unit tests.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
