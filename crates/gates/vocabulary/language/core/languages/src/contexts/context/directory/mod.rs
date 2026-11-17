use stack_domain_driven_design_core::identifier::Identifier;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::specification::{CandidateDecision, Specification};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::path::{Component, Path, PathBuf};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Directory {
    path: PathBuf,
}

impl Layer for Directory {
    type Layer = DomainLayer;
}

impl ValueObject for Directory {}

impl Identifier for Directory {}

impl Directory {
    const ROOT: &'static str = "crates";

    pub fn path(&self) -> &Path {
        match self {
            Self { path } => path,
        }
    }
}

impl Specification<PathBuf> for Directory {
    fn decide_acceptance(&self, candidate: &PathBuf) -> CandidateDecision {
        let Self { path } = self;
        let directory_components: Vec<Component<'_>> = path.components().collect();
        let candidate_components: Vec<Component<'_>> = candidate.components().collect();
        let Some(root_component) = directory_components.first() else {
            return CandidateDecision::Rejection;
        };
        let component_criterion = |component: &Component<'_>| component == root_component;
        let Some(root_position) = candidate_components.iter().rposition(component_criterion) else {
            return CandidateDecision::Rejection;
        };
        let directory_count = directory_components.len();
        let candidate_directory = candidate_components
            .iter()
            .skip(root_position)
            .take(directory_count);
        match candidate_directory.eq(&directory_components) {
            true => CandidateDecision::Acceptance,
            false => CandidateDecision::Rejection,
        }
    }
}

impl From<&str> for Directory {
    fn from(directory_name: &str) -> Self {
        let root_text = Self::ROOT;
        let root = Path::new(root_text);
        let path = root.join(directory_name);
        Self { path }
    }
}

impl From<(&Path, &str)> for Directory {
    fn from((root, directory_name): (&Path, &str)) -> Self {
        let path = root.join(directory_name);
        Self { path }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

        pub(super) fn directory_factory(root_text: &str) -> Directory {
            let root = Path::new(root_text);
            let directory_name = "harbor";
            Directory::from((root, directory_name))
        }
    }

    #[test]
    fn a_path_under_the_directory_is_admitted() {
        let directory_name = "harbor";
        let directory = Directory::from(directory_name);
        let path_text = "crates/harbor/src/core/dock/mod.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_path_under_another_directory_is_rejected() {
        let directory_name = "ship";
        let directory = Directory::from(directory_name);
        let path_text = "crates/harbor/src/core/dock/mod.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_path_deep_under_the_directory_is_admitted() {
        let directory_name = "harbor";
        let directory = Directory::from(directory_name);
        let path_text = "crates/harbor/dock/public/src/lib.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_path_under_another_root_is_admitted_by_the_directory_of_that_root() {
        let root_text = "packages";
        let directory = fixture::directory_factory(root_text);
        let path_text = "packages/harbor/src/main/java/Lamp.java";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_directory_under_a_root_rejects_the_same_directory_under_another_root() {
        let root_text = "crates";
        let directory = fixture::directory_factory(root_text);
        let path_text = "packages/harbor/src/main/java/Harbor.java";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_directory_of_two_folders_admits_a_path_beneath_both() {
        let directory_name = "ship/deck";
        let directory = Directory::from(directory_name);
        let path_text = "crates/ship/deck/src/lib.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Acceptance;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_path_naming_the_root_and_nothing_below_is_rejected() {
        let directory_name = "harbor";
        let directory = Directory::from(directory_name);
        let path_text = "crates";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_path_that_steps_back_above_the_root_is_rejected() {
        let directory_name = "ship";
        let directory = Directory::from(directory_name);
        let path_text = "crates/../ship/src/lib.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn only_the_last_root_of_a_path_leads_to_its_directory() {
        let directory_name = "harbor";
        let directory = Directory::from(directory_name);
        let path_text = "crates/harbor/crates/ship/lib.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }

    #[test]
    fn a_directory_of_no_path_at_all_rejects_every_path() {
        let directory = Directory::default();
        let path_text = "crates/harbor/src/lib.rs";
        let path = PathBuf::from(path_text);

        let decision = directory.decide_acceptance(&path);

        let expectation = CandidateDecision::Rejection;
        assert_eq!(decision, expectation);
    }
}
