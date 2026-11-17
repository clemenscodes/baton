use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct GatesCoverageCoreCompilerContext;

impl Layer for GatesCoverageCoreCompilerContext {
    type Layer = DomainLayer;
}

impl GatesCoverageCoreCompilerContext {
    const DIRECTORY_NAME: &'static str = "gates/coverage/core/compiler";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Gates", "Coverage", "Core", "Compiler"];
            let definition = "The crate gates_coverage_core_compiler, the crate of the domain of the coverage that holds the compiler, which composes the invocation that names its library.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Name"];
            let definition = "The name rustc of the compiler.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compiler"];
            let definition = "The compiler of Rust, rustc, which names the directory of its library, beside which the profile tools lie.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library", "Query"];
            let definition = "The arguments --print target-libdir, with which the compiler names the directory of its library.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Library", "Invocation"];
            let definition = "To compose the invocation that asks the compiler for the directory of its library.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Library"];
            let definition = "The library of the compiler for its host, whose directory the compiler names on request.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<GatesCoverageCoreCompilerContext> for Context {
    type Error = Error;

    fn try_from(_: GatesCoverageCoreCompilerContext) -> Result<Self, Self::Error> {
        let directory_name = GatesCoverageCoreCompilerContext::DIRECTORY_NAME;
        let term_texts = GatesCoverageCoreCompilerContext::TERM_TEXTS;
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
    fn every_definition_of_the_gates_coverage_core_compiler_context_is_a_term() {
        let gates_coverage_core_compiler_context = GatesCoverageCoreCompilerContext;

        let context = Context::try_from(gates_coverage_core_compiler_context);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
