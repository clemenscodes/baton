{{#include ../README.md}}

## The concept

`Report` keeps one region for every span. A region that llvm-cov exports once for every binary of the tests counts as reached where any of its copies is, so the report keeps the higher count of two regions of one span. It lists the paths of its sources, which the gate reads to find their test modules.

The port `Decoder` decodes a report from the export of llvm-cov and a test build from the messages of Cargo.
