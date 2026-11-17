# coverage

The check `coverage-<crate>` of every crate of the flake. It runs the tests of the crate
under `cargo llvm-cov nextest` and hands the export to the program of `crates/gates/coverage/command_line`,
which refuses every region outside the test module of a source that no test reaches. nextest runs
with `--no-tests=warn`, because a crate without a test, such as `products_dusk_requiem_core_authority`, would
otherwise end nextest with a failure before any export. Its binaries still run to list their tests
and write their profiles. The program counts the regions no test reaches and passes where that
count is zero, so the export of a crate without code, which holds no region, passes with no case of
its own. The tests run with the inputs and in the environment the attribute `check` of the crate names, as for the
check `<crate>`.

`cargo llvm-cov` alone cannot instrument `wasm32-unknown-unknown`, which has no
`profiler_builtins`, so the attribute `coverageCheck` of `stack_engine_adapter_gpu_webgpu`, which builds for the
browser alone, adds what the coverage of `wasm-bindgen-test` needs:

- `-Zno-profiler-runtime` drops `profiler_builtins`, and `minicov` is the runtime instead, which
  `--cfg=wasm_bindgen_unstable_test_coverage` links into the tests. The runner receives the
  profile from the page and writes it where `LLVM_PROFILE_FILE` names, one for every test.
- `-Clink-args=--no-gc-sections` keeps the sections of the coverage in the module, so `llvm-cov`
  reads the regions from the `.wasm` of the tests itself.
- `RUSTC_BOOTSTRAP` admits these flags on the toolchain of `rust-toolchain.toml`, so the profile
  and `llvm-cov` keep one LLVM and no second toolchain exists.
- The build of `minicov` compiles C for WebAssembly, so the check holds the unwrapped clang of the
  LLVM of rustc, because the wrapper of nixpkgs adds the hardening and the headers of the host. The
  option `-resource-dir` gives it the headers of the compiler, and `llvm-ar` packs the objects.
- `wasm-bindgen-test` demands `minicov` 0.3.8, whose runtime writes the raw profile of LLVM 22, so
  `rust-toolchain.toml` holds rustc 1.98.1, whose LLVM is 22, until upstream follows a newer LLVM.

The tests run in the headless Chromium the attribute `check` names, in one session for the crate,
and `WASM_BINDGEN_TEST_TIMEOUT` of that attribute bounds the session at 15 minutes. Past it the
session fails, and a failing session refuses the crate, so the limit bounds a test that hangs and
admits nothing. A crate that names `endToEnd = true` is measured through `cargo llvm-cov test`, as
nextest runs pure unit tests alone. A page that writes no profile can only leave regions unreached, and
where no page writes one `cargo llvm-cov` fails, so a lost profile can refuse the crate but never
passes it.
