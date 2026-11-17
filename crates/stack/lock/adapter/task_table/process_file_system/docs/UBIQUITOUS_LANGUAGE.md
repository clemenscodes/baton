# The ubiquitous language

| Word | What it means |
|---|---|
| Absence Code | The error number 3 of the kernel, ESRCH, for a process that does not exist. |
| Check Holder | To tell whether a process holds one of the device files through an open descriptor. |
| Command Name | The name comm of the file of the command of a process. |
| Configuration | The task configuration of the adapter. |
| Descriptor Name | The name fd of the directory of the descriptors of a process. |
| Descriptor Path | The directory fd of a process, which holds a link for every open descriptor. |
| Device Identity | The identity of a device file, rdev, which holds its major and its minor. |
| Entries | The entries of a directory under /proc. |
| Entry | An entry of a directory under /proc. |
| File Type | The type of the file an open descriptor points to. |
| Group Line | One line of the file cgroup of a process, of which the one of the unified hierarchy begins with 0::. |
| Group Name | The name cgroup of the file of the cgroup of a process. |
| Group Prefix | The prefix 0:: of the line of the unified hierarchy in the file cgroup of a process. |
| Group Text | The line of the file cgroup of a process. |
| Holder Presence | Whether a process holds one of the device files. |
| Kernel Signal | The signal of rustix a system signal sends. |
| Kill | The signal SIGKILL, which ends a process the kernel will not let survive. |
| Maybe Device Kind | The device kind of the file an open descriptor points to, where it is a character device. |
| Metadata | The metadata of the file an open descriptor points to. |
| Name Text | The name of an entry under /proc as a text. |
| Path | The path of a file of a process. |
| Process Identifiers | The identifiers of the processes under the directory of the processes. |
| Process Path | The directory of one process under the directory of the processes. |
| Read Text | To read the first line of a file of a process, or nothing where it has none. |
| Scan Failure | The error of a directory of the processes that could not be read. |
| Signal Call | The seam of the signals the adapter sends, so a test sends no signal to a real process. |
| Signal Call Type | The type of the signal call a task table holds. |
| Signal Failure | The error of a signal the kernel refused. |
| Signal Kind | Which signal a signal call sends, the termination or the kill. |
| Signal Target | The process identifier rustix sends a signal to. |
| Stack Lock Adapter Task Table Process File System | The crate stack_lock_adapter_task_table_process_file_system, the adapter of the port TaskTable of stack_lock_application over the descriptors, the commands and the cgroups under /proc and the signals of rustix. |
| System Signal | The signal call over the system call kill of rustix. |
| Task Configuration | The directory of the processes of the host. |
| Task Directory | The directory of the processes of the host, /proc unless an option names another. |
| Task Table | The adapter of the port TaskTable, which reads the descriptors, the commands and the cgroups of the processes under /proc and sends the signals through a signal call. |
| Task Table Port | The port TaskTable as the adapter imports it. |
| Termination | The signal of termination, SIGTERM, which asks a process to end. |
| Try Send Signal | To send a signal to a process. |
| Try Signal Process | To send a signal to a process and take a process that already ended as no error. |
