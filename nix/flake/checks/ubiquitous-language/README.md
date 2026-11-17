# ubiquitous-language

The check `ubiquitous-language`. It prints every book with `tool_vocabulary --book` over a copy of
`crates` and refuses a book that differs from the book in the tree, a book the vocabulary prints
that the tree does not have, a book of the tree the vocabulary does not print, a crate with a
context and no book, and a context whose path names no crate of the flake. It names every one of
them before it fails.
