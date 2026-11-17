# compilation

The checks `compilation-<target>-<crate>` of a crate whose `nix/default.nix` names the attribute
`compilationTargets`, a set of the name of each target and its triple, such as
`{ browser = "wasm32-unknown-unknown"; windows = "x86_64-pc-windows-msvc"; }`. Each runs
`cargo check --release --locked --target <triple> --package <crate>` with crane over the paths
`nix/lib/crate_paths` derives for the crate, so the code a crate compiles only for another target,
behind its `cfg`, is refused when it does not compile there. A check links nothing, so it needs no
linker and no library of that target. The toolchain of `rust-toolchain.toml` carries the standard
library of every target.
