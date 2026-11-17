# sibling

The check `sibling`. It refuses two siblings whose names begin with the same word, the rule of
`docs/STYLE.md` in "Files and modules": that word is the parent both of them miss. It reads names
only: the directories under `crates`, the fixed children of a crate root (`docs`, `src`, `nix`,
`assets` and `tests`) skipped and nothing inside `docs`, `nix`, `assets` and `tests` read; the module
directories under every `src`, the contexts under `crates/gates/vocabulary/gate/src/domain/contexts`
among them; and the modules every source declares inline, each compared with the modules declared
beside it in the same module. It splits a name at `_` and refuses two siblings with the same first
word, so `frame` beside `frame_instant` fails as `frame_rate` beside `frame_instant` does. It compares
only siblings, never a module with one nested inside it, so a collection beside its element nested
in it never collides.
