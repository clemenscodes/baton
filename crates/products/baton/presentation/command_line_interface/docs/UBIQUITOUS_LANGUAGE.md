# The ubiquitous language

| Word | What it means |
|---|---|
| Application | The name of the eligible application a run acquires the lock for. |
| Arguments | The arguments of the program baton, its options and its request. |
| Check Interval | The time in milliseconds between two checks of the program of a run, in which the guard also forwards a trapped signal of an end to it. |
| Command Line | The command line of the program baton, which composes the lock guard from its arguments and answers the request. |
| Deadline | The time in milliseconds a run waits for the GPU at most. |
| Elevation Argument | An argument of the elevation program in front of baton_host, -n by default so sudo never asks for a password. |
| Elevation Program | The program that runs baton_host as root, sudo unless an option names another. |
| Exit Byte | The exit code of the program of a run as the byte the program baton ends with. |
| Failure Code | The exit code 1 of a run whose program ended with a code that is no byte. |
| Force | Whether a run ends the users of the GPU instead of waiting for them. |
| Guard Adapters | The host helper and the program runner the lock guard drives. |
| Helper Spawner | The spawner the host helper starts the elevation program with. |
| Host Program | The path of baton_host. |
| Lock | The name of the lock a run acquires. |
| Maybe Deadline | The deadline of a run, where it names one. |
| Passthrough | Whether a run hands the GPU to vfio-pci for a guest. |
| Products Baton Presentation Command Line Interface | The crate products_baton_presentation_command_line_interface, the program baton without a privilege, which runs a program inside the lock of a GPU and prints the state of every lock. |
| Program | The program a run starts inside the lock, with its arguments, after --. |
| Request | The request of the program baton, run or status. |
| Run | The request that runs a program inside the lock of a GPU, the guard of the mutex. |
| Runner Spawner | The spawner the program runner starts the program of a run with. |
| Spawners | The two spawners of the host helper and of the program runner. |
| Status | The request that prints the state of every lock. |
| Try Respond Exit Code | To answer the request of the arguments and to answer the exit code of the program baton. |
