# The ubiquitous language

| Word | What it means |
|---|---|
| Component | One part of a path between its separators. |
| Decide Component Removal | To decide whether a parent component removes the last component: where the last component is a normal component, and never where it is a prefix, the root, the current directory, a parent component or missing. |
| Decode Manifest | To decode a manifest from its text, as the decoder does. |
| Decoder | The port that decodes a manifest from its text. |
| Dependency | A crate the manifest of a crate names under its dependencies or its development dependencies, for every platform or in the platform table of one, by its foreign name and by the path it leads to. |
| Dependency List | The foreign names of the crates one table of dependencies of a manifest names, and the paths of those it leads to by path. |
| Development Dependency | A crate the table dev-dependencies of a manifest names, whose languages only the test code of the sources of the crate is judged by. |
| Directory | A directory of the file system, as a path, and the directory of a context, which admits every path beneath it under the last root of the path. |
| Directory Entry | An entry of a directory as the file system lists it: a file, or a directory under it. |
| Entry | A dependency of a manifest: the foreign name of a crate and the path it leads to, where it names one; and the error of a table of dependencies with a second entry of one foreign name. |
| Entry Path | The path an entry leads to, which a manifest resolves against its directory, each parent step against the component in front of it, and which keeps a parent step no component stands in front of. |
| Gates Vocabulary Language Core Dependencies | The crate gates_vocabulary_language_core_dependencies, the dependencies of the crate of a source, with the port of the decoder of its manifests. |
| Last Component | The last component of the components of a path that are resolved so far, which a parent component removes where it is a normal component. |
| Manifest | The Cargo.toml of a crate or of the workspace, decoded into its package identifier and the entries of its dependencies, of its development dependencies and of the dependencies of its workspace, every path resolved against its directory. |
| Manifest Text | The text of a file that holds a manifest, which the decoder decodes. |
| Maybe Entry | The entry of a foreign name, or nothing. |
| Maybe Path | The path an entry names, or nothing. |
| Package Identifier | The name of the package of a crate as an identifier, its hyphens read as underscores, whose words keep the terms that spell them in use; an identifier of no word where a manifest has no package. |
| Root | The directory the directories of the contexts lie under, crates unless a context names another; and the manifest of the workspace, which a crate reads beside its own. |
| Workspace | The workspace of a root manifest, whose dependencies name the paths of the crates the crates of the workspace depend on. |
