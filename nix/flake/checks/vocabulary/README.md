# vocabulary

The check `vocabulary-<crate>` of every crate that has a context under
`crates/gates/vocabulary/gate/src/domain/contexts`. It hands every `.rs` file of the crate to the program
of `crates/gates/vocabulary/gate`, which parses each source with syn and refuses, at every occurrence and
with its line and its column, a name the code uses that its languages do not admit and a name the
code declares that the terms of its contexts do not spell, and further a source that is no Rust,
a manifest that is no manifest of a crate, an identifier with a piece that is no word, a comment,
and a test attribute anywhere but on the test module at the bottom of a source. A crate of no
source is refused as well, because then the filter of the paths is wrong.
