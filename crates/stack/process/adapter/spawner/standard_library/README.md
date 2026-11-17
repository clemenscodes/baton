# stack_process_adapter_spawner_standard_library

The one adapter that starts programs, over `std::process`. It fills `Spawner` and is the only crate
of baton whose tests start a program, so it names `endToEnd = true` and its tests run under
`test-processes`, never under nextest. Its tests run with
`cargo nextest run --profile debug -p stack_process_adapter_spawner_standard_library`.

## The book

- [The spawner over the standard library](docs/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
