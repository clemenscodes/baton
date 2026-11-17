# The press

The press `press::Press` is an application service over the writer of the books, a driven port of
the core. It runs one command, `press::BookEdition`, through its `SynchronousCommandHandler`, and
answers with the `press::edition::Edition` of every book, or with the `Error` that refused the
edition as a whole.

With no directory the book edition issues every book, with directories it issues the books of the
contexts whose directories they are, each into the `docs` of the directory of its context.

```
tool_vocabulary --book
tool_vocabulary --book crates/harbor
tool_vocabulary --book crates/harbor crates/ship
```

A book is the vocabulary as a page and nothing else, so nobody edits those pages by hand. A
directory that is not exactly the directory of a context, the current directory in front of it
aside, refuses the edition, and no book comes out of it. A context of no term yields a book whose
table has no row.

The edition holds a `press::edition::BookEntry` for every book, in the order of the contexts:

- `BookEntry::Publication`, a book that went out, with the count of its terms;
- `BookEntry::Omission`, a book of an edition of no directory whose `docs` directory is absent,
  which is passed over;
- `BookEntry::Error`, the error of a book that cannot be written, `Error::Book`, or of a directory
  whose presence cannot be detected, `Error::Directory`.

A book edition that names directories writes each book without a look at its directory. Each entry
reads as the line the command line reports.

The book edition changes the books on disk, so it is a command. It changes no aggregate: the press
computes every book from the vocabulary and writes it whole, and reads no book back. So it
implements no `Command` of an aggregate and travels in no `Dispatch`, whose identity would name
an aggregate that does not exist.
