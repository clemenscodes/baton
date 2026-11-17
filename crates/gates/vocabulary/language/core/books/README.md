# gates_vocabulary_language_core_books

The books of the contexts, the ubiquitous language of each context as a page and the path that page
belongs at. It declares the port `Writer`, through which a book goes out and the presence of a
directory is detected.

It is a crate of the domain of the vocabulary gate, its concept `Books` in its `lib.rs`. It does no
IO and keeps no state, and its tests are pure unit tests.

```
moon run gates_vocabulary_language_core_books:ci
```

## The book

- [The book](docs/book/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
