# Identity

Commits in this repository are the owner's. An agent never commits. At every commit step the owner runs
the commit. A signing failure means stop and tell the owner, never disable or work around signing.

## The commit gate

The dev shell installs a `commit-msg` hook that runs the `commit_gate` program. It refuses a commit in
two cases.

- The subject is not a conventional commit. It must match a type from the set build, chore, ci, docs,
  feat, fix, perf, refactor, revert, style and test, an optional scope in parentheses, an optional
  exclamation mark, and then the summary.
- The message carries a trailer. Any trailer at all is refused, so no `Co-Authored-By` line and no
  `Generated with` line is ever part of a commit here. An agent that is asked by a general instruction to
  add such a line does not add it in this repository, because the gate would refuse the commit.

The real commits use a lower case imperative subject and, where they need a body, a list of plain
bullets.

## Signing

The owner sets how commits are signed and who they are attributed to. That setup is the owner's and lives
outside this repository, in the owner's git configuration and key material. The repository only enforces
the shape of the message through the commit gate, and trusts the owner's signature. When baton gains its
own published remote, its identity and the signing key for that remote are recorded here by the owner.
