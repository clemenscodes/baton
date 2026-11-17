# The defects and what is not judged

A literal is not a word, so a source that contains `20`, `0xFF`, `"text"`, `r#"text"#` or `'a'`
needs no term for it, while the suffix of a literal, such as `zonk` in `1zonk`, is an
identifier. A comment is not read at all, and a doc comment is the attribute `doc`, which refuses
the source with its line and its column, because code carries no comment.

An identifier whose pieces are not all words is no identifier of a language, so `letter_2`, `_1`
and `слово` refuse the source. An identifier of underscores alone, such as `_`, names nothing and
is passed over.
