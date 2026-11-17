{{#include ../README.md}}

## The concept

`ProfileTools` reads the directory of the library of rustc from its output and composes the invocations of `llvm-objdump`, which lists the section headers of the executables, of `llvm-profdata`, which merges the profiles, and of `llvm-cov`, which exports their coverage for the executables of the tests and the sources of their package. It holds the empty export, `{"data": []}`, for tests that hold no region.
