use crate::identifier_tokens::IdentifierTokens;
use ::syn::visit::{self, Visit};
use ::syn::{Attribute, Meta, MetaList};
use gates_vocabulary_language_core_language::defects::defect::Defect;
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::collections::BTreeSet;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct AttributeVisitor {
    production_end: LineColumn,
    defects: Vec<Defect>,
    comment_starts: BTreeSet<LineColumn>,
}

impl Layer for AttributeVisitor {
    type Layer = InfrastructureLayer;
}

impl AttributeVisitor {
    const CONFIGURATION_NAME: &'static str = "cfg";
    const CONFIGURATION_ATTRIBUTE_NAME: &'static str = "cfg_attr";
    const COMMENT_ATTRIBUTE_NAME: &'static str = "doc";
    const TEST_NAME: &'static str = "test";

    pub(super) fn defects(&self) -> &[Defect] {
        match self {
            Self { defects, .. } => defects,
        }
    }

    pub(super) fn comment_starts(&self) -> &BTreeSet<LineColumn> {
        match self {
            Self { comment_starts, .. } => comment_starts,
        }
    }

    fn count_test_conditions(&self, attribute_content: &Meta) -> usize {
        let attribute_path = attribute_content.path();
        let configuration_name = Self::CONFIGURATION_NAME;
        let configuration_attribute_name = Self::CONFIGURATION_ATTRIBUTE_NAME;
        let test_name = Self::TEST_NAME;
        let condition_tokens = match attribute_content {
            Meta::List(MetaList {
                tokens: condition_token_stream,
                ..
            }) if attribute_path.is_ident(configuration_name)
                || attribute_path.is_ident(configuration_attribute_name) => {
                let token_stream = condition_token_stream.clone();
                IdentifierTokens::from(token_stream)
            }
            Meta::List(_) | Meta::Path(_) | Meta::NameValue(_) => IdentifierTokens::default(),
        };
        let mut test_condition_count: usize = 0;
        for condition_token in &condition_tokens {
            if condition_token.text() == test_name {
                test_condition_count += 1;
            }
        }
        test_condition_count
    }

    fn insert_comment_defect(&mut self, attribute: &Attribute) {
        let Attribute {
            pound_token: hash_mark,
            meta: attribute_content,
            ..
        } = attribute;
        let attribute_path = attribute_content.path();
        let comment_attribute_name = Self::COMMENT_ATTRIBUTE_NAME;
        if attribute_path.is_ident(comment_attribute_name) {
            let [hash_mark_span] = hash_mark.spans;
            let start_position = hash_mark_span.start();
            self.insert_comment(start_position);
        }
    }

    fn insert_comment(&mut self, start_position: LineColumn) {
        let Self {
            defects,
            comment_starts,
            ..
        } = self;
        let LineColumn {
            line,
            column: column_offset,
        } = start_position;
        let column = column_offset + 1;
        let comment = Defect::Comment { line, column };
        defects.push(comment);
        comment_starts.insert(start_position);
    }

    fn insert_test_attribute(&mut self, attribute: &Attribute) {
        let Attribute {
            pound_token: hash_mark,
            meta: attribute_content,
            ..
        } = attribute;
        let [hash_mark_span] = hash_mark.spans;
        let start_position = hash_mark_span.start();
        let attribute_path = attribute_content.path();
        let test_name = Self::TEST_NAME;
        let test_condition_count = self.count_test_conditions(attribute_content);
        let Self { production_end, .. } = self;
        if start_position < *production_end
            && (attribute_path.is_ident(test_name) || test_condition_count > 0)
        {
            self.insert_test_attribute_defect(start_position);
        }
    }

    fn insert_test_attribute_defect(&mut self, start_position: LineColumn) {
        let Self { defects, .. } = self;
        let LineColumn {
            line,
            column: column_offset,
        } = start_position;
        let column = column_offset + 1;
        let test_attribute = Defect::TestAttribute { line, column };
        defects.push(test_attribute);
    }
}

impl From<LineColumn> for AttributeVisitor {
    fn from(production_end: LineColumn) -> Self {
        let defects = Vec::new();
        let comment_starts = BTreeSet::new();
        Self {
            production_end,
            defects,
            comment_starts,
        }
    }
}

impl<'syntax> Visit<'syntax> for AttributeVisitor {
    fn visit_attribute(&mut self, node: &'syntax Attribute) {
        self.insert_comment_defect(node);
        self.insert_test_attribute(node);
        visit::visit_attribute(self, node);
    }
}
