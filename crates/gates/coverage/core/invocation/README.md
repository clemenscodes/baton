# gates_coverage_core_invocation

The invocation of a program: the program with its arguments, the variables it adds to its environment and the stream of its output, with its completion and the port `Spawner` that starts it.

It is a crate of the domain of the coverage, its concept in its `lib.rs`. It declares the driven port `Spawner`, which `gates_coverage_adapter_process` fills over the processes of the operating system. It does no IO itself, and its tests are pure unit tests.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
