use stack_domain_driven_design_core::layer::{DomainLayer, Layer};
use stack_domain_driven_design_core::value_object::ValueObject;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy)]
pub enum MappingPresence {
    Presence,
    Absence,
}

impl Layer for MappingPresence {
    type Layer = DomainLayer;
}

impl ValueObject for MappingPresence {}

impl MappingPresence {
    const MAPPING_SECTION_NAMES: [&'static [u8]; 2] = [b"__llvm_covmap", b".lcovmap"];
}

impl From<&[u8]> for MappingPresence {
    fn from(inspection_output: &[u8]) -> Self {
        let mapping_section_criterion = |word: &[u8]| Self::MAPPING_SECTION_NAMES.contains(&word);
        let mut words = inspection_output.split(u8::is_ascii_whitespace);
        if words.any(mapping_section_criterion) {
            Self::Presence
        } else {
            Self::Absence
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn section_headers_with_the_coverage_mapping_of_an_elf_executable_are_its_presence() {
        let inspection_text = r#"/target/debug/deps/harbor-0a1b:	file format elf64-x86-64
Sections:
Idx Name               Size     VMA              Type
 43 __llvm_covfun      000cc515 0000000000000000
 44 __llvm_covmap      0001f844 0000000000000000
"#;
        let inspection_output = inspection_text.as_bytes();

        let mapping_presence = MappingPresence::from(inspection_output);

        assert_eq!(mapping_presence, MappingPresence::Presence);
    }

    #[test]
    fn section_headers_with_the_coverage_mapping_of_a_coff_executable_are_its_presence() {
        let inspection_text = r#"C:\target\debug\deps\harbor-0a1b.exe:	file format coff-x86-64
Sections:
Idx Name          Size     VMA              Type
  5 .lcovmap      0001f844 0000000140100000 DATA
"#;
        let inspection_output = inspection_text.as_bytes();

        let mapping_presence = MappingPresence::from(inspection_output);

        assert_eq!(mapping_presence, MappingPresence::Presence);
    }

    #[test]
    fn section_headers_of_an_executable_without_a_region_are_the_absence_of_its_mapping() {
        let inspection_text = r#"/target/debug/deps/harbor-0a1b:	file format elf64-x86-64
Sections:
Idx Name               Size     VMA              Type
 14 __llvm_prf_names   000509d3 000000000006c274 DATA
 17 .text              0010b272 00000000000e2a80 TEXT
"#;
        let inspection_output = inspection_text.as_bytes();

        let mapping_presence = MappingPresence::from(inspection_output);

        assert_eq!(mapping_presence, MappingPresence::Absence);
    }

    #[test]
    fn an_empty_inspection_is_the_absence_of_a_mapping() {
        let inspection_output: &[u8] = &[];

        let mapping_presence = MappingPresence::from(inspection_output);

        assert_eq!(mapping_presence, MappingPresence::Absence);
    }
}
