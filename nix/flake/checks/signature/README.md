# signature

The check `signature`. Over what `tool_vocabulary --tokens` prints for the production code it refuses
an `as` cast to a number, a `char`, a `bool`, a pointer or `_` outside the body of an `impl` of
`From` or `TryFrom`, a function declared here that returns a `Result`, or a future whose `Output`
is one, and whose name does not begin with `try_`, a getter of `&self` named in the plural that
returns a `usize` and does not begin with `count`, a number with a type suffix, a number broken up
by underscores and hexadecimal in lower case. The items of an `impl` of a trait keep the names the
trait declares, and `fn main` keeps its name.
