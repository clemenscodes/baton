# gates_coverage_core_profile_directory

The profile directory of a measurement, where its tests write their profiles, and the port `ProfileStore` that creates it, lists it, removes it and writes the report file.

It is a crate of the domain of the coverage, its concept in its `lib.rs`. It declares the driven port `ProfileStore`, which `gates_coverage_adapter_file_system` fills. It does no IO itself, and its tests are pure unit tests.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
