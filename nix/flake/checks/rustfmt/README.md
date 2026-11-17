# rustfmt

The check `rustfmt`. It runs `cargo fmt --all --check` over the workspace with the rustfmt of
`nix/flake/packages/rustfmt` and the options of `rustfmt.toml`, and refuses a source of Rust that
rustfmt would change. `docs/GATES.md` states what it prints.
