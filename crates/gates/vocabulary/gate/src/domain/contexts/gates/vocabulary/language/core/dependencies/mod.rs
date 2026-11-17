use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageCoreDependenciesContext;

impl Layer for GatesVocabularyLanguageCoreDependenciesContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageCoreDependenciesContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/core/dependencies";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Core", "Dependencies"];
            let definition = "The crate gates_vocabulary_language_core_dependencies, the dependencies of the crate of a source, with the port of the decoder of its manifests.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory"];
            let definition = "A directory of the file system, as a path, and the directory of a context, which admits every path beneath it under the last root of the path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Path"];
            let definition = "The path an entry names, or nothing.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dependency"];
            let definition = "A crate the manifest of a crate names under its dependencies or its development dependencies, for every platform or in the platform table of one, by its foreign name and by the path it leads to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest"];
            let definition = "The Cargo.toml of a crate or of the workspace, decoded into its package identifier and the entries of its dependencies, of its development dependencies and of the dependencies of its workspace, every path resolved against its directory.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry"];
            let definition = "A dependency of a manifest: the foreign name of a crate and the path it leads to, where it names one; and the error of a table of dependencies with a second entry of one foreign name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Entry"];
            let definition = "The entry of a foreign name, or nothing.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decoder"];
            let definition = "The port that decodes a manifest from its text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Manifest"];
            let definition = "To decode a manifest from its text, as the decoder does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Component"];
            let definition = "One part of a path between its separators.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Root"];
            let definition = "The directory the directories of the contexts lie under, crates unless a context names another; and the manifest of the workspace, which a crate reads beside its own.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Workspace"];
            let definition = "The workspace of a root manifest, whose dependencies name the paths of the crates the crates of the workspace depend on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Development", "Dependency"];
            let definition = "A crate the table dev-dependencies of a manifest names, whose languages only the test code of the sources of the crate is judged by.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dependency", "List"];
            let definition = "The foreign names of the crates one table of dependencies of a manifest names, and the paths of those it leads to by path.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Package", "Identifier"];
            let definition = "The name of the package of a crate as an identifier, its hyphens read as underscores, whose words keep the terms that spell them in use; an identifier of no word where a manifest has no package.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest", "Text"];
            let definition = "The text of a file that holds a manifest, which the decoder decodes.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry", "Path"];
            let definition = "The path an entry leads to, which a manifest resolves against its directory, each parent step against the component in front of it, and which keeps a parent step no component stands in front of.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Directory", "Entry"];
            let definition = "An entry of a directory as the file system lists it: a file, or a directory under it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Last", "Component"];
            let definition = "The last component of the components of a path that are resolved so far, which a parent component removes where it is a normal component.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decide", "Component", "Removal"];
            let definition = "To decide whether a parent component removes the last component: where the last component is a normal component, and never where it is a prefix, the root, the current directory, a parent component or missing.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageCoreDependenciesContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageCoreDependenciesContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageCoreDependenciesContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageCoreDependenciesContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_core_dependencies_context_is_a_term() {
        let gates_vocabulary_language_core_dependencies_context = GatesVocabularyLanguageCoreDependenciesContext;

        let context = Context::try_from(gates_vocabulary_language_core_dependencies_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
