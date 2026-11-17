# comment

The gate `nix/build/comment`. It refuses a line of Nix or of a shell script that is a
comment, the shebang in the first line of a script excepted, because code carries no
comment and what it is for stands in the `README.md` beside it. It refuses a comment of Rust as
well, a `//` or a `/*` anywhere in a `.rs` file, the test modules included, as the gate of the
vocabulary sees only a doc comment. It scans each source letter by letter, so a comment sign inside
a string, a raw string or a character is no comment. `docs/GATES.md` states what it prints.
