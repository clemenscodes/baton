mod directory;

use crate::Error;
use crate::contexts::context::directory::Directory;
use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_language::source::SourceSpellings;
use gates_vocabulary_language_core_language::{Language, TermText, UnusedTerms};
use stack_domain_driven_design_core::aggregate_root::AggregateRoot;
use stack_domain_driven_design_core::entity::Entity;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use std::iter;
use std::path::Path;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Context {
    directory: Directory,
    language: Language,
    foreign_names: Vec<ForeignName>,
}

impl Layer for Context {
    type Layer = DomainLayer;
}

impl Entity for Context {
    type Identity = Directory;

    fn identity(&self) -> &Directory {
        &self.directory
    }
}

impl AggregateRoot for Context {}

impl Context {
    pub fn directory(&self) -> &Directory {
        &self.directory
    }

    pub fn language(&self) -> &Language {
        &self.language
    }

    pub(super) fn foreign_names(&self) -> &[ForeignName] {
        &self.foreign_names
    }

    pub fn select_unused_terms(&self, source_spellings: &SourceSpellings) -> UnusedTerms {
        let Self {
            directory,
            language,
            ..
        } = self;
        let mut directory_decision = CandidateDecision::Rejection;
        for (source_path, _) in source_spellings {
            let source_decision = directory.decide_acceptance(source_path);
            directory_decision = directory_decision | source_decision;
        }
        match directory_decision {
            CandidateDecision::Acceptance => {
                let terms = language.terms();
                terms.select_unused_terms(source_spellings)
            }
            CandidateDecision::Rejection => UnusedTerms::default(),
        }
    }
}

impl From<(&str, Language)> for Context {
    fn from((directory_name, language): (&str, Language)) -> Self {
        let directory = Directory::from(directory_name);
        let foreign_name = ForeignName::from(directory_name);
        let foreign_names = vec![foreign_name];
        Self {
            directory,
            language,
            foreign_names,
        }
    }
}

impl From<(&Path, &str, Language)> for Context {
    fn from((root, directory_name, language): (&Path, &str, Language)) -> Self {
        let directory = Directory::from((root, directory_name));
        let foreign_name = ForeignName::from(directory_name);
        let foreign_names = vec![foreign_name];
        Self {
            directory,
            language,
            foreign_names,
        }
    }
}

impl TryFrom<(&str, &[TermText], Vec<ForeignName>)> for Context {
    type Error = Error;

    fn try_from(
        (directory_name, term_texts, context_foreign_names): (&str, &[TermText], Vec<ForeignName>),
    ) -> Result<Self, Self::Error> {
        let language = Language::try_from(term_texts)?;
        let directory = Directory::from(directory_name);
        let foreign_name = ForeignName::from(directory_name);
        let directory_foreign_names = iter::once(foreign_name);
        let foreign_names = directory_foreign_names
            .chain(context_foreign_names)
            .collect();
        let context = Self {
            directory,
            language,
            foreign_names,
        };
        Ok(context)
    }
}

