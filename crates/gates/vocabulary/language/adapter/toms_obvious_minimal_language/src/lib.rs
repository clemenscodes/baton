mod manifest_tables;

use gates_vocabulary_language_core_dependencies::decoder::Decoder;
use gates_vocabulary_language_core_dependencies::manifest::entries::{Entries, Entry};
use gates_vocabulary_language_core_dependencies::manifest::{Manifest, ManifestText};
use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_language::identifier::Identifier;
use manifest_tables::ManifestTables;
use stack_domain_driven_design_core::adapter::Adapter;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use stack_domain_driven_design_core::port::{DrivenPort, Port};
use std::io::{self, ErrorKind};
use std::path::PathBuf;
use toml::{Table, Value};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct TomsObviousMinimalLanguage;

impl Layer for TomsObviousMinimalLanguage {
    type Layer = InfrastructureLayer;
}

impl Adapter for TomsObviousMinimalLanguage {}

impl Port for TomsObviousMinimalLanguage {}

impl DrivenPort for TomsObviousMinimalLanguage {}

impl TomsObviousMinimalLanguage {
    const DEPENDENCY_TABLE_NAME: &'static str = "dependencies";
    const DEVELOPMENT_DEPENDENCY_TABLE_NAME: &'static str = "dev-dependencies";
    const WORKSPACE_NAME: &'static str = "workspace";
    const PATH_NAME: &'static str = "path";
    const TABLE_ERROR_TEXT: &'static str = "the manifest's dependencies are not a table";
    const WORKSPACE_ERROR_TEXT: &'static str = "the manifest's workspace is not a table";
    const PATH_ERROR_TEXT: &'static str = "a dependency's path is not a string";
    const PACKAGE_TABLE_NAME: &'static str = "package";
    const PACKAGE_IDENTIFIER_NAME: &'static str = "name";
    const PACKAGE_IDENTIFIER_SEPARATOR: &'static str = "-";
    const IDENTIFIER_SEPARATOR: &'static str = "_";
    const PACKAGE_TABLE_ERROR_TEXT: &'static str = "the manifest's package is not a table";
    const PACKAGE_IDENTIFIER_ERROR_TEXT: &'static str = "the package name is not a string";

