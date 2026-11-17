mod attribute_visitor;
mod code_occurrences;
mod declaration_visitor;
mod error;
mod identifier_tokens;
mod production_tokens;
mod source_occurrences;
mod test_attribute_presence;

use crate::attribute_visitor::AttributeVisitor;
use crate::declaration_visitor::DeclarationVisitor;
use crate::error::Error;
use crate::identifier_tokens::IdentifierTokens;
use crate::production_tokens::ProductionTokens;
use crate::source_occurrences::SourceOccurrences;
use crate::test_attribute_presence::TestAttributePresence;
use gates_vocabulary_language_core_language::code::tokens::CodeTokens;
use gates_vocabulary_language_core_language::parser::Parser;
use gates_vocabulary_language_core_language::source::{Source, SourceText};
use proc_macro2::{LineColumn, TokenStream};
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::io::{self, ErrorKind};
use std::str::FromStr;
use syn::visit::Visit;
use syn::{File, Item, ItemMod};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct Syntax;

impl Layer for Syntax {
    type Layer = InfrastructureLayer;
}

impl Adapter for Syntax {}

impl Port for Syntax {}

impl DrivenPort for Syntax {}

impl Syntax {
    const TEST_MODULE_NAME: &'static str = "tests";
    const TEXT_END: LineColumn = LineColumn {
        line: usize::MAX,
        column: usize::MAX,
    };

    fn try_compose_source(&self, source_text: &SourceText) -> Result<Source, Error> {
        let text = source_text.text();
        let token_stream = TokenStream::from_str(text)?;
        let file_token_stream = token_stream.clone();
        let file: File = syn::parse2(file_token_stream)?;
        let source = self.compose_source(&file, token_stream);
        Ok(source)
    }

    fn compose_source(&self, file: &File, token_stream: TokenStream) -> Source {
        let production_end = self.measure_production_end(file);
        let mut attribute_visitor = AttributeVisitor::from(production_end);
        attribute_visitor.visit_file(file);
        let mut declaration_visitor = DeclarationVisitor::default();
        declaration_visitor.visit_file(file);
        let declaration_starts = declaration_visitor.starts();
        let mut production_tokens = ProductionTokens::from((production_end, declaration_starts));
        let production_token_stream = token_stream.clone();
        production_tokens.insert_tokens(production_token_stream);
        let identifier_tokens = IdentifierTokens::from(token_stream);
        let code_tokens = CodeTokens::from(production_tokens);
        let mut source_occurrences = SourceOccurrences::from((
            production_end,
            &attribute_visitor,
            &declaration_visitor,
            code_tokens,
        ));
        for identifier_token in &identifier_tokens {
            source_occurrences.insert_token(identifier_token);
        }
        Source::from(source_occurrences)
    }

    fn measure_production_end(&self, file: &File) -> LineColumn {
        let File { items, .. } = file;
        let text_end = Self::TEXT_END;
        let test_module_name = Self::TEST_MODULE_NAME;
        let Some(Item::Mod(ItemMod {
            attrs: attributes,
            ident: identifier,
            content: Some(_),
            ..
        })) = items.last()
        else {
            return text_end;
        };
        if identifier != test_module_name {
            return text_end;
        }
        for attribute in attributes {
            let test_attribute_presence = TestAttributePresence::from(attribute);
            match test_attribute_presence {
                TestAttributePresence::Presence { start_position } => return start_position,
                TestAttributePresence::Absence => {}
            }
        }
        text_end
    }
}

