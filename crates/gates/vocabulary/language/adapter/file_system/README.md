# gates_vocabulary_language_adapter_file_system

The adapter of the ports `Reader` and `Writer` over the files of the machine. It opens a file,
lists the sources under a directory, writes a file through a temporary file it then puts in place,
and detects whether a directory is present.

It is an adapter of the infrastructure of the vocabulary gate, its concept `FileSystem` in its
`lib.rs`. Its tests prove the error a failed replacement leaves behind.

```
moon run gates_vocabulary_language_adapter_file_system:ci
```

## The book

- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
