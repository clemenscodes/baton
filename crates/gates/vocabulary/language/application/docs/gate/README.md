# The gate

The gate `gate::Gate` is an application service over three driven ports of the core: the reader of
files, the decoder of manifests and the parser of sources. It answers three queries, each a `Query`
run by its `SynchronousQueryHandler`: `PathJudgment` judges the sources under its paths and answers
with `Ok` or with the `Errors` of every source it refuses, `TermJudgment` judges the terms of their
contexts, and `token::TokenSelection` answers with the `token::TokenReport` of their production tokens.

With paths and nothing else it is the gate. It receives the sources to judge as arguments
and searches for nothing by itself, so which files are checked is the caller's decision.

Which language a source is checked against follows from its path. A context is a
directory under its root, and the gate joins Rust's own vocabulary with the language of
every context whose directory holds the source, each of them already joined with the
foreign languages that context names, with the language of every context a dependency
of the source leads to by its path, and with the foreign language of every crate its
manifests name. The gate reads the manifest of every directory above the source, decodes
the nearest as the manifest of the crate and the next as the manifest of the workspace,
whose dependencies give the paths a crate names with `workspace = true`. Every path of a
manifest is read against the directory of that manifest. The gate then reads the manifest of
every crate a path dependency leads to, and of the crates those lead to, until no path is new,
and the contexts of all of them join the language. The production code is judged with
`[dependencies]` alone; the test code adds `[dev-dependencies]`, the crates they lead to, and the
words `tests`, `fixture`, `helper`, `factory` and `expectation`. A manifest that is not there is an
absence, a manifest that cannot be read is an error of the source, and a manifest the decoder
refuses is no manifest of a crate.

The gate parses the source with syn into its production code, in front of its test module, and its
test code, the test module without the names of its test functions. In the production code a name
the code uses and one of these languages admits as a name is let through, every other identifier is
taken apart, and its words have to be spelled, without a gap, by terms of these languages. A name
the code declares is never let through as a name: only the terms of its contexts spell it. In the
test code every identifier is taken apart, and the terms, the test words and the names of the
languages, each name as the spelling of its words, spell it. The gate names every occurrence it
refuses on a line of its own, as ``path:line:column: `identifier` is not made of words of this crate's vocabulary.``, with the identifier as the
source writes it, and after them every defect of the source, every test attribute, every comment
and every identifier of no words, on a line of its own.

## The terms

With `--terms` it judges the terms instead of the sources. It refuses every term of a context
that has a source among the paths when neither the package name of a crate of those sources, its
hyphens read as underscores, nor an identifier of the production code of any of them spells it,
so a term that only a test spells is refused. Because the production code of a crate keeps the
terms of the crates it depends on in use as well, the gate is handed the directory `crates` at
once.

```
tool_vocabulary --terms crates/harbor/src/lib.rs crates/ship/src/lib.rs
[LANGUAGE] `crates/harbor` defines the term `Zonk`, but no package name and no identifier in production code uses it.
```

With `--terms` and paths it judges the terms of the contexts. It examines every source as the gate
does and keeps the spellings its production code spells, and the spellings of the package name of
its crate, read from `[package]` with its hyphens as underscores. Every context that holds one of
the sources is refused with every term of it that no kept spelling holds. A source counts for every
context in its languages, so the production code of a crate keeps the terms of the crates it
depends on in use, and the path is the directory `crates`, under which it lists every source of the tree.

```
tool_vocabulary --terms crates
```

