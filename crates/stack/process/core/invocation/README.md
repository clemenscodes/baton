# stack_process_core_invocation

The invocation of a program, shared by every family that drives one. `Invocation` holds the program,
its arguments, its variables and its `Stream`, `Completion` is how the program ended, and the driven
port `Spawner` starts it and hands back a `RunningProgram` where the caller works beside it. Its
tests run with `cargo nextest run --profile debug -p stack_process_core_invocation`.

## The book

- [The invocation](docs/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
