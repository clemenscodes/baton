{{#include ../README.md}}

## The concept

`JavaScriptObjectNotation` decodes the export of llvm-cov into a report and the messages of Cargo into a test build. Only the regions of the kind of code are read, every other kind is an annotation it passes over, and a region counts only in the files the binary of its export keeps. A region that names a file index its function lacks is an error with that file index and the count of the files of its function.
