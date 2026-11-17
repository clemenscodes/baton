use stack_domain_driven_design_core::layer::{InfrastructureLayer, Layer};
use std::ffi::{OsStr, OsString};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct HelperConfiguration {
    elevation_program: OsString,
    elevation_arguments: Vec<OsString>,
    host_program: OsString,
}

impl Layer for HelperConfiguration {
    type Layer = InfrastructureLayer;
}

impl HelperConfiguration {
    pub fn elevation_program(&self) -> &OsStr {
        &self.elevation_program
    }

    pub fn elevation_arguments(&self) -> &[OsString] {
        &self.elevation_arguments
    }

    pub fn host_program(&self) -> &OsStr {
        &self.host_program
    }
}

impl From<(OsString, Vec<OsString>, OsString)> for HelperConfiguration {
    fn from(
        (elevation_program, elevation_arguments, host_program): (OsString, Vec<OsString>, OsString),
    ) -> Self {
        Self {
            elevation_program,
            elevation_arguments,
            host_program,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_configuration_keeps_its_programs_and_its_arguments() {
        let helper_configuration = HelperConfiguration::from((
            OsString::from("sudo"),
            vec![OsString::from("-n")],
            OsString::from("baton_host"),
        ));

        assert_eq!(helper_configuration.elevation_program(), "sudo");
        assert_eq!(
            helper_configuration.elevation_arguments(),
            [OsString::from("-n")]
        );
        assert_eq!(helper_configuration.host_program(), "baton_host");
    }
}
