use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesVocabularyLanguageAdapterSyntaxContext;

impl Layer for GatesVocabularyLanguageAdapterSyntaxContext {
    type Layer = DomainLayer;
}

impl GatesVocabularyLanguageAdapterSyntaxContext {
    const DIRECTORY_NAME: &'static str = "gates/vocabulary/language/adapter/syntax";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Vocabulary", "Language", "Adapter", "Syntax"];
            let definition = "The crate gates_vocabulary_language_adapter_syntax, the adapter of the parser over syn.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source", "Occurrences"];
            let definition = "The identifiers of a source, sorted into its production code and its tests, and the defects of its tokens.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Gate"];
            let definition = "The service that reads and parses the sources of a run and refuses a source where its text is no Rust, where its production code or its test code names or declares an unknown identifier or where it has a defect, and that refuses a context where its terms hold an unused term.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Infrastructure"];
            let definition = "The layer of the library whose adapters, the file system, the decoder over TOML and the parser over syn, fill its ports.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presence"];
            let definition = "The state of a directory that is there, of a manifest a directory holds and of an attribute that is exactly cfg(test).";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence"];
            let definition = "The state of a directory that is not there, of a manifest a directory does not hold and of an attribute that is anything else than cfg(test).";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Suffix"];
            let definition = "The end that follows the part of a text: the suffix separator and the process identifier that a temporary path carries behind the path of its file, and the name behind the digits or the quotes of a literal.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Syntax", "Literal"];
            let definition = "A literal of a token stream as syn reads it, together with its suffix.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token", "Stream"];
            let definition = "The tokens of a text or of a group, one after another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Token", "Tree"];
            let definition = "One token of a token stream.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Identifier", "Token"];
            let definition = "An identifier of a token stream, or the suffix of a literal, with the start of its token.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Source"];
            let definition = "To compose a source from the identifier tokens of its text, those in front of its production end into its production code and the others into its test code, apart from the tokens of its comments and of the names of its test functions, and from its defects.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Declaration", "Visitor"];
            let definition = "The visitor of the syntax of a source that inserts the start of every identifier the source declares and of the name of every test function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pattern"];
            let definition = "A pattern of Rust that binds or matches a value, such as the pattern of a let, a parameter, a closure or a loop.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Typed", "Pattern"];
            let definition = "The pattern inside a pattern with a type, such as the pattern of a parameter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Subpattern"];
            let definition = "The pattern behind the at sign of a binding, which makes the identifier in front of it a binding.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Reference", "Marker"];
            let definition = "The keyword ref in front of the identifier of a pattern, which makes the identifier a binding.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Mutability", "Marker"];
            let definition = "The keyword mut in front of the identifier of a pattern, which makes the identifier a binding.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Declaration"];
            let definition = "To insert the start of a declared identifier into a declaration visitor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Binding"];
            let definition = "To insert the identifier that a let, a parameter, a closure or a loop binds as its whole pattern, whatever its case, as a declaration.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Signature"];
            let definition = "To insert the name of a function, which its signature holds, unless it is the constant constructor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Implementation", "Item"];
            let definition = "To insert the name of an implementation item.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Trait", "Item"];
            let definition = "To insert the name of a trait item.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Implementation", "Item"];
            let definition = "An item of an impl block: a function, a constant or a type, whose name the trait it implements declares where it implements one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Trait", "Item"];
            let definition = "An item of a trait: a function, a constant or a type.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Start"];
            let definition = "The function main a program of Rust starts with, whose name Rust declares.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Attribute", "Visitor"];
            let definition = "The visitor of the syntax of a source that composes a defect of every test attribute in front of its production end and of every comment, and keeps the start of every comment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Count", "Test", "Conditions"];
            let definition = "To count how often the condition of an attribute cfg or cfg_attr names test, which makes that attribute a test attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Module"];
            let definition = "The module tests at the bottom of a source that holds its tests, whose code is the test code of the source.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Production", "End"];
            let definition = "The start of the attribute cfg(test) of the test module of a source, or the text end where a source has no test module, which separates its production code from its test code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Measure", "Production", "End"];
            let definition = "To measure the production end of a source from its last item, which is its test module where it is a module tests with the attribute cfg(test).";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Column", "Offset"];
            let definition = "A column of a line of a source counted from zero, as proc_macro2 counts it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Constant", "Constructor"];
            let definition = "The function new of a type that is a constant, the one constructor beside the conversions, whose name docs/STYLE.md declares.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Constant", "Marker"];
            let definition = "The keyword const in front of a function, which makes it callable at compile time.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Attribute", "Content"];
            let definition = "What an attribute holds behind its hash mark, its path and its tokens or its value.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The attribute cfg of Rust, which compiles what it stands on only under its condition.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Condition"];
            let definition = "The predicate of a configuration or of the attribute cfg_attr, under which Rust compiles what it stands on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hash", "Mark"];
            let definition = "The mark # an attribute opens with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Node"];
            let definition = "A part of the syntax of a source that a visitor visits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Signature"];
            let definition = "The name, the generic parameters, the parameters and the return type of a function.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Span"];
            let definition = "The location of a token in the text of a source, which its start states.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Syntax"];
            let definition = "The structure of the text of a source as Rust defines it, which a text that is no Rust lacks, and the adapter of the parser over the crate syn, which parses that structure.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group"];
            let definition = "A token tree of the tokens between a pair of delimiters.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Error", "Kind"];
            let definition = "The kind of an io::Error, such as the invalid data of a manifest that is no TOML.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Function"];
            let definition = "A function that carries the attribute test or, in the browser, wasm_bindgen_test, whose name is the sentence of its test and is not judged.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Test", "Function"];
            let definition = "To insert the start of the name of a test function into a declaration visitor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Token"];
            let definition = "To insert the occurrence of an identifier token into the code occurrences, as a declaration where a declaration starts at it and as an identifier everywhere else, and to refuse a token that is no occurrence.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Comment", "Defect"];
            let definition = "To insert the defect and the start of an attribute that is a comment.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Test", "Attribute"];
            let definition = "To insert the defect of a test attribute that stands in front of the end of the production code.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Code", "Occurrences"];
            let definition = "The occurrences of one part of a source while the syntax composes it, every identifier token sorted into the identifiers or the declarations of its code by the starts of the declarations.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Test", "Attribute", "Presence"];
            let definition = "The start of an attribute that is exactly cfg(test), or the absence of it in every other attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Punctuation", "Token"];
            let definition = "A sign of the code with the position it starts at, which turns into the punctuation among the production tokens.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Punctuation", "Token"];
            let definition = "To insert a punctuation token into the production tokens.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Insert", "Comment"];
            let definition = "To insert the comment defect of a comment attribute at its start position and to keep that start position among the comment starts.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesVocabularyLanguageAdapterSyntaxContext> for Context {
    type Error = Error;

    fn try_from(_: GatesVocabularyLanguageAdapterSyntaxContext) -> Result<Self, Self::Error> {
        let directory_name = GatesVocabularyLanguageAdapterSyntaxContext::DIRECTORY_NAME;
        let term_texts = GatesVocabularyLanguageAdapterSyntaxContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_gates_vocabulary_language_adapter_syntax_context_is_a_term() {
        let gates_vocabulary_language_adapter_syntax_context = GatesVocabularyLanguageAdapterSyntaxContext;

        let context = Context::try_from(gates_vocabulary_language_adapter_syntax_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
