# The ubiquitous language

| Word | What it means |
|---|---|
| Argument Texts | The arguments of the program of a guard run. |
| Program Absence | The error of a guard run that names no program after --. |
| Program Runner | The adapter of the port ProgramRunner, which runs the program of a guard run through a spawner and answers its exit code. |
| Program Runner Port | The port ProgramRunner as the adapter imports it. |
| Program Text | The program of a guard run as a text in an error. |
| Signal | The error of a program that a signal ended, so it has no exit code. |
| Spawner | The spawner of stack_process_core_invocation that starts the program. |
| Spawner Type | The type of the spawner a program runner holds. |
| Stack Lock Adapter Program Runner Spawner | The crate stack_lock_adapter_program_runner_spawner, the adapter of the port ProgramRunner of stack_lock_application over the spawner of stack_process_core_invocation. |
| Start Failure | The error of a program that could not be started. |
| Status | The exit status of a program as the operating system reports it. |
