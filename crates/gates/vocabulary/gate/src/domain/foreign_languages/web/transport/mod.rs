use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct WebTransport;

impl Layer for WebTransport {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for WebTransport {}

impl WebTransport {
    const NAME: &'static str = "wtransport";

    const NAMES: &'static [&'static str] = &[
        "accept",
        "accept_bi",
        "alpn_protocols",
        "builder_with_provider",
        "CertificateDer",
        "Client",
        "client",
        "ClientConfig",
        "close",
        "ConnectingError",
        "crypto",
        "default_provider",
        "der",
        "Endpoint",
        "endpoint_side",
        "keep_alive_interval",
        "max_concurrent_bidi_streams",
        "NotConnected",
        "open_bi",
        "pki_types",
        "PrivateKeyDer",
        "PrivatePkcs8KeyDer",
        "quinn",
        "RecvStream",
        "ring",
        "rustls",
        "secret_der",
        "self_signed",
        "SendStream",
        "Server",
        "server",
        "ServerConfig",
        "Sha256Digest",
        "StreamOpeningError",
        "StreamReadExactError",
        "StreamWriteError",
        "tls",
        "TLS13",
        "TransportConfig",
        "VarInt",
        "version",
        "WEBTRANSPORT_ALPN",
        "with_bind_address",
        "with_bind_default",
        "with_custom_tls",
        "with_custom_transport",
        "with_no_client_auth",
        "with_protocol_versions",
        "with_server_certificate_hashes",
        "with_single_cert",
        "wtransport",
    ];
}

impl TryFrom<WebTransport> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: WebTransport) -> Result<Self, Self::Error> {
        let foreign_name_text = WebTransport::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = WebTransport::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_wtransport_is_a_name() {
        let foreign_crate = WebTransport;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
