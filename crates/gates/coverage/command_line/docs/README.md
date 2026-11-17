{{#include ../README.md}}

## The program

`main` composes the gate with the adapters `JavaScriptObjectNotation`, `FileSystem` and `Syntax`, and the
measurement with the adapters `Process`, `FileSystem` and `JavaScriptObjectNotation`. The `Mode` chooses
between the gate and the measurement from the arguments, and the `Services` respond with the exit code of
the service the mode chooses, so `main` only composes them.
