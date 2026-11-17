# products_baton_presentation_command_line_interface

The program baton without a privilege, which runs a program inside the lock of a GPU and prints the state of every lock. The guard of a run traps SIGTERM, SIGINT and SIGHUP while its program runs, forwards them to the program as a request to end, and releases the lock once the program ended, so a supervisor that stops the guard stops the program with it. It checks the program every `--check-interval`, `BATON_CHECK_INTERVAL`, 200 ms by default. Its tests run with
`cargo nextest run --profile debug -p products_baton_presentation_command_line_interface`.

## The book

- [The crate](docs/README.md)
- [The ubiquitous language](docs/UBIQUITOUS_LANGUAGE.md)
