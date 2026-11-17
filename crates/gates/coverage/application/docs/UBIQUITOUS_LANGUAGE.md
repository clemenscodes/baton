# The ubiquitous language

| Word | What it means |
|---|---|
| Application | The layer of the coverage that holds its use cases, the judgment of a report by the gate and the measurement of a package, and their outcome. |
| Assignment | The commands of one measurement with the profile directory it keeps its profiles in. |
| Command | A command a measurement starts, as its words: the command that compiles the tests or the command that runs them. |
| Compilation | The compilation of the tests of a package, the first step of a measurement, whose output states the executables of the tests. |
| Compilation Word | A word of the command that compiles the tests and states their executables in messages of Cargo. |
| Compile Tests | To build the tests of an assignment and decode their test build from the messages of the build. |
| Compose Compilation Invocation | To compose the invocation of the compilation of the tests, which captures its output and compiles them instrumented, with the profile variable. |
| Compose Profile Root | To compose the profile root beside the report of a measurement. |
| Compose Test Invocation | To compose the invocation of the tests, which shows its output and runs instrumented with the profile variable. |
| Compose Variables | To compose the variables every invocation of a measurement gets: the profile variable and the instrumentation. |
| Coverage | The crate that judges whether the tests of a crate reach every region of its sources outside their test modules, and the context of that crate. |
| Decide Outcome | To decide the outcome of a judgment from its gaps: a success where no gap remains, and an error otherwise. |
| Decode Report | To decode a report out of an input through the decoder, or the error of the decoder as text. |
| Decode Report File | To open the report at a path through the reader and decode it, or the error that its file cannot be read. |
| Errors | The stream the command line writes every error and every gap of a judgment to, and the error of a measurement. |
| Exit Code | The status the program of coverage ends with, a success for the outcome of a success and a failure for the outcome of an error. |
| Export | The text that llvm-cov export writes in the Java Script Object Notation, holding the data of every binary of the tests, which a measurement writes into its report file, and the error of the adapter Java Script Object Notation for a text that is no export, which carries the error of serde_json. |
| Failure | A completion of a program that ended with a status other than success. |
| File | A file of an export, named by its filename, whose regions the data keeps, and the error of a code region whose file index its function lacks, which names that file index and the file count of that function; and the file of syn, the items of a source as syn parses them, whose modules the test module visitor visits. |
| Gap Count | How many gaps the gaps of a report hold, 0 for the outcome success. |
| Gate | The service of the application over the decoder, the reader and the parser that answers a report judgment with the judgment of its report. |
| Gates Coverage Application | The crate gates_coverage_application, the application of the coverage, one library: the gate that judges a report and the measurement that writes one. |
| Instrumentation | The flags with which Cargo builds the tests so that they count every region they reach. |
| Instrumentation Flag | The flag -Cinstrument-coverage, with which rustc counts every region a program reaches. |
| Instrumentation Variable Name | The name CARGO_BUILD_RUSTFLAGS of the variable that carries the instrumentation. |
| Judge Report | To decode a report out of the input, read the test boundaries of its sources, form its gaps outside those test boundaries and write the outcome of them, to write the error to the errors where the report cannot be decoded or a source cannot be read or is no Rust, and to flush the output and the errors. |
| Judgment | The outcome of a report judgment: the gaps of the report, read as the success text where there is none and as a line for every gap otherwise. |
| Line Separator | The line break between the lines of the gaps of a judgment. |
| Locate Profile Tools | To ask the compiler for the directory of its library and locate the profile tools beside it. |
| Measure | To measure the coverage of the tests of a package and report whether the measurement failed. |
| Measure Assignment | To build, run, merge and export the tests of an assignment into the report file it names. |
| Measure Report | To measure the coverage of a package from the words of a measurement and write its report, removing the profile directory whatever happens. |
| Measurement | The measurement of the coverage of the tests of one package: it builds the tests, runs them instrumented with a profile directory of their own, merges and exports their profiles and writes the report. Also the application service that runs a report measurement and the mode of the program that starts it. |
| Outcome | What a judgment or a measurement comes to, a success where no gap remains and the measurement wrote its report, and an error otherwise, and the exit code it leaves. |
| Process | The adapter of the spawner over the processes of the operating system. |
| Profile Absence | Tests that left no profile, so their build was not instrumented. |
| Profile Root Name | The name profiles of the profile root. |
| Program Start | The start of a program that failed, with its cause. |
| Read Test Boundaries | To read the source of every path a report names, to parse the test boundaries of each of those sources with the parser and to form the test boundaries of them all, and to refuse a source that cannot be read and a source that is no Rust. |
| Removal | The removal of the profile directory of a measurement, whose error the measurement reports after the error of its assignment. |
| Report Export | The error of a report whose file cannot be read, naming its path and the cause. |
| Report File | The report a measurement writes, with its path and its export. |
| Report Judgment | The query of the gate that judges the report in a file at a path or on an input, and answers with its judgment or the error that refused it. |
| Report Measurement | The command of a measurement, read from its words: the path of its report, the command that compiles the tests and the command that runs them. |
| Report Path | The path of the report of a measurement. |
| Report Word | The first word of a measurement, the path of its report. |
| Separator Word | The word -- between the command of the build and the command of the tests. |
| Service | A service of the program that its mode chooses: the judgment of a report or a measurement, as main composes it. |
| Success | The outcome of a gate whose count of gaps is zero, a report without a region with no case of its own, which ends the program with the exit code of a success. |
| Success Text | The line the gate writes to the output where no gap remains. |
| Syntax | The adapter of the parser over syn, which lexes the text of a source into its tokens with proc-macro2, parses those tokens into its file and visits every module of it with the test module visitor; the syntax tree of a source as syn parses it, which the test module visitor visits for the lifetime syntax; the error of the adapter for a text that is no Rust, which names the path of its source and the line and the column syn refuses it at and carries the error of syn, and the error of the gate that carries that error of the parser. |
| Test Word | A word of the command that runs the tests. |
| Write Failure | To write the failure of a program, naming the program and its exit status. |
| Write Profile Absence | To write the error of tests that left no profile, because their build was not instrumented. |
| Write Profile Directory Error | To write the error of a profile directory that cannot be created, listed or removed, naming the cause. |
| Write Program Start Error | To write the error of a program that cannot start, naming the program and the cause. |
| Write Report Export Error | To write the error of a report whose file cannot be read, naming its path and the cause. |
| Write Source Error | To write the error of a source that cannot be read, naming its path and the cause. |
| Write Words Error | To write the error of words that are no measurement, naming what a measurement takes. |
