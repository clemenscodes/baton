# test_processes

The runner of the native end-to-end tests. Cargo hands it a binary of tests, and it runs every test
of that binary in a process of its own, one after the other, because a test of a window takes the
event loop of its process, and it bounds every test at five minutes. The checks of the native crates
that name `endToEnd = true` name it as the runner of their target.

`nextest` runs pure unit tests and starts no program of its own, so these tests stay away from it,
as `nix/flake/checks/README.md` states.
