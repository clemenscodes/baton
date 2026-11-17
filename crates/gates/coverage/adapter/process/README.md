# gates_coverage_adapter_process

The adapter of the coverage that fills the driven port `Spawner` over the processes of the operating system.

It is a crate of the adapters of the coverage, its concept in its `lib.rs`. Its tests start programs, so they are end-to-end: the crate names `endToEnd = true`, runs under the runner `test-processes`, and nextest leaves it out.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
