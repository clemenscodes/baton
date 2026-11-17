use crate::domain::foreign_languages::berkeley_packet_filter::BerkeleyPacketFilter;
use crate::domain::foreign_languages::standard_library::StandardLibrary;
use ::gates_vocabulary_language_core_foreign_name::ForeignName;
use ::gates_vocabulary_language_core_language::TermText;
use ::gates_vocabulary_language_core_languages::Error;
use ::gates_vocabulary_language_core_languages::contexts::Context;
use ::stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct StackLockAdapterDeviceProgramBerkeleyPacketFilterContext;

impl Layer for StackLockAdapterDeviceProgramBerkeleyPacketFilterContext {
    type Layer = DomainLayer;
}

impl StackLockAdapterDeviceProgramBerkeleyPacketFilterContext {
    const DIRECTORY_NAME: &'static str = "stack/lock/adapter/device/program/berkeley_packet_filter";

    const TERM_TEXTS: &'static [TermText] = &[
        {
            let spelling: &[&str] = &[
                "Stack", "Lock", "Adapter", "Device", "Program", "Berkeley", "Packet", "Filter",
            ];
            let definition = "The crate stack_lock_adapter_device_program_berkeley_packet_filter, the adapter of the port DeviceProgram of stack_lock_application over the system call bpf and the unified cgroup hierarchy, with no library of bpf.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Program", "Port"];
            let definition = "The port DeviceProgram as the adapter imports it.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Configuration"];
            let definition = "The root of the unified hierarchy, the directory of the pins and the depth of the ancestry.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Configuration"];
            let definition = "The program configuration of the adapter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hierarchy", "Directory"];
            let definition = "The root of the unified cgroup hierarchy the device program is attached to, /sys/fs/cgroup unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pin", "Directory"];
            let definition = "The directory of bpffs the maps and the program are pinned under, /sys/fs/bpf/baton unless an option names another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Ancestry", "Depth"];
            let definition = "The number of levels of cgroups the device program checks for a permission, 16 by default.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Instruction"];
            let definition = "One instruction of the device program, eight bytes of an operation, two registers, an offset and an operand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Operation"];
            let definition = "The operation byte of an instruction.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Destination"];
            let definition = "The destination register of an instruction.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Source"];
            let definition = "The source register of an instruction.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Offset"];
            let definition = "The offset of an instruction, from the base of a register or to a jump.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Operand"];
            let definition = "The immediate operand of an instruction.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Load", "Word"];
            let definition = "The operation that loads a word from the memory of a register.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Store", "Word"];
            let definition = "The operation that stores a word into the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Store", "Double", "Word"];
            let definition = "The operation that stores a double word into the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Store", "Word", "Immediate"];
            let definition = "The operation that stores an immediate word into the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Move", "Register"];
            let definition = "The operation that moves a register into another.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Move", "Immediate"];
            let definition = "The operation that moves an immediate into a register.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Add", "Immediate"];
            let definition = "The operation that adds an immediate to a register.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["And", "Immediate"];
            let definition = "The operation that masks a register with an immediate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Load", "Double", "Word"];
            let definition = "The operation that loads a double word immediate, in two instructions, which carries the descriptor of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Call", "Helper"];
            let definition = "The operation that calls a helper of the kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Equality", "Jump"];
            let definition = "The operation that jumps when a register equals an immediate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Inequality", "Jump"];
            let definition = "The operation that jumps when a register differs from an immediate.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Exit"];
            let definition = "The operation that ends the device program with the verdict in the first register.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context", "Register"];
            let definition = "The register that holds the context of the device program, the access the kernel asks about.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack", "Register"];
            let definition = "The register that holds the stack of the device program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Map", "Source"];
            let definition = "The source that marks a load of a double word as the descriptor of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose"];
            let definition = "To make an instruction of its operation, its registers, its offset and its operand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Bytes"];
            let definition = "To compose the bytes of an instruction or of the device program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Bytes"];
            let definition = "The bytes of an instruction, of the device program or of an attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Register"];
            let definition = "The byte of an instruction that holds its source and its destination.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Offset", "Bytes"];
            let definition = "The two bytes of the offset of an instruction.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Operand", "Bytes"];
            let definition = "The four bytes of the operand of an instruction.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Filter"];
            let definition = "The program of the type of a device filter of a cgroup, assembled by hand, that reads the map of the devices and the map of the permissions and answers whether a cgroup may open a device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Instructions"];
            let definition = "The instructions of the device filter.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Map", "Lookup", "Helper"];
            let definition = "The helper of the kernel that looks a key up in a map, helper 1.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Ancestor", "Helper"];
            let definition = "The helper of the kernel that answers the identifier of the ancestor of the task at a level, helper 123.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Type", "Mask"];
            let definition = "The mask that keeps the kind of a device out of the access type.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Key", "Offset"];
            let definition = "The offset of the key of a device on the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Key", "Offset"];
            let definition = "The offset of the key of a permission on the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Offset"];
            let definition = "The offset of the lock of a device on the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Padding", "Offset"];
            let definition = "The offset of the padding of the key of a permission on the stack.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Allowance"];
            let definition = "The verdict that allows an access, one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Denial"];
            let definition = "The verdict that denies an access, zero.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Jump", "To", "Allowance"];
            let definition = "To make a jump from a position to the instruction of the allowance.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Load", "Map"];
            let definition = "The two instructions that load the descriptor of a map into a register.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Descriptor"];
            let definition = "The descriptor of the map of the devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Descriptor"];
            let definition = "The descriptor of the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Context"];
            let definition = "The register that holds the context of the device program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Stack"];
            let definition = "The register that holds the stack of the device program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Jumps"];
            let definition = "The positions of the jumps to the allowance, filled once the allowance is placed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Jump", "Position"];
            let definition = "The position of a jump to the allowance.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Allowance", "Position"];
            let definition = "The position of the instruction of the allowance.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Level"];
            let definition = "The level of an ancestor of the task the device program checks.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Level", "Operand"];
            let definition = "The level of an ancestor as an operand.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Distance"];
            let definition = "The offset of a jump from its position to the allowance.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Attribute"];
            let definition = "The attribute of a system call bpf, a buffer the kernel reads the arguments of the call from.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["License"];
            let definition = "The license the device program is loaded under.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Key", "Size"];
            let definition = "The size of the key of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value", "Size"];
            let definition = "The size of the value of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Entry", "Limit"];
            let definition = "The highest number of entries of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Descriptor"];
            let definition = "The descriptor of a map, of the program or of a cgroup.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Word"];
            let definition = "The descriptor of an object as a word of the attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Address"];
            let definition = "The address of a buffer as a word of the attribute.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Key", "Size"];
            let definition = "The size of the key of the map of the devices, twelve.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Key", "Size"];
            let definition = "The size of the key of the map of the permissions, sixteen.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Size"];
            let definition = "The size of the value of the map of the devices, the lock, four.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presence", "Size"];
            let definition = "The size of the value of the map of the permissions, one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Limit"];
            let definition = "The highest number of device files of the map of the devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Limit"];
            let definition = "The highest number of cgroups of the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Character", "Type"];
            let definition = "The kind two of a character device in the key of a device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Block", "Type"];
            let definition = "The kind one of a block device in the key of a device.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Absence", "Code"];
            let definition = "The error number two of the kernel for an object or a key that is absent.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Pin"];
            let definition = "The name of the pin of the map of the devices, devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Pin"];
            let definition = "The name of the pin of the map of the permissions, permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Program", "Pin"];
            let definition = "The name of the pin of the device program, program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Check", "Program"];
            let definition = "To tell whether the device program is already pinned, so it is installed once.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Create", "Map"];
            let definition = "To create a map through the system call bpf.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Load", "Program"];
            let definition = "To compose the device filter and load it through the system call bpf.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Open", "Hierarchy"];
            let definition = "To open the root of the unified hierarchy as a directory the device program is attached to.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Open", "Object"];
            let definition = "To open a pinned map or program at a path of bpffs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Delete"];
            let definition = "To delete a key of a map and take a key that is already absent as no error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Device", "Key"];
            let definition = "To compose the key of a device file, its kind, its major and its minor.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Compose", "Permission", "Key"];
            let definition = "To compose the key of a permission, the identifier of a cgroup and the lock.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Key"];
            let definition = "The key of a device file in the map of the devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Key"];
            let definition = "The key of a cgroup and a lock in the map of the permissions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Lock", "Value"];
            let definition = "The lock a device file belongs to, the value in the map of the devices.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Presence", "Value"];
            let definition = "The value of a permitted cgroup in the map of the permissions, one.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pin", "Path"];
            let definition = "The path of a pin of bpffs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Operation", "Text"];
            let definition = "The name of the operation of a system call bpf in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Code"];
            let definition = "The bytes of the device program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Map"];
            let definition = "The attribute of a creation of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Pin"];
            let definition = "The attribute of a pin of an object.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hierarchy", "File"];
            let definition = "The open directory of the root of the unified hierarchy.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Call"];
            let definition = "The error of a system call bpf that failed.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Hierarchy"];
            let definition = "The error of the root of the unified hierarchy that could not be opened.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Path"];
            let definition = "The path of a pin or of the root in an error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Position"];
            let definition = "The position of an instruction in the device program.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Kernel"];
            let definition = "The seam of the kernel over the eight operations of bpf the adapter needs, so the adapter is proven without root.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Kernel", "Type"];
            let definition = "The type of the device kernel a device program holds.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["System", "Kernel"];
            let definition = "The device kernel over the crate berkeley_packet_filter, the real system calls of bpf.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Device", "Map"];
            let definition = "The map of the devices, opened at its pin for an insertion or a deletion.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Permission", "Map"];
            let definition = "The map of the permissions, opened at its pin for a permission or a revocation.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Cgroup"];
            let definition = "The identifier of a control group of the unified hierarchy a permission admits.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Object"];
            let definition = "A pinned map or program the adapter fastens or opens at a path of bpffs.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Key"];
            let definition = "The bytes of a key of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Value"];
            let definition = "The bytes of a value of a map.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Create", "Hash", "Map"];
            let definition = "To create a hash map through the device kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Update", "Element"];
            let definition = "To update an element of a map through the device kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Delete", "Element"];
            let definition = "To delete an element of a map through the device kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Delete", "Key"];
            let definition = "To delete a key of a map and take a key that is already absent as no error.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Load", "Device", "Program"];
            let definition = "To compose the device filter and load it as the device program through the device kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Fasten", "Object"];
            let definition = "To fasten a map or the program at a path of bpffs, so a later run finds it again.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Attach", "Device", "Program"];
            let definition = "To attach the device program to the root of the unified hierarchy through the device kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Try", "Detach", "Device", "Program"];
            let definition = "To detach the device program from the root of the unified hierarchy through the device kernel.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Filter"];
            let definition = "The module of the device filter, the device program as its instructions.";
            TermText::new(spelling, definition)
        },
        {
            let spelling: &[&str] = &["Kernel"];
            let definition = "The module of the device kernel, the seam of the calls of the kernel.";
            TermText::new(spelling, definition)
        },
    ];
}