    fn try_decode_entries<'manifest>(
        &self,
        dependency_values: impl IntoIterator<Item = Option<&'manifest Value>>,
    ) -> io::Result<Entries> {
        let mut entry_list = Vec::new();
        for dependency_value in dependency_values {
            let table_entries = self.try_decode_dependency_value(dependency_value)?;
            for entry in table_entries {
                if !entry_list.contains(&entry) {
                    entry_list.push(entry);
                }
            }
        }
        let entries = Entries::try_from(entry_list);
        entries.map_err(io::Error::other)
    }

    fn try_decode_dependency_value(
        &self,
        dependency_value: Option<&Value>,
    ) -> io::Result<Vec<Entry>> {
        match dependency_value {
            None => Ok(Vec::new()),
            Some(Value::Table(dependency_table)) => {
                let mut table_entries = Vec::new();
                for (dependency_name, entry_value) in dependency_table {
                    let foreign_name_text = dependency_name.as_str();
                    let foreign_name = ForeignName::from(foreign_name_text);
                    let path = self.try_decode_path(entry_value)?;
                    let entry = Entry::from((foreign_name, path));
                    table_entries.push(entry);
                }
                Ok(table_entries)
            }
            Some(_) => {
                let error_kind = ErrorKind::InvalidData;
                let table_error_text = Self::TABLE_ERROR_TEXT;
                let error = io::Error::new(error_kind, table_error_text);
                Err(error)
            }
        }
    }

    fn try_decode_path(&self, entry_value: &Value) -> io::Result<Option<PathBuf>> {
        let path_name = Self::PATH_NAME;
        let path_value = entry_value.get(path_name);
        match path_value {
            None => {
                let path = None;
                Ok(path)
            }
            Some(Value::String(path_text)) => {
                let dependency_path = PathBuf::from(path_text);
                let path = Some(dependency_path);
                Ok(path)
            }
            Some(_) => {
                let error_kind = ErrorKind::InvalidData;
                let path_error_text = Self::PATH_ERROR_TEXT;
                let error = io::Error::new(error_kind, path_error_text);
                Err(error)
            }
        }
    }

    fn try_select_workspace_dependencies<'manifest>(
        &self,
        manifest_table: &'manifest Table,
    ) -> io::Result<Option<&'manifest Value>> {
        let workspace_name = Self::WORKSPACE_NAME;
        let workspace_value = manifest_table.get(workspace_name);
        let dependency_table_name = Self::DEPENDENCY_TABLE_NAME;
        match workspace_value {
            None => {
                let workspace_dependency_value = None;
                Ok(workspace_dependency_value)
            }
            Some(Value::Table(workspace_table)) => {
                let workspace_dependency_value = workspace_table.get(dependency_table_name);
                Ok(workspace_dependency_value)
            }
            Some(_) => {
                let error_kind = ErrorKind::InvalidData;
                let workspace_error_text = Self::WORKSPACE_ERROR_TEXT;
                let error = io::Error::new(error_kind, workspace_error_text);
                Err(error)
            }
        }
    }

    fn try_decode_workspace_entries(
        &self,
        workspace_dependency_value: Option<&Value>,
    ) -> io::Result<Entries> {
        let workspace_dependency_values = [workspace_dependency_value];
        self.try_decode_entries(workspace_dependency_values)
    }

    fn try_parse_manifest_table(&self, manifest_text: &ManifestText) -> io::Result<Table> {
        let text = manifest_text.text();
        let error_kind = ErrorKind::InvalidData;
        let manifest_error = |error| io::Error::new(error_kind, error);
        text.parse::<Table>().map_err(manifest_error)
    }

    fn try_select_package_identifier<'manifest>(
        &self,
        manifest_table: &'manifest Table,
    ) -> io::Result<Option<&'manifest Value>> {
        let package_table_name = Self::PACKAGE_TABLE_NAME;
        let package_table_value = manifest_table.get(package_table_name);
        let package_identifier_name = Self::PACKAGE_IDENTIFIER_NAME;
        match package_table_value {
            None => {
                let package_identifier_value = None;
                Ok(package_identifier_value)
            }
            Some(Value::Table(package_table)) => {
                let package_identifier_value = package_table.get(package_identifier_name);
                Ok(package_identifier_value)
            }
            Some(_) => {
                let error_kind = ErrorKind::InvalidData;
                let package_table_error_text = Self::PACKAGE_TABLE_ERROR_TEXT;
                let error = io::Error::new(error_kind, package_table_error_text);
                Err(error)
            }
        }
    }

    fn try_decode_package_identifier(
        &self,
        package_identifier_value: Option<&Value>,
    ) -> io::Result<Identifier> {
        match package_identifier_value {
            None => {
                let package_identifier = Identifier::default();
                Ok(package_identifier)
            }
            Some(Value::String(package_identifier_text)) => {
                self.try_decode_package_identifier_text(package_identifier_text)
            }
            Some(_) => {
                let error_kind = ErrorKind::InvalidData;
                let package_identifier_error_text = Self::PACKAGE_IDENTIFIER_ERROR_TEXT;
                let error = io::Error::new(error_kind, package_identifier_error_text);
                Err(error)
            }
        }
    }

    fn try_decode_package_identifier_text(
        &self,
        package_identifier_text: &str,
    ) -> io::Result<Identifier> {
        let package_identifier_separator = Self::PACKAGE_IDENTIFIER_SEPARATOR;
        let identifier_separator = Self::IDENTIFIER_SEPARATOR;
        let identifier_text = package_identifier_text
            .replace(package_identifier_separator, identifier_separator);
        let identifier_letters = identifier_text.as_str();
        let package_identifier = Identifier::try_from(identifier_letters);
        let error_kind = ErrorKind::InvalidData;
        let identifier_error = |error| io::Error::new(error_kind, error);
        package_identifier.map_err(identifier_error)
    }
}

