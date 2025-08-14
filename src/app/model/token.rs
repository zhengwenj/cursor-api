mod cache;
mod provider;
use core::fmt;
use std::{io, str::FromStr};

use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
pub(super) use cache::__init;
pub use cache::{Token, TokenKey};
pub use provider::{Provider, parse_providers};

use crate::{
    app::constant::HEADER_B64,
    common::{
        model::token::{StringI64, TokenPayload},
        utils::{byte_to_hex, hex_to_byte},
    },
};

#[derive(Debug)]
pub enum RandomnessError {
    InvalidLength,
    InvalidFormat,
}

impl fmt::Display for RandomnessError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength => write!(f, "Invalid Randomness length"),
            Self::InvalidFormat => write!(f, "Invalid format"),
        }
    }
}

impl std::error::Error for RandomnessError {}

#[derive(
    Clone, Copy, PartialEq, Eq, Hash, ::rkyv::Archive, ::rkyv::Deserialize, ::rkyv::Serialize,
)]
#[rkyv(derive(PartialEq, Eq, Hash))]
#[repr(transparent)]
pub struct Randomness(u64);

impl Randomness {
    #[inline]
    pub const fn from_u64(value: u64) -> Self { Self(value) }

    #[inline]
    pub const fn as_u64(self) -> u64 { self.0 }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 8]) -> Self { Self(u64::from_ne_bytes(bytes)) }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 8] { self.0.to_ne_bytes() }

    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn to_str<'buf>(&self, buf: &'buf mut [u8; 18]) -> &'buf mut str {
        let bytes: [u8; 8] = self.0.to_ne_bytes();

        byte_to_hex(bytes[0], unsafe { &mut *(buf.as_mut_ptr() as *mut _) });
        byte_to_hex(bytes[1], unsafe {
            &mut *(buf.as_mut_ptr().add(2) as *mut _)
        });
        byte_to_hex(bytes[2], unsafe {
            &mut *(buf.as_mut_ptr().add(4) as *mut _)
        });
        byte_to_hex(bytes[3], unsafe {
            &mut *(buf.as_mut_ptr().add(6) as *mut _)
        });
        buf[8] = b'-';
        byte_to_hex(bytes[4], unsafe {
            &mut *(buf.as_mut_ptr().add(9) as *mut _)
        });
        byte_to_hex(bytes[5], unsafe {
            &mut *(buf.as_mut_ptr().add(11) as *mut _)
        });
        buf[13] = b'-';
        byte_to_hex(bytes[6], unsafe {
            &mut *(buf.as_mut_ptr().add(14) as *mut _)
        });
        byte_to_hex(bytes[7], unsafe {
            &mut *(buf.as_mut_ptr().add(16) as *mut _)
        });

        // SAFETY: buf 只包含有效的 ASCII 字符
        unsafe { ::core::str::from_utf8_unchecked_mut(buf) }
    }
}

impl const Default for Randomness {
    #[inline(always)]
    fn default() -> Self { Self(0) }
}

impl fmt::Display for Randomness {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.to_str(&mut [0u8; 18]))
    }
}

impl FromStr for Randomness {
    type Err = RandomnessError;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 18 {
            return Err(RandomnessError::InvalidLength);
        }
        let bytes = s.as_bytes();

        if bytes[8] != b'-' || bytes[13] != b'-' {
            return Err(RandomnessError::InvalidFormat);
        }
        let mut result = [0u8; 8];

        result[0] = hex_to_byte(bytes[0], bytes[1]).ok_or(RandomnessError::InvalidFormat)?;
        result[1] = hex_to_byte(bytes[2], bytes[3]).ok_or(RandomnessError::InvalidFormat)?;
        result[2] = hex_to_byte(bytes[4], bytes[5]).ok_or(RandomnessError::InvalidFormat)?;
        result[3] = hex_to_byte(bytes[6], bytes[7]).ok_or(RandomnessError::InvalidFormat)?;
        result[4] = hex_to_byte(bytes[9], bytes[10]).ok_or(RandomnessError::InvalidFormat)?;
        result[5] = hex_to_byte(bytes[11], bytes[12]).ok_or(RandomnessError::InvalidFormat)?;
        result[6] = hex_to_byte(bytes[14], bytes[15]).ok_or(RandomnessError::InvalidFormat)?;
        result[7] = hex_to_byte(bytes[16], bytes[17]).ok_or(RandomnessError::InvalidFormat)?;

        Ok(Self(u64::from_ne_bytes(result)))
    }
}

impl ::serde::Serialize for Randomness {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.to_str(&mut [0u8; 18]))
    }
}

impl<'de> ::serde::Deserialize<'de> for Randomness {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        struct RandomnessVisitor;

