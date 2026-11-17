use gates_vocabulary_language_core_foreign_name::ForeignName;
use gates_vocabulary_language_core_languages::Error;
use gates_vocabulary_language_core_languages::foreign_languages::ForeignLanguage;
use stack_domain_driven_design_core::anti_corruption_layer::AntiCorruptionLayer;
use stack_domain_driven_design_core::layer::{DomainLayer, Layer};

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Copy, Default)]
pub(crate) struct RustTransportLayerSecurity;

impl Layer for RustTransportLayerSecurity {
    type Layer = DomainLayer;
}

impl AntiCorruptionLayer for RustTransportLayerSecurity {}

impl RustTransportLayerSecurity {
    const NAME: &'static str = "rustls";

    const NAMES: &'static [&'static str] = &[
        "assertion",
        "builder_with_provider",
        "CertificateDer",
        "CertificateError",
        "client",
        "ClientConfig",
        "crypto",
        "CryptoProvider",
        "danger",
        "dangerous",
        "default_provider",
        "DigitallySignedStruct",
        "Error",
        "HandshakeSignatureValid",
        "InvalidCertificate",
        "pki_types",
        "PrivateKeyDer",
        "PrivatePkcs8KeyDer",
        "ring",
        "rustls",
        "ServerCertVerified",
        "ServerCertVerifier",
        "ServerConfig",
        "ServerName",
        "SignatureScheme",
        "signature_verification_algorithms",
        "supported_schemes",
        "supported_verify_schemes",
        "TLS13",
        "UnixTime",
        "UnknownIssuer",
        "verify_server_cert",
        "verify_tls12_signature",
        "verify_tls13_signature",
        "version",
        "with_custom_certificate_verifier",
        "with_no_client_auth",
        "with_protocol_versions",
        "with_single_cert",
    ];
}

impl TryFrom<RustTransportLayerSecurity> for ForeignLanguage {
    type Error = Error;

    fn try_from(_: RustTransportLayerSecurity) -> Result<Self, Self::Error> {
        let foreign_name_text = RustTransportLayerSecurity::NAME;
        let foreign_name = ForeignName::from(foreign_name_text);
        let name_texts = RustTransportLayerSecurity::NAMES;
        Self::try_from((foreign_name, name_texts))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_name_of_rustls_is_a_name() {
        let foreign_crate = RustTransportLayerSecurity;

        let foreign_language = ForeignLanguage::try_from(foreign_crate);

        let error = foreign_language.err();
        let expectation = None;
        assert_eq!(error, expectation);
    }
}
