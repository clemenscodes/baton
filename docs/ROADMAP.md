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
| 6 | the proof on a host, below | open |

## The proof on a host

The proof runs on a host with a GPU under a lock, with the owner present, since it changes who may
open that GPU.

1. With the GPU locked and one eligible application under `run`, a shell outside the lock cannot open
   the file of the driver of the GPU or its render node, and the application uses the GPU.
2. A `run` of another eligible application waits and gets the GPU once the first ends, with a
   `--deadline` it fails and names the owner, and with `--force` it ends the owner and gets the GPU.
   Two waiting demands are served in their order. A `run` of an application that is not eligible is
   refused, and a guard killed with `SIGKILL` lets the lock pass on.
3. A unit of a service manager with a device policy, a program started from a terminal multiplexer and
   a program a portal starts are all denied the GPU.
4. A `run --passthrough` of a hypervisor binds the GPU to vfio-pci with no user outside its owner,
   starts the guest inside the lock, and `refresh` follows the renumbered card after the release.
5. A single GPU is proved with a cgroup of a trial as the default owner, which plays the desktop. A
   `run` of another eligible application waits for it or, with `--force`, ends it, and the end of the
   `run` gives the GPU back to the default owner.

## Decided by the owner

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