        impl ::serde::de::Visitor<'_> for RandomnessVisitor {
            type Value = Randomness;

            fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                formatter.write_str("a string in the format XXXXXXXX-XXXX-XXXX")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: ::serde::de::Error,
            {
                value.parse().map_err(E::custom)
            }
        }

        deserializer.deserialize_str(RandomnessVisitor)
    }
}

const _: [u8; 8] = [0; ::core::mem::size_of::<Randomness>()];
const _: () = assert!(::core::mem::align_of::<Randomness>() == 8);

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct Subject {
    pub provider: Provider,
    pub id: UserId,
}

impl Subject {
    #[inline]
    fn to_helper(self) -> SubjectHelper {
        SubjectHelper {
            provider: self.provider.to_helper(),
            id: self.id.to_bytes(),
        }
    }

    #[inline]
    fn from_str(s: &str) -> Result<Self, SubjectError> {
        let (provider, id_str) = s.split_once("|").ok_or(SubjectError::InvalidFormat)?;

        if provider.is_empty() {
            return Err(SubjectError::MissingProvider);
        }

        if id_str.is_empty() {
            return Err(SubjectError::MissingUserId);
        }

        let provider = Provider::from_str(provider)?;
        let id = id_str.parse().map_err(|_| SubjectError::InvalidUlid)?;

        Ok(Self { provider, id })
    }
}

impl fmt::Display for Subject {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.provider.0)?;
        f.write_str("|")?;
        f.write_str(self.id.to_str(&mut [0; 31]))
    }
}

#[derive(Debug)]
pub enum SubjectError {
    MissingProvider,
    MissingUserId,
    InvalidFormat,
    InvalidUlid,
    InvalidHex,
    UnsupportedProvider,
}

impl fmt::Display for SubjectError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::MissingProvider => "Missing provider",
            Self::MissingUserId => "Missing user_id",
            Self::InvalidFormat => "Invalid user_id format",
            Self::InvalidUlid => "Invalid ULID",
            Self::InvalidHex => "Invalid HEX",
            Self::UnsupportedProvider => "Unsupported provider",
        })
    }
}

impl std::error::Error for SubjectError {}

impl ::serde::Serialize for Subject {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> ::serde::Deserialize<'de> for Subject {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Self::from_str(&s).map_err(::serde::de::Error::custom)
    }
}

/// 用户标识符，支持两种格式的高效ID系统
///
/// 采用向前兼容设计，通过检查高32位区分格式：
/// - 旧格式：24字符十六进制，高32位为0
/// - 新格式：`user_` + 26字符ULID，充分利用128位空间
///
/// ULID时间戳特性确保新格式高32位非零，实现无歧义格式识别。
///
/// # Examples
///
/// ```
/// use your_crate::UserId;
///
/// // 新格式
/// let id = UserId::new(ulid::Ulid::new().0);
/// assert_eq!(id.to_string().len(), 31);
///
/// // 旧格式
/// let legacy = UserId::from_bytes([0, 0, 0, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
/// assert_eq!(legacy.to_string().len(), 24);
/// assert!(legacy.is_legacy());
/// ```
#[derive(
    Clone, Copy, PartialEq, Eq, Hash, ::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize,
)]
#[rkyv(derive(PartialEq, Eq, Hash))]
#[repr(transparent)]
pub struct UserId(u128);

impl UserId {
    const PREFIX: &'static str = "user_";

    #[inline]
    pub const fn new(id: u128) -> Self { Self(id) }

    #[inline]
    pub const fn from_u128(value: u128) -> Self { Self(value) }

    #[inline]
    pub const fn as_u128(self) -> u128 { self.0 }

    #[inline]
    pub const fn from_bytes(bytes: [u8; 16]) -> Self { Self(u128::from_ne_bytes(bytes)) }

    #[inline]
    pub const fn to_bytes(self) -> [u8; 16] { self.0.to_ne_bytes() }

    /// 检查是否为旧格式ID
    #[inline]
    pub const fn is_legacy(&self) -> bool {
        // Memory layout (little-endian): [低32位][次低32位][次高32位][最高32位]
        //                     index:         [0]      [1]       [2]       [3]
        // Memory layout (big-endian):    [最高32位][次高32位][次低32位][低32位]
        //                     index:         [0]       [1]       [2]      [3]
        let parts = unsafe { ::core::mem::transmute::<u128, [u32; 4]>(self.0) };

        #[cfg(target_endian = "little")]
        const HIGH_INDEX: usize = 3;
        #[cfg(target_endian = "big")]
        const HIGH_INDEX: usize = 0;

        parts[HIGH_INDEX] == 0
    }

