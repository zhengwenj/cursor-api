mod alias;
mod build_key;
mod checksum;
mod config;
mod cpp;
mod fetch_model;
mod hash;
mod log;
mod proxy;
mod state;
mod timestamp_header;
mod token;
mod usage_check;
// mod validity_range;
mod tz;
mod vision_ability;

use ::core::borrow::Borrow;
use ::std::borrow::Cow;

use crate::{
    common::{
        model::{
            ApiStatus,
            userinfo::{Session, StripeProfile, UserProfile},
        },
        utils::TrimNewlines as _,
    },
    core::model::Role,
};
use ahash::HashMap;
use proxy_pool::get_client_or_general;
use reqwest::Client;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

pub use alias::Alias;
pub use checksum::Checksum;
pub use config::AppConfig;
pub use cpp::{CppService, GcppHost};
pub use fetch_model::FetchMode;
pub use hash::Hash;
pub use timestamp_header::TimestampHeader;
pub use token::{
    Duration as TokenDuration, Randomness, RawToken, RawTokenHelper, Subject, Token, TokenKey,
    UserId,
};
pub use usage_check::UsageCheck;
pub use vision_ability::VisionAbility;
pub mod proxy_pool;
pub use build_key::{
    BuildKeyRequest, BuildKeyResponse, GetConfigVersionRequest, GetConfigVersionResponse,
    UsageCheckModelType,
};
pub use proxy::{
    ProxiesDeleteRequest, ProxiesDeleteResponse, ProxyAddRequest, ProxyInfoResponse,
    ProxyUpdateRequest, SetGeneralProxyRequest,
};
pub use state::{AppState, PageContent, Pages, TokenError, TokenManager};
// pub use validity_range::ValidityRange;
pub use tz::DateTime;

use super::constant::{EMPTY_STRING, STATUS_FAILURE, STATUS_PENDING, STATUS_SUCCESS};

#[derive(Clone, Copy, PartialEq, Archive, RkyvDeserialize, RkyvSerialize)]
#[repr(u8)]
pub enum LogStatus {
    Pending,
    Success,
    Failure,
}

impl Serialize for LogStatus {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.as_str_name())
    }
}

impl LogStatus {
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Self::Pending => STATUS_PENDING,
            Self::Success => STATUS_SUCCESS,
            Self::Failure => STATUS_FAILURE,
        }
    }

    pub fn from_str_name(s: &str) -> Option<Self> {
        match s {
            STATUS_PENDING => Some(Self::Pending),
            STATUS_SUCCESS => Some(Self::Success),
            STATUS_FAILURE => Some(Self::Failure),
            _ => None,
        }
    }
}

// 请求日志
#[derive(Serialize, Clone)]
pub struct RequestLog {
    pub id: u64,
    pub timestamp: DateTime,
    pub model: &'static str,
    pub token_info: LogTokenInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<Chain>,
    pub timing: TimingInfo,
    pub stream: bool,
    pub status: LogStatus,
    pub error: ErrorInfo,
}

impl RequestLog {
    #[inline(always)]
    pub fn token_key(&self) -> TokenKey { self.token_info.key }
}

#[derive(Serialize, Clone)]
pub struct Chain {
    #[serde(skip_serializing_if = "Prompt::is_none")]
    pub prompt: Prompt,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delays: Option<(String, Vec<(u32, f32)>)>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<ChainUsage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub think: Option<String>,
}

#[derive(Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct ChainUsage {
    pub input: i32,
    pub output: i32,
    pub cache_write: i32,
    pub cache_read: i32,
    pub cents: f32,
}

impl ChainUsage {
    pub fn to_openai(self) -> crate::core::model::openai::Usage {
        use crate::core::model::openai;
        crate::core::model::openai::Usage {
            prompt_tokens: self.input,
            completion_tokens: self.output,
            total_tokens: self.input + self.output,
            prompt_tokens_details: openai::PromptTokensDetails { cached_tokens: self.cache_read },
            // completion_tokens_details: openai::CompletionTokensDetails { reasoning_tokens: 0 },
        }
    }

    pub fn to_anthropic(self) -> crate::core::model::anthropic::Usage {
        use crate::core::model::anthropic;
        anthropic::Usage {
            input_tokens: self.input,
            output_tokens: self.output,
            cache_creation_input_tokens: self.cache_write,
            cache_read_input_tokens: self.cache_read,
        }
    }

