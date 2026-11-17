# Rust

`rust` is the language of Rust itself, which has no definitions here
because `fn` means nothing in a domain and is only allowed. It is the anti corruption
layer, and it exists so that the platform's vocabulary never has to enter the
domain's.

Its names stand in lists of their own. The keywords are the keywords and the primitive types of Rust, and the
standard library holds identifiers of the standard library and of the compiler, such as
`is_empty`, `as_bytes`, `Vec` and `should_panic`. Each of them is a name, allowed only as the
whole identifier it is and only where the code uses it, so `for`, `in`, `as` and `where` are no
words of `text_for_word` or `where_letter`. The words are `try`, the prefix of a function that
computes, and `type`, which every identifier may hold. So `text.is_empty()` passes on the names
of Rust alone, while `fn is`, `fn is_port` and `fn has_files` pass only in a context whose
language has `is`, `port`, `has` and `files`.

A word that only the standard library uses is not a word every domain may use, and
that is why the two are kept apart. A list of single words let `is` through because
`is_empty` exists, and with it every `fn is` any crate defined.

## Where the names of Rust come from

The list of the standard library was measured from the source of the standard library of Rust
1.98.1. It is every identifier that `core`, `alloc` and `std`
declare as public and stable, every macro the compiler builds in, every public module
and every public re-export, whose words the earlier list of single words of Rust
already admitted. So no identifier passes now that did not pass before.

Some names come from the compiler rather than from that source. `std` and `core` are
the crates every source can name, `main` is the function a program starts with, `repr`,
`should_panic` and `ignore` are attributes, and `feature` is the key of a `cfg`.

`is` was left out, although the standard library declares it for `Any` and for
`Error`, because a lone `is` is the word this split exists to keep out of a domain.

The keywords hold no word that is neither a keyword nor a primitive type. A word a crate
needs beyond them, such as `divisor`, is a term of the context of that crate.