    /// 高性能字符串转换，旧格式24字符，新格式31字符
    #[allow(clippy::wrong_self_convention)]
    #[inline]
    pub fn to_str<'buf>(&self, buf: &'buf mut [u8; 31]) -> &'buf mut str {
        if self.is_legacy() {
            let bytes = self.to_bytes();
            for (i, &byte) in bytes[4..].iter().enumerate() {
                buf[i * 2] = crate::common::utils::hex::HEX_CHARS[(byte >> 4) as usize];
                buf[i * 2 + 1] = crate::common::utils::hex::HEX_CHARS[(byte & 0x0f) as usize];
            }

            // SAFETY: HEX_CHARS确保有效ASCII输出
            unsafe {
                ::core::str::from_utf8_unchecked_mut(&mut *(buf.as_mut_ptr() as *mut [u8; 24]))
            }
        } else {
            unsafe {
                ::core::ptr::copy_nonoverlapping(Self::PREFIX.as_ptr(), buf.as_mut_ptr(), 5);
                ulid::Ulid(self.0).array_to_str(&mut *(buf.as_mut_ptr().add(5) as *mut [u8; 26]));
                ::core::str::from_utf8_unchecked_mut(buf)
            }
        }
    }
}

impl ::core::fmt::Display for UserId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(self.to_str(&mut [0; 31]))
    }
}

impl ::core::str::FromStr for UserId {
    type Err = SubjectError;

    fn from_str(s: &str) -> ::core::result::Result<Self, Self::Err> {
        match s.len() {
            31 => {
                let id_str = s
                    .strip_prefix(Self::PREFIX)
                    .ok_or(SubjectError::InvalidFormat)?;
                let id = ulid::Ulid::from_string(id_str).map_err(|_| SubjectError::InvalidUlid)?;
                Ok(Self(id.0))
            }
            24 => {
                let hex_array: &[u8; 24] = unsafe { s.as_bytes().try_into().unwrap_unchecked() };
                let hex_pairs = unsafe { hex_array.as_chunks_unchecked::<2>() };
                let mut result = [0u8; 16];

                for (dst, &[hi, lo]) in result[4..].iter_mut().zip(hex_pairs) {
                    *dst = crate::common::utils::hex::hex_to_byte(hi, lo)
                        .ok_or(SubjectError::InvalidHex)?;
                }

                Ok(Self::from_bytes(result))
            }
            _ => Err(SubjectError::MissingUserId),
        }
    }
}

impl ::serde::Serialize for UserId {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> ::core::result::Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.to_str(&mut [0; 31]))
    }
}

impl<'de> ::serde::Deserialize<'de> for UserId {
    #[inline]
    fn deserialize<D>(deserializer: D) -> ::core::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(::serde::de::Error::custom)
    }
}

const _: [u8; 16] = [0; ::core::mem::size_of::<UserId>()];
const _: () = assert!(::core::mem::align_of::<UserId>() == 16);

#[derive(Clone, Copy, PartialEq, Hash, ::rkyv::Archive, ::rkyv::Deserialize, ::rkyv::Serialize)]
pub struct Duration {
    pub start: i64,
    pub end: i64,
}

// impl Duration {
//     #[inline(always)]
//     pub const fn validity(&self) -> u32 {
//         (self.end - self.start) as u32
//     }

//     #[inline]
//     pub fn is_short(&self) -> bool {
//         TOKEN_VALIDITY_RANGE.is_short(self.validity())
//     }

//     #[inline]
//     pub fn is_long(&self) -> bool {
//         TOKEN_VALIDITY_RANGE.is_long(self.validity())
//     }
// }

#[derive(Debug)]
pub enum TokenError {
    InvalidHeader,
    InvalidFormat,
    InvalidBase64(base64::DecodeError),
    InvalidJson(io::Error),
    InvalidSubject(SubjectError),
    InvalidRandomness(RandomnessError),
    InvalidSignatureLength,
}

impl std::error::Error for TokenError {}

impl fmt::Display for TokenError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHeader => f.write_str("Invalid token header"),
            Self::InvalidFormat => f.write_str("Invalid token format"),
            Self::InvalidBase64(e) => write!(f, "Invalid base64: {e}"),
            Self::InvalidJson(e) => write!(f, "Invalid JSON: {e}"),
            Self::InvalidSubject(e) => write!(f, "Invalid subject: {e}"),
            Self::InvalidRandomness(e) => write!(f, "Invalid randomness: {e}"),
            Self::InvalidSignatureLength => f.write_str("Invalid signature length"),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Hash)]
pub struct RawToken {
    /// 用户标识符
    pub subject: Subject,
    /// 签名
    pub signature: [u8; 32],
    /// 持续时间
    pub duration: Duration,
    /// 随机字符串
    pub randomness: Randomness,
    /// 会话
    pub is_session: bool,
}

