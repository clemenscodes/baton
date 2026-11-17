use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};
use stack_lock_core_graphics_user::GraphicsUser;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(crate) enum AcquisitionStep {
    Ownership,
    Displacement { process_users: Vec<GraphicsUser> },
    Wait { user_text: String },
}

impl Layer for AcquisitionStep {
    type Layer = ApplicationLayer;
}
