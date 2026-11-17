# gates_vocabulary_language_core_language

A language: the vocabulary and the terms a source is judged against, and the identifiers of a source
it does not admit. It declares the ports `Parser`, through which a source is parsed, and `Reader`,
through which a source and a manifest are opened and the sources of a path are listed.

It is a crate of the domain of the vocabulary gate, its concept `Language` in its `lib.rs`. It does
no IO and keeps no state, and its tests are pure unit tests.

A `Source` holds its production code and its test code, each a `Code` of the occurrences of the
identifiers it names and of those it declares, and its `Defects`, as the parser hands them over.
Each `Occurrence` is the identifier as the source writes it, the line and the column it starts at,
and the `Identifier`, the words it falls apart into. A `Vocabulary` holds the spellings of the terms
of a language, a word admitted alone being a spelling of one word, and the names it admits. In the
production code a name the code uses and the vocabulary admits is let through without being taken
apart, and every other identifier has to be spelled, without a gap, by spellings one after another,
the last word of each in the singular or in the plural. A name the code declares has to be spelled
by the spellings alone. In the test code every identifier has to be spelled by the spellings and
the names. What is left over is the `UnknownVocabulary` of the source. Two languages are joined
with `BitOr`.

A term without a definition is not a term, a spelling without a word is not a spelling, an empty
word is not a word and a name without a word is not a name, so every conversion that can fail is
`TryFrom`, and its `Error` names what could not be made. Term texts that define one spelling a
second time are refused with the second of them.

```
moon run gates_vocabulary_language_core_language:ci
```

## The book

- [The code](docs/code/README.md)
- [The source and its test module](docs/source/README.md)
- [The defects and what is not judged](docs/defects/README.md)
- [The word](docs/vocabulary/word/README.md)
- [The name](docs/vocabulary/name/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
