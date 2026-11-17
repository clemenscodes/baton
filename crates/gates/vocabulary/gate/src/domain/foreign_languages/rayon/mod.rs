use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Rayon;

impl Layer for Rayon {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Rayon {}

impl Rayon {
    const NAME: &'static str = "rayon";

    const NAMES: &'static [&'static str] = &[
        "build",
        "flat_map_iter",
        "install",
        "into_par_iter",
        "IndexedParallelIterator",
        "IntoParallelIterator",
        "IntoParallelRefIterator",
        "par_chunks_mut",
        "par_iter",
        "par_sort",
        "par_sort_by",
        "par_sort_by_key",
        "par_sort_unstable",
        "ParallelIterator",
        "ParallelSliceMut",
        "rayon",
        "ThreadPoolBuilder",
    ];
}

impl TryFrom<Rayon> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Rayon) -> Result<Self, Self::Error> {
        let foreign_name_text = Rayon::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Rayon::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_rayon_is_a_name() {
        let rayon = Rayon;

        let foreign_language = ForeignLanguage::try_from(rayon);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
