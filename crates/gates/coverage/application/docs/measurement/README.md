# The measurement

The measurement `measurement::Measurement` runs the command `measurement::ReportMeasurement`
through its `SynchronousCommandHandler`. The command is read from the words of the program with
`ReportMeasurement::try_from`: the path of the report, the command that compiles the tests, `--` and
the command that runs them, or `Error::Words`.

The measurement creates a profile directory of its own beside the report, compiles the tests
instrumented and decodes their executables, runs the tests, locates the profile tools beside the
library of the compiler, inspects the executables for a coverage mapping, merges and exports the
profiles where there is one, removes the profile directory whatever happened, and writes the report
file. Every step that fails is an `Error`, and the error of the steps comes before the error of the
removal.

The measurement changes the report on disk, so it is a command. It changes no aggregate: the report
is computed whole from the tests and written, and nothing is read back. So it implements no
`Command` of an aggregate and travels in no `Dispatch`, whose identity would name an aggregate that
does not exist.
