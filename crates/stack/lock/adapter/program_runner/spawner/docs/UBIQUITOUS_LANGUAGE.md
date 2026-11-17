# The ubiquitous language

| Word | What it means |
|---|---|
| Argument Texts | The arguments of the program of a guard run. |
| Check Interval | The time between two checks of the running program, 200 ms unless an option names another. |
| Completion Outcome | The completion of the program, or the error of its start or of its wait. |
| Compose Wait Failure | To compose the wait failure of a program from the error of the operating system. |
| Configuration | The module of the runner configuration. |
| Lift Flag | The flag a lift of the trap sets, after which every trap signal runs its default action. |
| Lift Trap | To hand every later trap signal back to its default action, once the program ended. |
| Maybe Completion | The completion of the running program at a poll, none while it still runs. |
| Poll Signal Presence | To look whether a signal of an end arrived since the last poll and to clear it. |
| Program Absence | The error of a guard run that names no program after --. |
| Program Runner | The adapter of the port ProgramRunner, which runs the program of a guard run through a spawner, forwards a trapped signal of an end to it as a request to end, and answers its exit code. |
| Program Runner Port | The port ProgramRunner as the adapter imports it. |
| Program Text | The program of a guard run as a text in an error. |
| Registration | The registration of the two actions of the trap for one trap signal. |
| Registration Outcome | The outcome of every registration of the trap, the identifiers of its actions or the error of the first that failed. |
| Runner Configuration | The configuration of a program runner, the interval between two checks of its running program. |
| Signal | The error of a program that a signal ended, so it has no exit code. |
| Signal Flag | The flag the action of a trap signal sets and a poll of the trap clears. |
| Signal Presence | Whether a signal of an end arrived since the last poll of the trap, its presence or its absence. |
| Signal Trap | The seam of the signals of an end the guard traps while its program runs, so a test raises no signal in its own process. |
| Signal Trap Type | The type of the signal trap a program runner holds. |
| Spawner | The spawner of stack_process_core_invocation that starts the program. |
| Spawner Type | The type of the spawner a program runner holds. |
| Stack Lock Adapter Program Runner Spawner | The crate stack_lock_adapter_program_runner_spawner, the adapter of the port ProgramRunner of stack_lock_application over the spawner of stack_process_core_invocation. |
| Start Failure | The error of a program that could not be started. |
| Status | The exit status of a program as the operating system reports it. |
| System Trap | The signal trap over the crate signal-hook, which traps SIGTERM, SIGINT and SIGHUP. |
| Trap Failure | The error of a trap whose actions could not be installed. |
| Trap Signal | A signal of an end the trap catches, SIGTERM, SIGINT or SIGHUP. |
| Try Arm Trap | To install the actions of the trap for every trap signal, before the program starts. |
| Try Compose Exit Code | To answer the exit code of a completion, or the error of a program a signal ended. |
| Try Supervise Program | To start the program, to poll it every check interval, to request its end once a trap signal arrived, and to answer its completion. |
| Wait Failure | The error of a running program that could not be polled or asked to end. |
