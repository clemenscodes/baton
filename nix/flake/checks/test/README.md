# test

The check `test`. It refuses a source that starts a program in a crate that does not name
`endToEnd = true`, because a unit test starts no program, a source that names `arboard`, `ash`,
`ash_window`, `axum`, `cpal`, `gilrs`, `gpu_allocator`, `softbuffer`, `TcpListener`, `TcpStream`, `UdpSocket`, `ureq`, `winit` or `x11rb` in such a crate,
because a unit test touches no infrastructure and these libraries stand in front of it, a crate that
names `endToEnd = true` and that the `default-filter` of `.config/nextest.toml` leaves in, because
nextest runs pure unit tests alone, a file under a directory `tests` of a crate
outside `tests/assets`, because a test lives inline in the source it proves, and a decision inside a
`src/main.rs`, an
`if`, `else`, `match`, `while`, `for`, `loop`, `unwrap_or` or `?;` read from what
`tool_vocabulary --tokens` prints, because no test reaches a main and the gate of coverage does not
judge one. The body of a test is not judged, as decision 94 states, so the rule of the three blocks
and the rule of the functions beside the tests of the sibling repositories are not taken over.
