use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext;

impl Layer for GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/adapter/toms_obvious_minimal_language";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Gates",
                "Vocabulary",
                "Language",
                "Adapter",
                "Toms",
                "Obvious",
                "Minimal",
                "Language",
            ];
            let definition = "The crate gates_vocabulary_language_adapter_toms_obvious_minimal_language, the adapter of the decoder over TOML.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Dependency", "Table"];
            let definition = "The table of the dependencies of a manifest or of its workspace.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest", "Table"];
            let definition = "The table the text of a manifest parses into.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Table", "Error", "Text"];
            let definition = "The text of the error with which the decoder refuses a manifest whose dependencies are no table.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Entries"];
            let definition = "To decode the entries of the dependency tables of a manifest, an entry two tables name alike only once, or no entry where a manifest names no dependency, and to refuse dependencies that are no table and a second entry of one foreign name.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Path"];
            let definition = "To decode the path the entry of a dependency leads to, or no path where the entry names none, and to refuse a path that is no text.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Workspace", "Dependencies"];
            let definition = "To select the dependency table of the workspace of a manifest, or none where the manifest has no workspace, and to refuse a workspace that is no table.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Target"];
            let definition = "The table target of a manifest, which holds a platform table for every platform whose own dependencies the manifest names.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Platform"];
            let definition = "A system a crate is compiled for, such as the one the condition cfg(unix) or the architecture wasm32 describes, as the key of a platform table names it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Platform", "Table"];
            let definition = "The table under the target of a manifest that names the dependencies and the development dependencies of one platform.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Toms", "Obvious", "Minimal", "Language"];
            let definition = "The adapter of the decoder over TOML, the language Tom's Obvious, Minimal Language of the manifests of Cargo.";
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
            let spelling: &[&str] = &["Workspace", "Table"];
            let definition = "The table of the workspace of a root manifest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error", "Kind"];
            let definition = "The kind of an io::Error, such as the invalid data of a manifest that is no TOML.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Development", "Dependency", "Table"];
            let definition = "The table dev-dependencies of a manifest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Package", "Table"];
            let definition = "The table package of a manifest, whose name is the name of its crate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Package", "Identifier"];
            let definition = "To select the name of the package table of a manifest, or none where the manifest has no package, and to refuse a package that is no table.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Package", "Identifier"];
            let definition = "To decode the package identifier from the name of a package, or an identifier of no word where the package has no name, and to refuse a name that is no text or no identifier of words.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Table", "Name"];
            let definition = "The name of a table of a manifest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Parse", "Manifest", "Table"];
            let definition = "To parse the text of a manifest into its table.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Workspace", "Entries"];
            let definition = "To decode the entries of the dependencies of the workspace.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Manifest", "Tables"];
            let definition = "The table of a manifest and the table of every platform under its target, which together list the dependencies of the manifest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Select", "Dependency", "Values"];
            let definition = "Select the value of a table of dependencies from the manifest and from every platform, one after the other.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Decode", "Dependency", "Value"];
            let definition = "To decode the entries of one dependency value of a manifest: none where the table is missing, the entry of every dependency where it is a table, and an error where it is anything else.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Table", "Entries"];
            let definition = "The entries one dependency table of a manifest decodes into, before the entries that two tables share are kept once.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext> for Context {
    type Error = Error;

    fn try_from(
        _: GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext,
    ) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_vocabulary_language_adapter_toms_obvious_minimal_language_context_is_a_term()
     {
        let gates_vocabulary_language_adapter_toms_obvious_minimal_language_context = GatesVocabularyLanguageAdapterTomsObviousMinimalLanguageContext;

        let context = Context::try_from(
            gates_vocabulary_language_adapter_toms_obvious_minimal_language_context,
        );

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
