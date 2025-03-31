use std::sync::LazyLock;

use crate::{
    common::{
        model::{ApiStatus, userinfo::TokenProfile},
        utils::{TrimNewlines as _, generate_hash},
    },
    core::model::Role,
};
use lasso::{LargeSpur, ThreadedRodeo};
use proxy_pool::ProxyPool;
use reqwest::Client;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};

mod usage_check;
pub use usage_check::UsageCheck;
mod vision_ability;
pub use vision_ability::VisionAbility;
mod config;
pub use config::AppConfig;
mod build_key;
pub mod proxy_pool;
pub use build_key::*;
mod state;
pub use state::*;
mod proxy;
pub use proxy::*;
mod log;

use super::constant::{EMPTY_STRING, STATUS_FAILURE, STATUS_PENDING, STATUS_SUCCESS};

#[derive(Clone, Copy, PartialEq, Archive, RkyvDeserialize, RkyvSerialize)]
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
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub model: &'static str,
    pub token_info: TokenInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<Chain>,
    pub timing: TimingInfo,
    pub stream: bool,
    pub status: LogStatus,
    pub error: ErrorInfo,
}

#[derive(Serialize, Clone)]
pub struct Chain {
    #[serde(skip_serializing_if = "Prompt::is_none")]
    pub prompt: Prompt,
    pub delays: Vec<(String, f64)>,
    #[serde(skip_serializing_if = "OptionUsage::is_none")]
    pub usage: OptionUsage,
}

#[derive(Serialize, Clone, Archive, RkyvDeserialize, RkyvSerialize)]
pub enum OptionUsage {
    None,
    Uasge { input: i32, output: i32 },
}

impl OptionUsage {
    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        matches!(*self, Self::None)
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

static RODEO: LazyLock<ThreadedRodeo<LargeSpur>> = LazyLock::new(ThreadedRodeo::new);

#[derive(Debug, Clone)]
pub enum PromptContent {
    Leaked(&'static str),
    Shared(LargeSpur),
}

impl Serialize for PromptContent {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Leaked(s) => serializer.serialize_str(s),
            Self::Shared(key) => serializer.serialize_str(RODEO.resolve(key)),
        }
    }
}

impl PromptContent {
    pub fn into_owned(self) -> String {
        match self {
            Self::Leaked(s) => s.to_string(),
            Self::Shared(key) => RODEO.resolve(&key).to_string(),
        }
    }
}

impl Prompt {
    pub fn new(input: String) -> Self {
        let mut messages = Vec::new();
        let mut remaining = input.as_str();

        while !remaining.is_empty() {
            // 检查是否以任一开始标记开头
            let (role, start_tag) = if remaining.starts_with("<|BEGIN_SYSTEM|>\n") {
                (Role::System, "<|BEGIN_SYSTEM|>\n")
            } else if remaining.starts_with("<|BEGIN_USER|>\n") {
                (Role::User, "<|BEGIN_USER|>\n")
            } else if remaining.starts_with("<|BEGIN_ASSISTANT|>\n") {
                (Role::Assistant, "<|BEGIN_ASSISTANT|>\n")
            } else {
                return Self::Origin(input);
            };

            // 确定相应的结束标记
            let end_tag = match role {
                Role::System => "\n<|END_SYSTEM|>\n",
                Role::User => "\n<|END_USER|>\n",
                Role::Assistant => "\n<|END_ASSISTANT|>\n",
            };

            // 移除起始标记
            remaining = &remaining[start_tag.len()..];

            // 查找结束标记
            if let Some(end_index) = remaining.find(end_tag) {
                // 提取内容
                let content = if role == Role::System {
                    PromptContent::Leaked(crate::leak::intern_string(&remaining[..end_index]))
                } else {
                    PromptContent::Shared(
                        RODEO.get_or_intern(remaining[..end_index].trim_leading_newlines()),
                    )
                };
                messages.push(PromptMessage { role, content });

                // 移除当前消息（包括结束标记）
                remaining = &remaining[end_index + end_tag.len()..];

                // 如果消息之间有额外的换行符，将其跳过
                if remaining.as_bytes().first().copied() == Some(b'\n') {
                    remaining = &remaining[1..];
                }
            } else {
                return Self::Origin(input);
            }
        }

        Self::Parsed(messages)
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    #[inline(always)]
    pub const fn is_some(&self) -> bool {
        !self.is_none()
    }
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
    pub fn new(e: &str) -> Self {
        Self::Error(crate::leak::intern_string(e))
    }

    #[inline]
    pub fn new_details(e: &str, detail: &str) -> Self {
        Self::Details {
            error: crate::leak::intern_string(e),
            details: crate::leak::intern_string(detail),
        }
    }

    #[inline]
    pub fn add_detail(&mut self, detail: &str) {
        match self {
            ErrorInfo::None => {
                *self = Self::Details {
                    error: crate::leak::intern_string(EMPTY_STRING),
                    details: crate::leak::intern_string(detail),
                }
            }
            ErrorInfo::Error(error) => {
                *self = Self::Details {
                    error,
                    details: crate::leak::intern_string(detail),
                }
            }
            ErrorInfo::Details { details, .. } => {
                *details = crate::leak::intern_string(detail);
            }
        }
    }

    #[inline(always)]
    pub const fn is_none(&self) -> bool {
        matches!(*self, Self::None)
    }

    #[inline(always)]
    pub const fn is_some(&self) -> bool {
        !self.is_none()
    }
}

// 用于存储 token 信息
#[derive(Clone, Serialize, Deserialize, Archive, RkyvSerialize, RkyvDeserialize)]
pub struct TokenInfo {
    pub token: String,
    pub checksum: String,
    #[serde(skip_serializing, default = "generate_client_key")]
    pub client_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<TokenProfile>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
}

#[inline(always)]
fn generate_client_key() -> Option<String> {
    Some(generate_hash())
}

impl TokenInfo {
    /// 获取适用于此 token 的 HTTP 客户端
    ///
    /// 如果 tags 中包含 "proxy" 标签，会尝试使用其后一个标签作为代理 URL
    /// 例如: tags = ["proxy", "http://localhost:8080"] 将使用 http://localhost:8080 作为代理
    ///
    /// 如果没有找到有效的代理配置，将返回默认客户端
    pub fn get_client(&self) -> Client {
        // if let Some(tags) = &self.tags {
        //     // 查找 "proxy" 标签的位置
        //     if let Some(proxy_index) = tags.iter().position(|tag| tag == "proxy") {
        //         // 检查是否存在下一个标签作为代理 URL
        //         if proxy_index + 1 < tags.len() {
        //             // 获取代理 URL 并尝试创建对应的客户端
        //             return ProxyPool::get_client(&tags[proxy_index + 1]);
        //         }
        //     }
        // }
        // // 如果没有找到有效的代理配置，返回默认客户端
        // ProxyPool::get_general_client()
        if let Some(tags) = &self.tags {
            ProxyPool::get_client_or_general(tags.get(1).map(|s| s.as_str()))
        } else {
            ProxyPool::get_general_client()
        }
    }

