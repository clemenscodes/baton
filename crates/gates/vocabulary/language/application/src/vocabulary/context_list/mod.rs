use crate::Error;
use crate::vocabulary::ForeignLanguageList;
use gates_vocabulary_language_core_languages::Languages;
use gates_vocabulary_language_core_languages::contexts::Context;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct ContextList {
    contexts: Vec<Context>,
}

impl Layer for ContextList {
    type Layer = ApplicationLayer;
}

impl ContextList {
    pub(crate) fn try_compose_languages(
        self,
        foreign_language_list: ForeignLanguageList,
    ) -> Result<Languages, Error> {
        let Self { contexts } = self;
        let foreign_languages = foreign_language_list.into_iter().collect();
        Languages::try_from((contexts, foreign_languages)).map_err(Error::from)
    }
}

impl FromIterator<Context> for ContextList {
    fn from_iter<ContextsType: IntoIterator<Item = Context>>(
        context_sequence: ContextsType,
    ) -> Self {
        let contexts = context_sequence.into_iter().collect();
        Self { contexts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_core_language::Language;

    #[test]
    fn the_contexts_of_a_list_compose_the_languages_of_their_directories() {
        let language = Language::default();
        let directory_name = "harbor";
        let context = Context::from((directory_name, language));
        let context_sequence = [context];
        let context_list: ContextList = context_sequence.into_iter().collect();
        let foreign_language_list = ForeignLanguageList::default();

        let languages = context_list.try_compose_languages(foreign_language_list);

        let directory_texts: Vec<String> = languages
            .unwrap()
            .contexts()
            .into_iter()
            .map(|context| context.directory().path().display().to_string())
            .collect();
        let expectation = ["crates/harbor"];
        assert_eq!(directory_texts, expectation);
    }
}
