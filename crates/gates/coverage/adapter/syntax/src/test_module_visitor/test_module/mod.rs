mod test_attribute_presence;

use crate::test_module_visitor::test_module::test_attribute_presence::TestAttributePresence;
use ::syn::Attribute;
use gates_coverage_core_test_boundaries::test_boundary::{self, TestBoundary};
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::path::Path;

#[derive(Clone, Copy)]
pub(super) struct TestModule<'module> {
    attributes: &'module [Attribute],
    closing_line: usize,
}

impl Layer for TestModule<'_> {
    type Layer = InfrastructureLayer;
}

impl TestModule<'_> {
    pub(super) fn compose_test_boundaries(
        &self,
        path: &Path,
    ) -> Vec<Result<TestBoundary, test_boundary::Error>> {
        let Self { attributes, .. } = *self;
        let mut test_boundaries = Vec::new();
        for attribute in attributes {
            let test_attribute_presence = TestAttributePresence::from(attribute);
            match test_attribute_presence {
                TestAttributePresence::Presence { opening_line } => {
                    let test_boundary = self.try_compose_test_boundary(path, opening_line);
                    test_boundaries.push(test_boundary);
                }
                TestAttributePresence::Absence => {}
            }
        }
        test_boundaries
    }

    fn try_compose_test_boundary(
        &self,
        path: &Path,
        opening_line: usize,
    ) -> Result<TestBoundary, test_boundary::Error> {
        let Self { closing_line, .. } = *self;
        let test_boundary_path = path.to_path_buf();
        TestBoundary::try_from((test_boundary_path, opening_line, closing_line))
    }
}

impl<'module> From<(&'module [Attribute], usize)> for TestModule<'module> {
    fn from((attributes, closing_line): (&'module [Attribute], usize)) -> Self {
        Self {
            attributes,
            closing_line,
        }
    }
}
