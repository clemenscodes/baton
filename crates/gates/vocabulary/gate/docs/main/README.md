# The forms of the program

```
gates_vocabulary_gate crates/gates/vocabulary/gate
tool_vocabulary --terms crates
tool_vocabulary --tokens crates
tool_vocabulary --book
```

The first form judges the sources passed to it, where a directory stands for every Rust file under
it, and names every occurrence of an identifier
that is not made, without a gap, of whole terms of their languages, with its line and its column.
In the production code of a source a name the code declares has to be spelled by the terms of its
contexts, together with the words `try` and `type` of Rust, while the items of an `impl` of a
trait, `fn main` and `const fn new` keep the names the trait, Rust and `docs/STYLE.md` declare, and
a name it uses may also be a name of a foreign language or of Rust. In the test module every
identifier has to be spelled by the terms, the names of Rust and of the foreign languages taken as
spellings, and the words `tests`, `fixture`, `helper`, `factory` and `expectation`, and the
languages of the development dependencies count there as well; the name of a test function is not
judged. After those, it names every test attribute, every comment and every identifier of a
source that is not made of words. The second refuses every term of a context that neither a package name nor an
identifier of production code of the sources passed to it spells, and it is passed the directory
`crates`, so every source of the tree counts, because a crate keeps the terms of the crates it depends on in use. The third prints the
book of every context into `crates/<DIRECTORY_NAME>/docs/UBIQUITOUS_LANGUAGE.md`, for every context
whose crate has a `docs` directory, or, when directories of contexts follow it, the books of
those contexts. A book is never edited by hand.

The form with `--tokens` judges nothing. It prints the tokens of the production code of the sources
passed to it, one per line, with the path, the line, the column, the kind `declaration`,
`identifier`, `literal` or `punctuation` and the text, separated by tabs. It prints no string and
nothing of a test module, so the gates `word`, `signature`, `interface`, `test`, `layer` and
`adapter`, which read those lines, read names and never prose.
