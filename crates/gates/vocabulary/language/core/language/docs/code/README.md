# The code

A name the code uses, a path, a call, a type, a name inside a macro or the suffix of a literal,
passes where the terms of its languages spell it or where a foreign language or Rust lists it as
a name. A name the code declares passes only where the terms of its own context and of the
contexts of its path dependencies spell it, together with the words of Rust. A declaration is
the name of an item, a field, a variant, a generic parameter, a lifetime, a label, a renamed
import, or a binding of a parameter, a pattern or a closure. A name a foreign crate or Rust
lists is theirs, so the code calls `get` of std and never declares `fn get` again, and
`formatter`, `hasher` or `entries` is declared only where a context defines the term. This is
the rule of the production code; the test module follows its own, below.

Some declarations keep the name somebody else declares. The items of an `impl` of a trait
are named by the trait, `fn main` by Rust, and `const fn new` by `docs/STYLE.md`.

A binding is a declaration, and these are bindings whatever their case: the whole pattern
of a `let`, a parameter, a closure or a `for` loop, so `let Vec = 1;` declares `Vec`, an
identifier of a pattern behind `ref` or `mut`, and one in front of `@`. Any other identifier of
a pattern that begins with an upper case letter is no declaration, because inside a pattern it
names a constant or a unit variant, as `None` in `Some(None)`, and it is judged as a use.

The words live in Rust and are not taken out of the book. A book is what a person
reads and it is never an input to a tool. Where the two are made to agree it is by
generating the page from the words, and never the other way around.

Adding a word is adding a term, with its definition, in the module of the context
that means it. There is no list of forbidden words, because nothing is forbidden.
Every word means what it means, and a word whose meaning does not occur in that
context is simply the wrong word for what was named.
