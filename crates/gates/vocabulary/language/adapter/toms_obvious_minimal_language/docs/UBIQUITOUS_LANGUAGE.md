# The ubiquitous language

| Word | What it means |
|---|---|
| Decode Dependency Value | To decode the entries of one dependency value of a manifest: none where the table is missing, the entry of every dependency where it is a table, and an error where it is anything else. |
| Decode Entries | To decode the entries of the dependency tables of a manifest, an entry two tables name alike only once, or no entry where a manifest names no dependency, and to refuse dependencies that are no table and a second entry of one foreign name. |
| Decode Package Identifier | To decode the package identifier from the name of a package, or an identifier of no word where the package has no name, and to refuse a name that is no text or no identifier of words. |
| Decode Path | To decode the path the entry of a dependency leads to, or no path where the entry names none, and to refuse a path that is no text. |
| Decode Workspace Entries | To decode the entries of the dependencies of the workspace. |
| Dependency Table | The table of the dependencies of a manifest or of its workspace. |
| Development Dependency Table | The table dev-dependencies of a manifest. |
| Error Kind | The kind of an io::Error, such as the invalid data of a manifest that is no TOML. |
| Gate | The service that reads and parses the sources of a run and refuses a source where its text is no Rust, where its production code or its test code names or declares an unknown identifier or where it has a defect, and that refuses a context where its terms hold an unused term. |
| Gates Vocabulary Language Adapter Toms Obvious Minimal Language | The crate gates_vocabulary_language_adapter_toms_obvious_minimal_language, the adapter of the decoder over TOML. |
| Infrastructure | The layer of the library whose adapters, the file system, the decoder over TOML and the parser over syn, fill its ports. |
| Manifest Table | The table the text of a manifest parses into. |
| Manifest Tables | The table of a manifest and the table of every platform under its target, which together list the dependencies of the manifest. |
| Package Table | The table package of a manifest, whose name is the name of its crate. |
| Parse Manifest Table | To parse the text of a manifest into its table. |
| Platform | A system a crate is compiled for, such as the one the condition cfg(unix) or the architecture wasm32 describes, as the key of a platform table names it. |
| Platform Table | The table under the target of a manifest that names the dependencies and the development dependencies of one platform. |
| Select Dependency Values | Select the value of a table of dependencies from the manifest and from every platform, one after the other. |
| Select Package Identifier | To select the name of the package table of a manifest, or none where the manifest has no package, and to refuse a package that is no table. |
| Select Workspace Dependencies | To select the dependency table of the workspace of a manifest, or none where the manifest has no workspace, and to refuse a workspace that is no table. |
| Table Entries | The entries one dependency table of a manifest decodes into, before the entries that two tables share are kept once. |
| Table Error Text | The text of the error with which the decoder refuses a manifest whose dependencies are no table. |
| Table Name | The name of a table of a manifest. |
| Target | The table target of a manifest, which holds a platform table for every platform whose own dependencies the manifest names. |
| Toms Obvious Minimal Language | The adapter of the decoder over TOML, the language Tom's Obvious, Minimal Language of the manifests of Cargo. |
| Workspace Table | The table of the workspace of a root manifest. |
