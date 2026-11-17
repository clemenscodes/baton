use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageAdapterCommandLineContext;

impl Layer for GatesVocabularyLanguageAdapterCommandLineContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageAdapterCommandLineContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/adapter/command_line";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Report", "Book", "Entries"];
            let definition = "To report every book entry of an edition of the press and to combine their outcomes into one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &[
                "Gates",
                "Vocabulary",
                "Language",
                "Adapter",
                "Command",
                "Line",
            ];
            let definition = "The crate gates_vocabulary_language_adapter_command_line, the command line of the vocabulary gate and the session that serves it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Edition"];
            let definition = "The report of the edition of a press: every book entry that went out or was passed over on the report, and every error on the errors.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Publish", "Book"];
            let definition = "To dispatch the book edition of the directories the command line names to a press over the file system and to report its edition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Flush", "Stream"];
            let definition = "To flush the report and the errors of an edition, so every line written to them goes out.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Session"];
            let definition = "The vocabulary a command line is served with, and the report and the errors it writes to; it composes the adapters of the gate and the press and runs their use cases.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report"];
            let definition = "The stream every book that goes out, and every book whose directory is nowhere, is reported on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Line"];
            let definition = "The arguments the process is started with: the paths of the sources to judge, the paths of the sources whose terms to judge, or the directories whose books to publish.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit", "Code"];
            let definition = "The status the process exits with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Respond", "Exit", "Code"];
            let definition = "To judge the paths or their terms or to publish the books the command line names, through a session, and to respond with the exit code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Outcome"];
            let definition = "Whether the command line succeeded or failed, which the process exits with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presentation"];
            let definition = "The layer of the library a person or a program reaches: its command line.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Success"];
            let definition = "The outcome of a command line whose every judgment and every book succeeded.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Production", "Tokens"];
            let definition = "To write the production tokens of the sources to the report, one token on a line, so a gate reads the names and the signs of the code and no text of it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Judge", "Session"];
            let definition = "To judge the sources of a session, or the terms where it asks for them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Judgment"];
            let definition = "To write every error of the judgment of a gate to the errors, flush them and respond with the outcome of the judgment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Examine", "Session"];
            let definition = "To examine the paths of a session that publishes no book: to report their production tokens, or to judge them.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error", "Stream"];
            let definition = "The stream every error of a session is reported on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Gate"];
            let definition = "To compose the gate of a session over the vocabulary, the file system, the decoder of manifests and the parser of sources, or the errors that the vocabulary cannot form its languages with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path", "Resolution"];
            let definition = "The resolution of the path judgment of a session by its gate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Term", "Resolution"];
            let definition = "The resolution of the term judgment of a session by its gate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token", "Resolution"];
            let definition = "The resolution of the token selection of a session by its gate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Edition"];
            let definition = "To write every book entry of an edition to the report or to the errors, or the error that refused the edition, flush both and respond with the outcome.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Report", "Book", "Entry"];
            let definition = "To write a book entry to the report where its book went out or was passed over, and to the errors where an error refused it.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageAdapterCommandLineContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageAdapterCommandLineContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageAdapterCommandLineContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageAdapterCommandLineContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_adapter_command_line_context_is_a_term() {
        let gates_vocabulary_language_adapter_command_line_context = GatesVocabularyLanguageAdapterCommandLineContext;

        let context = Context::try_from(gates_vocabulary_language_adapter_command_line_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
