# rust_analyzer

The language server of Rust for the editor, `rust-analyzer`, built from the release `2026-11-09` of
rust-analyzer on top of `rust-analyzer-unwrapped` of nixpkgs. The overlay names it
`rust-language-server`, and the dev shell puts it on the `PATH` before the toolchain, so
`.vscode/rust-analyzer` starts this server and not the one of the toolchain.

The server of the toolchain 1.98.1 is older than the extension of the editor. The extension sends
settings the old server refuses, such as `inlayHints.implicitDrops.enable` as a string, so the
editor reports an invalid configuration. rust-analyzer is versioned apart from the compiler and
analyses older toolchains, so this server reads the sysroot and the proc macro server of the
toolchain 1.98.1 on the `PATH` while the compiler stays at 1.98.1.

The release follows the extension. When the extension sends a setting this server does not know,
raise `version` to the release the extension was built against and renew both hashes.
