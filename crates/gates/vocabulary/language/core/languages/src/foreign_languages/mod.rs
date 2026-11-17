mod foreign_language;

pub use foreign_language::ForeignLanguage;

use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_language::Language;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct ForeignLanguages {
    foreign_languages: Vec<ForeignLanguage>,
}

impl Layer for ForeignLanguages {
    type Layer = DomainLayer;
}

impl ValueObject for ForeignLanguages {}

impl ForeignLanguages {
    pub(super) fn compose_language(&self, foreign_names: &[ForeignName]) -> Language {
        let Self { foreign_languages } = self;
        let mut language = Language::default();
        for foreign_name in foreign_names {
            for foreign_language in foreign_languages {
                let language_name = foreign_language.foreign_name();
                if language_name == foreign_name {
                    language = &language | foreign_language.language();
                }
            }
        }
        language
    }
}

impl FromIterator<ForeignLanguage> for ForeignLanguages {
    fn from_iter<ForeignLanguagesType: IntoIterator<Item = ForeignLanguage>>(
        foreign_language_sequence: ForeignLanguagesType,
    ) -> Self {
        let foreign_languages = foreign_language_sequence.into_iter().collect();
        Self { foreign_languages }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::fixture as source_fixture;

    #[test]
    fn the_language_of_foreign_names_is_the_language_of_the_foreign_languages_they_name() {
        let parser_name_texts: &[&str] = &["Parser"];
        let decoder_name_texts: &[&str] = &["Deserialize"];
        let parser_text = "clap";
        let decoder_text = "toml";
        let parser_foreign_name = ForeignName::from(parser_text);
        let decoder_foreign_name = ForeignName::from(decoder_text);
        let parser_language = ForeignLanguage::try_from((parser_foreign_name, parser_name_texts))
            .unwrap();
        let decoder_language = ForeignLanguage::try_from((
            decoder_foreign_name,
            decoder_name_texts,
        ))
        .unwrap();
        let foreign_language_list = [parser_language, decoder_language];
        let foreign_languages: ForeignLanguages = foreign_language_list.into_iter().collect();
        let parser_name = ForeignName::from(parser_text);
        let foreign_names = [parser_name];
        let identifier_texts: &[&str] = &["Parser", "Deserialize"];
        let source = source_fixture::identifier_source_factory(identifier_texts);
        let production_code = source.production_code();

        let language = foreign_languages.compose_language(&foreign_names);

        let vocabulary = language.vocabulary();
        let unknown_vocabulary = vocabulary.select_unknown_vocabulary(production_code);
        let unknown_occurrence_texts: Vec<String> = unknown_vocabulary
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:2: `Deserialize`"];
        assert_eq!(unknown_occurrence_texts, expectation);
    }
}
