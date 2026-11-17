# gates_vocabulary_language_adapter_command_line

The command line of the vocabulary gate: the paths as arguments, with `--book`, `--terms` or
`--tokens`, parsed by clap, and the exit code it responds with. A repository that uses the library
builds its contexts in a program of its own, conventionally `crates/gates/vocabulary/gate`, fills
the port `Vocabulary` of `gates_vocabulary_language_application` with them and hands it to this
crate in a `Session`, which composes the adapters `FileSystem`, `TomsObviousMinimalLanguage` and
`Syntax` into the gate or the press of the application and serves the command line with it.

It is the presentation of the vocabulary gate, its concept `CommandLine` in its `lib.rs`. It reads
the arguments into a query or a command of the application, composes the adapters, runs the use
case and reports its outcome, and it names no crate of the core. Its tests fill the vocabulary with
a fixed adapter of their own and run the real adapters over sources in a temporary directory.

```
moon run gates_vocabulary_language_adapter_command_line:ci
```

## The book

- [The session](docs/session/README.md)
- [The tokens](docs/session/token_report/README.md)
- [The outcome](docs/outcome/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
