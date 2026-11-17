# Architecture

Every crate of baton is built with domain driven design and hexagonal architecture, and the design runs
through every layer, the infrastructure and the presentation included. `docs/STYLE.md` states how code
is written. This page states where it lives and what may reach what. A change that violates these rules
is a defect, even if it builds.

## The layers

Every type belongs to exactly one layer, and the crate `stack_domain_driven_design_core` makes that a
fact the compiler checks. A type implements `Layer` once, so a second layer on it is a conflicting
implementation that does not compile. The four layers are the domain, the application, the
infrastructure and the presentation. An inner layer never names an outer one.

Every concept is the building block of domain driven design that it is, and implements its trait from
`stack_domain_driven_design_core`. A layer marker alone is never enough.

| What the concept is | Its building block |
|---|---|
| a value without identity | `ValueObject`, an identity is an `Identifier` |
| a thing with identity over time | `Entity`, the root of a consistent whole is an `AggregateRoot`, kept by a `Repository` |
| a decision of the domain that belongs to no single entity | `DomainService` |
| the creation of a whole with its invariants | `Factory` |
| a rule that accepts or refuses a candidate | `Specification`, combined with the and, the or and the not |
| something that happened in the domain | `DomainEvent`, the reaction to it is a `Policy` |
| a use case that changes state | a `Command` with its handler, run by a `Service` or a `Dispatch` |
| a use case that only reads | a `Query` with its handler |
| a seam to a technology | a `Port`, filled by an `Adapter` |
| what a person or a program reaches | a `Presentation`, with its `Model` and `View` |

A type that is one of these and does not implement its trait is a defect.

## The kinds of crates

`crates` holds three kinds.

- `stack` holds the technology that knows no product. For baton that is `domain_driven_design`, the
  building blocks, and `lock`, the mutex around a GPU.
- `products` holds the products composed from the stack. For baton that is the product `baton` with
  the programs `baton` and `baton_host`.
- `gates` holds the tools that check the code, the `vocabulary` and the `coverage`.

## The shape of a family

Every family has the same shape.

- `core` is a directory, not a crate, and has no head type. It holds one crate per concept of the
  domain, each named for its full path. Two concepts whose names begin with the same word stand below a
  directory named for that word, so `machine/name` and `machine/directory`.
- `application` is exactly one library crate. Its library publishes only its ports and its own types, an
  anti corruption layer over the core. Every use case is a command or a query with a handler, run by one
  application service.
- `adapter/<port>/<technology>` is one crate per technology behind a port. Its type is named for the
  port it fills. No crate outside an adapter names a technology.
- `presentation/<program>` is a program a person or another program reaches. It is a crate with a
  `src/main.rs` and no library. It only parses input, composes adapters, calls one use case, and renders
  the outcome. It reaches the applications and the adapters it composes, never a crate of a core.

Every crate is named for its full path with the slashes turned into underscores, so
`crates/stack/lock/core/graphics_lock` is the crate `stack_lock_core_graphics_lock`. The moon
project id is the same string.

## Every technology is an adapter

Every technology baton drives, the system call `bpf`, sysfs, cgroup v2, `/proc`, the signals, the
records under `/run` and the programs it runs, is reached through a port the application declares and
filled by an adapter named for that technology. Where the translation of an adapter is pure, such as a
lock into the instructions of the device program, it is unit tested with plain values. Where an adapter
touches the kernel, its tests run against a tree of its own below a temporary directory, and the device
program against `/dev/zero` in a cgroup of its own, as the gate `test` demands.

## The privilege

Only `baton_host` runs as root, and only through the elevation program the host allows for exactly
that program. `baton` holds no privilege and reaches the helper through the port `HostHelper`.

## The crate tree of baton

The tree is the one of `docs/designs/BATON.md`, which the owner asked to be built on 2026-11-17. A crate appears
here when it is created.

```
crates/
  stack/
    domain_driven_design/core                  the shared kernel
    process/
      core/invocation                          the invocation of a program and the port that starts it
      adapter/spawner/standard_library         the spawner over the standard library, tested end to end
      adapter/spawner/transcript               the spawner that records, for the tests
    lock/
      core/name                                the name of a lock and of an application
      core/device/address                      the PCI address of a GPU
      core/device/file                         a device file of a GPU
      core/demand                              the demand of an application
      core/graphics/user                       a user of a GPU, a process or a console
      core/graphics/lock                       the aggregate of a lock
      application                              the commands, the queries, the two services and the ports
      adapter/control_group/unified_hierarchy  the cgroups of the applications
      adapter/device/program/berkeley_packet_filter  the device program and its maps
      adapter/device/tree/system_file_system   a GPU in sysfs, its consoles and its passthrough
      adapter/host_helper/elevation_program    baton_host through sudo
      adapter/lock_record/file_system          the records under /run
      adapter/pause/standard_library           the pause of a check
      adapter/program_runner/spawner           the program of a run
      adapter/task_table/process_file_system   the processes under /proc and their signals
  products/
    baton/presentation/command_line_interface  the program baton without a privilege
    baton/presentation/host                    the helper baton_host run as root
  gates/
    vocabulary/gate
    vocabulary/language
    coverage
```

## Ports and adapters

A port is a trait, never a closure, and covers one concept. An adapter is constructed by `From` or
`TryFrom` of its configuration, or `From` of the unit, so a composition is a list of types and holds no
code. The domain holds no state between calls, and all domain types are sendable and shareable across
threads. An adapter that owns state is an actor, a task owns the state and the receiver of a bounded
channel, and a cloneable handle implements the port.

## Enforcement outside the compiler

The compiler checks the layer of a type. The rest is checked by the gates in `docs/GATES.md` and by the
moon tag constraints in `.moon/workspace.yml`, where `enforceLayerRelationships` holds the layer edges
and the family tags add the edges that layers cannot express. The graph of the crates is the wall, the
traits only label the seams.
