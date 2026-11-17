use crate::acquisition::mode::AcquisitionMode;
use stack_domain_driven_design_core::layer::{ApplicationLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct GuardRun {
    lock_text: String,
    application_text: String,
    acquisition_mode: AcquisitionMode,
    program_texts: Vec<String>,
}

impl Layer for GuardRun {
    type Layer = ApplicationLayer;
}

impl GuardRun {
    pub fn lock_text(&self) -> &str {
        &self.lock_text
    }

    pub fn application_text(&self) -> &str {
        &self.application_text
    }

    pub fn acquisition_mode(&self) -> AcquisitionMode {
        self.acquisition_mode
    }

    pub fn program_texts(&self) -> &[String] {
        &self.program_texts
    }
}

impl From<(String, String, AcquisitionMode, Vec<String>)> for GuardRun {
    fn from(
        (lock_text, application_text, acquisition_mode, program_texts): (
            String,
            String,
            AcquisitionMode,
            Vec<String>,
        ),
    ) -> Self {
        Self {
            lock_text,
            application_text,
            acquisition_mode,
            program_texts,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_guard_run_keeps_its_lock_its_application_its_mode_and_its_program() {
        let acquisition_mode = AcquisitionMode::default();
        let program_texts = vec![String::from("davinci-resolve")];

        let guard_run = GuardRun::from((
            String::from("rtx3080"),
            String::from("resolve"),
            acquisition_mode,
            program_texts.clone(),
        ));

        assert_eq!(guard_run.lock_text(), "rtx3080");
        assert_eq!(guard_run.application_text(), "resolve");
        assert_eq!(guard_run.acquisition_mode(), acquisition_mode);
        assert_eq!(guard_run.program_texts(), program_texts.as_slice());
    }
}
