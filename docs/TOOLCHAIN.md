# Toolchain

baton is built with one pinned Rust toolchain and one pinned Nix flake. Nothing is built against a host
tool. The dev shell of the flake holds every tool, enter it with `nix develop`.

## Rust

The toolchain is pinned in `rust-toolchain.toml`.

- The channel is a fixed stable release, not a floating one.
- The components are clippy, the llvm tools, rustfmt, the analyzer and the sources.
- The target is `x86_64-unknown-linux-gnu`, the host.

rustfmt is not the one of the toolchain. The overlay `nix/flake/overlays` builds the rustfmt of
`nix/flake/packages/rustfmt` from the source of the same release with a patch of its own, which adds
the option `assignment_rhs_break`, and `rustfmt.toml` sets it to `SameLine`. The dev shell puts it on
the `PATH` and names it in `RUSTFMT`, and the check `rustfmt` formats with it.

rust-analyzer is not the one of the toolchain either. The server of 1.98.1 is older than the
extension of the editor and refuses settings it sends, so the dev shell puts the release of
`nix/flake/packages/rust_analyzer` on the `PATH` before the toolchain. It analyses with the sysroot of
1.98.1, and `.vscode/rust-analyzer` starts it through direnv.

VS Code reaches the tools of the dev shell through the scripts under `.vscode`, `rust-analyzer`, `nil`,
`moon` and `treefmt`, each of which starts its tool inside the dev shell through direnv, so the editor
formats Rust with the patched rustfmt and Nix with the formatter of the flake. `.vscode/settings.json`
names them, and `.vscode/extensions.json` recommends the extensions for Rust, Nix, moon, YAML and direnv.
An editor that starts rust-analyzer outside the dev shell points the setting
`rust-analyzer.rustfmt.overrideCommand` at the path `nix develop --command which rustfmt` prints.

The workspace is one virtual workspace with one root `Cargo.lock`. A subcrate inherits the version, the
edition and the publish setting from the workspace. Every external dependency is pinned to an exact
version and added with `cargo add`, never by hand. Unsafe code is forbidden across the workspace.

## Nix

The flake is thin, it delegates every output to a file under `nix/flake`. Builds are pure, never
`--impure`, never `import <nixpkgs>`. The inputs are nixpkgs, flake parts for the per system wiring, the
Rust overlay for the pinned toolchain, and crane for the Rust source filter and the crate builds. The
nixpkgs input is pinned to a revision whose `moon` substitutes from the binary cache, because on some
unstable revisions the `moon` derivation is uncached and fails to build from source. Bump it to a newer
revision only after checking that `moon` still substitutes there, with
`nix build --dry-run github:NixOS/nixpkgs/<rev>#moon`.

The one verification gate is `moon ci`. moon hashes its inputs through git, so it needs at least one
commit to exist. On a fresh clone the first commit, which is the owner's, is what lets `moon ci` run. Any
single gate is also buildable on its own with `nix build .#checks.x86_64-linux.<gate>`, which needs no
commit, and the dev shell runs the tests directly with `cargo nextest run --profile debug`.
