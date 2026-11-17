# The tokens

With `--tokens` it judges nothing and prints the tokens of the production code of the sources for
the gates that read names, one token per line: the path, the line, the column, the kind, which is
`declaration`, `identifier`, `literal` or `punctuation`, and the text, separated by tabs. A name
the code declares is a `declaration` by the rules above. A literal is printed only where it is a
number, so no string reaches a gate, and nothing from the test module does either. A path names a
source or a directory, which stands for every Rust file under it.

```
tool_vocabulary --tokens crates/harbor
crates/harbor/src/lib.rs	1	1	identifier	fn
crates/harbor/src/lib.rs	1	4	declaration	anchor
crates/harbor/src/lib.rs	1	10	punctuation	(
```