impl From<(Context, Language)> for Context {
    fn from((context, foreign_language): (Self, Language)) -> Self {
        let Self {
            directory,
            language: context_language,
            foreign_names,
        } = context;
        let language = &context_language | &foreign_language;
        Self {
            directory,
            language,
            foreign_names,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_core_language::vocabulary::Spellings;
    use gates_vocabulary_language_core_language::vocabulary::term;
    use gates_vocabulary_language_core_language::vocabulary::term::spelling;
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    mod fixture {
        use super::*;

        pub(super) const TERM_TEXTS: &[TermText] = &[{
            let spelling: &[&str] = &["Treasure"];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        }];

        pub(super) fn source_spellings_factory(path_text: &str) -> SourceSpellings {
            let word_texts: &[&str] = &["anchor"];
            let spellings = Spellings::try_from(word_texts).unwrap();
            let path = PathBuf::from(path_text);
            let path_spelling_entry = (path, spellings);
            let path_spelling_list = [path_spelling_entry];
            let path_spellings = BTreeMap::from(path_spelling_list);
            SourceSpellings::from(path_spellings)
        }
    }

    #[test]
    fn a_context_is_identified_by_its_directory() {
        let language = Language::default();
        let directory_name = "harbor";
        let context = Context::from((directory_name, language));

        let identity = context.identity();

        let path = identity.path();
        let expectation_text = "crates/harbor";
        let expectation = Path::new(expectation_text);
        assert_eq!(path, expectation);
    }

    #[test]
    fn the_root_of_a_context_named_by_its_directory_alone_is_crates() {
        let language = Language::default();
        let directory_name = "harbor";
        let context = Context::from((directory_name, language));

        let directory = context.directory();

        let path = directory.path();
        let expectation_text = "crates/harbor";
        let expectation = Path::new(expectation_text);
        assert_eq!(path, expectation);
    }

    #[test]
    fn a_context_names_the_foreign_language_of_its_directory_and_every_foreign_name_it_is_given() {
        let directory_name = "harbor";
        let term_texts = fixture::TERM_TEXTS;
        let parser_text = "clap";
        let decoder_text = "toml";
        let parser_name = ForeignName::from(parser_text);
        let decoder_name = ForeignName::from(decoder_text);
        let context_foreign_names = vec![parser_name, decoder_name];
        let context = Context::try_from((directory_name, term_texts, context_foreign_names))
            .unwrap();

        let foreign_names = context.foreign_names();

        let directory_foreign_name = ForeignName::from(directory_name);
        let parser_expectation = ForeignName::from(parser_text);
        let decoder_expectation = ForeignName::from(decoder_text);
        let expectation = [
            directory_foreign_name,
            parser_expectation,
            decoder_expectation,
        ];
        assert_eq!(foreign_names, expectation);
    }

    #[test]
    fn a_context_of_term_texts_holds_the_terms_of_those_texts() {
        let directory_name = "harbor";
        let term_texts = fixture::TERM_TEXTS;
        let foreign_names = Vec::new();

        let context = Context::try_from((directory_name, term_texts, foreign_names)).unwrap();

        let language = context.language();
        let terms = language.terms();
        let spelling_texts: Vec<String> = terms.into_iter().map(ToString::to_string).collect();
        let expectation = ["Treasure"];
        assert_eq!(spelling_texts, expectation);
    }

    #[test]
    fn a_context_of_a_term_text_that_is_no_term_is_an_error() {
        const TERM_TEXT: TermText = {
            let spelling: &[&str] = &[];
            let definition = "A thing.";
            TermText::new(spelling, definition)
        };
        let directory_name = "harbor";
        let term_text = TERM_TEXT;
        let term_texts: &[TermText] = &[term_text];
        let foreign_names = Vec::new();

        let context = Context::try_from((directory_name, term_texts, foreign_names));

        let spelling_error = spelling::Error::Term;
        let term_error = term::Error::Spelling {
            term_text,
            error: spelling_error,
        };
        let language_error = gates_vocabulary_language_core_language::Error::Term {
            error: term_error,
        };
        let error = Error::from(language_error);
        let expectation = Err(error);
        assert_eq!(context, expectation);
    }

    #[test]
    fn a_term_no_source_of_its_context_spells_is_unused() {
        let directory_name = "harbor";
        let term_texts = fixture::TERM_TEXTS;
        let foreign_names = Vec::new();
        let context = Context::try_from((directory_name, term_texts, foreign_names)).unwrap();
        let path_text = "crates/harbor/src/lib.rs";
        let source_spellings = fixture::source_spellings_factory(path_text);

        let unused_terms = context.select_unused_terms(&source_spellings);

        let unused_term_texts: Vec<String> = unused_terms
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["Treasure"];
        assert_eq!(unused_term_texts, expectation);
    }

    #[test]
    fn the_terms_of_a_context_no_source_lies_in_are_not_judged() {
        let directory_name = "harbor";
        let term_texts = fixture::TERM_TEXTS;
        let foreign_names = Vec::new();
        let context = Context::try_from((directory_name, term_texts, foreign_names)).unwrap();
        let path_text = "crates/ship/src/lib.rs";
        let source_spellings = fixture::source_spellings_factory(path_text);

        let unused_terms = context.select_unused_terms(&source_spellings);

        let expectation = UnusedTerms::default();
        assert_eq!(unused_terms, expectation);
    }

    #[test]
    fn a_context_of_a_root_and_a_directory_name_lies_in_that_root() {
        let root_text = "products";
        let root = Path::new(root_text);
        let directory_name = "harbor";
        let language = Language::default();

        let context = Context::from((root, directory_name, language));

        let path = context.directory().path();
        let foreign_names = context.foreign_names();
        let path_expectation_text = "products/harbor";
        let path_expectation = Path::new(path_expectation_text);
        let foreign_name_expectation = [ForeignName::from(directory_name)];
        assert_eq!(path, path_expectation);
        assert_eq!(foreign_names, foreign_name_expectation);
    }
}
