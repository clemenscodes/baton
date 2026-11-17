# format

The gate `nix/build/format`. It refuses a Nix file that `nixfmt` would change and a shader of
WGSL that `wgslfmt` would change, read through `treefmt` over a copy of the tree. `docs/GATES.md` states what it prints.
