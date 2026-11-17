mod error;
mod test_module_visitor;

use error::Error;

use crate::test_module_visitor::TestModuleVisitor;
use ::syn::File;
use ::syn::visit::Visit;
use gates_coverage_core_source::Source;
use gates_coverage_core_test_boundaries::Parser;
use gates_coverage_core_test_boundaries::TestBoundaries;
use proc_macro2::TokenStream;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::str::FromStr;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Syntax;

impl Layer for Syntax {
    type Layer = InfrastructureLayer;
}

impl Adapter for Syntax {}

impl Port for Syntax {}

impl DrivenPort for Syntax {}

impl Parser for Syntax {
    type Error = Error;

    fn try_parse_test_boundaries(&self, source: &Source) -> Result<TestBoundaries, Self::Error> {
        let path = source.path();
        let text = source.text();
        let source_tokens = TokenStream::from_str(text).map_err(::syn::Error::from);
        let syntax_error = |error| Error::from((path, error));
        let file: File = source_tokens
            .and_then(::syn::parse2)
            .map_err(syntax_error)?;
        let mut test_module_visitor = TestModuleVisitor::from(path);
        test_module_visitor.visit_file(&file);
        let test_boundaries = Result::from_iter(test_module_visitor);
        test_boundaries.map_err(Error::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    mod fixture {
        use super::*;

        pub(super) fn test_boundary_factory()
        -> gates_coverage_core_test_boundaries::test_boundary::TestBoundary {
            let path_text = "crates/harbor/src/lib.rs";
            let path = std::path::PathBuf::from(path_text);
            let opening_line: usize = 2;
            let closing_line: usize = 5;
            gates_coverage_core_test_boundaries::test_boundary::TestBoundary::try_from((
                path,
                opening_line,
                closing_line,
            ))
            .unwrap()
        }

        pub(super) fn source_factory(text: &str) -> Source {
            let path_text = "crates/harbor/src/lib.rs";
            let path = PathBuf::from(path_text);
            let source_text = String::from(text);
            Source::from((path, source_text))
        }

        pub(super) fn test_boundaries_factory() -> TestBoundaries {
            let test_boundary = fixture::test_boundary_factory();
            let test_boundary_sequence = [test_boundary];
            TestBoundaries::from_iter(test_boundary_sequence)
        }
    }

    #[test]
    fn the_test_module_a_test_attribute_stands_on_runs_from_the_attribute_to_its_closing_brace() {
        let text = r#"struct Treasure;
#[cfg(test)]
mod tests {
    fn chest() {}
}
struct Chest;
"#;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = fixture::test_boundaries_factory();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_published_test_module_is_a_test_module() {
        let text = r#"struct Treasure;
#[cfg(test)]
pub(crate) mod tests {
    fn chest() {}
}
"#;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = fixture::test_boundaries_factory();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_test_module_inside_a_module_is_a_test_module() {
        let text = r#"mod harbor {
    #[cfg(test)]
    mod tests {
        fn chest() {}
    }
}
"#;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = fixture::test_boundaries_factory();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_brace_in_a_literal_or_a_comment_closes_no_test_module() {
        let text = r#"struct Treasure;
#[cfg(test)]
mod tests {
    const TREASURE: &str = "}"; const CHEST: char = '}'; /* } */
}
struct Chest;
"#;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = fixture::test_boundaries_factory();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_test_attribute_on_anything_but_an_inline_module_tests_opens_no_test_module() {
        let text = r#"#[cfg(test)]
use std::fmt;
#[cfg(test)]
pub(crate) struct Treasure;
#[cfg(test)]
mod chest {}
#[cfg(test)]
mod tests;
"#;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = TestBoundaries::default();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_module_tests_without_the_test_attribute_is_no_test_module() {
        let text = r#"#[doc = "treasure"]
#[doc(hidden)]
#[cfg(feature = "chest")]
mod tests {}
"#;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = TestBoundaries::default();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_test_attribute_in_a_literal_or_a_comment_opens_no_test_module() {
        let text = r##"const TREASURE: &str = r#"
#[cfg(test)]
mod tests {}
"#;
/* #[cfg(test)]
mod tests {} */
"##;
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source).unwrap();

        let expectation = TestBoundaries::default();
        assert_eq!(test_boundaries, expectation);
    }

    #[test]
    fn a_text_that_is_no_rust_is_an_error() {
        let text = "fn harbor( {";
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source);

        let error_text = test_boundaries.unwrap_err().to_string();
        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:1:12: the source is not valid Rust: cannot parse string into token stream.";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_sequence_of_tokens_that_is_no_item_is_an_error() {
        let text = "struct Harbor; 1";
        let source = fixture::source_factory(text);
        let syntax = Syntax;

        let test_boundaries = syntax.try_parse_test_boundaries(&source);

        let error_text = test_boundaries.unwrap_err().to_string();
        let expectation = "[COVERAGE] crates/harbor/src/lib.rs:1:16: the source is not valid Rust: expected one of: `fn`, `extern`, `use`, `static`, `const`, `unsafe`, `mod`, `type`, `struct`, `enum`, `union`, `impl`, `trait`, `auto`, `default`, `macro`, identifier, `self`, `super`, `crate`, `::`.";
        assert_eq!(error_text, expectation);
    }
}
