{{#include ../README.md}}

## The concept

`MappingPresence` reads from the section headers `llvm-objdump` lists whether one of the executables carries `__llvm_covmap` or, in COFF, `.lcovmap`. Where none does, the tests hold no region, and the measurement writes the empty export.
