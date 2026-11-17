use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct AsynchronousChannel;

impl Layer for AsynchronousChannel {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for AsynchronousChannel {}

impl AsynchronousChannel {
    const NAME: &'static str = "async-channel";

    const NAMES: &'static [&'static str] = &[
        "async_channel",
        "bounded",
        "close",
        "Closed",
        "Empty",
        "force_send",
        "Full",
        "into_inner",
        "is_closed",
        "is_full",
        "Receiver",
        "receiver_count",
        "recv",
        "recv_blocking",
        "RecvError",
        "send",
        "send_blocking",
        "Sender",
        "SendError",
        "try_recv",
        "TryRecvError",
        "try_send",
        "TrySendError",
        "unbounded",
    ];
}

impl TryFrom<AsynchronousChannel> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: AsynchronousChannel) -> Result<Self, Self::Error> {
        let foreign_name_text = AsynchronousChannel::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = AsynchronousChannel::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_asynchronous_channel_is_a_name() {
        let asynchronous_channel = AsynchronousChannel;

        let foreign_language = ForeignLanguage::try_from(asynchronous_channel);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
