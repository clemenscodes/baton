pub mod mode;

use crate::Error;
use crate::acquisition::mode::AcquisitionMode;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_lock_core_name::{ApplicationName, LockName};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Acquisition {
    lock_name: LockName,
    application_name: ApplicationName,
    waiter_identifier: u32,
    acquisition_mode: AcquisitionMode,
}

impl Layer for Acquisition {
    type Layer = ApplicationLayer;
}

impl Acquisition {
    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }

    pub fn application_name(&self) -> &ApplicationName {
        &self.application_name
    }

    pub fn waiter_identifier(&self) -> u32 {
        self.waiter_identifier
    }

    pub fn acquisition_mode(&self) -> AcquisitionMode {
        self.acquisition_mode
    }
}

impl TryFrom<(String, String, u32, AcquisitionMode)> for Acquisition {
    type Error = Error;

    fn try_from(
        (lock_text, application_text, waiter_identifier, acquisition_mode): (
            String,
            String,
            u32,
            AcquisitionMode,
        ),
    ) -> Result<Self, Self::Error> {
        let lock_name = LockName::try_from(lock_text)?;
        let application_name = ApplicationName::try_from(application_text)?;
        Ok(Self {
            lock_name,
            application_name,
            waiter_identifier,
            acquisition_mode,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn an_acquisition_keeps_its_lock_its_application_its_waiter_and_its_mode() {
        let acquisition_mode = AcquisitionMode::from((None, false, true));

        let acquisition = Acquisition::try_from((
            String::from("rtx3080"),
            String::from("winix"),
            7,
            acquisition_mode,
        ))
        .unwrap();

        assert_eq!(acquisition.lock_name().text(), "rtx3080");
        assert_eq!(acquisition.application_name().text(), "winix");
        assert_eq!(acquisition.waiter_identifier(), 7);
        assert_eq!(acquisition.acquisition_mode(), acquisition_mode);
    }

    #[test]
    fn an_acquisition_of_a_wrong_name_is_refused() {
        let acquisition_mode = AcquisitionMode::default();

        let outcome = Acquisition::try_from((
            String::from("rtx3080"),
            String::from("--force"),
            7,
            acquisition_mode,
        ));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }

    #[test]
    fn the_command_of_a_wrong_lock_is_refused() {
        let outcome = Acquisition::try_from((
            String::from("--lock"),
            String::from("winix"),
            7,
            AcquisitionMode::default(),
        ));

        assert!(matches!(outcome, Err(Error::Name { .. })));
    }
}
