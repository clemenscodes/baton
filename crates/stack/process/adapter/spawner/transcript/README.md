# stack_process_adapter_spawner_transcript

An adapter of `Spawner` that starts nothing. It records every invocation and hands out the
completions it was made with, so the adapters that drive a program are unit tested without one. Its
tests run with `cargo nextest run --profile debug -p stack_process_adapter_spawner_transcript`.

## The book

- [The transcript spawner](docs/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
