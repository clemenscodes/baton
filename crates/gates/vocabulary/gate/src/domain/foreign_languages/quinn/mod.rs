use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(super) struct Quinn;

impl Layer for Quinn {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for Quinn {}

impl Quinn {
    const NAME: &'static str = "quinn";

    const NAMES: &'static [&'static str] = &[
        "accept",
        "accept_bi",
        "client",
        "ClientConfig",
        "connect",
        "ConnectError",
        "Connection",
        "ConnectionError",
        "crypto",
        "Endpoint",
        "keep_alive_interval",
        "local_addr",
        "NoInitialCipherSuite",
        "open_bi",
        "QuicClientConfig",
        "QuicServerConfig",
        "quinn",
        "read_exact",
        "ReadExactError",
        "RecvStream",
        "rustls",
        "SendStream",
        "server",
        "ServerConfig",
        "set_default_client_config",
        "transport_config",
        "TransportConfig",
        "with_crypto",
        "write_all",
        "WriteError",
    ];
}

impl TryFrom<Quinn> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: Quinn) -> Result<Self, Self::Error> {
        let foreign_name_text = Quinn::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = Quinn::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_quinn_is_a_name() {
        let foreign_crate = Quinn;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
