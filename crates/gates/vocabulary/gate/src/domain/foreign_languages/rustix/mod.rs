use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Rustix;

impl Layer for Rustix {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Rustix {}

impl Rustix {
    const NAME: &'static str = "rustix";

    const NAMES: &'static [&'static str] = &[
        "CWD",
        "Fifo",
        "FileType",
        "flock",
        "major",
        "minor",
        "mknodat",
        "Mode",
        "RUSR",
        "WUSR",
        "FlockOperation",
        "from_child",
        "from_raw",
        "fs",
        "KILL",
        "LockExclusive",
        "kill_process",
        "Pid",
        "process",
        "rustix",
        "Signal",
        "TERM",
    ];
}

impl TryFrom<Rustix> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Rustix) -> Result<Self, Self::Error> {
        let foreign_name_text = Rustix::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Rustix::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_rustix_is_a_name() {
        let rustix = Rustix;

        let foreign_language = ForeignLanguage::try_from(rustix);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
