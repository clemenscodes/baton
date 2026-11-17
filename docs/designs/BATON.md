# The design of baton

baton is a mutex around GPUs. Its purpose is to make a GPU of a system available to an application
whenever it needs it, around the problems of the driver and of the operating system. The desktop and
the operating system never take a GPU under a lock, so it is free whenever an application on the host
or a guest of a virtual machine asks for it. This page states how the lock works, why it works that
way, and the tree, the ports and the adapters that build it.

## Why one program at the root

A lock that denies a GPU to a list of known cgroups is a denylist. Every process in a cgroup nobody
listed sees the GPU, such as a program started from a terminal multiplexer, a service outside the
listed slices or a program a portal starts. A lock that depends on a service manager to place its
cgroups breaks where that service manager is absent or places them differently, and a lock that
replaces one program by another is open for a moment between both. baton is an allowlist instead,
enforced by the kernel, independent of every init.

What the kernel does was measured on Linux 7.0.14 as root, in a tree of cgroups of its own, on
`/dev/zero` and on a node `c 1 250` that no driver serves, so a permitted open answers `ENXIO` and a
denied one `EPERM`.

1. **A denial attached with `BPF_F_ALLOW_MULTI` cannot be lifted below.** The kernel runs every
   program of the cgroup and of every ancestor attached with multi, and an access passes only when
   every one allows it. A program that allows everything on a child leaves the child denied, so an
   allowlist cannot be built from a denial above and a permission below in multi.
2. **A denial attached with `BPF_F_ALLOW_OVERRIDE` is lifted below, by anything.** A child with a
   program of its own runs only its own, so any descendant that gets any device program escapes the
   denial, and a service manager attaches a device program to every unit with a device policy.
3. **One program that reads the cgroup of the opening task holds.** The program checks the access
   against the locked device, then asks `bpf_get_current_ancestor_cgroup_id`, helper 123, for the
   ancestor of the task at the level of the permitted cgroup and compares its identifier, the inode of
   the directory of the cgroup. The permitted cgroup and every descendant open the device, its parent
   and a sibling are denied, and a sibling with a program that allows everything stays denied.
4. **The program works at the root of the unified hierarchy.** Attached to `/sys/fs/cgroup` with
   multi beside the programs a service manager attaches there, it denies and permits as above. A
   detach with the flags 0 removes it, and a detach with the flags of multi is refused with `EINVAL`,
   so a detach always passes 0.
5. **A user cannot load or lift the program.** A load by an unprivileged user ends with `EPERM`.
6. **A user moves a process only between cgroups it owns.** With the permitted cgroup owned by root,
   the move of a user ends with `EACCES`, so the helper that runs as root moves the process.
7. **The program never revokes a descriptor.** A descriptor opened in the permitted cgroup still reads
   after its process moves to a denied cgroup, where a new open is denied. The lock governs the opens
   after its attachment.
8. **The program with both maps holds, and every change is an update of a map.** The program with an
   ancestry unrolled to a depth of 16 passes the verifier and attaches. An admission opens the device
   to its cgroup, a dismissal denies it again, a permission for another lock opens nothing, and the
   removal of the device from its map opens it everywhere, all without a new program.

## The mutex

A GPU is a lock. The lock always knows the applications that may use its GPU at all, and only one of
them owns it at a time. An acquisition is the announcement of a demand, never a seizure. Whether the
demand waits for the GPU or ends its current users is the choice of the caller, and only an
acquisition with `--force` ever ends a process that uses the GPU. A system with a single GPU works the
same way as a system with a second GPU, since the single GPU only has the desktop as its default owner.

One device program is attached once to the root of the unified hierarchy with multi. It denies every
open of a device file of a locked GPU unless the opening task lives in the cgroup of the current owner
of that lock or below it, and it allows every other device untouched. No service manager can lift it,
and it never needs to know the desktop, the session or any init. Every program that uses a locked GPU
runs inside the lock, in the cgroup of its owner, and every program outside never reaches it. Nothing
is locked out by name.

### The eligible applications and the owner

- A lock holds the list of its eligible applications, every application that may use its GPU at all,
  such as a renderer, a model server, a hypervisor or, on a system with a single GPU, the desktop.
  `admit` adds one and `dismiss` removes one. Every eligible application has its own cgroup,
  `/sys/fs/cgroup/baton/<lock>/<application>`, owned by root and outside the tree of any service
  manager, so one application may run many programs.
