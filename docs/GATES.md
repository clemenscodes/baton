# Gates

A gate checks the code and refuses it. Every gate is a flake check under `nix/flake/checks/<gate>`
with a README beside it, and a moon task. `moon ci` runs them all. A gate reads only the paths it
needs, and prints what it refuses as full English sentences, each line carrying a tag in brackets,
and it ends by saying how to fix it. `nix/flake/checks/README.md`
states how the checks are made.

## The gates of every crate

| Gate | What it refuses | Status |
|---|---|---|
| `<crate>` | a crate that does not build or whose unit tests fail under nextest, or whose end to end tests fail under `test-processes` | green |
| `vocabulary-<crate>` | an identifier of the crate built from a word outside its context, the contexts of its dependencies, a foreign language or Rust | green |
| `coverage-<crate>` | a region of the crate that no test covers | green |

## The gates of the whole tree

| Gate | What it refuses | Status |
|---|---|---|
| `format` | Nix that the formatter would change | green |
| `statix` | a statix lint | green |
| `deadnix` | an unused Nix binding | green |
| `comment` | a comment in Nix, shell or Rust | green |
| `unsafe-code` | a crate that does not forbid `unsafe` | green |
| `rustfmt` | a difference of the patched rustfmt | green |
| `shell` | a shellcheck finding in the hooks | green |
| `word` | a name that holds a word this repository does not admit | green |
| `verb` | a function named for a thing that is not a getter | green |
| `sibling` | two siblings that share a first word without their parent | green |
| `signature` | an `as` outside a conversion, a result without `try_`, a suffix or an underscore in a number, lower case hexadecimal | green |
| `interface` | a glob import, a re export past a child, a re export of another crate | green |
| `participle` | a term whose head is a participle | green |
| `unused-term` | a term that no package name and no identifier of the production code spells | green |
| `ubiquitous-language` | a book of the vocabulary that differs from what the vocabulary prints | green |
| `book` | a book of a crate that mdbook cannot build | green |
| `test` | a test that starts a program outside an end to end crate, a test file outside `tests/assets`, a decision in a `main.rs` | green |
| `layer` | a type that names the wrong layer for its directory, a dependency that points outward, a presentation that reaches a core | green |
| `adapter` | a crate that is not an adapter and not a program depending on an adapter | green |
| `dependencies` | a moon `dependsOn` that differs from the path dependencies in `Cargo.toml` | green |
| `crate-name` | a package name or a moon id that differs from the path of the crate | green |
| `pipeline` | a flake check with no moon task, or a task that names a check that does not exist | green |
| `module` | a module of NixOS whose rules of udev, sudo and activation do not hold a lock | green |
| `outputs` | a difference between the flake outputs and `docs/OUTPUTS.md` | green |

## The commit gate

A `commit-msg` hook, installed by the dev shell. The subject must be a conventional commit, and the
message must carry no trailer. See `docs/IDENTITY.md`.

## How a gate is added

Write the check at `nix/flake/checks/<gate>/default.nix` with a README beside it, wire it into
`nix/flake/checks/default.nix`, add the moon task `nix/build/<gate>` to the root `moon.yml` and into the
`ci` task, and add a row here. The gate `pipeline` then holds the flake and moon in agreement, and
`outputs` holds the flake and `docs/OUTPUTS.md` in agreement.
