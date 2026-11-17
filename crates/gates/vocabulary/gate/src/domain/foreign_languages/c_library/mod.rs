use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct CLibrary;

impl Layer for CLibrary {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for CLibrary {}

impl CLibrary {
    const NAME: &'static str = "libc";

    const NAMES: &'static [&'static str] = &["libc", "syscall", "SYS_bpf", "unsafe"];
}

impl TryFrom<CLibrary> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: CLibrary) -> Result<Self, Self::Error> {
        let foreign_name_text = CLibrary::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = CLibrary::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_c_library_is_a_name() {
        let c_library = CLibrary;

        let foreign_language = ForeignLanguage::try_from(c_library);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
