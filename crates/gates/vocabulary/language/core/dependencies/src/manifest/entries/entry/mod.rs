mod entry_path;

use crate::manifest::entries::entry::entry_path::EntryPath;
use gates_vocabulary_language_core_foreign_name::ForeignName;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Entry {
    foreign_name: ForeignName,
    path: Option<EntryPath>,
}

impl Layer for Entry {
    type Layer = DomainLayer;
}

impl ValueObject for Entry {}

impl Entry {
    pub(crate) fn foreign_name(&self) -> &ForeignName {
        &self.foreign_name
    }

    pub(crate) fn maybe_path(&self) -> Option<&Path> {
        self.path.as_ref().map(EntryPath::path)
    }
}

impl From<(ForeignName, Option<PathBuf>)> for Entry {
    fn from((foreign_name, dependency_path): (ForeignName, Option<PathBuf>)) -> Self {
        let path = dependency_path.map(EntryPath::from);
        Self { foreign_name, path }
    }
}

impl From<(&Entry, &Path)> for Entry {
    fn from((entry, directory): (&Entry, &Path)) -> Self {
        let Entry { foreign_name, path } = entry;
        let directory_foreign_name = foreign_name.clone();
        let directory_entry_path = |entry_path: &EntryPath| {
            EntryPath::from((entry_path, directory))
        };
        let directory_path = path.as_ref().map(directory_entry_path);
        Self {
            foreign_name: directory_foreign_name,
            path: directory_path,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_entry_of_a_path_leads_to_that_path() {
        let path_text = "../port";
        let foreign_name_text = "port";
        let path = PathBuf::from(path_text);
        let dependency_path = Some(path);
        let foreign_name = ForeignName::from(foreign_name_text);

        let entry = Entry::from((foreign_name, dependency_path));

        let entry_path = entry.maybe_path();
        let path_expectation = Path::new(path_text);
        let expectation = Some(path_expectation);
        assert_eq!(entry_path, expectation);
    }

    #[test]
    fn an_entry_of_a_directory_leads_to_its_path_inside_that_directory() {
        let path_text = "../port";
        let foreign_name_text = "port";
        let directory_text = "/workspace/crates/language";
        let path = PathBuf::from(path_text);
        let dependency_path = Some(path);
        let foreign_name = ForeignName::from(foreign_name_text);
        let entry = Entry::from((foreign_name, dependency_path));
        let directory = Path::new(directory_text);

        let directory_entry = Entry::from((&entry, directory));

        let entry_path = directory_entry.maybe_path();
        let expectation_text = "/workspace/crates/port";
        let path_expectation = Path::new(expectation_text);
        let expectation = Some(path_expectation);
        assert_eq!(entry_path, expectation);
    }

    #[test]
    fn an_entry_without_a_path_leads_nowhere_in_any_directory() {
        let foreign_name_text = "syn";
        let directory_text = "/workspace";
        let dependency_path = None;
        let foreign_name = ForeignName::from(foreign_name_text);
        let entry = Entry::from((foreign_name, dependency_path));
        let directory = Path::new(directory_text);

        let directory_entry = Entry::from((&entry, directory));

        let entry_path = directory_entry.maybe_path();
        let expectation = None;
        assert_eq!(entry_path, expectation);
    }

    #[test]
    fn an_entry_of_a_relative_directory_keeps_the_parent_steps_it_cannot_resolve() {
        let path_text = "../port";
        let foreign_name_text = "port";
        let directory_text = ".";
        let path = PathBuf::from(path_text);
        let dependency_path = Some(path);
        let foreign_name = ForeignName::from(foreign_name_text);
        let entry = Entry::from((foreign_name, dependency_path));
        let directory = Path::new(directory_text);

        let directory_entry = Entry::from((&entry, directory));

        let entry_path = directory_entry.maybe_path();
        let expectation_text = "./../port";
        let path_expectation = Path::new(expectation_text);
        let expectation = Some(path_expectation);
        assert_eq!(entry_path, expectation);
    }
}
