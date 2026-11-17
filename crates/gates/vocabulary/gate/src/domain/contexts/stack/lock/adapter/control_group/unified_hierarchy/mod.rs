use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterControlGroupUnifiedHierarchyContext;

impl Layer for StackLockAdapterControlGroupUnifiedHierarchyContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterControlGroupUnifiedHierarchyContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/control_group/unified_hierarchy";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack",
                "Lock",
                "Adapter",
                "Control",
                "Group",
                "Unified",
                "Hierarchy",
            ];
            let definition = "The crate stack_lock_adapter_control_group_unified_hierarchy, the adapter of the port ControlGroup of stack_lock_application over the files of the unified cgroup hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Control", "Group"];
            let definition = "The adapter of the port ControlGroup, which creates, reads, fills and removes the cgroups of the eligible applications below the root of the unified hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Control", "Group", "Port"];
            let definition = "The port ControlGroup as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group", "Configuration"];
            let definition = "The root of the unified cgroup hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The group configuration of the adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hierarchy", "Directory"];
            let definition = "The root of the unified cgroup hierarchy, /sys/fs/cgroup unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Group", "Path"];
            let definition = "To compose the directory of a cgroup below the root of the unified hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Relative", "Text"];
            let definition = "The path of a cgroup without its leading slash.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Events", "Path"];
            let definition = "The file cgroup.events of a cgroup, which tells whether it holds a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Events", "Name"];
            let definition = "The name cgroup.events of the file that tells whether a cgroup holds a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Events", "Text"];
            let definition = "The text of the file cgroup.events of a cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Population", "Line"];
            let definition = "The line populated 1 of a cgroup that holds a process.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Processes", "Path"];
            let definition = "The file cgroup.procs of a cgroup, which a process is moved into by writing its identifier.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Processes", "Name"];
            let definition = "The name cgroup.procs of the file a process is moved into.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Error"];
            let definition = "To turn an error of the file system into an error of a cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Group"];
            let definition = "The error of a cgroup that could not be created, read, filled or removed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Metadata"];
            let definition = "The metadata of the directory of a cgroup, whose inode is the identifier of the cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Line"];
            let definition = "A line of the file cgroup.events.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a file of a cgroup in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Enter", "Group"];
            let definition = "To write the own process into the processes of a cgroup.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterControlGroupUnifiedHierarchyContext> for Context {
    type Error = Error;

    fn try_from(
        _: StackLockAdapterControlGroupUnifiedHierarchyContext,
    ) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterControlGroupUnifiedHierarchyContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterControlGroupUnifiedHierarchyContext::TERM_TEXTS;
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
    fn every_definition_of_the_stack_lock_adapter_control_group_unified_hierarchy_context_is_a_term()
     {
        let context_type = StackLockAdapterControlGroupUnifiedHierarchyContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
