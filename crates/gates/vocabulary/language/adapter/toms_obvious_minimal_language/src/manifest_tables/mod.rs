mod platform_tables;

use crate::manifest_tables::platform_tables::PlatformTables;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::io::{self, ErrorKind};
use std::iter;
use toml::{Table, Value};

#[derive(Clone, Debug, PartialEq)]
pub(super) struct ManifestTables<'manifest> {
    manifest_table: &'manifest Table,
    platform_tables: Vec<&'manifest Table>,
}

impl Layer for ManifestTables<'_> {
    type Layer = InfrastructureLayer;
}

impl<'manifest> ManifestTables<'manifest> {
    const TARGET_NAME: &'static str = "target";
    const TARGET_ERROR_TEXT: &'static str = "the manifest's targets are not a table of tables";

    pub(super) fn select_dependency_values(
        &self,
        table_name: &str,
    ) -> impl Iterator<Item = Option<&'manifest Value>> {
        let Self {
            manifest_table,
            platform_tables,
        } = self;
        let manifest_value = manifest_table.get(table_name);
        let platform_value = |platform_table: &'manifest Table| platform_table.get(table_name);
        let platform_values: Vec<Option<&'manifest Value>> = platform_tables
            .iter()
            .copied()
            .map(platform_value)
            .collect();
        iter::once(manifest_value).chain(platform_values)
    }
}

impl<'manifest> TryFrom<&'manifest Table> for ManifestTables<'manifest> {
    type Error = io::Error;

    fn try_from(manifest_table: &'manifest Table) -> Result<Self, Self::Error> {
        let target_name = Self::TARGET_NAME;
        let target_value = manifest_table.get(target_name);
        let error_kind = ErrorKind::InvalidData;
        let target_error_text = Self::TARGET_ERROR_TEXT;
        let platform_tables = match target_value {
            None => Vec::new(),
            Some(Value::Table(target)) => {
                let target_platform_tables = PlatformTables::try_from(target)?;
                target_platform_tables.into_iter().collect()
            }
            Some(_) => {
                let error = io::Error::new(error_kind, target_error_text);
                return Err(error);
            }
        };
        let manifest_tables = Self {
            manifest_table,
            platform_tables,
        };
        Ok(manifest_tables)
    }
}
