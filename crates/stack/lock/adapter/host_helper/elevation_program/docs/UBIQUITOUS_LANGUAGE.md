# The ubiquitous language

| Word | What it means |
|---|---|
| Arguments | The arguments of the elevation program. |
| Configuration | The helper configuration of the adapter. |
| Elevation Arguments | The arguments of the elevation program in front of baton_host, -n for sudo so it never asks for a password. |
| Elevation Program | The program that runs baton_host as root, sudo unless an option names another. |
| Helper Configuration | The elevation program, its arguments and the path of baton_host. |
| Host Helper | The adapter of the port HostHelper, which runs baton_host with a request through the elevation program. |
| Host Helper Port | The port HostHelper as the adapter imports it. |
| Host Program | The path of baton_host. |
| Invocation | The invocation of the elevation program. |
| Output | The standard output of baton_host. |
| Output Text | The standard output of baton_host as a text. |
| Request | The error of a request of baton_host that ended without success. |
| Request Arguments | The request and its options baton_host reads. |
| Request Name | The name of the request of baton_host as a text in an error, such as acquire. |
| Spawner | The spawner of stack_process_core_invocation that starts the elevation program. |
| Spawner Type | The type of the spawner a host helper holds. |
| Stack Lock Adapter Host Helper Elevation Program | The crate stack_lock_adapter_host_helper_elevation_program, the adapter of the port HostHelper of stack_lock_application, which runs baton_host through an elevation program such as sudo. |
| Start Failure | The error of an elevation program that could not be started. |
| Status | The exit status of baton_host as the operating system reports it. |
| Try Spawn Request | To run baton_host with a request through the elevation program and answer its output. |