impl Decoder for TomsObviousMinimalLanguage {
    fn try_decode_manifest(&self, manifest_text: &ManifestText) -> io::Result<Manifest> {
        let manifest_table = self.try_parse_manifest_table(manifest_text)?;
        let package_identifier_value = self.try_select_package_identifier(&manifest_table)?;
        let workspace_dependency_value = self.try_select_workspace_dependencies(&manifest_table)?;
        let manifest_tables = ManifestTables::try_from(&manifest_table)?;
        let package_identifier = self.try_decode_package_identifier(package_identifier_value)?;
        let dependency_table_name = Self::DEPENDENCY_TABLE_NAME;
        let development_dependency_table_name = Self::DEVELOPMENT_DEPENDENCY_TABLE_NAME;
        let dependency_values = manifest_tables.select_dependency_values(dependency_table_name);
        let dependency_entries = self.try_decode_entries(dependency_values)?;
        let development_dependency_values = manifest_tables
            .select_dependency_values(development_dependency_table_name);
        let development_dependency_entries = self
            .try_decode_entries(development_dependency_values)?;
        let workspace_entries = self.try_decode_workspace_entries(workspace_dependency_value)?;
        let manifest = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        Ok(manifest)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) mod fixture {
        use super::*;
        use gates_vocabulary_language_core_foreign_name::ForeignName;
        use std::path::Path;

        pub(crate) fn foreign_name_entry_factory(foreign_name_text: &str) -> Entry {
            let foreign_name = ForeignName::from(foreign_name_text);
            let path = None;
            Entry::from((foreign_name, path))
        }

        pub(crate) fn path_entry_factory(crate_path_text: &str) -> Entry {
            let crate_path = Path::new(crate_path_text);
            let directory_name = crate_path.file_name().unwrap();
            let foreign_name_text = directory_name.to_str().unwrap();
            let foreign_name = ForeignName::from(foreign_name_text);
            let entry_path = crate_path.to_path_buf();
            let path = Some(entry_path);
            Entry::from((foreign_name, path))
        }
    }

    #[test]
    fn a_manifest_decodes_into_the_entries_of_its_dependencies() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[package]
name = "harbor"

[dependencies]
anchor.workspace = true
port = { path = "../port", package = "plumbing-port" }
rope = "=0.13.0"
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language
            .try_decode_manifest(&manifest_text)
            .unwrap();

        let package_identifier_text = "harbor";
        let workspace_name_text = "anchor";
        let port_name_text = "port";
        let foreign_name_text = "rope";
        let port_path_text = "../port";
        let port_path = PathBuf::from(port_path_text);
        let dependency_path = Some(port_path);
        let port_name = ForeignName::from(port_name_text);
        let workspace_entry = fixture::foreign_name_entry_factory(workspace_name_text);
        let port_entry = Entry::from((port_name, dependency_path));
        let foreign_name_entry = fixture::foreign_name_entry_factory(foreign_name_text);
        let entry_list = vec![workspace_entry, port_entry, foreign_name_entry];
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let development_dependency_entries = Entries::default();
        let workspace_entries = Entries::default();
        let expectation = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        assert_eq!(manifest, expectation);
    }

    #[test]
    fn a_manifest_decodes_into_the_entries_of_its_development_dependencies() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[package]
name = "proc-macro2"

[dev-dependencies]
tokio.workspace = true
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language
            .try_decode_manifest(&manifest_text)
            .unwrap();

        let package_identifier_text = "proc_macro2";
        let foreign_name_text = "tokio";
        let foreign_name_entry = fixture::foreign_name_entry_factory(foreign_name_text);
        let entry_list = vec![foreign_name_entry];
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let dependency_entries = Entries::default();
        let development_dependency_entries = Entries::try_from(entry_list).unwrap();
        let workspace_entries = Entries::default();
        let expectation = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        assert_eq!(manifest, expectation);
    }

    #[test]
    fn a_manifest_decodes_into_the_entries_of_the_dependencies_of_its_platforms() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[package]
name = "harbor"

[dependencies]
anchor.workspace = true

[target.'cfg(target_arch = "wasm32")'.dependencies]
anchor = { workspace = true, features = ["sail"] }
rope.workspace = true

