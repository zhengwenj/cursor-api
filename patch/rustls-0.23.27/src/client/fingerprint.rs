use alloc::{vec, vec::Vec};

use crate::{
    msgs::{enums::NamedGroup, handshake::SupportedProtocolVersions},
    CipherSuite, SignatureScheme,
};

/// 固定的客户端TLS指纹配置
///
/// 目标指纹:
/// - JA3: 71dc8c533dd919ae9f4963224a4ba8fd
/// - JA3字符串: 771,4865-4866-4867-49199-49195-49200-49196-49191-52393-52392-49161-49171-49162-49172-156-157-47-53,0-23-65281-10-11-35-13-51-45-43,29-23-24,0
/// - JA4: t13d181000_5d04281c6031_78e6aca7449b
/// - JA4_r: t13d181000_002f,0035,009c,009d,1301,1302,1303,c009,c00a,c013,c014,c027,c02b,c02c,c02f,c030,cca8,cca9_000a,000b,000d,0017,0023,002b,002d,0033,ff01_0403,0804,0401,0503,0805,0501,0806,0601,0201
pub(crate) struct TlsFingerprint;

impl TlsFingerprint {
    /// ClientExtension::SupportedVersions
    pub(super) fn supported_versions() -> SupportedProtocolVersions {
        SupportedProtocolVersions {
            tls13: true,
            tls12: true,
        }
    }

    // /// ALProtocolNegotiation
    // pub(crate) fn alprotocol_negotiation() -> Vec<ProtocolName> {
    //     // static O: OnceLock<Vec<ProtocolName>> = OnceLock::new();
    //     vec![ProtocolName::from(b"h2".to_vec())]
    // }

    /// pyload cipher_suites
    pub(super) fn cipher_suites() -> Vec<CipherSuite> {
        vec![
            // 0x1301
            CipherSuite::TLS13_AES_128_GCM_SHA256,
            // 0x1302
            CipherSuite::TLS13_AES_256_GCM_SHA384,
            // 0x1303
            CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
            // 0xc02f
            CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_GCM_SHA256,
            // 0xc02b
            CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_GCM_SHA256,
            // 0xc030
            CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_GCM_SHA384,
            // 0xc02c
            CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_GCM_SHA384,
            // 0xc027
            CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA256,
            // 0xcca9
            CipherSuite::TLS_ECDHE_ECDSA_WITH_CHACHA20_POLY1305_SHA256,
            // 0xcca8
            CipherSuite::TLS_ECDHE_RSA_WITH_CHACHA20_POLY1305_SHA256,
            // 0xc009
            CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_128_CBC_SHA,
            // 0xc013
            CipherSuite::TLS_ECDHE_RSA_WITH_AES_128_CBC_SHA,
            // 0xc00a
            CipherSuite::TLS_ECDHE_ECDSA_WITH_AES_256_CBC_SHA,
            // 0xc014
            CipherSuite::TLS_ECDHE_RSA_WITH_AES_256_CBC_SHA,
            // 0x009c
            CipherSuite::TLS_RSA_WITH_AES_128_GCM_SHA256,
            // 0x009d
            CipherSuite::TLS_RSA_WITH_AES_256_GCM_SHA384,
            // 0x002f
            CipherSuite::TLS_RSA_WITH_AES_128_CBC_SHA,
            // 0x0035
            CipherSuite::TLS_RSA_WITH_AES_256_CBC_SHA,
        ]
    }

    /// ClientExtension::SignatureAlgorithms
    pub(super) fn signature_algorithms() -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::ECDSA_NISTP256_SHA256, // 0x0403
            SignatureScheme::RSA_PSS_SHA256,        // 0x0804 (rsa_pss_rsae_sha256)
            SignatureScheme::RSA_PKCS1_SHA256,      // 0x0401
            SignatureScheme::ECDSA_NISTP384_SHA384, // 0x0503
            SignatureScheme::RSA_PSS_SHA384,        // 0x0805 (rsa_pss_rsae_sha384)
            SignatureScheme::RSA_PKCS1_SHA384,      // 0x0501
            SignatureScheme::RSA_PSS_SHA512,        // 0x0806 (rsa_pss_rsae_sha512)
            SignatureScheme::RSA_PKCS1_SHA512,      // 0x0601
            SignatureScheme::RSA_PKCS1_SHA1,        // 0x0201
        ]
    }

    /// ClientExtension::NamedGroups
    pub(super) fn supported_groups() -> Vec<NamedGroup> {
        vec![
            NamedGroup::X25519,    // 0x001d (29)
            NamedGroup::secp256r1, // 0x0017 (23)
            NamedGroup::secp384r1, // 0x0018 (24)
        ]
    }

    // /// ClientExtension Order
    // pub(super) fn extensions() -> Vec<ExtensionType> {
    //     vec![
    //         ExtensionType::ServerName,           // 0x0000
    //         ExtensionType::ExtendedMasterSecret, // 0x0017
    //         ExtensionType::RenegotiationInfo,    // 0xff01
    //         ExtensionType::EllipticCurves,       // 0x000a
    //         ExtensionType::ECPointFormats,       // 0x000b
    //         ExtensionType::SessionTicket,        // 0x0023
    //         // ExtensionType::ALProtocolNegotiation, // 0x0010
    //         ExtensionType::SignatureAlgorithms, // 0x000d
    //         ExtensionType::KeyShare,            // 0x0033
    //         ExtensionType::PSKKeyExchangeModes, // 0x002d
    //         ExtensionType::SupportedVersions,   // 0x002b
    //         ExtensionType::Padding,             // 0x0015
    //         ExtensionType::PreSharedKey,        // 0x0029
    //     ]
    // }
}
