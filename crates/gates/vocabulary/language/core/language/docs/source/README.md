# The source and its test module

A `#[cfg(test)] mod tests` that is the last item of a source is its test code, and it is judged as
strictly as the production code in front of it: tests use the vocabulary too. Every identifier
of it, whether the code names or declares it, passes where the terms of its languages spell it,
together with the names of Rust and of its foreign languages, every name taken as the spelling of
its words, and the words `tests`, `fixture`, `helper`, `factory` and `expectation`, which
`docs/STYLE.md` prescribes for tests, alone or in a compound such as `overlay_factory`. So a test
binds `waker`, `oneshot_sender` or `flush_error_stream` where std, tokio and the terms name their
words, and never a word of its own. The test code uses the languages of the development
dependencies of its crate as well, which the production code does not. The name of a function
with `#[test]`, or in the browser `#[wasm_bindgen_test]`, is not judged, because `docs/STYLE.md` requires a test to be named as a sentence,
and a sentence is made of ordinary English.

A term exists only because production code uses it. With `--terms` the gate refuses every term no
identifier of production code and no package name of a crate spells, so no test keeps a term in
use, and a word a test needs is a word of the production code or no word at all.

A test attribute anywhere else, on another item, inside the code or on a test module that
something follows, refuses the source, because a test lives in its test module at the bottom and
nowhere else. A test attribute is `#[test]`, and a `cfg` or a `cfg_attr` whose condition names
`test` anywhere, so `#[cfg(not(test))]` and `#[cfg(any(test))]` are test attributes as well, and
only `#[cfg(test)]` opens the test module at the bottom. A comment refuses the source wherever it
stands, in the test module as well.
