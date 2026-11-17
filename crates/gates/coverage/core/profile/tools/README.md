# gates_coverage_core_profile_tools

The profile tools of a measurement: the tools of LLVM beside rustc that inspect the executables of the tests, merge their profiles and export their coverage.

It is a crate of the domain of the coverage, its concept in its `lib.rs`. It does no IO and keeps no state, and its tests are pure unit tests.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
