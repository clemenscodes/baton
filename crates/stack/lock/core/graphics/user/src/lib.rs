use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;
use std::fmt;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum GraphicsUser {
    ProcessUser {
        process_identifier: u32,
        command_text: String,
        control_group_text: String,
    },
    ConsoleUser {
        console_index: u32,
    },
    FramebufferUser {
        device_text: String,
        driver_text: String,
    },
}

impl Layer for GraphicsUser {
    type Layer = DomainLayer;
}

impl ValueObject for GraphicsUser {}

impl GraphicsUser {
    pub fn maybe_process_identifier(&self) -> Option<u32> {
        match self {
            Self::ProcessUser {
                process_identifier, ..
            } => Some(*process_identifier),
            Self::ConsoleUser { .. } | Self::FramebufferUser { .. } => None,
        }
    }

    pub fn check_membership(&self, member_text: &str) -> bool {
        match self {
            Self::ProcessUser {
                control_group_text, ..
            } => {
                let member_prefix = format!("{member_text}/");
                control_group_text == member_text || control_group_text.starts_with(&member_prefix)
            }
            Self::ConsoleUser { .. } | Self::FramebufferUser { .. } => false,
        }
    }
}

impl fmt::Display for GraphicsUser {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ProcessUser {
                process_identifier,
                command_text,
                ..
            } => write!(formatter, "the process {process_identifier} {command_text}"),
            Self::ConsoleUser { console_index } => {
                write!(formatter, "the virtual console {console_index}")
            }
            Self::FramebufferUser {
                device_text,
                driver_text,
            } => write!(formatter, "the framebuffer {device_text} of {driver_text}"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fixture {
        use super::*;

        pub(crate) fn process_user_factory(control_group_text: &str) -> GraphicsUser {
            GraphicsUser::ProcessUser {
                process_identifier: 4242,
                command_text: String::from("resolve"),
                control_group_text: String::from(control_group_text),
            }
        }
    }

    #[test]
    fn a_process_belongs_to_its_control_group_and_to_every_ancestor_of_it() {
        let graphics_user = fixture::process_user_factory("/baton/rtx3080/resolve/render");

        let check_membership = graphics_user.check_membership("/baton/rtx3080/resolve");

        assert!(check_membership);
        assert!(graphics_user.check_membership("/baton/rtx3080/resolve/render"));
        assert!(!graphics_user.check_membership("/baton/rtx3080/res"));
        assert!(!graphics_user.check_membership("/baton/rtx3080/winix"));
    }

    #[test]
    fn only_a_process_names_its_identifier_and_belongs_to_a_control_group() {
        let graphics_user = [
            fixture::process_user_factory("/baton/rtx3080/resolve"),
            GraphicsUser::ConsoleUser { console_index: 1 },
            GraphicsUser::FramebufferUser {
                device_text: String::from("efi-framebuffer.0"),
                driver_text: String::from("efi-framebuffer"),
            },
        ];

        let maybe_process_identifier = graphics_user
            .clone()
            .map(|graphics_user| graphics_user.maybe_process_identifier());

        assert_eq!(maybe_process_identifier, [Some(4242), None, None]);
        assert!(!graphics_user[1].check_membership("/baton/rtx3080/resolve"));
        assert!(!graphics_user[2].check_membership("/baton/rtx3080/resolve"));
    }

    #[test]
    fn every_user_reads_as_a_phrase() {
        let graphics_user = [
            fixture::process_user_factory("/baton/rtx3080/resolve"),
            GraphicsUser::ConsoleUser { console_index: 1 },
            GraphicsUser::FramebufferUser {
                device_text: String::from("efi-framebuffer.0"),
                driver_text: String::from("efi-framebuffer"),
            },
        ];

        let text = graphics_user.map(|graphics_user| graphics_user.to_string());

        assert_eq!(
            text,
            [
                "the process 4242 resolve",
                "the virtual console 1",
                "the framebuffer efi-framebuffer.0 of efi-framebuffer",
            ]
        );
    }
}
