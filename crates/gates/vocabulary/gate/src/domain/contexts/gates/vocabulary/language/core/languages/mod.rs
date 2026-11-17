use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageCoreLanguagesContext;

impl Layer for GatesVocabularyLanguageCoreLanguagesContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageCoreLanguagesContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/core/languages";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Core", "Languages"];
            let definition = "The crate gates_vocabulary_language_core_languages, every language of a repository: Rust, the contexts and the foreign languages.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Foreign", "Language"];
            let definition = "The names a crate uses from a crate outside it, named by the foreign name of that crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context"];
            let definition = "A directory under a root with the language of its crate and the foreign names it admits, whose language joins the foreign languages it names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Context"];
            let definition = "To select the context whose directory is exactly a path, without the current directory in front of it, or to refuse a path that is the directory of no context.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context", "Language"];
            let definition = "The language a source is judged against by its path: the language of Rust and the languages of the contexts that admit the path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Context", "Language"];
            let definition = "To join the language of Rust with the languages of every context that admits the path of a source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Dependency", "Language"];
            let definition = "To join the languages of the contexts the paths of a dependency list lie in with the foreign languages its foreign names name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Rust"];
            let definition = "The language of Rust itself: its keywords and the names of its standard library, which every source admits where its code uses them, and the words try and type, which every identifier may hold.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Keyword"];
            let definition = "A word Rust reserves or a primitive type of Rust, which every source admits as the whole identifier it is and never as a word of a longer one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Standard", "Library"];
            let definition = "The standard library of Rust, whose names every source admits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Language"];
            let definition = "To join the languages of the contexts that admit a path or the languages of the foreign languages that foreign names name, and to compose the language of the keywords, of the standard library, of the words try and type of Rust or of the test words.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Current", "Directory"];
            let definition = "The component . of a path, which names the directory the path starts in and which the directory of a context leaves out.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Word"];
            let definition = "A word docs/STYLE.md prescribes for tests, tests, fixture, helper, factory or expectation, which the test code of every source may hold alone and in a compound.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageCoreLanguagesContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageCoreLanguagesContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageCoreLanguagesContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageCoreLanguagesContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_core_languages_context_is_a_term() {
        let gates_vocabulary_language_core_languages_context = GatesVocabularyLanguageCoreLanguagesContext;

        let context = Context::try_from(gates_vocabulary_language_core_languages_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
