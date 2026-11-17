use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use std::time::Duration;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub struct LockSetting {
    check_interval: Duration,
    end_deadline: Duration,
}

impl Layer for LockSetting {
    type Layer = ApplicationLayer;
}

impl LockSetting {
    pub fn check_interval(&self) -> Duration {
        self.check_interval
    }

    pub fn end_deadline(&self) -> Duration {
        self.end_deadline
    }
}

impl From<(Duration, Duration)> for LockSetting {
    fn from((check_interval, end_deadline): (Duration, Duration)) -> Self {
        Self {
            check_interval,
            end_deadline,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_setting_keeps_its_interval_and_its_deadline() {
        let check_interval = Duration::from_millis(200);
        let end_deadline = Duration::from_millis(10000);

        let lock_setting = LockSetting::from((check_interval, end_deadline));

        assert_eq!(lock_setting.check_interval(), check_interval);
        assert_eq!(lock_setting.end_deadline(), end_deadline);
    }
}
