{{#include ../README.md}}

## The concept

`Invocation` is built from the words of a command, the first of which is its program, and an invocation without a program is an error. Its `Stream` either captures the output, such as the messages of a build, or passes it through. Its `Variable` is a name with a value. The `Completion` of an invocation is a success with its output or a failure with its status.
