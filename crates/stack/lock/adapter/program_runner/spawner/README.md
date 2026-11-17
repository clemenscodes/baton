# stack_lock_adapter_program_runner_spawner

The adapter of the port ProgramRunner of stack_lock_application over the spawner of stack_process_core_invocation. It starts the program of a guard run, polls it every check interval of its `RunnerConfiguration`, and keeps the guard alive until the program ends. While the program runs, the seam `SignalTrap` traps SIGTERM, SIGINT and SIGHUP, and the runner forwards every trapped signal to the program as a request to end, so a supervisor or a terminal that ends the guard ends its program too and the guard still releases the lock. Once the program ended, the trap is lifted and every later signal runs its default action. `SystemTrap` fills the seam over the safe API of the crate signal-hook, and a test fills it with a fixture. Its tests run with
`cargo nextest run --profile debug -p stack_lock_adapter_program_runner_spawner`.

## The book

- [The crate](docs/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
