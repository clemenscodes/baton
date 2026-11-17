use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockCoreGraphicsLockContext;

impl Layer for StackLockCoreGraphicsLockContext {
    type Layer = DomainLayer;
}

impl StackLockCoreGraphicsLockContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/core/graphics/lock";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &["Stack", "Lock", "Core", "Graphics", "Lock"];
            let definition = "The crate stack_lock_core_graphics_lock, the lock of a GPU, the aggregate that holds its GPUs, its eligible applications, its owner and its queue of demands.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Lock"];
            let definition = "The lock of a GPU, the mutex the kernel enforces, with its name, its index in the maps of the device program, its devices, its device files, its eligible applications, its default owner, its owner, its queue of demands, the consoles it unbound and its passthrough.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Map", "Index"];
            let definition = "The number of a lock in the maps of the device program, which tells the locks of one host apart.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Addresses"];
            let definition = "The PCI addresses of the GPUs a lock holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Files"];
            let definition = "The device files of the GPUs of a lock the device program locks.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Names"];
            let definition = "The eligible applications of a lock, every application that may use its GPU at all.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Default", "Owner"];
            let definition = "The eligible application that holds a lock whenever no other owns it, such as the desktop of a system with a single GPU, where there is one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Default", "Owner"];
            let definition = "The eligible application that holds a lock whenever no other owns it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Owner"];
            let definition = "The eligible application that owns a lock, where one does.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Graphics", "Demands"];
            let definition = "The queue of the demands of a lock, in the order they were made.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Console", "Users"];
            let definition = "The consoles and the framebuffers of the firmware an acquisition unbound from the GPU of a lock, which its release binds again.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Passthrough", "Presence"];
            let definition = "Whether the GPU of a lock is handed to vfio-pci for a guest.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Persistence", "Presence"];
            let definition = "Whether the persistence mode of an NVIDIA GPU was on before its passthrough, so its release turns it on again.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "Permitted", "Application"];
            let definition = "The application the device program permits to open the GPU of a lock, its owner or else its default owner, where there is one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Maybe", "First", "Demand"];
            let definition = "The demand a lock serves next, where it has one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Path", "Text"];
            let definition = "The path of the cgroup of an eligible application below the root of the unified hierarchy, /baton, then the name of the lock and then the name of the application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tree", "Path", "Text"];
            let definition = "The path of the tree of the cgroups of a lock below the root of the unified hierarchy, /baton and then the name of the lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Tree", "Name"];
            let definition = "The name of the tree of every cgroup of baton below the root of the unified hierarchy, baton.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Eligibility"];
            let definition = "To tell whether an application is eligible for a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Add", "Device"];
            let definition = "To add a GPU to a lock, refusing one it already holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Replace", "Device", "Files"];
            let definition = "To set the device files a lock locks and to answer the files it no longer locks.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Previous", "Files"];
            let definition = "The device files a lock locked before a replacement.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Admit"];
            let definition = "To make an application eligible for a lock, as its default owner where asked, refusing a second default owner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Default", "Presence"];
            let definition = "Whether an admitted application becomes the default owner of its lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Dismiss"];
            let definition = "To make an application ineligible again with its demands and its default, refusing the owner.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Eligible", "Name"];
            let definition = "The name of an eligible application of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Record", "Demand"];
            let definition = "To put the demand of an eligible application into the queue of a lock, once for every process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Repetition", "Presence"];
            let definition = "Whether the queue already holds a demand of the same process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Recorded", "Demand"];
            let definition = "A demand the queue of a lock holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Drop", "Demand"];
            let definition = "To take the demand of a process out of the queue of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Retain", "Demands"];
            let definition = "To keep in the queue only the demands whose process still lives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Check", "Presence"];
            let definition = "To tell whether a process still lives.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Assign", "Demand", "Owner"];
            let definition = "To make the application of a queued demand the owner of a lock and take the demand out of the queue, and to leave a demand that is not queued alone.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Release"];
            let definition = "To end the ownership of the owner of a lock, refusing any other application.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Clear", "Owner"];
            let definition = "To end the ownership of an owner that ended without a release.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Console", "Users"];
            let definition = "To remember the consoles and the framebuffers an acquisition unbound, once each.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Remove", "Console", "Users"];
            let definition = "To empty the record of the consoles and the framebuffers and answer them, which a release binds again.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Record", "Passthrough"];
            let definition = "To remember that the GPU of a lock is handed to vfio-pci, and whether its persistence mode was on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Clear", "Passthrough"];
            let definition = "To forget the passthrough of a lock and answer whether its persistence mode was on.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Check", "Application"];
            let definition = "To refuse an application that is not eligible for a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Text"];
            let definition = "The name of a lock as a text in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Text"];
            let definition = "The name of an application as a text in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Owner", "Text"];
            let definition = "The owner of a lock as a text in an error, or nobody.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Address", "Text"];
            let definition = "The PCI address of a device as a text in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Repetition"];
            let definition = "The error of a device a lock already holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Application", "Absence"];
            let definition = "The error of an application that is not eligible for a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Default", "Owner", "Conflict"];
            let definition = "The error of a second default owner of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Owner", "Conflict"];
            let definition = "The error of a dismissal of the owner of a lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Owner", "Mismatch"];
            let definition = "The error of a release by an application that does not own the lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Recover", "Owner"];
            let definition = "To set the owner of a lock from its record, refusing an application that is not eligible.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockCoreGraphicsLockContext> for Context {
    type Error = Error;

    fn try_from(_: StackLockCoreGraphicsLockContext) -> Result<Self, Self::Error> {
        let directory_name = StackLockCoreGraphicsLockContext::DIRECTORY_NAME;
        let term_texts = StackLockCoreGraphicsLockContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_core_graphics_lock_context_is_a_term() {
        let context_type = StackLockCoreGraphicsLockContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
