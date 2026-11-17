# gates_coverage_core_test_boundaries

The test boundaries of a source: the lines its test modules open and close on, and the port `Parser` that parses them from the source.

It is a crate of the domain of the coverage, its concept in its `lib.rs`. It declares the driven port `Parser`, which `gates_coverage_adapter_syntax` fills over syn and proc-macro2. It does no IO itself, and its tests are pure unit tests.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