- Only an eligible application acquires the lock. An acquisition for any other is refused.
- A lock has at most one owner. The map of the permissions therefore holds at most one cgroup per
  lock, and a change of the owner is a removal and an insertion in the same map.
- A lock may name one eligible application as its default owner, which holds it whenever no other
  does. On a system with a single GPU that is the desktop, so the desktop keeps its GPU until another
  application acquires it and gets it back on the release.
- A default owner may yield, admitted with `--yielding`. Every acquisition of another application then
  ends the processes of the default owner without `--force`, whether it holds the lock as its default
  or acquired it itself, while it still waits for every other user. A yielding default owner suits a
  program that a supervisor starts again after it ended, which runs under `baton run`, waits in the
  queue and gets the GPU back after the release, such as a controller of the lights of one card. The
  desktop on a system with a single GPU is a default owner that does not yield.
- A lock is held while the cgroup of its owner is populated, which the kernel reports in
  `cgroup.events`. An owner that ends without a release leaves an empty cgroup, and the lock passes on
  as on a release, to the first waiting demand or to the default owner. No daemon keeps the state.

### The users of a GPU

A user of a GPU is everything that holds it, found the same way for every GPU and every use.

- A process that holds a device file of the GPU, found from the descriptors under `/proc`. The
  processes of the owner are users of this kind, and so are a compositor, a display manager or a
  daemon when they hold the GPU, and never when they run on another one. A display manager is
  therefore only ever touched when the target GPU really runs it.
- A console of the kernel drawn on the GPU, a virtual console bound to its framebuffer or the
  framebuffer of the firmware, found under `/sys/class/vtconsole` and the framebuffers of the device.

A process that uses the GPU is never ended by the lock unless the caller asked for it. A console holds
no work, so an acquisition unbinds it and records it whatever the caller chose, and the release binds
it again.

### The demand, the wait and the force

An acquisition runs in this order.

1. It refuses an application that is not eligible.
2. It records its demand in the queue of the lock under `/run`, with the process that waits for it.
   The demands are served in the order they came, and a demand whose process ended is dropped.
3. It waits until its demand is the first and the GPU has no user left outside the application. It
   checks again after an interval that is an option, never a fixed wait. A `--deadline` bounds the
   wait, after which the acquisition fails with an error that names the owner and every user that
   still holds the GPU, and its demand leaves the queue. Without a deadline it waits as a mutex waits.
4. With `--force` it does not wait for the users. It ends every process that uses the GPU outside the
   application, first with `SIGTERM` and after the deadline of the end with `SIGKILL`. A user that
   survives fails the acquisition.
5. Once the GPU is free it removes the permission of the previous owner, unbinds the consoles, inserts
   the permission of the application, moves the process of the request into its cgroup and leaves the
   queue.

A release removes the permission of its owner, binds the consoles again, and gives the lock to the
first waiting demand or, without one, to the default owner. Starting the desktop again after the
release is the act of the init or the display manager of the host, which baton never replaces.

### An example

A renderer owns the lock of a GPU and renders, and a hypervisor asks for the same GPU to pass it
through to a guest.

- Without `--force` the guard of the hypervisor records its demand and waits. The renderer renders on
  undisturbed, no other application gets the GPU in the meantime, and `status` shows the renderer as
  the owner and the hypervisor as the first demand. Once the renderer ends, the GPU goes to the
  hypervisor, is bound to vfio-pci, and the guest starts. With a `--deadline` the guard gives up after
  it, names the renderer as the owner, and the render is never lost.
- With `--force` the renderer gets `SIGTERM` and after the deadline `SIGKILL`, the render is lost, and
  the guest starts. That is always the explicit choice of the caller, never the default.

### The device program and its maps

The program holds no list of its own. It reads two maps of the kernel, so every change of an owner is
an update of a map, never a new program, and the lock is never open for a moment.

- The map of the devices holds a key of the type, the major and the minor of a device file, and the
  lock it belongs to.
- The map of the permissions holds a key of the identifier of a cgroup and the lock.

For an open the program finds the lock of the device file. A miss allows the open. On a hit it asks
for the ancestor of the task at every level from 1 to a fixed depth, unrolled, and looks up each one
with the lock in the map of the permissions. A hit allows the open, otherwise it is denied. The root
cgroup, level 0, is never an owner, since it would permit every process.

