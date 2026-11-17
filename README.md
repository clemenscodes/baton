# baton

baton is a mutex around GPUs. A GPU is a lock. The lock knows the applications that may use the GPU
at all, and only one of them owns it at a time, while every other process of the system cannot open
it. The kernel enforces the lock with one device program at the root of the unified hierarchy, so no
desktop, no service manager and no portal reaches a GPU it does not own.

An application announces its demand and waits for the GPU, or takes it with `--force` from the users
that hold it. A guest of a virtual machine is an application like every other, so the passthrough of a GPU to a
virtual machine is an acquisition that binds the GPU to vfio-pci, and a system with a single GPU
works the same way with the desktop as its default owner.

```
baton run --lock rtx3080 --application render -- blender
baton run --lock rtx3080 --application guest --passthrough -- qemu-system-x86_64 ...
baton status
```

Everything is written in Rust and packaged with Nix.

## The tree

| Directory | What it holds |
|---|---|
| `crates/stack` | the technology that knows no product, `domain_driven_design` the building blocks and `lock` the mutex around a GPU |
| `crates/products` | the products composed from the stack, `baton` with its two programs |
| `crates/gates` | what checks the code and refuses it, the `vocabulary` and the `coverage` |

Every family has the same shape. `core` for its domain, `application` for the one library that holds
its use cases and declares its ports, `adapter` for the technologies behind those ports, and
`presentation` for what a person or a program reaches. Every concept of a core is a crate of its own,
and every crate is named for its full path with the slashes turned into underscores.

## The documents

The documents directly in `docs` bind. A design below `docs/designs` states how a part works. A plan
below `docs/plans` is a working note for a change that is not built yet, and it is removed once the
change is in the tree.

| Document | What it states |
|---|---|
| `docs/ROADMAP.md` | where baton is going, its requirements, the build order, and the decisions of the owner |
| `docs/ARCHITECTURE.md` | where code lives, the layers, and what may reach what |
| `docs/STYLE.md` | how code is written, its naming above everything else |
| `docs/GATES.md` | every gate and what it refuses |
| `docs/OUTPUTS.md` | what the flake builds and checks |
| `docs/TOOLCHAIN.md` | the toolchain |
| `docs/IDENTITY.md` | who signs the commits, and the commit gate |
| `docs/designs/BATON.md` | how the lock works and why, the programs, the tree, the ports and the adapters |

`CLAUDE.md` holds the standing rules for every agent that works here. Every crate carries its own
`README.md` and a book under its `docs`.

## Checking the tree

The dev shell of the flake holds every tool, `nix develop`. Unit tests run with
`cargo nextest run --profile debug -p <crate>`, never with `cargo test`. The one gate that counts is
`moon ci`, run once over a finished tree.
