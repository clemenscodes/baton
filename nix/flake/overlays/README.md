# overlays

The overlay of nixpkgs that the flake applies beside `rust-overlay`.

It adds `rust-formatter`, the rustfmt of `nix/flake/packages/rustfmt`, which the dev shell and the
check `rustfmt` run in place of the rustfmt of the toolchain, `test-processes`, the runner of
`nix/flake/packages/test_processes` that the end to end tests of a native crate run under, and
`rust-language-server`, the rust-analyzer of `nix/flake/packages/rust_analyzer` that the dev shell
puts before the analyzer of the toolchain.
