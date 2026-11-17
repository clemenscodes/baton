# checks

The checks of the flake, one attribute for every gate that a copy of the tree can run.

`nextest` runs pure unit tests, which start no program and touch no infrastructure. A crate whose
tests reach a browser, an X server, a compositor, a GPU driver, a sound server or the input devices
holds end-to-end tests: its attribute `check` names `endToEnd = true`, and its checks run them
through `cargo test` under the runner of its target, the `wasm-bindgen-test-runner` in one headless
Chromium for a crate of the browser, and `nix/flake/packages/test_processes` for a native crate, one
process for every test, each bounded. The `default-filter` of `.config/nextest.toml` leaves out every
such crate in every profile, so `cargo nextest run` on a machine never reaches them, and the gate
`test` refuses a crate that names `endToEnd = true` and stands outside that filter.

Every crate of the flake gets its check `<crate>`, which builds and tests it, and its check
`coverage-<crate>`; every crate that has a context under `crates/gates/vocabulary/gate/src/domain/contexts`
gets its check `vocabulary-<crate>`. Beside them stand the gates of the whole tree: `format`,
`statix`, `deadnix`, `comment`, `unsafe-code`, `ubiquitous-language`, `participle`, `sibling`, `verb`,
`unused-term`, `rustfmt`, `word`, `signature`, `interface`, `test`, `layer`, `adapter`,
`dependencies`, `crate-name`, `pipeline`, `outputs`, `book`, `shell` and `license`. Each check reads
only the paths it needs, cut out of the tree. `docs/GATES.md` states what each refuses and what
it prints.

The paths the checks of a crate read come from Cargo: `nix/lib/crate_paths` follows the path
dependencies of its `Cargo.toml` and takes the manifest and the sources of every crate it reaches.
The `nix/default.nix` of a crate names by hand only what Cargo does not name: `compilationPaths`,
what compiling the crate reads besides its sources, such as a `build.rs`, which every crate that
reaches it receives too, and `testPaths`, what its own tests read, such as `tests/assets`.

A crate whose tests need more than Rust names it in the attribute `check` of its
`nix/default.nix`: a function of the packages it names, called with `callPackage`, that yields
the `inputs` its tests link and load and the `environment` they run in, such as
`{ mesa, vulkan-loader }: { inputs = [ mesa vulkan-loader ]; environment = { VK_DRIVER_FILES = "${mesa}/share/vulkan/icd.d/lvp_icd.x86_64.json"; VK_LOADER_DRIVERS_SELECT = "lvp_icd.x86_64.json"; }; }`,
which draws with the driver lavapipe of Mesa in the sandbox, without a GPU. The checks `<crate>`
and `coverage-<crate>` of that crate receive both. A crate that names nothing gets nothing, and
its checks are the derivations they were before.

A crate whose code for the browser is tested beside its native tests names the attribute
`browserCheck` in the same form, and gets the check `browser-<crate>`, which builds and tests it
a second time with those inputs and that environment, such as the target `wasm32-unknown-unknown`
and the runner `wasm-bindgen-test-runner` in a headless Chromium. Its checks `<crate>` and
`coverage-<crate>` stay native, so the coverage of its native tests is judged as before.

A program that declares Cargo features, such as the feature `scenario` of the program of the client of a
game, names the attribute `features`, a list of the names of its features, and gets the check
`feature-<feature>-<crate>` for each, which builds and tests it a second time with `--features <feature>`,
with the inputs and the environment of its attribute `check`. Its check `<crate>` stays without the
feature, so both builds of the program are gates.

A crate that compiles code for another target behind its `cfg`, such as the program of the client
of a game, names the attribute `compilationTargets`, a set of the name of each target and its
triple, and gets the check `compilation-<target>-<crate>` for each, which checks it for that target
with `cargo check` and links nothing, as `compilation/README.md` states.

A crate that builds for the browser alone has no native tests, so its check `coverage-<crate>`
measures them in the browser. It names the attribute `coverageCheck`, a function of the packages
it names, called with `callPackage`, whose result extends what the attribute `check` names for
that check alone: the `arguments` of `cargo llvm-cov`, the `tools` the build runs, the
`environment`, added to the one of `check`, and `vendoredPackage`, which replaces a vendored
crate. A crate that does not name it gets nothing, and its check is the derivation it was before.
`nix/flake/checks/coverage/README.md` states what `stack_engine_adapter_gpu_webgpu` names.

The gates that judge the production code of Rust read what `tool_vocabulary --tokens` prints, so they
read what Rust tokenizes and what syn declares and never a string, a comment or a test module.
`dependency_table` is no check: it is the program that prints the dependencies between the crates
for `dependencies`, `layer` and `adapter`. The check `license` runs the program of the package
`content_factory` of the flake over the recipes of the tree.