    pub fn timezone_name(&self) -> &'static str {
        use std::str::FromStr as _;
        if let Some(Some(Ok(tz))) = self.tags.as_ref().map(|tags| {
            tags.first()
                .filter(|s| !s.is_empty())
                .map(|s| chrono_tz::Tz::from_str(s.as_str()))
        }) {
            tz.name()
        } else {
            super::lazy::GENERAL_TIMEZONE.name()
        }
    }

    // pub fn now(&self) -> chrono::DateTime<chrono_tz::Tz> {
    //     use std::str::FromStr as _;
    //     if let Some(Some(Ok(tz))) = self.tags.as_ref().map(|tags| {
    //         tags.get(0)
    //             .filter(|s| !s.is_empty())
    //             .map(|s| chrono_tz::Tz::from_str(s.as_str()))
    //     }) {
    //         use chrono::TimeZone as _;
    //         tz.from_utc_datetime(&chrono::Utc::now().naive_utc())
    //     } else {
    //         super::lazy::now_in_general_timezone()
    //     }
    // }
}

// TokenUpdateRequest 结构体
pub type TokenUpdateRequest = Vec<TokenInfo>;

#[derive(Deserialize)]
pub struct TokenAddRequest {
    pub tokens: Vec<TokenAddRequestTokenInfo>,
    #[serde(default)]
    pub tags: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct TokenAddRequestTokenInfo {
    pub token: String,
    #[serde(default)]
    pub checksum: Option<String>,
}

// TokensDeleteRequest 结构体
#[derive(Deserialize)]
pub struct TokensDeleteRequest {
    #[serde(default)]
    pub tokens: Vec<String>,
    #[serde(default)]
    pub expectation: DeleteResponseExpectation,
}

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
    pub fn needs_updated_tokens(&self) -> bool {
        matches!(
            self,
            DeleteResponseExpectation::UpdatedTokens | DeleteResponseExpectation::Detailed
        )
    }

    pub fn needs_failed_tokens(&self) -> bool {
        matches!(
            self,
            DeleteResponseExpectation::FailedTokens | DeleteResponseExpectation::Detailed
        )
    }
}

// TokensDeleteResponse 结构体
#[derive(Serialize)]
pub struct TokensDeleteResponse {
    pub status: ApiStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_tokens: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_tokens: Option<Vec<String>>,
}

#[derive(Serialize)]
pub struct TokenInfoResponse {
    pub status: ApiStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<Vec<TokenInfo>>,
    pub tokens_count: usize,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

// 标签相关的请求/响应结构体
#[derive(Deserialize)]
pub struct TokenTagsUpdateRequest {
    pub tokens: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct CommonResponse {
    pub status: ApiStatus,
    pub message: Option<String>,
}

// impl CommonResponse {
//     pub fn into_normal_response(self) -> NormalResponse<()> {
//         NormalResponse {
//             status: self.status,
//             data: None,
//             message: self.message,
//         }
//     }
// }