The helper is outside every lock too, so the programs it runs on a GPU, such as `nvidia-smi` for the
persistence and the drain before and after a passthrough, would be denied like every other process.
`enclose` therefore creates the cgroup of the helper, `/sys/fs/cgroup/baton/host`, and permits it for
every lock beside its owner. The helper moves itself there before it runs such a program, and no other
process ever lives there. That cgroup is never an owner and never an eligible application, and no lock
is named `host`. `remove` revokes it for its lock, and the empty cgroup stays for the next `enclose`. The program and its maps are
pinned in bpffs, so they live in the kernel with no process holding them, and a later run finds them
again.

### What a lock locks

A lock names a device by its PCI address and covers every function of its slot, so a GPU at
`0000:03:00.0` brings its audio at `0000:03:00.1`. Its device files are found in two ways.

- Every node with a file `dev` below `/sys/bus/pci/devices/<address>`, such as the card and the
  render node of DRM, a framebuffer, the adapters of I2C and the devices of ALSA of the audio
  function. Block devices are not scanned.
- The file of `nvidia` of a GPU of the NVIDIA driver, which is not in sysfs, by the minor in
  `/proc/driver/nvidia/gpus/<address>/information`.

Around a passthrough the map only grows. The bind adds the file of the group to the files of the
driver, and the release adds the files the driver of the host brings back without deleting the file
of the group, so a node that the driver creates again with its former number is locked the whole
time. A plain `refresh` inserts the files it finds before it deletes the ones that are gone, so a
refresh never opens a file for a moment, and it removes what a passthrough left behind.

The device files of a device change. The card of DRM may be renumbered after a release from
vfio-pci, and a device bound to vfio-pci has no file of its own beside the file of its group under
`/dev/vfio`, found by the link `iommu_group` of the device. `refresh` finds the files of every lock
again and replaces the entries of the map of the devices, and the passthrough runs it after every
handover, so the group of a bound GPU is locked like its files of the driver were.

### The passthrough to a guest

Handing a GPU to a guest has long meant hook scripts written by hand for one host, which stop the
display manager, unbind the consoles, unload the driver, bind vfio-pci and undo every step after the
guest. With the lock most of that is no knowledge of a guest at all. The acquisition already waits
for or, with `--force`, ends every user of the GPU, the display manager only where it runs on the
GPU, and unbinds the consoles. The one knowledge of the passthrough is the binding to vfio-pci.

An acquisition with `--passthrough`, once the lock is the application's and the GPU has no user left,
binds the GPU to vfio-pci.

1. It refuses an IOMMU group that holds a bridge or a function of another device.
2. It sets the persistence of an NVIDIA GPU off and waits until the driver lets go of it.
3. It unbinds every function of the group from its driver, writes `vfio-pci` into `driver_override`
   and binds it to vfio-pci within a deadline.
4. It runs `refresh`, so the file of the group under `/dev/vfio` belongs to the lock.

The release of a guest clears `driver_override`, lets the kernel probe the driver of the host again,
runs `refresh` and releases the lock as every release does. The modules of the driver stay loaded,
since a driver that lets go of one GPU needs no unloading, and a host whose driver cannot let go is
named by the error of the unbinding.

### A single GPU

On a system with a single GPU the desktop is an eligible application and the default owner. An
application or a guest acquires the lock, waits until the desktop ends or ends it with `--force`, and
the release gives the lock back to the desktop. That is the same mechanism as for a second GPU, with
no step of its own.

## The programs

baton has two programs, one without a privilege and one helper that runs as root, only through an
elevation program the host allows for exactly that helper.

`baton_host` answers these requests.

- `enclose --lock <name> --device <address>` puts a GPU under a lock and attaches the program to the
  root when it is absent. It prints the device files it locks.
- `admit --lock <name> --application <name> [--default-owner [--yielding]]` makes an application
  eligible and creates its cgroup, with `--yielding` as a default owner that yields. `dismiss --lock <name> --application <name>` makes it ineligible again.
- `acquire --lock <name> --application <name> --process <pid> [--deadline <ms>] [--force]
  [--passthrough]` records the demand, waits for the GPU or ends its users, moves the process into the
  cgroup of the application and returns once it owns the lock. Root may move any process, so no
  delegation of cgroups is needed, which works under every init.
