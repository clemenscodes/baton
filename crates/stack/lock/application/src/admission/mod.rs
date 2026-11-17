use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_lock_core_name::{ApplicationName, LockName};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Admission {
    lock_name: LockName,
    application_name: ApplicationName,
    default_presence: bool,
    yield_presence: bool,
}

impl Layer for Admission {
    type Layer = ApplicationLayer;
}

impl Admission {
    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }

    pub fn application_name(&self) -> &ApplicationName {
        &self.application_name
    }

    pub fn default_presence(&self) -> bool {
        self.default_presence
    }

    pub fn yield_presence(&self) -> bool {
        self.yield_presence
    }
}

impl TryFrom<(String, String, bool, bool)> for Admission {
    type Error = Error;

    fn try_from(
        (lock_text, application_text, default_presence, yield_presence): (
            String,
            String,
            bool,
            bool,
        ),
    ) -> Result<Self, Self::Error> {
        let lock_name = LockName::try_from(lock_text)?;
        let application_name = ApplicationName::try_from(application_text)?;
        Ok(Self {
            lock_name,
            application_name,
            default_presence,
            yield_presence,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_command_keeps_its_lock_and_its_application() {
        let command = Admission::try_from((
            String::from("rtx3080"),
            String::from("desktop"),
            true,
            false,
        ))
        .unwrap();

        assert_eq!(command.lock_name().text(), "rtx3080");
        assert_eq!(command.application_name().text(), "desktop");
        assert!(command.default_presence());
    }

    #[test]
    fn the_command_of_a_wrong_name_is_refused() {
        let outcome = Admission::try_from((
            String::from("rtx3080"),
            String::from("--help"),
            true,
            false,
        ));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }

    #[test]
    fn the_command_of_a_wrong_lock_is_refused() {
        let outcome = Admission::try_from((
            String::from("--lock"),
            String::from("desktop"),
            true,
            false,
        ));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }
}
