# The session

The crate of a repository that uses the library fills the port `Vocabulary` of the application
with its contexts and hands it to the command line as a `Session`, together with the report and
the error stream. Its `main.rs` parses, delegates, reports and returns the status, and decides
nothing. Where the contexts and the foreign languages come from is that repository's own design.

```rust
fn main() -> io::Result<ExitCode> {
    let command_line = CommandLine::parse();
    let contexts = Contexts;
    let report = stdout();
    let errors = stderr();
    let session = Session::from((contexts, report, errors));
    command_line.try_respond_exit_code(session)
}
```

`try_respond_exit_code` reads the arguments into a query or a command of the application, and the
session composes the adapters, the file system `FileSystem`, the decoder `TomsObviousMinimalLanguage`
and the parser `Syntax`, into the gate or the press, runs the use case and reports its outcome:
`PathJudgment` for paths, `TermJudgment` with `--terms`, `TokenSelection` with `--tokens` and
`BookEdition` with `--book`. Where the vocabulary cannot form its languages, the service is never
made, and its error goes to the error stream like every other. Every error of a source or of a book
is written to the error stream and ends the process with a failure. An error of the streams
themselves is returned, so the process ends with it rather than losing it.

The crates of the library publish these names.

| Name | What it is |
|---|---|
| `gates_vocabulary_language_core_language::Language` | a vocabulary, by `TryFrom` from `&[TermText]` for the terms of a context and from `&[&str]` for names that are let through without being taken apart, joined with another by `\|` |
| `gates_vocabulary_language_core_language::TermText` | the spelling and the definition of one term, as a context writes it, built by the constant function `new` |
| `gates_vocabulary_language_core_foreign_name::ForeignName` | the name of a foreign language, from the text of the name of its crate |
| `gates_vocabulary_language_core_languages::contexts::Context` | one directory and its language, by `TryFrom` from `(directory_name, term_texts, foreign_names)` under `crates` with the foreign languages it names, from `(directory_name, language)` under `crates`, from `(root, directory_name, language)` under any root, and from `(context, language)` joined with a foreign language |
| `gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage` | the names a crate uses from one crate outside it, by `TryFrom` from `(foreign_name, names)` |
| `gates_vocabulary_language_application::vocabulary::Vocabulary` | the port that lists the contexts as a `ContextList` and the foreign languages as a `ForeignLanguageList`, each collected from its items |
| `gates_vocabulary_language_adapter_command_line::Session` | the vocabulary, the report and the error stream, from `(vocabulary, report, errors)` |
| `gates_vocabulary_language_adapter_command_line::CommandLine` | the arguments, parsed by clap: the paths, `--book`, `--terms` or `--tokens`, and `try_respond_exit_code` |
| `gates_vocabulary_language_application::Error` | why the vocabulary, a source or a book failed, the error a port of the vocabulary answers with |
