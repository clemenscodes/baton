# The ubiquitous language

| Word | What it means |
|---|---|
| Acquisition | The command that announces the demand of an eligible application for the GPU of a lock and returns once the application owns it, waiting for the GPU or ending its users as its mode says. |
| Acquisition Mode | How an acquisition takes the GPU, its deadline, whether it forces the users out, and whether it hands the GPU to vfio-pci. |
| Acquisition Step | What one round of an acquisition found, the ownership, the processes it must end first, or the users it waits for. |
| Address Files | The device files of one GPU of a lock. |
| Administration | The lock administration and its steps. |
| Admission | The command that makes an application eligible for a lock, as its default owner where asked, and creates its cgroup. |
| Check Interval | The interval between two checks of a wait. |
| Check Process Presence | To tell whether a process of the host still lives. |
| Check Survival | To tell whether a process that was asked to end still lives. |
| Compose Demand Text | To write the owner and the demands of a lock a waiting acquisition names. |
| Compose Group Error | To turn an error of the control group into an error of the application. |
| Compose Helper Error | To turn an error of the host helper into an error of the application. |
| Compose Program Error | To turn an error of the device program into an error of the application. |
| Compose Record Error | To turn an error of the lock record into an error of the application. |
| Compose Runner Error | To turn an error of the program runner into an error of the application. |
| Compose Status Texts | To write the state of a lock as lines. |
| Compose Task Error | To turn an error of the process table into an error of the application. |
| Compose Texts | To write every item of a list as a text. |
| Compose Tree Error | To turn an error of the device tree into an error of the application. |
| Control Group | The driven port of the cgroups of the eligible applications. |
| Deadline | The time an acquisition waits at most. |
| Deadline Expiry | The error of an acquisition whose deadline passed while the GPU stayed in use. |
| Demand Texts | The demands of a lock as texts. |
| Device | The module of the ports of a GPU, its program and its tree. |
| Device Persistence | Whether the persistence mode of one GPU was on before its passthrough. |
| Device Program | The driven port of the one device program at the root of the unified hierarchy and its maps of the device files and of the permissions. |
| Device Tree | The driven port of a GPU in sysfs, its device files, its consoles and its binding to vfio-pci. |
| Dismissal | The command that makes an application ineligible for a lock again and removes its cgroup. |
| Displacement | The step of a forced acquisition whose processes must end before the application owns the lock. |
| Duration | The time a pause waits. |
| Enclosure | The command that puts a GPU under a lock, installs the device program where it is absent and locks the device files of the GPU. |
| End Deadline | The time a process has between the request to end it and its kill. |
| Exit Code | The exit code of the program of a guard run. |
| Force Presence | Whether an acquisition ends the users of the GPU instead of waiting for them. |
| Foreign Users | The processes that hold the GPU outside the application and outside a yielding owner, for which an acquisition without --force waits. |
| Former Owner | The application that owned a lock before its release or its end. |
| Group Identifier | The identifier of a cgroup, the inode of its directory, which the map of the permissions holds. |
| Guard | The lock guard and its ports. |
| Guard Identifier | The identifier of the process of the guard, which waits for the demand. |
| Guard Ports | The ports the lock guard drives, the host helper and the program runner. |
| Guard Run | The command of the program without a privilege that acquires a lock for a program, runs it and releases the lock when it ends. |
| Host Helper | The driven port of the program without a privilege that runs baton_host through the elevation program. |
| Host Identifier | The identifier of the cgroup of the helper, which every lock permits. |
| Host Path Text | The path of the cgroup of the helper, which every lock permits. |
| Item Type | The type of the items a list of texts is made from. |
| Items | The values a list of texts is made from. |
| Lock | The lock of a GPU, the module of its ports, its services and its setting. |
| Lock Absence | The error of a request for a lock that does not exist. |
| Lock Administration | The application service of the helper that handles every command and query of a lock with the ports of the host. |
| Lock Guard | The application service of the program without a privilege that runs a program inside a lock through the helper. |
| Lock Ports | The ports the lock administration drives, the device program, the device tree, the control group, the lock record, the process table and the pause. |
| Lock Record | The driven port that keeps the locks of the host under /run and holds them exclusively while a request changes them. |
| Lock Setting | The interval of a check and the deadline between the request to end a process and its kill. |
| Maybe Application Name | The application whose processes a search of the users leaves out, where there is one. |
| Maybe Deadline | The deadline of an acquisition, where it has one. |
| Maybe Graphics Lock | The lock of a name among the records, where it exists. |
| Maybe Yielding Owner | The default owner of a lock when it yields, or nothing. |
| Mode | The acquisition mode. |
| Name | The error of a name that is no name of a lock or of an application. |
| Owner Name | The name of the owner of a lock. |
| Owner Presence | The error of a removal of a lock that has an owner. |
| Ownership | The step of an acquisition whose application now owns the lock. |
| Passthrough Text | Whether a lock is passed through, as on or off. |
| Path Text | The path of a cgroup below the root of the unified hierarchy. |
| Pause | The driven port that waits the interval of a check. |
| Permission Presence | Whether the dismissed application is the one the device program permits. |
| Permitted Application | The application the device program permits for a lock. |
| Population Presence | Whether a cgroup holds a process. |
| Program | The device program at the root of the unified hierarchy. |
| Program Runner | The driven port that runs the program of a guard run as a child and waits for its end. |
| Program Texts | The program of a guard run and its arguments. |
| Record | The records of the locks of the host. |
| Record Lease | The exclusive lease of the records of the locks, which ends when it is dropped. |
| Record Yield Presence | To record on a lock whether its default owner yields. |
| Refresh | The command that finds the device files of every lock again, after a GPU went to vfio-pci or came back from it. |
| Release | The command that ends the ownership of the owner of a lock and gives the GPU back. |
| Release Outcome | The outcome of the release of a guard run. |
| Removal | The command that takes a lock away with its records, its cgroups and its device files, and the device program with the last lock. |
| Run Outcome | The outcome of the program of a guard run. |
| Setting | The lock setting. |
| Stack Lock Application | The crate stack_lock_application, the application of the mutex around a GPU, its commands, its queries, its two services and the ports of the kernel, the host and the programs it drives. |
| Stale Files | The device files of a lock that a scan no longer finds, deleted from the map after the new ones are inserted. |
| Status Query | The query of the state of every lock of the host. |
| Status Texts | The lines of the state of the locks. |
| Step | The acquisition step. |
| Survivor Users | The processes that survived their kill. |
| Task Table | The driven port of the processes of the host, the ones that hold a device file and their end. |
| Tree | The device tree of a GPU in sysfs. |
| Try Acquire Lock | To ask the helper for the acquisition of a lock. |
| Try Advance Acquisition | To run one round of an acquisition with the records held. |
| Try Bind Console User | To bind a console or a framebuffer of the firmware to the GPU again. |
| Try Bind Passthrough | To hand a GPU to vfio-pci and answer whether its persistence mode was on. |
| Try Check Population | To tell whether a cgroup holds a process. |
| Try Create Group | To create a cgroup with its parents and answer its identifier. |
| Try Delete Device Files | To delete device files from the map of the devices. |
| Try Delete Lock | To delete the record of a lock. |
| Try End Process Users | To end the processes that use a GPU, first with the request to end and after the end deadline with the kill. |
| Try Enter Group | To move the own process of the helper into a cgroup. |
| Try Enter Host | To move the helper into its own cgroup, so the programs it runs reach the GPU of a lock. |
| Try Extend Device Files | To add the device files a scan finds to a lock and to its map without deleting any, around a passthrough. |
| Try Force End | To kill a process. |
| Try Insert Device Files | To insert device files into the map of the devices for a lock. |
| Try Install Program | To install the device program at the root of the unified hierarchy where it is absent. |
| Try Lease Records | To lease the records of the locks exclusively. |
| Try Load Lock | To load the record of a lock. |
| Try Load Locks | To load the record of every lock. |
| Try Move Process | To move a process into a cgroup. |
| Try Permit Application | To permit the cgroup of an application in the map of the permissions. |
| Try Permit Group | To permit a cgroup for a lock in the map of the permissions. |
| Try Permit Host | To create the cgroup of the helper and permit it for a lock. |
| Try Read Console Users | To find the consoles and the framebuffers of the firmware drawn on a GPU. |
| Try Read Device Files | To find the device files of a GPU. |
| Try Read Group Identifier | To read the identifier of a cgroup. |
| Try Read Status | To ask the helper for the state of every lock. |
| Try Release Lock | To ask the helper for the release of a lock. |
| Try Release Passthrough | To give a GPU back from vfio-pci to the driver of the host, with its persistence mode where it was on. |
| Try Remove Group | To remove a cgroup. |
| Try Remove Program | To detach the device program from the root of the unified hierarchy. |
| Try Request End | To ask a process to end. |
| Try Restore Lock | To give the GPU of a lock back after its owner, its passthrough, its consoles and its permission. |
| Try Revoke Application | To take the permission of an application out of the map of the permissions. |
| Try Revoke Group | To take the permission of a cgroup for a lock out of the map of the permissions. |
| Try Revoke Host | To revoke the permission of the cgroup of the helper for a lock. |
| Try Scan Device Files | To read the device files of every device of a lock. |
| Try Scan Process Users | To find every process that holds one of the device files. |
| Try Scan Users | To find every user of the GPU of a lock, leaving out the processes of an application where one is named. |
| Try Settle Lock | To drop the demands whose process ended and give back the GPU of an owner whose cgroup is empty. |
| Try Spawn Program | To run a program as a child and answer its exit code. |
| Try Store Lock | To store the record of a lock. |
| Try Unbind Console User | To unbind a console or a framebuffer of the firmware from the GPU. |
| Try Update Device Files | To find the device files of the GPUs of a lock again, delete the files it no longer has from the map of the devices and insert the ones it has. |
| Turn Presence | Whether the demand of an acquisition is the first of the queue. |
| User Query | The query of every user of the GPU of a lock. |
| User Survival | The error of a forced acquisition whose user survived its kill. |
| User Text | The users or the demands an acquisition waits for, as a text. |
| Wait | The step of an acquisition that waits for the users it names. |
| Wait Duration | The time a wait has lasted. |
| Yield Presence | Whether an admitted default owner yields the lock to every acquisition of another application. |
| Yielding Owner | The default owner of a lock that yields it to an acquisition. |
| Yielding Users | The processes of a yielding owner that hold the GPU, which an acquisition ends without --force. |
