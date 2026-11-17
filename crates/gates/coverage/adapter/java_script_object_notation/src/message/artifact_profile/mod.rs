use serde::Deserialize;
use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default, Deserialize)]
pub(super) struct ArtifactProfile {
    #[serde(rename = "test")]
    test_flag: bool,
}

impl Layer for ArtifactProfile {
    type Layer = InfrastructureLayer;
}

impl ArtifactProfile {
    pub(super) fn test_flag(&self) -> bool {
        let Self { test_flag } = self;
        *test_flag
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_artifact_profile_states_whether_cargo_compiled_a_test() {
        let text = r#"{"test":true,"opt_level":"0"}"#;

        let artifact_profile: ArtifactProfile = serde_json::from_str(text).unwrap();

        assert!(artifact_profile.test_flag());
    }
}
