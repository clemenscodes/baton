# The ubiquitous language

| Word | What it means |
|---|---|
| Child | The process of the operating system a started program runs in, as the standard library hands it back. |
| Child Program | The running program of this adapter, the child process with the stream its invocation named. |
| Compose Command | To turn an invocation into the command of the standard library, its program, its arguments, its variables and its three streams. |
| Compose Completion | To turn the exit status of a program and its output into a completion. |
| Compose Status Completion | To make the completion of a program from its exit status and its output. |
| Exit Status | The status a program of the host ended with. |
| Maybe Completion | The completion of a program, where it has ended. |
| Maybe Status | The exit status of a program, where it has ended. |
| Read Completion | To read the captured output of a child program to its end, to wait for its exit and to turn its status into a completion. |
| Send Termination | To send the signal of termination to a child, on a host that has signals. |
| Spawner | The adapter of the port Spawner of stack_process_core_invocation over std::process. |
| Stack Process Adapter Spawner Standard Library | The crate stack_process_adapter_spawner_standard_library, the one adapter of the spawner, which starts programs through the processes of the standard library. |
| Standard Library | The standard library of Rust, whose process module starts the programs. |
| Status Text | The exit status of a program as the text a failure carries. |
