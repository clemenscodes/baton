use gates_vocabulary_language_core_language::Error;
use gates_vocabulary_language_core_language::Language;
use stack_domain_driven_design_core::factory::Factory;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Keywords {
    name_texts: &'static [&'static str],
}

impl Layer for Keywords {
    type Layer = DomainLayer;
}

impl Factory<Language> for Keywords {
    type Blueprint<'blueprint> = ();
    type Error = Error;

    fn try_create_product(&self, _: ()) -> Result<Language, Error> {
        let Self { name_texts } = self;
        let keyword_texts: &[&str] = name_texts;
        Language::try_from(keyword_texts)
    }
}

impl Keywords {
    const NAMES: &'static [&'static str] = &[
        "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum",
        "extern", "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move",
        "mut", "pub", "ref", "return", "self", "static", "struct", "super", "trait", "true",
        "type", "use", "where", "while", "try", "bool", "char", "str", "f32", "f64", "i8", "i16",
        "i32", "i64", "i128", "isize", "u8", "u16", "u32", "u64", "u128", "usize",
    ];
}

impl From<&'static [&'static str]> for Keywords {
    fn from(name_texts: &'static [&'static str]) -> Self {
        Self { name_texts }
    }
}

impl From<()> for Keywords {
    fn from(_: ()) -> Self {
        let name_texts = Self::NAMES;
        Self::from(name_texts)
    }
}
