# gates_coverage_core_report

The report of a coverage: its regions by span, the report file a measurement writes, and the port `Decoder` through which the gate and the measurement read a report and a test build.

It is a crate of the domain of the coverage, its concept in its `lib.rs`. It declares the driven port `Decoder`, which `gates_coverage_adapter_java_script_object_notation` fills over serde_json. It does no IO itself, and its tests are pure unit tests.

## The book

- [The report file](docs/file/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
