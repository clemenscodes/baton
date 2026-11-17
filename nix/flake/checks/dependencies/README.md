# dependencies

The check `dependencies`. It refuses a member of the workspace that is no crate of the tree and a
crate that is no member, a crate without a `moon.yml`, and a `dependsOn` of a `moon.yml` that does
not name exactly the crates the `Cargo.toml` of the crate names by path: each production
dependency with the scope `production`, and each development dependency that is no production
dependency with the scope `development`.

`dependsOn` is the only place a crate declares what it depends on. moon reads the graph of the
projects from it, so the constraints of `.moon/workspace.yml` judge every edge, and the task
`sources` of `.moon/tasks/crate.yml` carries the hash of every crate a crate reaches, directly or
through others, into the hash of its gates.

It also keeps the authority of the game out of every client. The module `authority` of
`stack_game_application` compiles only with its Cargo feature `authority`, which moon cannot see, so the
check reads the manifests instead. A crate enables the feature when its `Cargo.toml` names
`stack_game_application` among its production dependencies with `features = ["authority"]`, or when one
of its own features names `stack_game_application/authority`. The check follows the production
dependencies of every crate tagged `client` or `client_game_adapter` and refuses each one that reaches
such a crate, directly or through others, naming the path through which the feature arrives. The program
of the client of Dusk Requiem carries `dusk_requiem_authority` and links the world in memory for the
interval `docs/ARCHITECTURE.md` documents, so the check does not judge it.
