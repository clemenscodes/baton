use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockCoreGraphicsUserContext;

impl Layer for StackLockCoreGraphicsUserContext {
    type Layer = DomainLayer;
}

impl StackLockCoreGraphicsUserContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/core/graphics/user";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Graphics", "User"];
            let definition = "The crate stack_lock_core_graphics_user, a user of a GPU, a process that holds one of its device files or a console of the kernel drawn on it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "User"];
            let definition = "Something that holds a GPU, a process with an open device file of it, a virtual console drawn on its framebuffer, or a framebuffer of the firmware on it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process", "User"];
            let definition = "A process that holds a device file of a GPU, with its identifier, its command and its cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Console", "User"];
            let definition = "A virtual console of the kernel drawn on a framebuffer of a GPU.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Framebuffer", "User"];
            let definition = "A framebuffer of the firmware on a GPU, such as efi-framebuffer.0, bound to its driver.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Process", "Identifier"];
            let definition = "The identifier of a process of the host.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Command", "Text"];
            let definition = "The command of a process, as /proc names it in comm.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Control", "Group", "Text"];
            let definition = "The path of the cgroup of a process below the root of the unified hierarchy, such as /baton/rtx3080/resolve.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Console", "Index"];
            let definition = "The index of a virtual console of the kernel under /sys/class/vtconsole, such as 1 for vtcon1.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Text"];
            let definition = "The name of the device of a framebuffer of the firmware, such as efi-framebuffer.0.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Driver", "Text"];
            let definition = "The name of the driver a framebuffer of the firmware is bound to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Process", "Identifier"];
            let definition = "The identifier of the process of a graphics user, where it is a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Membership"];
            let definition = "To tell whether a process lives in a cgroup or below it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Member", "Text"];
            let definition = "The path of the cgroup a process is checked against.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Member", "Prefix"];
            let definition = "The path of a cgroup with a slash after it, which every descendant of it begins with.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Text"];
            let definition = "The text of a graphics user.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockCoreGraphicsUserContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockCoreGraphicsUserContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockCoreGraphicsUserContext::DIRECTORY_NAME;
        let term_texts = StackLockCoreGraphicsUserContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let foreign_name = ForeignName::from(standard_library_name);
        let foreign_names = vec![foreign_name];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_stack_lock_core_graphics_user_context_is_a_term() {
        let context_type = StackLockCoreGraphicsUserContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
