# The errors

The application has one `Error`, and the gate gathers the errors of a run into `gate::Errors`, every
source examined.

The gate refuses with `Error::Language`, which names the source, every occurrence of an identifier
its languages do not admit and every defect of the source, one line each, with `Error::Terms`,
which names the directory of a context and every unused term of it, one line each, with
`Error::Syntax`, which names the source and the error of the parser, with `Error::Manifest`, which
names a manifest the decoder refuses, with `Error::File`, which names a file that cannot be opened
or read or a directory that cannot be listed, and with `Error::SourceAbsence`, which names a
directory without a Rust file under it. The press refuses with `Error::Book`, which names a book
that cannot be written, and with `Error::Directory`, which names a directory whose presence cannot
be detected. Languages that cannot be formed from the vocabulary are `Error::Vocabulary`.

The adapters open and read a file, decode the text of a manifest, parse the text of a source and
write a book, and that fails with an `io::Error`, which the application keeps inside its error
beside the path, so the domain never knows a type of an adapter and no error is turned into a text.
The parser over syn keeps its own error inside that `io::Error`, a text that is no Rust with its
line, its column and the error of syn. Every other flaw of a source it hands over as a `Defect` of
the source, each with its line and its column: an identifier with a piece that is no word with the
error of that word, a test attribute that opens no test module at the bottom, and a comment. The
decoder over toml refuses a manifest whose dependencies or whose workspace are no table and a path
of a dependency that is no text.
