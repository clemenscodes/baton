# pipeline

The check `pipeline`. It refuses a check of the flake no task of moon builds, a task that names a
check the flake does not have, a task of the root `moon.yml` that is not in its `ci`, and a gate
whose task does not list a path the gate reads in `nix/flake/checks/default.nix`, because moon would
then answer from a cache that never saw the change. A gate that takes its source from a filter
instead of a list of paths is not among the sources it compares.

The tasks of `.moon/tasks/crate.yml` count once for every project tagged `crate`, the project that
inherits them, so a project without a crate, such as `server_application`, builds no check of a
crate.