- `release --lock <name> --application <name>` releases the lock of an owner.
- `refresh` finds the device files of every lock again.
- `remove --lock <name>` takes a GPU out of its lock, and the program leaves the root with the last
  one.
- `users --lock <name>` names every user of the GPU of a lock, every process and every console.
- `status` prints every lock, its GPU, its device files, its eligible applications, its owner, its
  default owner and its queue of demands.

`baton` answers these requests.

- `run --lock <name> --application <name> [--deadline <ms>] [--force] [--passthrough] -- <program>`
  is the guard of the mutex. It acquires the lock through the helper for its own process, starts the
  program as its child, waits for it and releases the lock when it ends, so the program and every
  child it starts use the GPU for exactly as long as the program runs. A guard that dies leaves an
  empty cgroup once its program ends, and the lock passes on as on a release.
- `status` prints what `baton_host status` prints.

The host runs `enclose` for every GPU it locks and `admit` for each of its eligible applications once
at its start, before any program could open the GPU. Where that runs is the matter of the
configuration of the host, never of baton.

Every option is a flag with a variable beside it and a default.

| Program | Option | Variable | Default |
|---|---|---|---|
| `baton_host` | `--hierarchy-directory` | `BATON_HIERARCHY_DIRECTORY` | `/sys/fs/cgroup` |
| `baton_host` | `--system-directory` | `BATON_SYSTEM_DIRECTORY` | `/`, the root below which `sys`, `proc` and `dev` are read |
| `baton_host` | `--record-directory` | `BATON_RECORD_DIRECTORY` | `/run/baton` |
| `baton_host` | `--pin-directory` | `BATON_PIN_DIRECTORY` | `/sys/fs/bpf/baton`, the directory in bpffs of the device program and its maps |
| `baton_host` | `--ancestry-depth` | `BATON_ANCESTRY_DEPTH` | 16 |
| `baton_host` | `--check-interval` | `BATON_CHECK_INTERVAL` | 200 ms |
| `baton_host` | `--end-deadline` | `BATON_END_DEADLINE` | 10000 ms, the time a process has between `SIGTERM` and `SIGKILL` |
| `baton_host` | `--binding-deadline` | `BATON_BINDING_DEADLINE` | 10000 ms |
| `baton_host` | `--console-program` | `BATON_CONSOLE_PROGRAM` | `con2fbmap` |
| `baton_host` | `--persistence-program` | `BATON_PERSISTENCE_PROGRAM` | `nvidia-smi` |
| `baton` | `--elevation-program` | `BATON_ELEVATION_PROGRAM` | `sudo` |
| `baton` | `--elevation-argument` | `BATON_ELEVATION_ARGUMENT` | `-n` |
| `baton` | `--host-program` | `BATON_HOST_PROGRAM` | `baton_host` |

## The module of NixOS

The flake exports `nixosModules.default`, so a NixOS host wires baton declaratively and a module of
another project runs its programs inside a lock. A host names its locks in `baton.locks`, each with
the PCI address of its GPU, its eligible applications, an optional default owner and whether it
yields.

- A lock must stand before any service opens its GPU, or that service holds it and every acquisition
  waits for it. A rule of udev therefore runs the enclosure of a lock when a driver other than
  vfio-pci binds its GPU, both at the coldplug of the start and after every passthrough. The
  enclosure encloses the GPU and admits its applications, or refreshes a lock that already stands.
  udev hands an event to a program that listens only after every rule ran, so the lock stands before
  any desktop or sound server hears of the GPU. The rule depends on udev, never on an init.
- A second rule runs `baton_host refresh` whenever a node with a device file appears below the slot of
  the GPU, so a node of a driver that probes late is locked before any program hears of it.
- The activation admits an application that a rebuild adds to a lock that already stands.
- The members of `baton.group` run `baton_host acquire`, `release` and `status` through sudo without a
  password, and nothing else of the helper.
- `baton.program` is the path of `baton`, wrapped with the paths of sudo and of `baton_host`. A module
  of another project adds its application to `baton.locks.<lock>.applications` and starts its program
  with `${config.baton.program} run --lock <lock> --application <application> -- <program>`.

A daemon that holds a GPU for as long as it runs, such as `nvidia-persistenced`, is denied a locked GPU
like every other process. The persistence of a locked NVIDIA GPU is therefore off unless its owner sets
it.

