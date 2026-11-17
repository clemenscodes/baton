use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::CandidateDecision;
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Component, PathBuf};
use std::vec::IntoIter;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub(super) struct ComponentList<'path> {
    components: Vec<Component<'path>>,
}

impl Layer for ComponentList<'_> {
    type Layer = DomainLayer;
}

impl ValueObject for ComponentList<'_> {}

impl ComponentList<'_> {
    fn decide_component_removal(last_component: Option<&Component<'_>>) -> CandidateDecision {
        match last_component {
            Some(Component::Normal(_)) => CandidateDecision::Acceptance,
            Some(
                Component::Prefix(_)
                | Component::RootDir
                | Component::CurDir
                | Component::ParentDir,
            )
            | None => CandidateDecision::Rejection,
        }
    }
}

impl<'path> From<&'path PathBuf> for ComponentList<'path> {
    fn from(directory_path: &'path PathBuf) -> Self {
        let mut component_list: Vec<Component<'_>> = Vec::new();
        for component in directory_path.components() {
            match component {
                Component::ParentDir => {
                    let last_component = component_list.last();
                    match Self::decide_component_removal(last_component) {
                        CandidateDecision::Acceptance => {
                            component_list.pop();
                        }
                        CandidateDecision::Rejection => component_list.push(component),
                    }
                }
                Component::Prefix(_)
                | Component::RootDir
                | Component::CurDir
                | Component::Normal(_) => component_list.push(component),
            }
        }
        Self {
            components: component_list,
        }
    }
}

impl<'path> IntoIterator for ComponentList<'path> {
    type Item = Component<'path>;
    type IntoIter = IntoIter<Component<'path>>;

    fn into_iter(self) -> Self::IntoIter {
        let Self { components } = self;
        components.into_iter()
    }
}