[target.'cfg(unix)'.dev-dependencies]
tokio.workspace = true
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language
            .try_decode_manifest(&manifest_text)
            .unwrap();

        let package_identifier_text = "harbor";
        let dependency_name_text = "anchor";
        let platform_name_text = "rope";
        let development_dependency_name_text = "tokio";
        let dependency_entry = fixture::foreign_name_entry_factory(dependency_name_text);
        let platform_entry = fixture::foreign_name_entry_factory(platform_name_text);
        let development_dependency_entry = fixture::foreign_name_entry_factory(
            development_dependency_name_text,
        );
        let entry_list = vec![dependency_entry, platform_entry];
        let development_dependency_list = vec![development_dependency_entry];
        let package_identifier = Identifier::try_from(package_identifier_text).unwrap();
        let dependency_entries = Entries::try_from(entry_list).unwrap();
        let development_dependency_entries = Entries::try_from(development_dependency_list)
            .unwrap();
        let workspace_entries = Entries::default();
        let expectation = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        assert_eq!(manifest, expectation);
    }

    #[test]
    fn a_root_manifest_decodes_into_the_entries_of_its_workspace() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[workspace.dependencies]
ship = { path = "crates/ship" }
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language
            .try_decode_manifest(&manifest_text)
            .unwrap();

        let crate_path_text = "crates/ship";
        let path_entry = fixture::path_entry_factory(crate_path_text);
        let workspace_list = vec![path_entry];
        let package_identifier = Identifier::default();
        let dependency_entries = Entries::default();
        let development_dependency_entries = Entries::default();
        let workspace_entries = Entries::try_from(workspace_list).unwrap();
        let expectation = Manifest::from((
            package_identifier,
            dependency_entries,
            development_dependency_entries,
            workspace_entries,
        ));
        assert_eq!(manifest, expectation);
    }

    #[test]
    fn a_package_without_a_name_decodes_into_an_identifier_of_no_word() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[package]
version = "0.0.0"
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language
            .try_decode_manifest(&manifest_text)
            .unwrap();

        let expectation = Manifest::default();
        assert_eq!(manifest, expectation);
    }

    #[test]
    fn a_text_that_is_no_manifest_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = "[dependencies";
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_kind = error.as_ref().map(io::Error::kind);
        let error_kind_expectation = ErrorKind::InvalidData;
        let expectation = Some(error_kind_expectation);
        assert_eq!(error_kind, expectation);
    }

    #[test]
    fn a_manifest_whose_dependencies_are_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = "dependencies = 5";
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's dependencies are not a table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_development_dependencies_are_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = "dev-dependencies = 5";
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's dependencies are not a table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_workspace_dependencies_are_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[workspace]
dependencies = 5
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's dependencies are not a table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_workspace_is_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = "workspace = 5";
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's workspace is not a table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_target_is_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = "target = 5";
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's targets are not a table of tables";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_platform_is_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[target]
unix = 5
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's targets are not a table of tables";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_platform_dependencies_are_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[target.unix]
dependencies = 5
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's dependencies are not a table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_package_is_no_table_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = "package = 5";
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the manifest's package is not a table";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_package_name_is_no_text_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[package]
name = 5
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "the package name is not a string";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_whose_package_name_is_no_identifier_of_words_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[package]
name = "harbor-2"
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "`2` is not a word";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }

    #[test]
    fn a_manifest_with_a_path_of_a_dependency_that_is_no_text_is_an_error() {
        let toms_obvious_minimal_language = TomsObviousMinimalLanguage;
        let text = r#"[dependencies]
port = { path = 5 }
"#;
        let manifest_text = ManifestText::from(text);

        let manifest = toms_obvious_minimal_language.try_decode_manifest(&manifest_text);

        let error = manifest.err();
        let error_text = error.as_ref().map(io::Error::to_string);
        let expectation_text = "a dependency's path is not a string";
        let text_expectation = String::from(expectation_text);
        let expectation = Some(text_expectation);
        assert_eq!(error_text, expectation);
    }
}
