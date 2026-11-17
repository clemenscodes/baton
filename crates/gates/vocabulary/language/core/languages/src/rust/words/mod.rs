use gates_vocabulary_language_core_language::Error;
use gates_vocabulary_language_core_language::Language;
use gates_vocabulary_language_core_language::vocabulary::{Spellings, Vocabulary};
use stack_domain_driven_design_core::factory::Factory;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Words {
    word_texts: &'static [&'static str],
}

impl Layer for Words {
    type Layer = DomainLayer;
}

impl Factory<Language> for Words {
    type Blueprint<'blueprint> = ();
    type Error = Error;

    fn try_create_product(&self, _: ()) -> Result<Language, Error> {
        let Self { word_texts } = self;
        let rust_word_texts: &[&str] = word_texts;
        let spellings = Spellings::try_from(rust_word_texts)?;
        let vocabulary = Vocabulary::from(spellings);
        let language = Language::from(vocabulary);
        Ok(language)
    }
}

impl Words {
    const WORDS: &'static [&'static str] = &["try", "type"];
}

impl From<&'static [&'static str]> for Words {
    fn from(word_texts: &'static [&'static str]) -> Self {
        Self { word_texts }
    }
}

impl From<()> for Words {
    fn from(_: ()) -> Self {
        let word_texts = Self::WORDS;
        Self::from(word_texts)
    }
}
