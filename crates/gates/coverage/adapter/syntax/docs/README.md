{{#include ../README.md}}

## The concept

`Syntax` parses a source with syn, so an attribute or a brace inside a string or a comment opens and closes nothing, and visits every inline module `tests` that the attribute `#[cfg(test)]` stands on, at any depth, for its test boundary. A source that is not Rust is an error with its path, the line and the column where syn rejects it and the reason syn gives.
