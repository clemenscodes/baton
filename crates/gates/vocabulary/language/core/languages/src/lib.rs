pub mod contexts;
pub mod foreign_languages;

mod error;
mod rust;
mod test_words;

pub use error::Error;

use crate::contexts::{Context, Contexts};
use crate::foreign_languages::{ForeignLanguage, ForeignLanguages};
use crate::rust::Rust;
use crate::test_words::TestWords;
use gates_vocabulary_language_core_dependencies::DependencyList;
use gates_vocabulary_language_core_language::Language;
use stack_domain_driven_design_core::factory::Factory;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Languages {
    rust: Rust,
    test_word_language: Language,
    contexts: Contexts,
    foreign_languages: ForeignLanguages,
}

impl Layer for Languages {
    type Layer = DomainLayer;
}

impl ValueObject for Languages {}

impl Languages {
    pub fn contexts(&self) -> &Contexts {
        match self {
            Self { contexts, .. } => contexts,
        }
    }

    pub fn test_word_language(&self) -> &Language {
        &self.test_word_language
    }

    pub fn compose_context_language(&self, source_path: &Path) -> Language {
        let Self { rust, contexts, .. } = self;
        let rust_language = rust.language();
        let context_language = contexts.compose_language(source_path);
        rust_language | &context_language
    }

    pub fn compose_dependency_language(&self, dependency_list: &DependencyList) -> Language {
        let Self {
            contexts,
            foreign_languages,
            ..
        } = self;
        let mut path_language = Language::default();
        for dependency_path in dependency_list.paths() {
            let context_language = contexts.compose_language(dependency_path);
            path_language = &path_language | &context_language;
        }
        let foreign_names = dependency_list.foreign_names();
        let foreign_language = foreign_languages.compose_language(foreign_names);
        &path_language | &foreign_language
    }
}

impl TryFrom<(Vec<Context>, Vec<ForeignLanguage>)> for Languages {
    type Error = Error;

    fn try_from(
        (context_list, foreign_language_list): (Vec<Context>, Vec<ForeignLanguage>),
    ) -> Result<Self, Self::Error> {
        let rust_language = Rust::try_from(());
        let test_words = TestWords::from(());
        let test_word_language = test_words.try_create_product(()).map_err(Error::from);
        let rust_languages = |rust: Rust| {
            let word_languages = |word_language: Language| {
                let foreign_languages: ForeignLanguages = foreign_language_list
                    .into_iter()
                    .collect();
                let contexts = Contexts::from((context_list, &foreign_languages));
                Self {
                    rust,
                    test_word_language: word_language,
                    contexts,
                    foreign_languages,
                }
            };
            test_word_language.map(word_languages)
        };
        rust_language.and_then(rust_languages)
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::tests::fixture as entry_fixture;
    use crate::tests::fixture as source_fixture;
    use gates_vocabulary_language_core_dependencies::manifest::entries::Entries;
    use gates_vocabulary_language_core_language::TermText;
    use gates_vocabulary_language_core_language::UnknownVocabulary;
    use std::path::PathBuf;

    pub(crate) mod fixture {
        use super::*;
        use gates_vocabulary_language_core_dependencies::manifest::entries::Entry;
        use gates_vocabulary_language_core_foreign_name::ForeignName;
        use gates_vocabulary_language_core_language::Code;
        use gates_vocabulary_language_core_language::code::tokens::CodeTokens;
        use gates_vocabulary_language_core_language::defects::Defects;
        use gates_vocabulary_language_core_language::occurrence::Occurrence;
        use gates_vocabulary_language_core_language::source::Source;

        fn occurrences_factory<OccurrencesType: FromIterator<Occurrence>>(
            texts: &[&str],
        ) -> OccurrencesType {
            let line: usize = 1;
            let mut occurrence_list = Vec::new();
            for (position, text) in texts.iter().copied().enumerate() {
                let column = position + 1;
                let occurrence = Occurrence::try_from((text, line, column)).unwrap();
                occurrence_list.push(occurrence);
            }
            occurrence_list.into_iter().collect()
        }

        pub(crate) fn identifier_source_factory(identifier_texts: &[&str]) -> Source {
            let declaration_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn declaration_source_factory(declaration_texts: &[&str]) -> Source {
            let identifier_texts: &[&str] = &[];
            let identifiers = occurrences_factory(identifier_texts);
            let declarations = occurrences_factory(declaration_texts);
            let production_code = Code::from((identifiers, declarations));
            let test_code = Code::default();
            let defects = Defects::default();
            let production_tokens = CodeTokens::default();
            Source::from((production_code, production_tokens, test_code, defects))
        }

        pub(crate) fn foreign_name_entry_factory(foreign_name_text: &str) -> Entry {
            let foreign_name = ForeignName::from(foreign_name_text);
            let path = None;
            Entry::from((foreign_name, path))
        }

        pub(crate) const TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        }];

        pub(crate) fn context_languages_factory() -> Languages {
            let term_texts = TERM_TEXTS;
            let language = Language::try_from(term_texts).unwrap();
            let directory_name = "harbor";
            let context = Context::from((directory_name, language));
            let contexts = vec![context];
            let foreign_languages = Vec::new();
            Languages::try_from((contexts, foreign_languages)).unwrap()
        }

        pub(super) fn foreign_languages_factory() -> Languages {
            let term_texts = TERM_TEXTS;
            let directory_name = "harbor";
            let context_foreign_name_text = "lighthouse";
            let dependency_foreign_name_text = "anchor";
            let context_foreign_name = ForeignName::from(context_foreign_name_text);
            let foreign_names = vec![context_foreign_name];
            let context = Context::try_from((directory_name, term_texts, foreign_names)).unwrap();
            let contexts = vec![context];
            let context_name_texts: &[&str] = &["Lamp"];
            let dependency_name_texts: &[&str] = &["Anchor"];
            let directory_name_texts: &[&str] = &["Quay"];
            let context_language_foreign_name = ForeignName::from(context_foreign_name_text);
            let dependency_foreign_name = ForeignName::from(dependency_foreign_name_text);
            let directory_foreign_name = ForeignName::from(directory_name);
            let context_foreign_language = ForeignLanguage::try_from((
                context_language_foreign_name,
                context_name_texts,
            ))
            .unwrap();
            let dependency_foreign_language = ForeignLanguage::try_from((
                dependency_foreign_name,
                dependency_name_texts,
            ))
            .unwrap();
            let directory_foreign_language = ForeignLanguage::try_from((
                directory_foreign_name,
                directory_name_texts,
            ))
            .unwrap();
            let foreign_languages = vec![
                context_foreign_language,
                dependency_foreign_language,
                directory_foreign_language,
            ];
            Languages::try_from((contexts, foreign_languages)).unwrap()
        }
    }