## The tree

baton has one family of the stack, `lock`, the mutex around a GPU. The product `baton` composes it
into the two programs.

### The core

Every concept is a crate of its own below `crates/stack/lock/core`.

| Crate | What it holds |
|---|---|
| `name` | `LockName`, the name of a lock, and `ApplicationName`, the name of an eligible application, both of letters, digits, `_` and `-` and never starting with `-`, and a lock never named `host` |
| `device/address` | `DeviceAddress`, the PCI address of a GPU, its domain, bus, slot and function, and the slot that holds every function of the card |
| `device/file` | `DeviceFile`, the kind, the major and the minor of a device file of a GPU |
| `graphics/user` | `GraphicsUser`, a process with its identifier and its command, or a console of the kernel with its number |
| `demand` | `GraphicsDemand`, the demand of an eligible application with the process that waits for it |
| `graphics/lock` | `GraphicsLock`, the aggregate of a lock, its index in the maps, its GPUs, its eligible applications, its default owner, its owner, its queue of demands, the consoles it unbound and whether the GPU is passed through, with the decisions of the domain on a demand and a release |

`GraphicsLock` holds the invariants. Its owner and its default owner are eligible, it has one owner at
most, its demands come in the order they were made and one per process, and a demand of an
application that is not eligible is refused.

### The application

`crates/stack/lock/application` holds the commands, the queries, two application services and the
ports.

The commands are `Enclosure`, `Admission`, `Dismissal`, `Acquisition`, `Release`, `Refresh`,
`Removal` and `GuardRun`. The queries are `UserQuery` and `StatusQuery`. `LockAdministration` handles
them for the helper, and `LockGuard` handles the run of a program and the status for the program
without a privilege.

| Port | What it does |
|---|---|
| `DeviceProgram` | installs the device program at the root of the unified hierarchy once, finds it again in a later run, and writes the map of the devices and the map of the permissions |
| `DeviceTree` | reads a GPU in sysfs, its functions, its device files, its consoles and its driver, unbinds and binds its consoles, and hands it to vfio-pci and back |
| `ControlGroup` | creates and removes the cgroup of an application, reads its identifier and whether it is populated, moves a process into it, and moves the helper into its own cgroup |
| `LockRecord` | loads and stores a lock under `/run`, lists the locks, and holds the records exclusively while a request changes them |
| `TaskTable` | finds every process that holds a device file with its cgroup and its command, tells whether a process lives, and ends one with `SIGTERM` or `SIGKILL` |
| `Pause` | waits one interval, so a wait is always an interval of an option, never a fixed wait |
| `HostHelper` | runs `baton_host` with a request through the elevation program, for the program without a privilege |
| `ProgramRunner` | starts the program of a run as a child and waits for its end |

### The adapters

Every adapter is named for its port and its technology below `crates/stack/lock/adapter`.

| Crate | What fills the port |
|---|---|
| `device/program/berkeley_packet_filter` | the system call `bpf` through the safe API of the crate `berkeley_packet_filter`, with no libbpf and no compiler, so baton holds no `unsafe` |
| `device/tree/system_file_system` | sysfs, `/proc/driver/nvidia`, the console mapping of `con2fbmap` and `nvidia-smi` for the persistence and the drain of an NVIDIA GPU |
| `control_group/unified_hierarchy` | the files of cgroup v2 below its mount |
| `lock_record/file_system` | a file of JSON per lock under the directory of the records, with `flock` through rustix |
| `task_table/process_file_system` | the descriptors, the cgroups and the commands under `/proc`, and the signals through rustix |
| `pause/standard_library` | `std::thread::sleep` |
| `host_helper/elevation_program` | the spawner of a process with the elevation program, `sudo -n` unless the option names another |
| `program_runner/spawner` | the spawner of a process |

### The product

`crates/products/baton/presentation` holds the two programs.

- `host` is `baton_host`. It parses a request, composes every adapter of `LockAdministration`,
  dispatches one command or query, and prints its outcome.
- `command_line_interface` is `baton`, with `run` and `status`.

## The proof of every step

Every adapter that touches the kernel is tested end to end with a tree of its own under a temporary
directory where the kernel is not needed, and the device program is tried on `/dev/zero` in a cgroup of
its own, never on a GPU.
