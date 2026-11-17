# verb

The check `verb`. For every crate that has a context it reads the terms its context defines as a
thing and refuses a function whose name begins, after `try_`, with such a thing while its body
is neither one line nor a `match self` whose arms are values, that is a function named for a
thing that is no getter. A test, a function that returns `bool` and a function whose name begins
with a verb of several words the context defines are let through.
