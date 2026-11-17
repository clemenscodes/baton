# gates_coverage_command_line

The program `tool_coverage`, which measures the coverage of the tests of a package and judges the report
that measurement writes.

```
tool_coverage measure <report> <compilation command> -- <test command>
tool_coverage <report>
```

`measure` compiles the tests with the compilation command, which states their executables in messages of
Cargo, runs them with the test command, both instrumented and with a directory of profiles of their
own, merges and exports those profiles with the tools of LLVM beside rustc and writes the export to
the report. Without a path the gate reads the export from its standard input.

It is the presentation of the coverage. It reaches the application, `gates_coverage_application`, and
the four adapters it composes it with, and never a crate of the core. It reads its arguments into the
query `ReportJudgment` or the command `ReportMeasurement`, composes the gate or the measurement over
the adapters, runs the use case and reports its outcome: the success text on the standard output,
every gap and every error on the standard error, and the exit code of the outcome. Its binary keeps
the name `tool_coverage`, which every task of the coverage runs. Its tests run the real adapters over
reports on an input and sources in a temporary directory.

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
