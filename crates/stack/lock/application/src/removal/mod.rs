use crate::Error;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_lock_core_name::LockName;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Removal {
    lock_name: LockName,
}

impl Layer for Removal {
    type Layer = ApplicationLayer;
}

impl Removal {
    pub fn lock_name(&self) -> &LockName {
        &self.lock_name
    }
}

impl TryFrom<String> for Removal {
    type Error = Error;

    fn try_from(lock_text: String) -> Result<Self, Self::Error> {
        let lock_name = LockName::try_from(lock_text)?;
        Ok(Self { lock_name })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_removal_keeps_its_lock() {
        let removal = Removal::try_from(String::from("rtx3080")).unwrap();

        assert_eq!(removal.lock_name().text(), "rtx3080");
        assert!(Removal::try_from(String::new()).is_err());
    }
}
