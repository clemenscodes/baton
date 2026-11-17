# The ubiquitous language

| Word | What it means |
|---|---|
| Absence | The state of a directory that is not there, of a manifest a directory does not hold and of an attribute that is anything else than cfg(test). |
| Application | The layer of the library whose services, the gate and the press, run its use cases, the judgments of a gate and the edition of the books, through its ports. |
| Assemble Book | To assemble the book of every context, or the book of the context of every directory the command line names. |
| Book Edition | The command of a press that writes the book of every context, or of the context of every directory it names, and answers with the edition. |
| Book Entry | What became of one book of an edition: its publication, its omission where its directory is absent, or the error that refused it. |
| Byte | A byte of the text of a book, as its file receives it. |
| Close Dependency List | To add to a dependency list the paths of the crates its crates depend on by path, and the paths of theirs, until no path is new. |
| Closure | The paths of a dependency list together with every path the crates at those paths depend on by path, one after another; the closure of the development dependencies holds the crates the tests of a crate reach. |
| Compose Book Entry | To write a book and compose its entry, after a look at its directory where the book edition names no directory. |
| Compose Crate Examination | To read the dependencies of the crate of a source and compose the examination every source of that crate shares. |
| Compose Judgment | To compose the judgment of a list of errors: a success where the list is empty, and its errors where it is not. |
| Compose Languages | To compose the languages of the gate or the press from the contexts and the foreign languages of a vocabulary. |
| Compose Present Book Entry | To compose the publication of a book whose directory is present, its omission where its directory is absent, and the error of a directory the press cannot detect. |
| Compose Publication Entry | To write a book and compose its publication, or the error that refuses it. |
| Context List | The contexts of a vocabulary, as the port of the vocabulary lists them, from which the gate and the press compose their languages. |
| Crate Examination | The part of an examination that every source of one crate shares: the package identifier and the languages of its production code and of its tests, composed once for the crate; a source outside every crate has one of its own. |
| Dependency Language | The language of the crates a source depends on: the languages of the contexts the paths of the closure of a dependency list lie in, and the foreign languages its foreign names name. |
| Edition | The outcome of a book edition: a book entry for every book of it. |
| Error Kind | The kind of an io::Error, such as the invalid data of a manifest that is no TOML. |
| Examination | A source as the gate reads it at a path, with the language of its production code and the test language of its test code. |
| Foreign Language List | The foreign languages of a vocabulary, as the port of the vocabulary lists them. |
| Gate | The service that reads and parses the sources of a run and refuses a source where its text is no Rust, where its production code or its test code names or declares an unknown identifier or where it has a defect, and that refuses a context where its terms hold an unused term. |
| Gates Vocabulary Language Application | The crate gates_vocabulary_language_application, the application of the vocabulary gate: the gate that judges the sources of a run and the terms of their contexts, and the press that issues the books of the contexts. |
| Judge Path | To examine every path of a run, to refuse every source of them that its languages do not admit, and to report every error and to flush the errors. |
| Judge Source | To refuse an examination with every identifier of its production code and of its test code that their languages do not admit and with every defect of its source. |
| Judge Terms | To examine every path of a run and to refuse every context whose directory holds one of them and whose terms hold an unused term, and to report every error and to flush the errors. |
| Judgment | The judging of the paths of a command line or of their terms by a gate, every error reported to the errors. |
| List Contexts | To list every context of the vocabulary, or the error that one of them cannot be formed with. |
| List Foreign Languages | To list every foreign language of the vocabulary, or the error that one of them cannot be formed with. |
| List Path Sources | To list the sources one path names, an error where there is no source under it. |
| Locate Crate Directory | To find the nearest directory above a source with a manifest, the directory of its crate, or none where there is no manifest. |
| Manifest | The concept of the manifest file and the manifest presence of a directory the gate reads. |
| Manifest Presence | The manifest of a directory, or its absence where the directory holds no Cargo.toml. |
| Omission | The book entry of a book that was passed over, because the directory of its context is absent. |
| Path Judgment | The query of a gate that judges every source under the paths it names and answers with every error of them. |
| Pending Path | A path of the closure of a dependency list whose manifests are still to be read. |
| Presence | The state of a directory that is there, of a manifest a directory holds and of an attribute that is exactly cfg(test). |
| Press | The service that assembles the books of the contexts, detects their directories and writes every book into its directory. |
| Publication | The book entry of a book that went out, with the count of its terms. |
| Read Crate Dependencies | To read the manifests of directories one after another, the nearest first, into the dependencies of the crate whose manifest is the first of them. |
| Read Dependencies | To read the dependencies and the development dependencies of the crate a source belongs to from the manifests of the directories above it, and to close both dependency lists over the manifests of the crates they lead to, through the reader and the decoder. |
| Read Manifest | To read and decode the manifest of a directory, its paths resolved against that directory, or its absence where the directory has none. |
| Read Source | To read the text of a path through the reader and to parse it through the parser, and to refuse a file the reader cannot read and a text the parser cannot parse. |
| Reason | The text of the error of a decoder without the line break at its end, which the error of a manifest names. |
| Report | The part of the concept of a token that a token selection answers with, the token report. |
| Select Crate Examination | To select the crate examination of a source: the one of its crate directory where an earlier source of that crate composed it, and a new one composed from its manifest where none did. |
| Select Examinations | To examine every source of a list in its order, composing the crate examination of each crate once and sharing it among the sources of that crate. |
| Select Production Spellings | To select the spellings of the language of the production code of an examination that spell an identifier of that code or its package identifier. |
| Select Production Tokens | Read every source of a list and select the tokens of its production code, in the order of the source. |
| Select Term Errors | To select an error for every context of the gate whose unused terms are not none. |
| Selection | The part of the concept of a token that selects the production tokens of a run, the token selection. |
| Source Absence | The error of a path with no Rust source under it, such as an empty directory. |
| Syntax | The structure of the text of a source as Rust defines it, which a text that is no Rust lacks, and the adapter of the parser over the crate syn, which parses that structure. |
| Term Judgment | The query of a gate that judges the terms of every context a source under its paths lies in and answers with every unused term. |
| Test Language | The language of the test code of a source: the language of its production code joined with the test words and with the dependency language of its development dependencies. |
| Token | A token of the production code of a source, the concept of the token selection and the token report. |
| Token Report | The production tokens of the sources of a token selection, one token on a line with its path, its line, its column, its kind and its text. |
| Token Selection | The query of a gate that selects the production tokens of every source under its paths and answers with their token report. |
| Vocabulary | The port through which the application lists the contexts of the vocabulary and its foreign languages, which the program of the gate fills with its own. |
| Write Book | To put the text of a book into its file through a writer, to flush the file and to replace it. |
| Write Directory Error | To write the error of a directory whose presence the writer cannot detect, naming its path and the cause. |
| Write File Error | To write the error of a file that cannot be read, naming its path and the cause. |
| Write Language Error | To write one line for every occurrence of unknown vocabulary and for every defect of a source, each naming its path. |
| Write Manifest Error | To write the error of a crate manifest that cannot be decoded, naming its path and the reason without its last line break. |
| Write Source Absence | To write the error of a directory under which there is no Rust source, naming its path. |
| Write Syntax Error | To write the error of a source the parser refuses, naming its path and the error of the parser. |
| Write Terms Error | To write one line for every unused term a context defines, each naming the path of the context. |
