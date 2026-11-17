use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ForeignLanguageList {
    foreign_languages: Vec<ForeignLanguage>,
}

impl Layer for ForeignLanguageList {
    type Layer = ApplicationLayer;
}

impl FromIterator<ForeignLanguage> for ForeignLanguageList {
    fn from_iter<ForeignLanguagesType: IntoIterator<Item = ForeignLanguage>>(
        foreign_language_sequence: ForeignLanguagesType,
    ) -> Self {
        let foreign_languages = foreign_language_sequence.into_iter().collect();
        Self { foreign_languages }
    }
}

impl IntoIterator for ForeignLanguageList {
    type Item = ForeignLanguage;
    type IntoIter = IntoIter<ForeignLanguage>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { foreign_languages } = self;
        foreign_languages.into_iter()
    }
}
