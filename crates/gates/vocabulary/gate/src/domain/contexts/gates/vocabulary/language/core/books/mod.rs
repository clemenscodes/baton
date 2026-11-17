use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageCoreBooksContext;

impl Layer for GatesVocabularyLanguageCoreBooksContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageCoreBooksContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/core/books";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Core", "Books"];
            let definition = "The crate gates_vocabulary_language_core_books, the books of the contexts, with the port of the writer they go out through.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Separator", "Escape"];
            let definition = "The backslash and the bar that a row shows in place of a bar inside a definition, so the bar separates no columns.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Book"];
            let definition = "The page of a context that lists its terms with their definitions, a table with no row for a context of no term.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Documents"];
            let definition = "The directory docs of a crate, where its book is written.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["File", "Name"];
            let definition = "The name of the file a book is written to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Table", "Head"];
            let definition = "The line of a book that names its two columns.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Table", "Rule"];
            let definition = "The line of a book under its table head.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Writer"];
            let definition = "The port that creates the temporary file of a path, replaces the file with it and detects a directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Create", "File"];
            let definition = "To create the temporary file of a path for a text, as the writer does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Replace", "File"];
            let definition = "To put the temporary file of a path in the place of its file, and to remove the temporary file where that fails.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Detect", "Directory"];
            let definition = "To detect whether a directory is present or absent, for the writer and for the book of a press.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory", "Presence"];
            let definition = "Whether a directory is present or absent.";
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
            let spelling: &[&str] = &["Heading"];
            let definition = "The first line of a book.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Row"];
            let definition = "One line of the table of a book, a term and its definition.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageCoreBooksContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageCoreBooksContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageCoreBooksContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageCoreBooksContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_core_books_context_is_a_term() {
        let gates_vocabulary_language_core_books_context = GatesVocabularyLanguageCoreBooksContext;

        let context = Context::try_from(gates_vocabulary_language_core_books_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
