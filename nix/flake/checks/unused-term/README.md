# unused-term

The check `unused-term`. It hands every `.rs` file under `crates`, in one call, to the program of
`crates/gates/vocabulary/gate` with the option `--terms`, which refuses every term of a context that neither
the package name of a crate of the tree nor an identifier of the production code of a source of
the tree spells, so a term that only a test spells is refused. A source of a crate counts for the
contexts of the crates it depends on by path as well, which is why the check reads the whole tree
at once and never one crate alone. A tree of no source is refused as well, because then the
filter of the paths is wrong.