impl TryFrom<StackLockAdapterDeviceProgramBerkeleyPacketFilterContext> for Context {
    type Error = Error;

    fn try_from(
        _: StackLockAdapterDeviceProgramBerkeleyPacketFilterContext,
    ) -> Result<Self, Self::Error> {
        let directory_name = StackLockAdapterDeviceProgramBerkeleyPacketFilterContext::DIRECTORY_NAME;
        let term_texts = StackLockAdapterDeviceProgramBerkeleyPacketFilterContext::TERM_TEXTS;
        let standard_library_name = StandardLibrary::NAME;
        let standard_library_foreign_name = ForeignName::from(standard_library_name);
        let berkeley_packet_filter_name = BerkeleyPacketFilter::NAME;
        let berkeley_packet_filter_foreign_name = ForeignName::from(berkeley_packet_filter_name);
        let foreign_names = vec![
            standard_library_foreign_name,
            berkeley_packet_filter_foreign_name,
        ];
        Self::try_from((directory_name, term_texts, foreign_names))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_definition_of_the_stack_lock_adapter_device_program_berkeley_packet_filter_context_is_a_term()
     {
        let context_type = StackLockAdapterDeviceProgramBerkeleyPacketFilterContext;

        let context = Context::try_from(context_type);

        let error = context.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
