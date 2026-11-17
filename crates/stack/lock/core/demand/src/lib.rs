use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use stack_lock_core_name::ApplicationName;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GraphicsDemand {
    application_name: ApplicationName,
    waiter_identifier: u32,
}

impl Layer for GraphicsDemand {
    type Layer = DomainLayer;
}

impl ValueObject for GraphicsDemand {}

impl GraphicsDemand {
    pub fn application_name(&self) -> &ApplicationName {
        &self.application_name
    }

    pub fn waiter_identifier(&self) -> u32 {
        self.waiter_identifier
    }
}

impl From<(ApplicationName, u32)> for GraphicsDemand {
    fn from((application_name, waiter_identifier): (ApplicationName, u32)) -> Self {
        Self {
            application_name,
            waiter_identifier,
        }
    }
}

impl fmt::Display for GraphicsDemand {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} for the process {}",
            self.application_name, self.waiter_identifier
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_demand_keeps_its_application_and_its_process() {
        let application_name = ApplicationName::try_from(String::from("winix")).unwrap();

        let graphics_demand = GraphicsDemand::from((application_name.clone(), 4242));

        assert_eq!(graphics_demand.application_name(), &application_name);
        assert_eq!(graphics_demand.waiter_identifier(), 4242);
        assert_eq!(graphics_demand.to_string(), "winix for the process 4242");
    }
}