impl Parser for Syntax {
    fn try_parse_source(&self, source_text: &SourceText) -> io::Result<Source> {
        let source = self.try_compose_source(source_text);
        let error_kind = ErrorKind::InvalidData;
        let syntax_error = |error| io::Error::new(error_kind, error);
        source.map_err(syntax_error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gates_vocabulary_language_core_language::defects::Defects;

    #[test]
    fn a_name_the_code_uses_is_an_identifier_of_the_source() {
        let syntax = Syntax;
        let text = "fn harbor() { let anchor = chain.length(); }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let identifiers = production_code.identifiers();
        let identifier_texts: Vec<String> = identifiers
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:1: `fn`",
            "1:15: `let`",
            "1:28: `chain`",
            "1:34: `length`",
        ];
        assert_eq!(identifier_texts, expectation);
    }

    #[test]
    fn the_production_tokens_of_a_source_keep_their_order_and_leave_its_prose_out() {
        let syntax = Syntax;
        let text = r#"fn harbor() -> u8 {
    let anchor = 0xFF;
    let text = "the written word";
    anchor
}

#[cfg(test)]
mod tests {
    fn written() {}
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_tokens = source.production_tokens();
        let token_texts: Vec<String> = production_tokens
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1\t1\tidentifier\tfn",
            "1\t4\tdeclaration\tharbor",
            "1\t10\tpunctuation\t(",
            "1\t11\tpunctuation\t)",
            "1\t13\tpunctuation\t-",
            "1\t14\tpunctuation\t>",
            "1\t16\tidentifier\tu8",
            "1\t19\tpunctuation\t{",
            "2\t5\tidentifier\tlet",
            "2\t9\tdeclaration\tanchor",
            "2\t16\tpunctuation\t=",
            "2\t18\tliteral\t0xFF",
            "2\t22\tpunctuation\t;",
            "3\t5\tidentifier\tlet",
            "3\t9\tdeclaration\ttext",
            "3\t14\tpunctuation\t=",
            "3\t34\tpunctuation\t;",
            "4\t5\tidentifier\tanchor",
            "5\t1\tpunctuation\t}",
        ];
        assert_eq!(token_texts, expectation);
    }

    #[test]
    fn a_bracket_of_the_code_is_punctuation_of_its_tokens() {
        let syntax = Syntax;
        let text = "const ANCHORS: [u8; 2] = [0, 1];";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_tokens = source.production_tokens();
        let token_count = production_tokens.into_iter().count();
        let expectation: usize = 15;
        assert_eq!(token_count, expectation);
    }