    #[test]
    fn a_source_of_a_context_is_judged_by_rust_its_context_and_the_foreign_languages_it_names() {
        let languages = fixture::foreign_languages_factory();
        let source_path_text = "crates/harbor/src/lib.rs";
        let source_path = Path::new(source_path_text);
        let identifier_texts: &[&str] = &["struct", "Treasure", "Lamp", "Anchor", "Quay"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let language = languages.compose_context_language(source_path);

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:4: `Anchor`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_source_outside_every_context_is_judged_by_rust_alone() {
        let languages = fixture::foreign_languages_factory();
        let source_path_text = "nix/flake/default.nix";
        let source_path = Path::new(source_path_text);
        let identifier_texts: &[&str] = &["struct", "Treasure", "Lamp", "Anchor", "Quay"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let language = languages.compose_context_language(source_path);

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:2: `Treasure`",
            "1:3: `Lamp`",
            "1:4: `Anchor`",
            "1:5: `Quay`",
        ];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_dependency_on_the_path_of_a_context_lets_its_language_through() {
        let languages = fixture::foreign_languages_factory();
        let directory_path_text = "crates/harbor";
        let directory_path = PathBuf::from(directory_path_text);
        let path_list = [directory_path];
        let dependency_list: DependencyList = path_list.into_iter().collect();
        let identifier_texts: &[&str] = &["struct", "Treasure", "Lamp", "Anchor", "Quay"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let language = languages.compose_dependency_language(&dependency_list);

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:1: `struct`", "1:4: `Anchor`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn a_dependency_on_a_crate_of_a_foreign_language_lets_its_names_through() {
        let languages = fixture::foreign_languages_factory();
        let dependency_foreign_name_text = "anchor";
        let foreign_name_entry = entry_fixture::foreign_name_entry_factory(
            dependency_foreign_name_text,
        );
        let entry_list = vec![foreign_name_entry];
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let workspace_entries = Entries::default();
        let dependency_list = DependencyList::from((&dependency_entries, &workspace_entries));
        let identifier_texts: &[&str] = &["struct", "Treasure", "Lamp", "Anchor", "Quay"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let language = languages.compose_dependency_language(&dependency_list);

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:1: `struct`",
            "1:2: `Treasure`",
            "1:3: `Lamp`",
            "1:5: `Quay`",
        ];
        assert_eq!(unknown_occurrence_texts, expectation);
    }

    #[test]
    fn the_test_word_language_spells_the_words_docs_style_prescribes_for_tests() {
        let languages = fixture::context_languages_factory();
        let declaration_texts: &[&str] = &["tests", "fixture", "helper", "factory", "expectation"];
        let source = source_fixture::declaration_source_factory(declaration_texts);
        let production_code = source.production_code();

        let language = languages.test_word_language();

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let expectation = UnknownVocabulary::default();
        assert_eq!(unknown_vocabulary, expectation);
    }

    #[test]
    fn the_languages_of_the_same_contexts_are_the_same_every_time() {
        let term_texts = fixture::TERM_TEXTS;
        let directory_name = "harbor";
        let foreign_names = Vec::new();
        let context = Context::try_from((directory_name, term_texts, foreign_names)).unwrap();
        let contexts = vec![context];
        let foreign_languages = Vec::new();
        let languages = fixture::context_languages_factory();

        let languages_copy = Languages::try_from((contexts, foreign_languages));

        let expectation = Ok(languages);
        assert_eq!(languages_copy, expectation);
    }

    #[test]
    fn languages_lend_their_contexts() {
        let languages = fixture::context_languages_factory();

        let contexts = languages.contexts();

        let directory_path = |context: &Context| context.directory().path().to_path_buf();
        let directory_paths: Vec<PathBuf> = contexts.into_iter().map(directory_path).collect();
        let directory_path_text = "crates/harbor";
        let expectation = [PathBuf::from(directory_path_text)];
        assert_eq!(directory_paths, expectation);
    }
}
