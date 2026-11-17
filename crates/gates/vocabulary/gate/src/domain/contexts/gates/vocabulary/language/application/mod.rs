use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageApplicationContext;

impl Layer for GatesVocabularyLanguageApplicationContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageApplicationContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/application";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Application"];
            let definition = "The crate gates_vocabulary_language_application, the application of the vocabulary gate: the gate that judges the sources of a run and the terms of their contexts, and the press that issues the books of the contexts.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dependency", "Language"];
            let definition = "The language of the crates a source depends on: the languages of the contexts the paths of the closure of a dependency list lie in, and the foreign languages its foreign names name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gate"];
            let definition = "The service that reads and parses the sources of a run and refuses a source where its text is no Rust, where its production code or its test code names or declares an unknown identifier or where it has a defect, and that refuses a context where its terms hold an unused term.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judge", "Path"];
            let definition = "To examine every path of a run, to refuse every source of them that its languages do not admit, and to report every error and to flush the errors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Source"];
            let definition = "To read the text of a path through the reader and to parse it through the parser, and to refuse a file the reader cannot read and a text the parser cannot parse.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Dependencies"];
            let definition = "To read the dependencies and the development dependencies of the crate a source belongs to from the manifests of the directories above it, and to close both dependency lists over the manifests of the crates they lead to, through the reader and the decoder.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judgment"];
            let definition = "The judging of the paths of a command line or of their terms by a gate, every error reported to the errors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application"];
            let definition = "The layer of the library whose services, the gate and the press, run its use cases, the judgments of a gate and the edition of the books, through its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presence"];
            let definition = "The state of a directory that is there, of a manifest a directory holds and of an attribute that is exactly cfg(test).";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence"];
            let definition = "The state of a directory that is not there, of a manifest a directory does not hold and of an attribute that is anything else than cfg(test).";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Syntax"];
            let definition = "The structure of the text of a source as Rust defines it, which a text that is no Rust lacks, and the adapter of the parser over the crate syn, which parses that structure.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error", "Kind"];
            let definition = "The kind of an io::Error, such as the invalid data of a manifest that is no TOML.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reason"];
            let definition = "The text of the error of a decoder without the line break at its end, which the error of a manifest names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Production", "Tokens"];
            let definition = "Read every source of a list and select the tokens of its production code, in the order of the source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Language"];
            let definition = "The language of the test code of a source: the language of its production code joined with the test words and with the dependency language of its development dependencies.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Examination"];
            let definition = "A source as the gate reads it at a path, with the language of its production code and the test language of its test code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judge", "Source"];
            let definition = "To refuse an examination with every identifier of its production code and of its test code that their languages do not admit and with every defect of its source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Production", "Spellings"];
            let definition = "To select the spellings of the language of the production code of an examination that spell an identifier of that code or its package identifier.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Term", "Errors"];
            let definition = "To select an error for every context of the gate whose unused terms are not none.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judge", "Terms"];
            let definition = "To examine every path of a run and to refuse every context whose directory holds one of them and whose terms hold an unused term, and to report every error and to flush the errors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Manifest"];
            let definition = "To read and decode the manifest of a directory, its paths resolved against that directory, or its absence where the directory has none.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Read", "Crate", "Dependencies"];
            let definition = "To read the manifests of directories one after another, the nearest first, into the dependencies of the crate whose manifest is the first of them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Close", "Dependency", "List"];
            let definition = "To add to a dependency list the paths of the crates its crates depend on by path, and the paths of theirs, until no path is new.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Closure"];
            let definition = "The paths of a dependency list together with every path the crates at those paths depend on by path, one after another; the closure of the development dependencies holds the crates the tests of a crate reach.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pending", "Path"];
            let definition = "A path of the closure of a dependency list whose manifests are still to be read.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest", "Presence"];
            let definition = "The manifest of a directory, or its absence where the directory holds no Cargo.toml.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List", "Path", "Sources"];
            let definition = "To list the sources one path names, an error where there is no source under it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Absence"];
            let definition = "The error of a path with no Rust source under it, such as an empty directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Crate", "Examination"];
            let definition = "The part of an examination that every source of one crate shares: the package identifier and the languages of its production code and of its tests, composed once for the crate; a source outside every crate has one of its own.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Examinations"];
            let definition = "To examine every source of a list in its order, composing the crate examination of each crate once and sharing it among the sources of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Locate", "Crate", "Directory"];
            let definition = "To find the nearest directory above a source with a manifest, the directory of its crate, or none where there is no manifest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Crate", "Examination"];
            let definition = "To read the dependencies of the crate of a source and compose the examination every source of that crate shares.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "File", "Error"];
            let definition = "To write the error of a file that cannot be read, naming its path and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Manifest", "Error"];
            let definition = "To write the error of a crate manifest that cannot be decoded, naming its path and the reason without its last line break.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Syntax", "Error"];
            let definition = "To write the error of a source the parser refuses, naming its path and the error of the parser.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Language", "Error"];
            let definition = "To write one line for every occurrence of unknown vocabulary and for every defect of a source, each naming its path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Terms", "Error"];
            let definition = "To write one line for every unused term a context defines, each naming the path of the context.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Source", "Absence"];
            let definition = "To write the error of a directory under which there is no Rust source, naming its path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Crate", "Examination"];
            let definition = "To select the crate examination of a source: the one of its crate directory where an earlier source of that crate composed it, and a new one composed from its manifest where none did.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Judgment"];
            let definition = "To compose the judgment of a list of errors: a success where the list is empty, and its errors where it is not.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Assemble", "Book"];
            let definition = "To assemble the book of every context, or the book of the context of every directory the command line names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Press"];
            let definition = "The service that assembles the books of the contexts, detects their directories and writes every book into its directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Book"];
            let definition = "To put the text of a book into its file through a writer, to flush the file and to replace it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Byte"];
            let definition = "A byte of the text of a book, as its file receives it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Vocabulary"];
            let definition = "The port through which the application lists the contexts of the vocabulary and its foreign languages, which the program of the gate fills with its own.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context", "List"];
            let definition = "The contexts of a vocabulary, as the port of the vocabulary lists them, from which the gate and the press compose their languages.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Foreign", "Language", "List"];
            let definition = "The foreign languages of a vocabulary, as the port of the vocabulary lists them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List", "Contexts"];
            let definition = "To list every context of the vocabulary, or the error that one of them cannot be formed with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["List", "Foreign", "Languages"];
            let definition = "To list every foreign language of the vocabulary, or the error that one of them cannot be formed with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Languages"];
            let definition = "To compose the languages of the gate or the press from the contexts and the foreign languages of a vocabulary.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path", "Judgment"];
            let definition = "The query of a gate that judges every source under the paths it names and answers with every error of them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Term", "Judgment"];
            let definition = "The query of a gate that judges the terms of every context a source under its paths lies in and answers with every unused term.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token"];
            let definition = "A token of the production code of a source, the concept of the token selection and the token report.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token", "Selection"];
            let definition = "The query of a gate that selects the production tokens of every source under its paths and answers with their token report.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token", "Report"];
            let definition = "The production tokens of the sources of a token selection, one token on a line with its path, its line, its column, its kind and its text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Book", "Edition"];
            let definition = "The command of a press that writes the book of every context, or of the context of every directory it names, and answers with the edition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Edition"];
            let definition = "The outcome of a book edition: a book entry for every book of it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Book", "Entry"];
            let definition = "What became of one book of an edition: its publication, its omission where its directory is absent, or the error that refused it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Publication"];
            let definition = "The book entry of a book that went out, with the count of its terms.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Omission"];
            let definition = "The book entry of a book that was passed over, because the directory of its context is absent.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Book", "Entry"];
            let definition = "To write a book and compose its entry, after a look at its directory where the book edition names no directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Present", "Book", "Entry"];
            let definition = "To compose the publication of a book whose directory is present, its omission where its directory is absent, and the error of a directory the press cannot detect.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Publication", "Entry"];
            let definition = "To write a book and compose its publication, or the error that refuses it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest"];
            let definition = "The concept of the manifest file and the manifest presence of a directory the gate reads.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Write", "Directory", "Error"];
            let definition = "To write the error of a directory whose presence the writer cannot detect, naming its path and the cause.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Selection"];
            let definition = "The part of the concept of a token that selects the production tokens of a run, the token selection.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report"];
            let definition = "The part of the concept of a token that a token selection answers with, the token report.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageApplicationContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageApplicationContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageApplicationContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageApplicationContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_gates_vocabulary_language_application_context_is_a_term() {
        let gates_vocabulary_language_application_context = GatesVocabularyLanguageApplicationContext;

        let context = Context::try_from(gates_vocabulary_language_application_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
