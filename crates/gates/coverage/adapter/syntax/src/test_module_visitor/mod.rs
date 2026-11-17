mod test_module;

use crate::test_module_visitor::test_module::TestModule;
use ::syn::ItemMod;
use ::syn::visit::{self, Visit};
use gates_coverage_core_test_boundaries::test_boundary::{self, TestBoundary};
use proc_macro2::LineColumn;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::{Path, PathBuf};
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct TestModuleVisitor {
    path: PathBuf,
    test_boundaries: Vec<Result<TestBoundary, test_boundary::Error>>,
}

impl Layer for TestModuleVisitor {
    type Layer = InfrastructureLayer;
}

impl TestModuleVisitor {
    const TEST_MODULE_NAME: &'static str = "tests";

    fn push_test_boundaries(&mut self, module: &ItemMod) {
        let Self {
            path,
            test_boundaries,
        } = self;
        let test_module_name = Self::TEST_MODULE_NAME;
        if let ItemMod {
            attrs: attributes,
            ident: identifier,
            content: Some((brace, _)),
            ..
        } = module
            && identifier == test_module_name
        {
            let LineColumn {
                line: closing_line, ..
            } = brace.span.join().end();
            let module_attributes = attributes.as_slice();
            let test_module = TestModule::from((module_attributes, closing_line));
            let module_test_boundaries = test_module.compose_test_boundaries(path);
            test_boundaries.extend(module_test_boundaries);
        }
    }
}

impl<'syntax> Visit<'syntax> for TestModuleVisitor {
    fn visit_item_mod(&mut self, module: &'syntax ItemMod) {
        self.push_test_boundaries(module);
        visit::visit_item_mod(self, module);
    }
}

impl From<&Path> for TestModuleVisitor {
    fn from(source_path: &Path) -> Self {
        let path = source_path.to_path_buf();
        let test_boundaries = Vec::new();
        Self {
            path,
            test_boundaries,
        }
    }
}

impl IntoIterator for TestModuleVisitor {
    type Item = Result<TestBoundary, test_boundary::Error>;
    type IntoIter = IntoIter<Result<TestBoundary, test_boundary::Error>>;

    fn into_iter(self) -> Self::IntoIter {
        let Self {
            test_boundaries, ..
        } = self;
        test_boundaries.into_iter()
    }
}
