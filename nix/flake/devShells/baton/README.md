# The development shell of baton

The one shell that holds every tool baton is built and checked with: the pinned Rust toolchain, the
rust-analyzer of `nix/flake/packages/rust_analyzer` before the one of the toolchain, moon,
cargo-nextest, the Nix formatter, statix and deadnix. Entering it installs the commit gate and runs `moon sync`, which brings the projects and the hooks of moon
in line with the workspace and writes the schemas of its configuration under `.moon/cache/schemas`, where
`.vscode/settings.json` points the YAML extension. moon reads the history of git, so `moon sync` fails
until the first commit exists, and the shell prints its error. Enter it with
`nix develop`. An editor reaches its tools through direnv, as the scripts under `.vscode` do.
