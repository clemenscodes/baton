# Roadmap

This page states where baton is going, what it must do, and the order the work is done in. How the
lock works and why lives in `docs/designs/BATON.md`. A decision of the owner is recorded here and
stands until the owner changes it.

## The goal

Every GPU of a host that is under a lock is used by exactly one eligible application at a time, and
by no other process. An application gets the GPU when it needs it, around the problems of the driver
and of the operating system, whether it runs on the host or is a guest of a virtual machine, and on a system
with a single GPU as on one with a second GPU.

## The requirements

- The kernel enforces a lock with one device program at the root of the unified hierarchy, and no
  service manager can lift it.
- A lock knows its eligible applications, and only one of them owns it.
- An acquisition is a demand. It waits for the GPU, gives up after a deadline, or with `--force` ends
  the users of the GPU, as its caller chooses.
- A process that uses the GPU is never ended without `--force`.
- An acquisition with `--passthrough` binds the GPU to vfio-pci, and its release gives the GPU back
  to the driver of the host.
- No daemon keeps the state, and an owner that ends without a release lets the lock pass on.
- baton writes no persistent host configuration and depends on no init system.
- The whole tree is green under `moon ci`, and every crate obeys `docs/STYLE.md` and
  `docs/ARCHITECTURE.md`.

## The order of the work

| Step | What it delivers | Status |
|---|---|---|
| 1 | the gate suite, green over the empty tree | in the tree |
| 2 | the core of `lock`, with its unit tests | in the tree |
| 3 | the application of `lock` and its ports | in the tree |
| 4 | the adapters, the device program over the crate `berkeley_packet_filter` | in the tree |
| 5 | the programs `baton` and `baton_host` and the flake packages | in the tree |
| 6 | the proof on a host, below | proved on Linux 7.0.14 with a second GPU on 2026-11-17 |

## The proof on a host

The proof runs on a host with a GPU under a lock, with the owner present, since it changes who may
open that GPU.

Every point below held on 2026-11-17 on a host with a second NVIDIA GPU and its HDMI audio, the lock
enclosed by the rule of udev of the module, a yielding default owner that drives the lights of the
card, and a hypervisor that passes the card through to a guest and starts the view of a capture card
inside the lock. The proof found five defects, each fixed and tested before the next run.

- The helper ran its own `nvidia-smi` outside the lock and was denied, so it gained the cgroup
  `baton/host`, permitted for every lock.
- A refresh deleted the files of a node before the driver created it again, so a program that
  watches `/dev` opened it first. A refresh now inserts before it deletes, a passthrough only adds
  files, and a file leaves the map only once another device holds its identity.
- The device program denied the creation of a locked node, so devtmpfs could not create the nodes
  of the GPU after a passthrough. It now governs only opens.
- The task table read the first line of the cgroup file of a process, which is a line of cgroup v1
  where one is mounted, so no process was ever a member of an application. It now reads the line of
  the unified hierarchy.
- A guard ended by its supervisor left its program running in the lock. The guard now forwards
  `SIGTERM`, `SIGINT` and `SIGHUP` to its program and releases after it.

The one gap no lock closes is a descriptor a program opened before the enclosure, since the program
never revokes a descriptor. The rule of udev therefore encloses a GPU at the bind of its driver,
before any service starts.

1. With the GPU locked and one eligible application under `run`, a shell outside the lock cannot open
   the file of the driver of the GPU or its render node, and the application uses the GPU.
2. A `run` of another eligible application waits and gets the GPU once the first ends, with a
   `--deadline` it fails and names the owner, and with `--force` it ends the owner and gets the GPU.
   Two waiting demands are served in their order. A `run` of an application that is not eligible is
   refused, and a guard killed with `SIGKILL` lets the lock pass on. A guard ended with `SIGTERM`,
   `SIGINT` or `SIGHUP` ends its program with `SIGTERM`, waits for it and releases the lock.
3. A unit of a service manager with a device policy, a program started from a terminal multiplexer and
   a program a portal starts are all denied the GPU.
4. A `run --passthrough` of a hypervisor binds the GPU to vfio-pci with no user outside its owner,
   starts the guest inside the lock, and `refresh` follows the renumbered card after the release.
5. A single GPU is proved with a cgroup of a trial as the default owner, which plays the desktop. A
   `run` of another eligible application waits for it or, with `--force`, ends it, and the end of the
   `run` gives the GPU back to the default owner.

## Decided by the owner

- A default owner may yield, chosen per lock, so a controller of the lights of a card ends at every
  acquisition and gets the GPU back after the release (owner, 2026-11-17).
- The helper reaches a locked GPU through a cgroup of its own, `baton/host`, which no lock is named
  after (owner, 2026-11-17).
- A host encloses its locks through a rule of udev of the module of NixOS, never through an init,
  and other projects reach a lock through the module (owner, 2026-11-17).

- The lock is an allowlist, never a denylist (owner, 2026-11-17).
- The lock is a mutex around GPUs and works for a system with a single GPU (owner, 2026-11-17).
- An acquisition is a demand, waiting or forcing is the choice of the caller, and only `--force` ends
  a process that uses the GPU (owner, 2026-11-17).
- The binding to vfio-pci is the only knowledge of a passthrough (owner, 2026-11-17).
- The cgroups of the applications live in the tree `/sys/fs/cgroup/baton` (owner, 2026-11-17).
- The group of vfio of a bound GPU is locked too (owner, 2026-11-17).
- The name is `baton` (owner, 2026-11-17).
- The one call of the system call `bpf` lives in a crate of its own, `berkeley_packet_filter`, with a
  safe API, so baton forbids `unsafe` without an exception (owner, 2026-11-17).
