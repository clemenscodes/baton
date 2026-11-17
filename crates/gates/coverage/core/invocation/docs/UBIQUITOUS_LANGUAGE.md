# The ubiquitous language

| Word | What it means |
|---|---|
| Absence | The lack of what a measurement needs, named by what is lacking. |
| Argument | An argument of an invocation, after its program, and an argument the program is given on its command line. |
| Capture | The stream of an invocation whose output the measurement reads, such as the messages of the build. |
| Completion | How a program ended: a success with the output it captured, or a failure with its status. |
| Domain | The layer of the report, its regions, spans, positions and gaps, the sources, their test boundaries and the sections these decide, and the ports the gate reaches the world through, the decoder, the reader and the parser. |
| Failure | A completion of a program that ended with a status other than success. |
| Gates Coverage Core Invocation | The crate gates_coverage_core_invocation, the crate of the domain of the coverage that holds the invocation, a program with its arguments, its variables and its stream, its completion and the port of the spawner. |
| Inheritance | The stream of an invocation that writes its output where the measurement writes its own, such as the run of the tests. |
| Invocation | A program with its arguments, the variables of its environment and the stream of its output, as a measurement starts it. |
| Name | The name of a variable of the environment, of a profile directory or of a tool. |
| Output | The standard output of a program: the stream the gate writes the success text to, and what a program a measurement starts writes there, as the measurement captures it, such as the messages of a compilation, the library directory of the compiler or an export. |
| Program | The program an invocation starts. |
| Program Absence | A command that names no program. |
| Sequence | The values of a list one after another, as an iterator yields them, such as the arguments of the program or the words of a command. |
| Spawn | To start an invocation, wait for its completion and yield its output or the error of its program. |
| Spawn Invocation | To start an invocation, capture or pass on its output as its stream says, and wait for its completion. |
| Spawner | The driven port that starts an invocation and waits for its completion. |
| Status | The status a program ended with, as the operating system states it. |
| Stream | Whether a measurement captures the output of an invocation or the invocation writes it where the measurement writes its own. |
| Success | The outcome of a gate whose count of gaps is zero, a report without a region with no case of its own, which ends the program with the exit code of a success. |
| Text | A string as it was read or as it is written: the text of a source, the success text and the text of a path. |
| Value | The value of a variable of the environment. |
| Variable | A variable of the environment of an invocation, with its name and its value. |
| Word | A word of the command line of a measurement, the path of its report, a word of the command that compiles the tests, the separator or a word of the command that runs them, and a word of the arguments of an invocation. |
