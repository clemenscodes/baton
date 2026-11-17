use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageCoreForeignNameContext;

impl Layer for GatesVocabularyLanguageCoreForeignNameContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageCoreForeignNameContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/core/foreign_name";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Core", "Foreign", "Name"];
            let definition = "The crate gates_vocabulary_language_core_foreign_name, the name of a crate outside the workspace whose names a source may use.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Name"];
            let definition = "An identifier a language admits in exactly its words, without a definition; and the text a file, a directory, a crate, an attribute, a module, a function or a key of a manifest is called by, such as Cargo.toml, cfg or path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "A string, such as the text of a source, of a word, of a stem or of a definition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Foreign", "Name"];
            let definition = "The name of a foreign language, as a context, the directory of a context or an entry of a manifest names it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Domain"];
            let definition = "The layer of the library: its model of languages, sources and books, and the ports through which it reaches the world.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageCoreForeignNameContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageCoreForeignNameContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageCoreForeignNameContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageCoreForeignNameContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_core_foreign_name_context_is_a_term() {
        let gates_vocabulary_language_core_foreign_name_context = GatesVocabularyLanguageCoreForeignNameContext;

        let context = Context::try_from(gates_vocabulary_language_core_foreign_name_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
