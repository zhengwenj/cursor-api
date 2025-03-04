use crate::common::model::{ApiStatus, userinfo::TokenProfile};
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
pub mod proxy_pool;
mod build_key;
pub use build_key::*;
mod state;
pub use state::*;
mod proxy;
pub use proxy::*;

use super::constant::{STATUS_FAILURE, STATUS_PENDING, STATUS_SUCCESS};

#[derive(Clone, Archive, RkyvDeserialize, RkyvSerialize)]
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
#[derive(Serialize, Clone, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct RequestLog {
    pub id: u64,
    pub timestamp: chrono::DateTime<chrono::Local>,
    pub model: String,
    pub token_info: TokenInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<Chain>,
    pub timing: TimingInfo,
    pub stream: bool,
    pub status: LogStatus,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

#[derive(Serialize, Clone, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct Chain {
    pub prompt: String,
    pub delays: Vec<(String, f64)>,
}

#[derive(Serialize, Clone, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct TimingInfo {
    pub total: f64, // 总用时(秒)
    // #[serde(skip_serializing_if = "Option::is_none")]
    // pub first: Option<f64>, // 首字时间(秒)
}

// 用于存储 token 信息
#[derive(Clone, Serialize, Archive, RkyvSerialize, RkyvDeserialize)]
pub struct TokenInfo {
    pub token: String,
    pub checksum: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub profile: Option<TokenProfile>,
    pub tags: Option<Vec<String>>,
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
}

// TokenUpdateRequest 结构体
#[derive(Deserialize)]
pub struct TokenUpdateRequest {
    pub tokens: String,
}

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
            DeleteResponseExpectation::UpdatedTokens
                | DeleteResponseExpectation::Detailed
        )
    }

    pub fn needs_failed_tokens(&self) -> bool {
        matches!(
            self,
            DeleteResponseExpectation::FailedTokens
                | DeleteResponseExpectation::Detailed
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