    pub fn to_anthropic_delta(self) -> crate::core::model::anthropic::MessageDeltaUsage {
        use crate::core::model::anthropic;
        anthropic::MessageDeltaUsage {
            input_tokens: if self.input == 0 {
                None
            } else {
                Some(self.input)
            },
            output_tokens: self.output,
            cache_creation_input_tokens: self.cache_write,
            cache_read_input_tokens: self.cache_read,
        }
    }
}

#[derive(Serialize, Clone)]
#[serde(untagged)]
pub enum Prompt {
    None,
    Origin(String),
    Parsed(Vec<PromptMessage>),
}

#[derive(Serialize, Clone)]
pub struct PromptMessage {
    role: Role,
    content: PromptContent,
}

#[derive(Clone)]
#[repr(transparent)]
pub struct PromptContent(crate::leak::ArcStr);

impl Serialize for PromptContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.0)
    }
}

impl PromptContent {
    #[inline]
    pub fn into_owned(self) -> String { self.0.as_str().to_owned() }
}

impl Prompt {
    pub fn new(input: String) -> Self {
        let mut messages = Vec::new();
        let mut remaining = input.as_str();

        while !remaining.is_empty() {
            // 检查是否以任一开始标记开头，并确定相应的结束标记
            let (role, end_tag, content) =
                if let Some(r) = remaining.strip_prefix("<|BEGIN_SYSTEM|>\n") {
                    (Role::System, "\n<|END_SYSTEM|>\n", r)
                } else if let Some(r) = remaining.strip_prefix("<|BEGIN_USER|>\n") {
                    (Role::User, "\n<|END_USER|>\n", r)
                } else if let Some(r) = remaining.strip_prefix("<|BEGIN_ASSISTANT|>\n") {
                    (Role::Assistant, "\n<|END_ASSISTANT|>\n", r)
                } else {
                    return Self::Origin(input);
                };

            // 更新remaining为去除前缀后的内容
            remaining = content;

            // 查找结束标记
            if let Some((content_part, after_end)) = remaining.split_once(end_tag) {
                // 提取内容
                let content = PromptContent(crate::leak::intern_arc(
                    content_part.trim_leading_newlines(),
                ));
                messages.push(PromptMessage { role, content });

                // 移动到结束标记之后
                remaining = after_end;

                // 跳过消息之间的额外换行符
                if remaining.as_bytes().first().copied() == Some(b'\n') {
                    remaining = unsafe { remaining.get_unchecked(1..) };
                }
            } else {
                return Self::Origin(input);
            }
        }

        Self::Parsed(messages)
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool { matches!(*self, Self::None) }

    #[inline(always)]
    pub const fn is_some(&self) -> bool { !self.is_none() }
}

#[derive(Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct TimingInfo {
    pub total: f64, // 总用时(秒)
}

#[derive(Serialize, Clone, Copy)]
#[serde(untagged)]
pub enum ErrorInfo {
    None,
    Error(&'static str),
    Details {
        error: &'static str,
        details: &'static str,
    },
}

impl ErrorInfo {
    #[inline]
    pub fn new<S: Borrow<str>>(e: S) -> Self { Self::Error(crate::leak::intern_static(e)) }

    #[inline]
    pub fn new_details<S: Borrow<str>>(e: S, detail: S) -> Self {
        Self::Details {
            error: crate::leak::intern_static(e),
            details: crate::leak::intern_static(detail),
        }
    }

    #[inline]
    pub fn add_detail<S: Borrow<str>>(&mut self, detail: S) {
        match self {
            ErrorInfo::None =>
                *self = Self::Details {
                    error: EMPTY_STRING,
                    details: crate::leak::intern_static(detail),
                },
            ErrorInfo::Error(error) =>
                *self = Self::Details {
                    error,
                    details: crate::leak::intern_static(detail),
                },
            ErrorInfo::Details { details, .. } => {
                *details = crate::leak::intern_static(detail);
            }
        }
    }

