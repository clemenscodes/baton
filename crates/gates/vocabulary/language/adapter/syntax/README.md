# gates_vocabulary_language_adapter_syntax

The adapter of the port `Parser` over syn. It parses a source of Rust into the occurrences of the
identifiers its production code and its test code name and declare, the tokens of its production
code, and its defects: a test attribute in front of the test module, a comment and an identifier
of no words. A text that is no Rust is an error with its line and its column.

It is an adapter of the infrastructure of the vocabulary gate, its concept `Syntax` in its
`lib.rs`. Its tests parse sources they hold as text.

```
moon run gates_vocabulary_language_adapter_syntax:ci
```

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
