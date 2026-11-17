use crate::manifest_tables::ManifestTables;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io::{self, ErrorKind};
use std::vec::IntoIter;
use toml::{Table, Value};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct PlatformTables<'manifest> {
    platform_tables: Vec<&'manifest Table>,
}

impl Layer for PlatformTables<'_> {
    type Layer = InfrastructureLayer;
}

impl<'manifest> TryFrom<&'manifest Table> for PlatformTables<'manifest> {
    type Error = io::Error;

    fn try_from(target: &'manifest Table) -> Result<Self, Self::Error> {
        let error_kind = ErrorKind::InvalidData;
        let target_error_text = ManifestTables::TARGET_ERROR_TEXT;
        let mut platform_tables = Vec::new();
        for platform_value in target.values() {
            let Value::Table(platform_table) = platform_value else {
                let error = io::Error::new(error_kind, target_error_text);
                return Err(error);
            };
            platform_tables.push(platform_table);
        }
        let target_platform_tables = Self { platform_tables };
        Ok(target_platform_tables)
    }
}

impl<'manifest> IntoIterator for PlatformTables<'manifest> {
    type Item = &'manifest Table;
    type IntoIter = IntoIter<&'manifest Table>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { platform_tables } = self;
        platform_tables.into_iter()
    }
}