    pub fn contains(&self, pat: &str) -> bool {
        match *self {
            Self::None => false,
            Self::Error(error) => error.contains(pat),
            Self::Details { error, details } => error.contains(pat) || details.contains(pat),
        }
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool { matches!(*self, Self::None) }

    #[inline(always)]
    pub const fn is_some(&self) -> bool { !self.is_none() }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ExtToken {
    /// 主token - 可以是client或web token
    pub primary_token: Token,
    /// 次要token - 如果存在，必定是web token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_token: Option<Token>,
    pub checksum: Checksum,
    #[serde(skip_serializing_if = "Hash::is_nil", default = "Hash::random")]
    pub client_key: Hash,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config_version: Option<uuid::Uuid>,
    #[serde(skip_serializing_if = "uuid::Uuid::is_nil")]
    pub session_id: uuid::Uuid,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proxy: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timezone: Option<chrono_tz::Tz>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gcpp_host: Option<GcppHost>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user: Option<UserProfile>,
}

impl ExtToken {
    #[inline]
    pub fn clone_without_user(&self) -> Self {
        Self {
            primary_token: self.primary_token.clone(),
            secondary_token: None,
            checksum: self.checksum,
            client_key: self.client_key,
            config_version: self.config_version,
            session_id: self.session_id,
            proxy: self.proxy.clone(),
            timezone: self.timezone,
            gcpp_host: self.gcpp_host,
            user: None,
        }
    }

    #[inline]
    pub fn clone_without_config_version(&self) -> Self {
        Self {
            primary_token: self.primary_token.clone(),
            secondary_token: None,
            checksum: self.checksum,
            client_key: self.client_key,
            config_version: None,
            session_id: self.session_id,
            proxy: self.proxy.clone(),
            timezone: self.timezone,
            gcpp_host: self.gcpp_host,
            user: None,
        }
    }

    /// 获取适用于此 token 的 HTTP 客户端
    #[inline]
    pub fn get_client(&self) -> Client { get_client_or_general(self.proxy.as_deref()) }

    /// 获取此 token 关联的时区
    #[inline]
    fn get_timezone(&self) -> chrono_tz::Tz {
        self.timezone
            .unwrap_or_else(|| *super::lazy::GENERAL_TIMEZONE)
    }

    #[inline]
    pub fn get_gcpp_host(&self) -> GcppHost {
        self.gcpp_host
            .unwrap_or_else(|| *super::lazy::GENERAL_GCPP_HOST)
    }

    /// 返回关联的时区名称
    #[inline]
    pub fn timezone_name(&self) -> &'static str { self.get_timezone().name() }

    /// 获取当前时区的当前时间
    #[inline]
    pub fn now(&self) -> chrono::DateTime<chrono_tz::Tz> {
        use ::chrono::TimeZone as _;
        self.get_timezone()
            .from_utc_datetime(&DateTime::naive_now())
    }
}

// 用于存储 token 信息
#[derive(Clone, Serialize, Deserialize)]
pub struct TokenInfo {
    pub bundle: ExtToken,
    #[serde(default)]
    pub status: TokenStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe: Option<StripeProfile>,
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub sessions: Vec<Session>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
struct ExtTokenHelper {
    primary_token: RawTokenHelper,
    secondary_token: Option<RawTokenHelper>,
    checksum: Checksum,
    client_key: Hash,
    config_version: Option<uuid::Uuid>,
    session_id: uuid::Uuid,
    proxy: Option<String>,
    timezone: Option<String>,
    gcpp_host: Option<GcppHost>,
    user: Option<UserProfile>,
}

impl ExtTokenHelper {
    #[inline]
    fn new(token_info: &ExtToken) -> Self {
        Self {
            primary_token: token_info.primary_token.raw().to_helper(),
            secondary_token: token_info
                .secondary_token
                .as_ref()
                .map(|t| t.raw().to_helper()),
            checksum: token_info.checksum,
            client_key: token_info.client_key,
            config_version: token_info.config_version,
            session_id: token_info.session_id,
            proxy: token_info.proxy.clone(),
            timezone: token_info.timezone.map(|tz| tz.to_string()),
            gcpp_host: token_info.gcpp_host,
            user: token_info.user.clone(),
        }
    }

    #[inline]
    fn extract(self) -> ExtToken {
        ExtToken {
            primary_token: Token::new(self.primary_token.extract(), None),
            secondary_token: self.secondary_token.map(|h| Token::new(h.extract(), None)),
            checksum: self.checksum,
            client_key: self.client_key,
            config_version: self.config_version,
            session_id: self.session_id,
            proxy: self.proxy,
            timezone: self.timezone.map(|s| __unwrap_panic!(s.parse())),
            gcpp_host: self.gcpp_host,
            user: self.user,
        }
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize)]
struct TokenInfoHelper {
    alias: String,
    bundle: ExtTokenHelper,
    status: TokenStatus,
    stripe: Option<StripeProfile>,
    sessions: Vec<Session>,
}

impl TokenInfoHelper {
    #[inline]
    fn new(token_info: &TokenInfo, alias: String) -> Self {
        Self {
            alias,
            bundle: ExtTokenHelper::new(&token_info.bundle),
            status: token_info.status,
            stripe: token_info.stripe,
            sessions: token_info.sessions.clone(),
        }
    }

    #[inline]
    fn extract(self) -> (TokenInfo, String) {
        (
            TokenInfo {
                bundle: self.bundle.extract(),
                status: self.status,
                stripe: self.stripe,
                sessions: self.sessions,
            },
            self.alias,
        )
    }
}

#[derive(Clone, Copy, Serialize, Archive, RkyvSerialize, RkyvDeserialize)]
pub struct LogTokenInfo {
    #[serde(serialize_with = "serialize_token_key")]
    pub key: TokenKey,
    pub stripe: Option<StripeProfile>,
}

fn serialize_token_key<S>(key: &TokenKey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: ::serde::Serializer,
{
    // use ::serde::ser::SerializeStruct as _;
    // let mut state = serializer.serialize_struct("TokenKey", 2)?;
    // state.serialize_field("user_id", &key.user_id.as_u128())?;
    // state.serialize_field("id", &key.randomness.as_u64())?;
    // state.end()
    serializer.serialize_str(&key.to_string())
}

#[derive(Default, Clone, Copy, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
#[serde(rename_all = "lowercase")]
#[repr(u8)]
pub enum TokenStatus {
    #[default]
    Enabled,
    Disabled,
}

impl TokenInfo {
    #[inline(always)]
    pub fn is_enabled(&self) -> bool { matches!(self.status, TokenStatus::Enabled) }
}

// pub struct TokenValidityRange {
//     short: ValidityRange,
//     long: ValidityRange,
// }

// impl TokenValidityRange {
//     #[inline]
//     pub(super) fn new(short: ValidityRange, long: ValidityRange) -> Self {
//         Self { short, long }
//     }

//     #[inline]
//     pub fn is_short(&self, val: u32) -> bool {
//         self.short.is_valid(val)
//     }

//     #[inline]
//     pub fn is_long(&self, val: u32) -> bool {
//         self.long.is_valid(val)
//     }
// }

// TokenUpdateRequest 结构体
pub type TokenUpdateRequest = Vec<(String, TokenInfo)>;

#[derive(Deserialize)]
pub struct TokensAddRequest {
    pub tokens: Vec<TokensAddRequestTokenInfo>,
    #[serde(default)]
    pub status: TokenStatus,
}

#[derive(Deserialize)]
pub struct TokensAddRequestTokenInfo {
    pub alias: Option<String>,
    pub token: String,
    pub checksum: Option<String>,
    pub client_key: Option<String>,
    pub session_id: Option<String>,
    pub config_version: Option<String>,
    pub proxy: Option<String>,
    pub timezone: Option<String>,
    pub gcpp_host: Option<String>,
}

// TokensDeleteRequest 结构体
#[derive(Deserialize)]
pub struct TokensDeleteRequest {
    #[serde(default)]
    pub aliases: Vec<String>,
    #[serde(default)]
    pub include_failed_tokens: bool,
}

// TokensDeleteResponse 结构体
#[derive(Serialize)]
pub struct TokensDeleteResponse {
    pub status: ApiStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_tokens: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct TokensInfoResponse {
    pub status: ApiStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<(usize, Alias, TokenInfo)>>,
    pub tokens_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Cow<'static, str>>,
}

#[derive(Serialize)]
pub struct CommonResponse {
    pub status: ApiStatus,
    pub message: Cow<'static, str>,
}

#[derive(Deserialize)]
pub struct TokensStatusSetRequest {
    pub aliases: Vec<String>,
    pub status: TokenStatus,
}

pub type TokensAliasSetRequest = HashMap<String, String>;

#[derive(Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum DeleteResponseExpectation {
    #[default]
    Simple,
    UpdatedTokens,
    FailedTokens,
    Detailed,
}

impl DeleteResponseExpectation {
    #[inline]
    pub fn needs_updated_tokens(&self) -> bool {
        matches!(
            self,
            DeleteResponseExpectation::UpdatedTokens | DeleteResponseExpectation::Detailed
        )
    }

    #[inline]
    pub fn needs_failed_tokens(&self) -> bool {
        matches!(
            self,
            DeleteResponseExpectation::FailedTokens | DeleteResponseExpectation::Detailed
        )
    }
}

#[derive(Deserialize)]
pub struct TokensProxySetRequest {
    pub aliases: Vec<String>,
    pub proxy: Option<String>,
}

#[derive(Deserialize)]
pub struct TokensTimezoneSetRequest {
    pub aliases: Vec<String>,
    pub timezone: Option<chrono_tz::Tz>,
}
