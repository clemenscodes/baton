mod context;

pub use context::Context;

use crate::Error;
use crate::foreign_languages::ForeignLanguages;
use gates_vocabulary_language_core_language::Language;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Component, Path, PathBuf};
use std::slice::Iter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Contexts {
    contexts: Vec<Context>,
}

impl Layer for Contexts {
    type Layer = DomainLayer;
}

impl ValueObject for Contexts {}

impl Contexts {
    pub(super) fn compose_language(&self, path: &Path) -> Language {
        let Self { contexts } = self;
        let candidate = path.to_path_buf();
        let mut language = Language::default();
        for context in contexts {
            let directory = context.directory();
            let decision = directory.decide_acceptance(&candidate);
            match decision {
                CandidateDecision::Acceptance => language = &language | context.language(),
                CandidateDecision::Rejection => {}
            }
        }
        language
    }

    pub fn try_select_context(&self, path: &Path) -> Result<&Context, Error> {
        let Self { contexts } = self;
        let current_directory = Component::CurDir;
        let component_criterion = |component: &Component<'_>| *component != current_directory;
        let candidate: PathBuf = path.components().filter(component_criterion).collect();
        for context in contexts {
            let directory = context.directory();
            let directory_path = directory.path();
            if directory_path == candidate {
                return Ok(context);
            }
        }
        let error_path = path.to_path_buf();
        let error = Error::Context { path: error_path };
        Err(error)
    }
}

impl From<(Vec<Context>, &ForeignLanguages)> for Contexts {
    fn from((context_list, foreign_languages): (Vec<Context>, &ForeignLanguages)) -> Self {
        let language_context = |context: Context| {
            let foreign_names = context.foreign_names();
            let foreign_language = foreign_languages.compose_language(foreign_names);
            Context::from((context, foreign_language))
        };
        let contexts = context_list.into_iter().map(language_context).collect();
        Self { contexts }
    }
}

impl<'contexts> IntoIterator for &'contexts Contexts {
    type Item = &'contexts Context;
    type IntoIter = Iter<'contexts, Context>;

    fn into_iter(self) -> Self::IntoIter {
        let Contexts { contexts } = self;
        contexts.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::foreign_languages::ForeignLanguage;
    use crate::tests::fixture as source_fixture;
    use gates_vocabulary_language_core_foreign_name::ForeignName;
    use gates_vocabulary_language_core_language::TermText;

    mod fixture {
        use super::*;

        pub(super) fn contexts_factory() -> Contexts {
            const TERM_TEXTS: &[TermText] = &[{
                let spelling: &[&str] = &["Treasure"];
                let definition = "A thing.";
                TermText::new(spelling, definition)
            }];
            let term_texts = TERM_TEXTS;
            let directory_name = "harbor";
            let parser_text = "clap";
            let parser_name = ForeignName::from(parser_text);
            let foreign_names = vec![parser_name];
            let context = Context::try_from((directory_name, term_texts, foreign_names)).unwrap();
            let context_list = vec![context];
            let parser_name_texts: &[&str] = &["Parser"];
            let parser_foreign_name = ForeignName::from(parser_text);
            let parser_language = ForeignLanguage::try_from((
                parser_foreign_name,
                parser_name_texts,
            ))
            .unwrap();
            let foreign_language_list = [parser_language];
            let foreign_languages: ForeignLanguages = foreign_language_list.into_iter().collect();
            Contexts::from((context_list, &foreign_languages))
        }
    }

    #[test]
    fn the_language_of_a_path_is_its_context_and_the_foreign_languages_it_names() {
        let contexts = fixture::contexts_factory();
        let path_text = "crates/harbor/src/lib.rs";
        let path = Path::new(path_text);
        let identifier_texts: &[&str] = &["struct", "Treasure", "Parser", "Anchor"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let language = contexts.compose_language(path);

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
    fn the_context_of_a_directory_is_the_context_of_exactly_that_directory() {
        let contexts = fixture::contexts_factory();
        let path_text = "./crates/harbor/";
        let path = Path::new(path_text);

        let context = contexts.try_select_context(path).unwrap();

        let directory = context.directory();
        let directory_path = directory.path();
        let expectation_text = "crates/harbor";
        let expectation = Path::new(expectation_text);
        assert_eq!(directory_path, expectation);
    }

    #[test]
    fn a_path_beneath_the_directory_of_a_context_is_an_error() {
        let contexts = fixture::contexts_factory();
        let path_text = "crates/harbor/src";
        let path = Path::new(path_text);

        let context = contexts.try_select_context(path);

        let error_path = PathBuf::from(path_text);
        let error = Error::Context { path: error_path };
        let expectation = Err(error);
        assert_eq!(context, expectation);
    }

    #[test]
    fn the_directory_of_a_context_under_another_root_is_an_error() {
        let contexts = fixture::contexts_factory();
        let path_text = "elsewhere/crates/harbor";
        let path = Path::new(path_text);

        let context = contexts.try_select_context(path);

        let error_path = PathBuf::from(path_text);
        let error = Error::Context { path: error_path };
        let expectation = Err(error);
        assert_eq!(context, expectation);
    }

    #[test]
    fn a_path_no_context_admits_is_an_error() {
        let contexts = fixture::contexts_factory();
        let path_text = "crates/nowhere";
        let path = Path::new(path_text);

        let context = contexts.try_select_context(path);

        let error_path = PathBuf::from(path_text);
        let error = Error::Context { path: error_path };
        let expectation = Err(error);
        assert_eq!(context, expectation);
    }
}
