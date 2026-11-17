use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Tokio;

impl Layer for Tokio {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Tokio {}

impl Tokio {
    const NAME: &'static str = "tokio";

    const NAMES: &'static [&'static str] = &[
        "biased",
        "block_on",
        "blocking_recv",
        "blocking_send",
        "build",
        "Builder",
        "channel",
        "Closed",
        "enable_io",
        "enable_time",
        "enter",
        "flavor",
        "error",
        "Handle",
        "interval",
        "join",
        "Interval",
        "interval_at",
        "JoinError",
        "metrics",
        "mpsc",
        "new_current_thread",
        "new_multi_thread",
        "num_alive_tasks",
        "oneshot",
        "Receiver",
        "recv",
        "RecvError",
        "runtime",
        "Runtime",
        "select",
        "send",
        "Sender",
        "SendError",
        "send_replace",
        "sleep_until",
        "spawn",
        "spawn_blocking",
        "start_paused",
        "sync",
        "task",
        "tokio",
        "try_current",
        "TryCurrentError",
        "wait_for",
        "watch",
        "worker_threads",
        "yield_now",
    ];
}

impl TryFrom<Tokio> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Tokio) -> Result<Self, Self::Error> {
        let foreign_name_text = Tokio::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Tokio::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_tokio_is_a_name() {
        let tokio = Tokio;

        let foreign_language = ForeignLanguage::try_from(tokio);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
