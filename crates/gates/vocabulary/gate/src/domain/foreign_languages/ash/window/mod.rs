use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(in crate::domain::foreign_languages) struct AshWindow;

impl Layer for AshWindow {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for AshWindow {}

impl AshWindow {
    const NAME: &'static str = "ash-window";

    const NAMES: &'static [&'static str] = &["ash_window", "create_surface"];
}

impl TryFrom<AshWindow> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: AshWindow) -> Result<Self, Self::Error> {
        let foreign_name_text = AshWindow::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = AshWindow::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_ash_window_is_a_name() {
        let ash_window = AshWindow;

        let foreign_language = ForeignLanguage::try_from(ash_window);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
