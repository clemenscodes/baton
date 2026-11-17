use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::ffi::{OsStr, OsString};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct Variable {
    name: OsString,
    value: OsString,
}

impl Layer for Variable {
    type Layer = DomainLayer;
}

impl ValueObject for Variable {}

impl Variable {
    pub fn name(&self) -> &OsStr {
        &self.name
    }

    pub fn value(&self) -> &OsStr {
        &self.value
    }
}

impl From<(OsString, OsString)> for Variable {
    fn from((name, value): (OsString, OsString)) -> Self {
        Self { name, value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_variable_keeps_its_name_and_its_value() {
        let name = OsString::from("LLVM_PROFILE_FILE");
        let value = OsString::from("/tmp/profiles/%p-%m.profraw");

        let variable = Variable::from((name, value));

        let name_expectation = OsStr::new("LLVM_PROFILE_FILE");
        let value_expectation = OsStr::new("/tmp/profiles/%p-%m.profraw");
        assert_eq!(variable.name(), name_expectation);
        assert_eq!(variable.value(), value_expectation);
    }
}
