# gates_coverage_application

The application of the coverage, one library: the gate, which judges a report and refuses every
region outside a test module that no test reaches, and the measurement, which builds and runs the
tests of a package instrumented and writes the report the gate judges. It is a crate of the layer
`application`.

Its `lib.rs` publishes its services `gate::Gate` and `measurement::Measurement`, the query
`gate::ReportJudgment` with its answer `gate::Judgment`, the command `measurement::ReportMeasurement`,
the `Outcome` either comes to and its `Error`; its parts are private modules. Each service takes the
driven ports of the core it needs as type parameters, the decoder, the reader and the parser for the
gate and the spawner, the profile store and the decoder for the measurement, so the program
`tool_coverage` composes the adapters and names no crate of the core. The application writes to no
stream: the program reports the judgment and the error.

```
moon run gates_coverage_application:ci
```

## The book

- [The gate](docs/gate/README.md)
- [The measurement](docs/measurement/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
