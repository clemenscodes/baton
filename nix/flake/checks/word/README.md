# word

The check `word`. It refuses a name that holds a word `docs/STYLE.md` or decisions 96 to 98 of the
plan refuse. It reads names and leaves prose alone: the names of Rust are the declarations
`tool_vocabulary --tokens` prints for the production code, so a string, a comment, a definition, a test
module and the sentence of a test never reach it, and so does nothing the code only uses, such as
`size_of` of the standard library. An item of an `impl` of a trait, `fn main` and `const fn new`
keep the names the trait, Rust and `docs/STYLE.md` declare. Beside them it reads the package name
of every manifest, the bindings and the arguments of Nix with its strings left out, and the
variables and the functions of a shell script and of the shell inside a string of Nix, where
only the name of an assignment, of a `for` and of a function is read.

A name falls apart at its underscores, its hyphens and its capitals into words, so `HeldSlot` is
`held` and `slot`.

- In any name no word may be a form of `carry`, `stand`, `lie`, `sit`, `hold`, `say`, `speak`,
  `tell`, `ask` and `aim`, or `nought`, `corpus`, `place`, `whole`, `reading`, `written`, `given`,
  `at`, `of`, `over`, `wrote`, `number`, `found`, `one`, `other`, `later`, `behind` and `rest`.
- No word may be a form of `refuse` or `refusal`, in the crates under `crates/gates`,
  under `nix/flake/checks`, in `commit_gate` and in the hooks excepted, because a gate
  really refuses (decision 96).
- No name calls a thing a run, such as `first_run` or `run_of`.
- A function begins, after `try_`, with none of `get`, `make`, `build`, `run`, `do`, `handle`,
  `process`, `execute`, `apply`, `perform`, `take`, `hold`, `find`, `collect` and `print`, is no
  `new` and no `with_`, and is none of `first`, `last`, `end` and `inside` alone.
- A module is none of `util`, `utils`, `helper`, `helpers`, `common`, `misc`, `shared`, `strip`,
  `parse`, `unpack`, `load` and `sync`.
- A value or a constant is alone none of the acts and participles of `docs/STYLE.md`, such as
  `taken`, `kept`, `done` and `own`, none of the stopgaps above, none of `first`, `last`, `start`,
  `end` and `inside`, and no number such as `four`, and its last word is no past tense and no
  gerund, the nouns of the gate such as `setting` excepted.

The word `shape` stays allowed, as a word of the domain here (decision 98). The word `answer` is
read neither: `docs/STYLE.md` says a program does not answer, and the numbered decisions of the
plan name the answers of rules as a noun of the domain.
