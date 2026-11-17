use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_lock_core_name::{ApplicationName, LockName};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Dismissal {
    lock_name: LockName,
    application_name: ApplicationName,
}

impl Layer for Dismissal {
    type Layer = ApplicationLayer;
}

impl Dismissal {
    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }

    pub fn application_name(&self) -> &ApplicationName {
        &self.application_name
    }
}

impl TryFrom<(String, String)> for Dismissal {
    type Error = Error;

    fn try_from((lock_text, application_text): (String, String)) -> Result<Self, Self::Error> {
        let lock_name = LockName::try_from(lock_text)?;
        let application_name = ApplicationName::try_from(application_text)?;
        Ok(Self {
            lock_name,
            application_name,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_command_keeps_its_lock_and_its_application() {
        let command = Dismissal::try_from((String::from("rtx3080"), String::from("desktop")))
            .unwrap();

        assert_eq!(command.lock_name().text(), "rtx3080");
        assert_eq!(command.application_name().text(), "desktop");
    }

    #[test]
    fn the_command_of_a_wrong_name_is_refused() {
        let outcome = Dismissal::try_from((String::from("rtx3080"), String::from("--help")));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }

    #[test]
    fn the_command_of_a_wrong_lock_is_refused() {
        let outcome = Dismissal::try_from((String::from("--lock"), String::from("desktop")));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }
}
