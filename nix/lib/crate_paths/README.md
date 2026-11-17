# crate_paths

The paths of the tree that the checks of one crate read, derived from Cargo. It reads the
`Cargo.toml` of the crate and follows every path dependency it names: its `[dependencies]`,
`[build-dependencies]` and `[dev-dependencies]`, the same tables under every platform of
`[target]`, and every entry with `workspace = true` through the `[workspace.dependencies]` of the
root `Cargo.toml`. From every crate it reaches it follows the dependencies and the build
dependencies again, and never the development dependencies, because Cargo builds those only for
the crate under test. For every crate on the way it yields its `Cargo.toml` and its `src`, and
beside them the `Cargo.toml`, the `Cargo.lock` and the `.config/nextest.toml` of the workspace.

The `nix/default.nix` of a crate names by hand only what Cargo does not name:

- `compilationPaths`, what compiling the crate reads besides its manifest and its sources, such as
  its `build.rs`, the shaders that build script reads, or `.cargo/config.toml` for the browser.
  Every crate that reaches the crate receives these paths as well, because it compiles the crate
  too.
- `testPaths`, what the tests of the crate read, such as its `tests` or the content under
  `crates/products/dusk_requiem/presentation/simulation/assets`. Only the checks of the crate itself receive them.

`nix/flake/default.nix` sets the result as the `paths` of every crate, which `nix/lib/crate_source`
and the check `vocabulary-<crate>` cut out of the tree.
