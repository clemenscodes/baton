# dependency_table

A program of the checks, no check itself. Run in the root of the tree, it prints one line for
every dependency of a crate on another crate of the workspace, with the directory of the crate,
the scope `production` or `development` and the directory of the dependency, for
`[dependencies]`, `[dev-dependencies]` and the same tables of every platform under `[target]`.
The checks `dependencies`, `layer` and `adapter` read those lines.
