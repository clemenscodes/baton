mod component_list;

use crate::manifest::entries::entry::entry_path::component_list::ComponentList;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct EntryPath {
    path: PathBuf,
}

impl Layer for EntryPath {
    type Layer = DomainLayer;
}

impl ValueObject for EntryPath {}

impl EntryPath {
    pub(super) fn path(&self) -> &Path {
        &self.path
    }
}

impl From<PathBuf> for EntryPath {
    fn from(path: PathBuf) -> Self {
        Self { path }
    }
}

impl From<(&EntryPath, &Path)> for EntryPath {
    fn from((entry_path, directory): (&EntryPath, &Path)) -> Self {
        let EntryPath {
            path: dependency_path,
        } = entry_path;
        let directory_path = directory.join(dependency_path);
        let component_list = ComponentList::from(&directory_path);
        let path = component_list.into_iter().collect();
        Self { path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_entry_path_keeps_its_path() {
        let path_text = "crates/harbor";
        let dependency_path = PathBuf::from(path_text);
        let entry_path = EntryPath::from(dependency_path);

        let path = entry_path.path();

        let expectation = Path::new(path_text);
        assert_eq!(path, expectation);
    }

    #[test]
    fn an_entry_path_of_a_directory_resolves_the_parent_steps_it_can_resolve() {
        let path_text = "../harbor";
        let directory_text = "crates/lighthouse";
        let path = PathBuf::from(path_text);
        let entry_path = EntryPath::from(path);
        let directory = Path::new(directory_text);

        let directory_entry_path = EntryPath::from((&entry_path, directory));

        let expectation_text = "crates/harbor";
        let expectation = Path::new(expectation_text);
        let directory_path = directory_entry_path.path();
        assert_eq!(directory_path, expectation);
    }

    #[test]
    fn an_entry_path_of_a_relative_directory_keeps_the_parent_steps_it_cannot_resolve() {
        let path_text = "../../harbor";
        let directory_text = "lighthouse";
        let path = PathBuf::from(path_text);
        let entry_path = EntryPath::from(path);
        let directory = Path::new(directory_text);

        let directory_entry_path = EntryPath::from((&entry_path, directory));

        let expectation_text = "../harbor";
        let expectation = Path::new(expectation_text);
        let directory_path = directory_entry_path.path();
        assert_eq!(directory_path, expectation);
    }
}
