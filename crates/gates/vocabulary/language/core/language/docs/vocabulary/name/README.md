# The name

An identifier is one name, and a name is the words it is made of in their order,
taken apart the same way a source is. Two writings of the same words are the same
name, so `as_ref` and `AsRef` are one name, while `is_empty` and `empty_is` are two.
A name has no plural, so the name `file` does not let `files` through.

A source is judged identifier by identifier. An identifier its language admits as a
name is let through, and its words are not judged at all. Every other name has to be made, without a gap, of whole
terms of the language and of the words the language admits on their own, one after
another. A term admits its words only together and in its order, so the term
`Reached Speed` admits `reached_speed`, and neither `reached` alone nor
`reached_length`. A language of names comes from `&[&str]`, which a foreign
language uses as well as Rust does, and Rust alone admits the single words `try` and `type`
as spellings of one word.
