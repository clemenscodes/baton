# The ubiquitous language

| Word | What it means |
|---|---|
| Argument | One word an invocation hands its program after the name of the program. |
| Capture | The stream whose standard output the spawner collects and hands back in the completion, while the standard error stays inherited. |
| Completion | How a program ended, a success with its captured output or a failure with its status. |
| Failure | The completion of a program that ended with a status other than zero. |
| Inheritance | The stream that hands the program the terminal of the caller, so a person sees and types into it. |
| Invocation | A program with its arguments, its variables and the stream it writes its output to, everything a spawner needs to start it. |
| Maybe Code | The exit code of a program that ended in a failure, if the system names one. |
| Name | The name of a variable of the environment. |
| Output | The bytes a program wrote to its standard output under a capture. |
| Poll Completion | To look whether a running program has ended, without waiting for it. |
| Program | The name or the path of the executable an invocation starts, such as qemu-system-x86_64 or ssh. |
| Request End | To ask a running program to end, with the signal of termination where the host has one, and never with a kill. |
| Running Program | A program a spawner started and that may still run, which the caller waits for once its own work beside it is done. |
| Spawn Invocation | To start the program of an invocation and to wait until it ends. |
| Spawner | The driven port that starts the program of an invocation, the one seam between baton and the processes of the operating system. |
| Stack Process Core Invocation | The crate stack_process_core_invocation, the invocation of a program and the driven port that starts it. |
| Start Invocation | To start the program of an invocation and to hand back the running program without waiting for it. |
| Status | The text of the exit status of a program that did not end with zero, as the operating system reports it. |
| Stream | Where the output of a program goes, captured for the caller or inherited from the terminal of the caller. |
| Success | The completion of a program that ended with the status zero, with the output it wrote where its stream was a capture. |
| Value | The value of a variable of the environment. |
| Variable | One variable of the environment an invocation sets for its program, with its name and its value. |
| Wait Completion | To wait until a running program ends and to hand back its completion. |
