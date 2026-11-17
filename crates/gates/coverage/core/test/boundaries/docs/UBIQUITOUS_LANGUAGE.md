# The ubiquitous language

| Word | What it means |
|---|---|
| Closing Line | The line a test boundary closes on: the line of the end of the brace of its test module. |
| Decision | The decision of a test boundary or of the test boundaries about a span: its acceptance outside every test module and its rejection inside one, which decides whether an unreached region is a gap. |
| Gates Coverage Core Test Boundaries | The crate gates_coverage_core_test_boundaries, the crate of the domain of the coverage that holds the test boundaries of a source, and the port of the parser. |
| Module | A module of the file of a source as syn parses it, which the test module visitor visits, and a test module where it is the inline module of the test module name that a test attribute stands on. |
| Opening Line | The line a test boundary opens on: the line of the hash mark of the test attribute of its test module. |
| Parse Test Boundaries | To parse the text of a source into the test boundary of every test module of it, and to refuse a text that is no Rust with a syntax error and a test module that is no test boundary with the error of that test boundary. |
| Parser | The port that parses the test boundaries of a source, and the parser of the gate. |
| Production | The part of a source outside its test module, whose spans the test boundaries accept and whose spans no test reaches are gaps. |
| Sequence | The values of a list one after another, as an iterator yields them, such as the arguments of the program or the words of a command. |
| Start Line | The line an export region starts on, as llvm-cov writes it, and the line of the start of a span, which a test boundary locates. |
| Test | A test of the package a measurement measures, which counts the regions it reaches while it runs. |
| Test Boundaries | The test boundary of every test module of the sources the gate reads, each once and in order, a specification of a span that accepts a span every test boundary accepts, and the test boundaries the parser parses out of a source, which the gate forms them of. |
| Test Boundary | The path of a source and the opening line and the closing line of a test module of that source, a specification of a span that accepts a span outside it and refuses one whose start line lies from its opening line to its closing line in its source, and refuses an opening line below the first line or after its closing line; and the error of the adapter Syntax for a test module that is no test boundary, which carries the error of that test boundary. |
| Test Module | The part of a source from its opening line to its closing line: an inline module tests that the test attribute stands on, which holds the tests of that source and whose spans the test boundaries refuse, so they are never a gap. |
