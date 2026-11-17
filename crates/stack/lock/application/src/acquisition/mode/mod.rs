use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub struct AcquisitionMode {
    maybe_deadline: Option<Duration>,
    force_presence: bool,
    passthrough_presence: bool,
}

impl Layer for AcquisitionMode {
    type Layer = ApplicationLayer;
}

impl AcquisitionMode {
    pub fn maybe_deadline(&self) -> Option<Duration> {
        self.maybe_deadline
    }

    pub fn force_presence(&self) -> bool {
        self.force_presence
    }

    pub fn passthrough_presence(&self) -> bool {
        self.passthrough_presence
    }
}

impl From<(Option<Duration>, bool, bool)> for AcquisitionMode {
    fn from(
        (maybe_deadline, force_presence, passthrough_presence): (Option<Duration>, bool, bool),
    ) -> Self {
        Self {
            maybe_deadline,
            force_presence,
            passthrough_presence,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_mode_keeps_its_deadline_its_force_and_its_passthrough() {
        let maybe_deadline = Some(Duration::from_millis(500));

        let acquisition_mode = AcquisitionMode::from((maybe_deadline, true, false));

        assert_eq!(acquisition_mode.maybe_deadline(), maybe_deadline);
        assert!(acquisition_mode.force_presence());
        assert!(!acquisition_mode.passthrough_presence());
    }
}
