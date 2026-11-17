use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageAdapterFileSystemContext;

impl Layer for GatesVocabularyLanguageAdapterFileSystemContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageAdapterFileSystemContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/adapter/file_system";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Gates",
                "Vocabulary",
                "Language",
                "Adapter",
                "File",
                "System",
            ];
            let definition = "The crate gates_vocabulary_language_adapter_file_system, the adapter of the reader and the writer over the files of the machine.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gate"];
            let definition = "The service that reads and parses the sources of a run and refuses a source where its text is no Rust, where its production code or its test code names or declares an unknown identifier or where it has a defect, and that refuses a context where its terms hold an unused term.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Infrastructure"];
            let definition = "The layer of the library whose adapters, the file system, the decoder over TOML and the parser over syn, fill its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "System"];
            let definition = "The adapter of the reader and the writer over the files of the machine.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Temporary", "Path"];
            let definition = "The path of a file with a suffix, beside the file, which a text is first put into and then renamed over the file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Temporary", "Path"];
            let definition = "To compose the temporary path beside the path of a file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process", "Identifier"];
            let definition = "The identifier the operating system gives the running process, which a temporary path carries in its suffix.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Suffix"];
            let definition = "The end that follows the part of a text: the suffix separator and the process identifier that a temporary path carries behind the path of its file, and the name behind the digits or the quotes of a literal.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Removal"];
            let definition = "The removal of a temporary file whose replacement failed, and the error where that removal failed as well.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Replacement"];
            let definition = "The renaming of the temporary file of a path over that file.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Metadata"];
            let definition = "What the file system states about a path, such as whether it is a directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error", "Kind"];
            let definition = "The kind of an io::Error, such as the invalid data of a manifest that is no TOML.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Walk", "Directory"];
            let definition = "To walk a directory and every directory under it and list every Rust file in them, in their order.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pending", "Directory"];
            let definition = "A directory under the directory a walk starts at whose entries are still to be listed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Extension"];
            let definition = "The extension rs that the name of a Rust file ends with.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageAdapterFileSystemContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageAdapterFileSystemContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageAdapterFileSystemContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageAdapterFileSystemContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_adapter_file_system_context_is_a_term() {
        let gates_vocabulary_language_adapter_file_system_context = GatesVocabularyLanguageAdapterFileSystemContext;

        let context = Context::try_from(gates_vocabulary_language_adapter_file_system_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
