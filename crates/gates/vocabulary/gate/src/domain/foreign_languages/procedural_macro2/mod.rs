use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct ProceduralMacro2;

impl Layer for ProceduralMacro2 {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for ProceduralMacro2 {}

impl ProceduralMacro2 {
    const NAME: &'static str = "proc-macro2";

    const NAMES: &'static [&'static str] = &[
        "as_char",
        "Brace",
        "Bracket",
        "call_site",
        "column",
        "delimiter",
        "Delimiter",
        "Group",
        "Ident",
        "join",
        "LexError",
        "line",
        "LineColumn",
        "Literal",
        "Parenthesis",
        "proc_macro2",
        "Punct",
        "span",
        "Span",
        "span_close",
        "span_open",
        "start",
        "stream",
        "TokenStream",
        "TokenTree",
    ];
}

impl TryFrom<ProceduralMacro2> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: ProceduralMacro2) -> Result<Self, Self::Error> {
        let foreign_name_text = ProceduralMacro2::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = ProceduralMacro2::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_the_procedural_macro2_is_a_name() {
        let procedural_macro2 = ProceduralMacro2;

        let foreign_language = ForeignLanguage::try_from(procedural_macro2);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
