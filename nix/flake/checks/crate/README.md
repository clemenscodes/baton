# crate

The check `<crate>` of every crate of the flake. It builds the crate with crane under the
profile `release` and runs its tests with nextest under the profile `release` of
`.config/nextest.toml`, so a crate that does not build and a test that fails are refused. nextest
runs pure unit tests. A crate whose tests reach a browser, an X server, a compositor, a GPU driver,
a sound server or the input devices names `endToEnd = true`, and its tests run through `cargo test`
under the runner of its target. It
reads the paths `nix/lib/crate_paths` derives for the crate, and links and runs the tests with the
inputs and in the environment its attribute `check` names, as `nix/flake/checks/README.md`
states.
