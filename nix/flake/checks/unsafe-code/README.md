# unsafe-code

The gate `nix/build/unsafe-code`. It refuses `unsafe` in a source of Rust outside
`crates/stack/engine/adapter/gpu/vulkan/src/vulkan`, the one module of the one crate that `docs/STYLE.md`
admits it in, the lint `unsafe_code` set in the manifest of any crate but `stack_engine_adapter_gpu_vulkan`,
and a manifest of the workspace that does not forbid `unsafe_code`. A word `unsafe` inside a
string is no use of it. `docs/GATES.md` states what it prints.