    #[test]
    fn a_name_the_code_declares_is_a_declaration_of_the_source() {
        let syntax = Syntax;
        let text = "fn harbor() { let anchor = chain.length(); }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:4: `harbor`", "1:19: `anchor`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn an_identifier_is_kept_as_the_source_writes_it() {
        let syntax = Syntax;
        let text = "struct JsonReport;";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:8: `JsonReport`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn every_kind_of_item_declares_its_name() {
        let syntax = Syntax;
        let text = r#"mod harbor {}
#[derive(Clone)]
struct Quay { anchor: Chain }
struct Knot(Rope);
extern crate rope;
enum Tide { Flood }
union Cargo { crate_weight: Weight }
type Mooring = Rope;
const DEPTH: Fathom = Fathom;
static WATER: Sea = Sea;
trait Keel = Hull;
macro_rules! signal { () => {} }
extern crate lantern as beacon;
use pier::Dock as Berth;
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:5: `harbor`",
            "3:8: `Quay`",
            "3:15: `anchor`",
            "4:8: `Knot`",
            "6:6: `Tide`",
            "6:13: `Flood`",
            "7:7: `Cargo`",
            "7:15: `crate_weight`",
            "8:6: `Mooring`",
            "9:7: `DEPTH`",
            "10:8: `WATER`",
            "11:7: `Keel`",
            "12:14: `signal`",
            "13:25: `beacon`",
            "14:19: `Berth`",
        ];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_trait_and_an_impl_of_no_trait_declare_the_names_of_their_items() {
        let syntax = Syntax;
        let text = r#"trait Vessel { fn sail(); const HULL: Wood; type Mast; }
impl Ship { fn steer() {} const RUDDER: Iron = Iron; type Canvas = Cloth; signal!(); }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:7: `Vessel`",
            "1:19: `sail`",
            "1:33: `HULL`",
            "1:50: `Mast`",
            "2:16: `steer`",
            "2:33: `RUDDER`",
            "2:59: `Canvas`",
        ];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn the_items_of_an_impl_of_a_trait_are_names_the_trait_declares() {
        let syntax = Syntax;
        let text = r#"impl Display for Ship { fn fmt(&self, formatter: Formatter) {} const HULL: Wood = Wood; type Mast = Pine; signal!(); }
trait Vessel { signal!(); }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:39: `formatter`", "2:7: `Vessel`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn the_generic_parameters_the_labels_and_the_bindings_are_declarations() {
        let syntax = Syntax;
        let text = r#"fn sail<'voyage, Cargo, const LENGTH: usize>(crew: Crew) {
    'route: loop {}
    let wind = |gust| gust;
    match tide { None => {}, Some(current) => {} }
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:4: `sail`",
            "1:10: `voyage`",
            "1:18: `Cargo`",
            "1:31: `LENGTH`",
            "1:46: `crew`",
            "2:6: `route`",
            "3:9: `wind`",
            "3:17: `gust`",
            "4:35: `current`",
        ];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_capitalized_binding_of_a_let_a_parameter_a_closure_or_a_loop_is_a_declaration() {
        let syntax = Syntax;
        let text = r#"fn sail(Crew: Hand) {
    let Vec = 1;
    let Rope: Knot = 2;
    let wind = |Gust| Gust;
    for Wave in tide {}
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:4: `sail`",
            "1:9: `Crew`",
            "2:9: `Vec`",
            "3:9: `Rope`",
            "4:9: `wind`",
            "4:17: `Gust`",
            "5:9: `Wave`",
        ];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_capitalized_name_inside_a_pattern_is_a_use_unless_a_marker_or_a_subpattern_binds_it() {
        let syntax = Syntax;
        let text = r#"fn sail() {
    match tide { None => {}, Some(ref Current) => {}, Wave @ Some(_) => {}, Some(mut Swell) => {} }
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "1:4: `sail`",
            "2:39: `Current`",
            "2:55: `Wave`",
            "2:86: `Swell`",
        ];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn the_items_of_a_foreign_block_declare_their_names() {
        let syntax = Syntax;
        let text = r#"extern "C" { fn hoist(); static FLAG: Cloth; type Pennant; }"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:17: `hoist`", "1:33: `FLAG`", "1:51: `Pennant`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn the_function_main_is_a_name_rust_declares() {
        let syntax = Syntax;
        let text = "fn main() {}";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_count = declarations.into_iter().count();
        let expectation: usize = 0;
        assert_eq!(declaration_count, expectation);
    }

    #[test]
    fn the_constant_constructor_new_is_a_name_docs_style_declares() {
        let syntax = Syntax;
        let text = "impl Harbor { const fn new() {} fn new_anchor() {} const fn hoist() {} }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:36: `new_anchor`", "1:61: `hoist`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_raw_identifier_is_the_identifier_it_names() {
        let syntax = Syntax;
        let text = "fn r#harbor() {}";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:4: `harbor`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn the_suffix_of_a_literal_is_an_identifier_and_the_literal_is_none() {
        let syntax = Syntax;
        let text = r#"signal!(1zonk 1.5flarn "text" 'a' 20);"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let identifiers = production_code.identifiers();
        let identifier_texts: Vec<String> = identifiers
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:1: `signal`", "1:9: `zonk`", "1:15: `flarn`"];
        assert_eq!(identifier_texts, expectation);
    }

    #[test]
    fn the_test_module_at_the_bottom_is_no_part_of_the_production_code() {
        let syntax = Syntax;
        let text = r#"struct Harbor;

#[cfg(test)]
mod tests { fn unlisted() {} }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let identifiers = production_code.identifiers();
        let identifier_texts: Vec<String> = identifiers
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:1: `struct`"];
        assert_eq!(identifier_texts, expectation);
    }

    #[test]
    fn the_test_module_at_the_bottom_names_the_identifiers_of_the_test_code() {
        let syntax = Syntax;
        let text = r#"struct Harbor;
#[cfg(test)]
mod tests { use super::*; }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let test_code = source.test_code();
        let identifiers = test_code.identifiers();
        let identifier_texts: Vec<String> = identifiers
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = [
            "2:3: `cfg`",
            "2:7: `test`",
            "3:1: `mod`",
            "3:13: `use`",
            "3:17: `super`",
        ];
        assert_eq!(identifier_texts, expectation);
    }

    #[test]
    fn the_test_module_at_the_bottom_declares_the_identifiers_of_the_test_code() {
        let syntax = Syntax;
        let text = r#"struct Harbor;
#[cfg(test)]
mod tests { fn anchor_factory() { let anchor = 1; } }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let test_code = source.test_code();
        let declarations = test_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["3:5: `tests`", "3:16: `anchor_factory`", "3:39: `anchor`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn the_name_of_a_test_function_is_neither_named_nor_declared() {
        let syntax = Syntax;
        let text = r#"#[cfg(test)]
mod tests {
    #[test]
    fn a_harbor_holds_a_ship() {}
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let test_code = source.test_code();
        let identifiers = test_code.identifiers();
        let declarations = test_code.declarations();
        let occurrences = identifiers.into_iter().chain(declarations);
        let occurrence_texts: Vec<String> = occurrences.map(ToString::to_string).collect();
        let expectation = [
            "1:3: `cfg`",
            "1:7: `test`",
            "2:1: `mod`",
            "3:7: `test`",
            "4:5: `fn`",
            "2:5: `tests`",
        ];
        assert_eq!(occurrence_texts, expectation);
    }

    #[test]
    fn the_name_of_a_test_function_of_the_browser_is_neither_named_nor_declared() {
        let syntax = Syntax;
        let text = r#"#[cfg(test)]
mod tests {
    #[wasm_bindgen_test]
    async fn a_harbor_holds_a_ship() {}
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let test_code = source.test_code();
        let identifiers = test_code.identifiers();
        let declarations = test_code.declarations();
        let occurrences = identifiers.into_iter().chain(declarations);
        let occurrence_texts: Vec<String> = occurrences.map(ToString::to_string).collect();
        let expectation = [
            "1:3: `cfg`",
            "1:7: `test`",
            "2:1: `mod`",
            "3:7: `wasm_bindgen_test`",
            "4:5: `async`",
            "4:11: `fn`",
            "2:5: `tests`",
        ];
        assert_eq!(occurrence_texts, expectation);
    }

    #[test]
    fn a_module_tests_without_the_test_attribute_is_a_part_of_the_source() {
        let syntax = Syntax;
        let text = "mod tests { struct Harbor; }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:5: `tests`", "1:20: `Harbor`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_whole_pattern_that_destructures_declares_the_identifiers_inside_it() {
        let syntax = Syntax;
        let text = "fn hoist(_: Rope) { let (anchor, Chain) = rope; }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:4: `hoist`", "1:26: `anchor`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_module_of_another_name_at_the_bottom_is_a_part_of_the_source() {
        let syntax = Syntax;
        let text = r#"struct Harbor;
#[cfg(unix)]
mod harbor { struct Quay; }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let production_code = source.production_code();
        let declarations = production_code.declarations();
        let declaration_texts: Vec<String> = declarations
            .into_iter()
            .map(ToString::to_string)
            .collect();
        let expectation = ["1:8: `Harbor`", "3:5: `harbor`", "3:21: `Quay`"];
        assert_eq!(declaration_texts, expectation);
    }

    #[test]
    fn a_test_attribute_on_anything_but_the_test_module_is_a_defect() {
        let syntax = Syntax;
        let text = r#"#[cfg(test)]
use std::fmt;
fn zonk_flarn() {}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "1:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn code_after_the_test_module_is_a_defect() {
        let syntax = Syntax;
        let text = r#"#[cfg(test)]
mod tests {}
fn zonk_flarn() {}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "1:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_test_attribute_inside_the_code_is_a_defect() {
        let syntax = Syntax;
        let text = r#"struct Harbor {
    #[cfg(test)]
    anchor: Chain,
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "2:5: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_test_attribute_of_the_whole_source_is_a_defect() {
        let syntax = Syntax;
        let text = r#"#![cfg(test)]
struct Harbor;
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "1:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_test_outside_the_test_module_is_a_defect() {
        let syntax = Syntax;
        let text = r#"struct Harbor;
#[test]
fn harbor() {}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "2:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_configuration_whose_condition_names_the_test_anywhere_is_a_defect() {
        let syntax = Syntax;
        let text = r#"#[cfg(not(test))]
fn harbor() {}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "1:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_conditional_attribute_whose_condition_names_the_test_is_a_defect() {
        let syntax = Syntax;
        let text = r#"#[cfg_attr(test, derive(Debug))]
struct Harbor;
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "1:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_module_tests_at_the_bottom_under_another_condition_than_the_test_is_a_defect() {
        let syntax = Syntax;
        let text = r#"struct Harbor;
#[cfg(any(test))]
mod tests {}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "2:1: the test attribute does not open the `tests` module at the bottom of the source",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_configuration_of_another_condition_is_no_test_attribute() {
        let syntax = Syntax;
        let text = r#"#[cfg(unix)]
struct Harbor;
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_expectation = Defects::default();
        let expectation = &defect_expectation;
        assert_eq!(defects, expectation);
    }

    #[test]
    fn a_doc_comment_is_a_defect() {
        let syntax = Syntax;
        let text = r#"struct Quay;
/// A harbor.
struct Harbor;
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "2:1: the `doc` attribute is a comment, and code must not contain comments",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_doc_comment_of_the_whole_source_is_a_defect() {
        let syntax = Syntax;
        let text = r#"//! A harbor.
struct Harbor;
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "1:1: the `doc` attribute is a comment, and code must not contain comments",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_doc_comment_inside_the_test_module_is_a_defect() {
        let syntax = Syntax;
        let text = r#"struct Harbor;
#[cfg(test)]
mod tests {
    /// A test.
    fn harbor() {}
}
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "4:5: the `doc` attribute is a comment, and code must not contain comments",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn an_identifier_with_a_piece_of_digits_is_a_defect() {
        let syntax = Syntax;
        let text = "fn harbor() { let letter_2 = 1; }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = ["1:19: `letter_2` is not an identifier made of words"];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn an_identifier_with_a_letter_outside_the_alphabet_of_english_is_a_defect() {
        let syntax = Syntax;
        let text = "fn harbor() { let слово = 1; }";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = ["1:19: `слово` is not an identifier made of words"];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_source_of_several_defects_holds_every_one_of_them() {
        let syntax = Syntax;
        let text = r#"/// A harbor.
#[test]
fn harbor() { let letter_2 = 1; }
"#;
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text).unwrap();

        let defects = source.defects();
        let defect_texts: Vec<String> = defects.into_iter().map(ToString::to_string).collect();
        let expectation = [
            "2:1: the test attribute does not open the `tests` module at the bottom of the source",
            "1:1: the `doc` attribute is a comment, and code must not contain comments",
            "3:19: `letter_2` is not an identifier made of words",
        ];
        assert_eq!(defect_texts, expectation);
    }

    #[test]
    fn a_text_that_is_no_rust_is_an_error() {
        let syntax = Syntax;
        let text = "fn harbor( {";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text);

        let error = source.unwrap_err();
        let error_text = error.to_string();
        let expectation = "1:12: the source is not valid Rust: cannot parse string into token stream";
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_sequence_of_tokens_that_is_no_item_is_an_error() {
        let syntax = Syntax;
        let text = "struct Harbor; 1";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text);

        let error = source.unwrap_err();
        let error_text = error.to_string();
        let expectation = r#"1:16: the source is not valid Rust: expected one of: `fn`, `extern`, `use`, `static`, `const`, `unsafe`, `mod`, `type`, `struct`, `enum`, `union`, `impl`, `trait`, `auto`, `default`, `macro`, identifier, `self`, `super`, `crate`, `::`"#;
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_raw_literal_without_its_quote_is_an_error() {
        let syntax = Syntax;
        let text = "const TEXT: usize = 1; r##";
        let source_text = SourceText::from(text);

        let source = syntax.try_parse_source(&source_text);

        let error = source.unwrap_err();
        let error_kind = error.kind();
        let expectation = ErrorKind::InvalidData;
        assert_eq!(error_kind, expectation);
    }
}
