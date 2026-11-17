{{#include ../README.md}}

## The concept

`Span` keeps the path of a source and two positions, and it admits no end before its start. A `position::Position` is a line and a column, both counted from 1. A span is shown as its path and the position where it starts, which is how the gate names a gap.
