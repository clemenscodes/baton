# crate-name

The check `crate-name`. It refuses a crate whose package name in its `Cargo.toml` is not its path
under `crates` with every `/` replaced by `_`, and a crate whose `id` in its `moon.yml` is not that
name either, so `crates/stack/engine/adapter/gpu/vulkan` is `stack_engine_adapter_gpu_vulkan` in both. It names
every crate and every project of moon that differs, and it refuses a tree without a manifest under
`crates`, because then the filter of its source is wrong.

A name that leaves out no segment of its path says where the crate lives, and two crates can never
share a name, because two directories never share a path. `docs/STYLE.md` states the rule under
"Crates".
