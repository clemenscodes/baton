# baton, working agreement

baton is a mutex around GPUs. A GPU is a lock that always knows the applications that may use it at
all, and only one of them owns it at a time. The kernel enforces the lock with one device program at
the root of the unified hierarchy, so a process outside the owner never opens the GPU. An acquisition
announces a demand, and the caller chooses whether it waits or, with `--force`, ends the users of the
GPU. A guest of a virtual machine is an application like every other, and a system with a single GPU
works the same way with the desktop as its default owner.

Read this file, every document directly in `docs`, and the README and book of every crate you touch,
before you touch a single file. Read `docs/designs/BATON.md` first, it states how the lock works and
why.

## The first rule, above every other

**Every identifier says WHAT it is, never HOW it came to be.** This is the most important rule of this
repository, and it is not negotiable.

- Every identifier contains a noun, the thing it names. A participle, an adjective or an adverb stands
  only in front of the noun it qualifies, never alone. A word that only reports an act or a state names
  nothing.
- Where no noun fits, the name in the code is wrong. Rename the code, never admit the word.
- The full naming law is in `docs/STYLE.md`. Read it before you add or change any identifier.

## Design for the finished tool, never for today

Every port, every structure and every API is designed as if baton were complete, with every feature it
will ever have. A milestone is never a reason for a shortcut. Before you write code, ask whether its
design still holds when baton is finished. Where it does not, it is not written. Where a plan
describes a shortcut for a milestone, design the finished version and propose it to the owner before any
code.

## How we work

**Nothing is rushed.** There is no deadline. Precision and correctness beat speed. If something is
unclear, stop, name the confusion, and clear it before touching code.

**Plan before building.** Every non trivial change is designed and written down, as a plan under
`docs/plans` or a design under `docs/designs`, and agreed with the owner before implementation starts.
A plan is removed once its change is in the tree, and what stays true of it moves into a design.
Every name of a crate, a module and a directory is proposed and approved before it is created.

**This repository is the only memory.** Every fact, decision, preference and finding worth keeping lives
in this repository, in a document, a crate README, a book page, or this file, never in agent memory and
never left only in a conversation.

**Verify before claiming done.** A derivation is not done until it builds. A crate is not done until its
tests pass. A tool is not done until it runs and its output is checked. Builds are pure, never
`--impure`, never `import <nixpkgs>`.

## The GPUs of the host are real

baton runs as root on a real host. A trial of the device program runs on `/dev/zero` or on
a node no driver serves, in a cgroup of its own below a tree of its own, and is removed after it. No
agent attaches a program to the root of the unified hierarchy that locks a device the owner uses, no
agent ends a process that holds a GPU, and no agent hands a GPU to vfio-pci, unless the owner asked
for that run. A detach always passes the flags 0, since the kernel refuses a detach with the flags of
multi.

## The gate

`moon ci` is the one gate, and nothing else counts. Run it without flags, exactly once, over a finished
tree. An agent checks its own work with `cargo nextest run --profile debug -p <crate>` and `cargo fmt`
for the crates it changed, then hands over. Never run moon with `--force`, a cached result is a result.
Never run `cargo test`, it has no time bound, run tests with `cargo nextest run --profile debug`. Two
PreToolUse hooks under `.claude/hooks` repeat these two rules.

## Commits

Commits are the owner's. An agent never commits. The commit gate refuses any message that carries a
trailer, so no `Co-Authored-By` or `Generated with` line is ever added to a commit here. See
`docs/IDENTITY.md`. A signing failure means stop and tell the owner, never work around signing.

## Style of prose

Complete, grammatically correct sentences. The only sentence punctuation is the period and the comma.
No colons or semicolons in prose, no dashes of any kind, and never the character U+2014, in code,
comments, commit messages, documents or replies. Real umlauts in German, never the ASCII replacements.
The naming rules bind code, not prose, a document is free.
