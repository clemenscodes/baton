use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::collections::BTreeSet;
use std::collections::btree_set::IntoIter;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Paths {
    paths: BTreeSet<PathBuf>,
}

impl Layer for Paths {
    type Layer = DomainLayer;
}

impl ValueObject for Paths {}

impl FromIterator<PathBuf> for Paths {
    fn from_iter<PathsType: IntoIterator<Item = PathBuf>>(path_sequence: PathsType) -> Self {
        let paths = BTreeSet::from_iter(path_sequence);
        Self { paths }
    }
}

impl IntoIterator for Paths {
    type Item = PathBuf;
    type IntoIter = IntoIter<PathBuf>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { paths } = self;
        paths.into_iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn paths_yield_every_path_once_in_its_order() {
        let main_path_text = "crates/harbor/src/main.rs";
        let path_text = "crates/harbor/src/lib.rs";
        let path_texts = [main_path_text, path_text, main_path_text];
        let paths: Paths = path_texts.into_iter().map(PathBuf::from).collect();

        let path_sequence = Vec::from_iter(paths);

        let path_expectation = PathBuf::from(path_text);
        let main_path_expectation = PathBuf::from(main_path_text);
        let expectation = vec![path_expectation, main_path_expectation];
        assert_eq!(path_sequence, expectation);
    }
}
