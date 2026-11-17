# adapter

The check `adapter`. An adapter is a crate every type of which names the layer of the
infrastructure, read from what `tool_vocabulary --tokens` prints, so the set is derived from the code
and from no list. The check refuses a production dependency on an adapter from any crate but
another adapter and a program, a crate with a `src/main.rs`, which is where adapters are composed.
A development dependency is free.