// impl PartialEq for RawToken {
//     fn eq(&self, other: &Self) -> bool {
//         self.id.id == other.id.id && self.randomness == other.randomness
//     }
// }

impl RawToken {
    #[inline(always)]
    fn to_token_payload(self) -> TokenPayload {
        TokenPayload {
            sub: self.subject,
            time: StringI64(self.duration.start),
            exp: self.duration.end,
            randomness: self.randomness,
            is_session: self.is_session,
        }
    }

    #[inline(always)]
    pub(super) fn to_helper(self) -> RawTokenHelper {
        RawTokenHelper {
            subject: self.subject.to_helper(),
            duration: self.duration,
            randomness: self.randomness,
            is_session: self.is_session,
            signature: self.signature,
        }
    }

    #[inline(always)]
    pub const fn key(&self) -> TokenKey {
        TokenKey {
            user_id: self.subject.id,
            randomness: self.randomness,
        }
    }

    #[inline(always)]
    pub const fn is_web(&self) -> bool { !self.is_session }

    #[inline(always)]
    pub const fn is_session(&self) -> bool { self.is_session }
}

impl fmt::Display for RawToken {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{HEADER_B64}{}.{}",
            URL_SAFE_NO_PAD.encode(__unwrap!(serde_json::to_vec(&self.to_token_payload()))),
            URL_SAFE_NO_PAD.encode(self.signature)
        )
    }
}

impl FromStr for RawToken {
    type Err = TokenError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        // 1. 分割并验证token格式
        let parts = token
            .strip_prefix(HEADER_B64)
            .ok_or(TokenError::InvalidHeader)?;

        let (payload_b64, signature_b64) =
            parts.split_once('.').ok_or(TokenError::InvalidFormat)?;

        // 2. 解码payload和signature
        let payload = URL_SAFE_NO_PAD
            .decode(payload_b64)
            .map_err(TokenError::InvalidBase64)?;

        let signature = URL_SAFE_NO_PAD
            .decode(signature_b64)
            .map_err(TokenError::InvalidBase64)?
            .try_into()
            .map_err(|_| TokenError::InvalidSignatureLength)?;

        // 3. 解析payload
        let payload: TokenPayload = serde_json::from_slice(&payload).map_err(|e| {
            let e: io::Error = e.into();
            match e.downcast::<SubjectError>() {
                Ok(e) => TokenError::InvalidSubject(e),
                Err(e) => match e.downcast::<RandomnessError>() {
                    Ok(e) => TokenError::InvalidRandomness(e),
                    Err(e) => TokenError::InvalidJson(e),
                },
            }
        })?;

        // 4. 构造RawToken
        Ok(Self {
            subject: payload.sub,
            duration: Duration {
                start: payload.time.0,
                end: payload.exp,
            },
            randomness: payload.randomness,
            is_session: payload.is_session,
            signature,
        })
    }
}

impl<'de> ::serde::Deserialize<'de> for RawToken {
    #[inline]
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        s.parse().map_err(::serde::de::Error::custom)
    }
}

#[derive(::rkyv::Archive, ::rkyv::Deserialize, ::rkyv::Serialize)]
#[repr(u8)]
pub enum ProviderHelper {
    Auth0,
    Github,
    Google,
    // Workos,
    Other(String) = u8::MAX,
}

impl ProviderHelper {
    #[inline]
    fn try_extract(self) -> Result<Provider, SubjectError> {
        match self {
            Self::Auth0 => Provider::from_str(provider::AUTH0),
            Self::Github => Provider::from_str(provider::GITHUB),
            Self::Google => Provider::from_str(provider::GOOGLE_OAUTH2),
            Self::Other(s) => Provider::from_str(&s),
        }
    }
}

#[derive(::rkyv::Archive, ::rkyv::Deserialize, ::rkyv::Serialize)]
pub struct SubjectHelper {
    provider: ProviderHelper,
    id: [u8; 16],
}

impl SubjectHelper {
    #[inline]
    fn try_extract(self) -> Result<Subject, SubjectError> {
        Ok(Subject {
            provider: self.provider.try_extract()?,
            id: UserId::from_bytes(self.id),
        })
    }
}

#[derive(::rkyv::Archive, ::rkyv::Deserialize, ::rkyv::Serialize)]
pub struct RawTokenHelper {
    pub subject: SubjectHelper,
    pub signature: [u8; 32],
    pub duration: Duration,
    pub randomness: Randomness,
    pub is_session: bool,
}

impl RawTokenHelper {
    #[inline]
    pub(super) fn extract(self) -> RawToken {
        RawToken {
            subject: __unwrap_panic!(self.subject.try_extract()),
            duration: self.duration,
            randomness: self.randomness,
            is_session: self.is_session,
            signature: self.signature,
        }
    }
}
