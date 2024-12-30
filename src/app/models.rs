use super::{constant::*, token::UserUsageInfo};
use crate::message::Message;
use chrono::{DateTime, Local};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::sync::RwLock;

// 页面内容类型枚举
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type", content = "content")]
pub enum PageContent {
    #[serde(rename = "default")]
    Default, // 默认行为
    #[serde(rename = "text")]
    Text(String), // 纯文本
    #[serde(rename = "html")]
    Html(String), // HTML 内容
}

impl Default for PageContent {
    fn default() -> Self {
        Self::Default
    }
}

// 静态配置
#[derive(Clone)]
pub struct AppConfig {
    enable_stream_check: bool,
    include_stop_stream: bool,
    vision_ability: VisionAbility,
    enable_slow_pool: bool,
    allow_claude: bool,
    auth_token: String,
    token_file: String,
    token_list_file: String,
    route_prefix: String,
    pub start_time: chrono::DateTime<chrono::Local>,
    pages: Pages,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum VisionAbility {
    #[serde(rename = "none", alias = "disabled")]
    None,
    #[serde(rename = "base64", alias = "base64-only")]
    Base64,
    #[serde(rename = "all", alias = "base64-http")]
    All,
}

impl VisionAbility {
    pub fn from_str(s: &str) -> Result<Self, &'static str> {
        match s.to_lowercase().as_str() {
            "none" | "disabled" => Ok(Self::None),
            "base64" | "base64-only" => Ok(Self::Base64),
            "all" | "base64-http" => Ok(Self::All),
            _ => Err("Invalid VisionAbility value"),
        }
    }
}

impl Default for VisionAbility {
    fn default() -> Self {
        Self::Base64
    }
}

#[derive(Clone)]
pub struct Pages {
    pub root_content: PageContent,
    pub logs_content: PageContent,
    pub config_content: PageContent,
    pub tokeninfo_content: PageContent,
    pub shared_styles_content: PageContent,
    pub shared_js_content: PageContent,
}

impl Default for Pages {
    fn default() -> Self {
        Self {
            root_content: PageContent::Default,
            logs_content: PageContent::Default,
            config_content: PageContent::Default,
            tokeninfo_content: PageContent::Default,
            shared_styles_content: PageContent::Default,
            shared_js_content: PageContent::Default,
        }
    }
}

// 运行时状态
pub struct AppState {
    pub total_requests: u64,
    pub active_requests: u64,
    pub request_logs: Vec<RequestLog>,
    pub token_infos: Vec<TokenInfo>,
}

// 全局配置实例
lazy_static! {
    pub static ref APP_CONFIG: RwLock<AppConfig> = RwLock::new(AppConfig::default());
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            enable_stream_check: true,
            include_stop_stream: true,
            vision_ability: VisionAbility::Base64,
            enable_slow_pool: false,
            allow_claude: false,
            auth_token: String::new(),
            token_file: ".token".to_string(),
            token_list_file: ".token-list".to_string(),
            route_prefix: String::new(),
            start_time: chrono::Local::now(),
            pages: Pages::default(),
        }
    }
}

impl AppConfig {
    pub fn init(
        enable_stream_check: bool,
        include_stop_stream: bool,
        vision_ability: VisionAbility,
        enable_slow_pool: bool,
        allow_claude: bool,
        auth_token: String,
        token_file: String,
        token_list_file: String,
        route_prefix: String,
    ) {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.enable_stream_check = enable_stream_check;
            config.include_stop_stream = include_stop_stream;
            config.vision_ability = vision_ability;
            config.enable_slow_pool = enable_slow_pool;
            config.allow_claude = allow_claude;
            config.auth_token = auth_token;
            config.token_file = token_file;
            config.token_list_file = token_list_file;
            config.route_prefix = route_prefix;
        }
    }

    pub fn get_stream_check() -> bool {
        APP_CONFIG
            .read()
            .map(|config| config.enable_stream_check)
            .unwrap_or(true)
    }

    pub fn get_stop_stream() -> bool {
        APP_CONFIG
            .read()
            .map(|config| config.include_stop_stream)
            .unwrap_or(true)
    }

    pub fn get_vision_ability() -> VisionAbility {
        APP_CONFIG
            .read()
            .map(|config| config.vision_ability.clone())
            .unwrap_or_default()
    }

    pub fn get_slow_pool() -> bool {
        APP_CONFIG
            .read()
            .map(|config| config.enable_slow_pool)
            .unwrap_or(false)
    }

    pub fn get_allow_claude() -> bool {
        APP_CONFIG
            .read()
            .map(|config| config.allow_claude)
            .unwrap_or(false)
    }

    pub fn get_auth_token() -> String {
        APP_CONFIG
            .read()
            .map(|config| config.auth_token.clone())
            .unwrap_or_default()
    }

    pub fn get_token_file() -> String {
        APP_CONFIG
            .read()
            .map(|config| config.token_file.clone())
            .unwrap_or_default()
    }

    pub fn get_token_list_file() -> String {
        APP_CONFIG
            .read()
            .map(|config| config.token_list_file.clone())
            .unwrap_or_default()
    }

    pub fn get_route_prefix() -> String {
        APP_CONFIG
            .read()
            .map(|config| config.route_prefix.clone())
            .unwrap_or_default()
    }

    pub fn get_page_content(path: &str) -> Option<PageContent> {
        APP_CONFIG.read().ok().map(|config| match path {
            ROUTER_ROOT_PATH => config.pages.root_content.clone(),
            ROUTER_LOGS_PATH => config.pages.logs_content.clone(),
            ROUTER_CONFIG_PATH => config.pages.config_content.clone(),
            ROUTER_TOKENINFO_PATH => config.pages.tokeninfo_content.clone(),
            ROUTER_SHARED_STYLES_PATH => config.pages.shared_styles_content.clone(),
            ROUTER_SHARED_JS_PATH => config.pages.shared_js_content.clone(),
            _ => PageContent::Default,
        })
    }

    pub fn update_stream_check(enable: bool) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.enable_stream_check = enable;
            Ok(())
        } else {
            Err("无法更新配置")
        }
    }

    pub fn update_stop_stream(enable: bool) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.include_stop_stream = enable;
            Ok(())
        } else {
            Err("无法更新配置")
        }
    }

    pub fn update_vision_ability(new_ability: VisionAbility) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.vision_ability = new_ability;
            Ok(())
        } else {
            Err("无法更新配置")
        }
    }

    pub fn update_slow_pool(enable: bool) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.enable_slow_pool = enable;
            Ok(())
        } else {
            Err("无法更新配置")
        }
    }

    pub fn update_allow_claude(enable: bool) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.allow_claude = enable;
            Ok(())
        } else {
            Err("无法更新配置")
        }
    }

    pub fn update_page_content(path: &str, content: PageContent) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            match path {
                ROUTER_ROOT_PATH => config.pages.root_content = content,
                ROUTER_LOGS_PATH => config.pages.logs_content = content,
                ROUTER_CONFIG_PATH => config.pages.config_content = content,
                ROUTER_TOKENINFO_PATH => config.pages.tokeninfo_content = content,
                ROUTER_SHARED_STYLES_PATH => config.pages.shared_styles_content = content,
                ROUTER_SHARED_JS_PATH => config.pages.shared_js_content = content,
                _ => return Err("无效的路径"),
            }
            Ok(())
        } else {
            Err("无法更新配置")
        }
    }

    pub fn reset_stream_check() -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.enable_stream_check = true;
            Ok(())
        } else {
            Err("无法重置配置")
        }
    }

    pub fn reset_stop_stream() -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.include_stop_stream = true;
            Ok(())
        } else {
            Err("无法重置配置")
        }
    }

    pub fn reset_vision_ability() -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.vision_ability = VisionAbility::Base64;
            Ok(())
        } else {
            Err("无法重置配置")
        }
    }

    pub fn reset_slow_pool() -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.enable_slow_pool = false;
            Ok(())
        } else {
            Err("无法重置配置")
        }
    }

    pub fn reset_allow_claude() -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            config.allow_claude = false;
            Ok(())
        } else {
            Err("无法重置配置")
        }
    }

    pub fn reset_page_content(path: &str) -> Result<(), &'static str> {
        if let Ok(mut config) = APP_CONFIG.write() {
            match path {
                ROUTER_ROOT_PATH => config.pages.root_content = PageContent::Default,
                ROUTER_LOGS_PATH => config.pages.logs_content = PageContent::Default,
                ROUTER_CONFIG_PATH => config.pages.config_content = PageContent::Default,
                ROUTER_TOKENINFO_PATH => config.pages.tokeninfo_content = PageContent::Default,
                ROUTER_SHARED_STYLES_PATH => {
                    config.pages.shared_styles_content = PageContent::Default
                }
                ROUTER_SHARED_JS_PATH => config.pages.shared_js_content = PageContent::Default,
                _ => return Err("无效的路径"),
            }
            Ok(())
        } else {
            Err("无法重置配置")
        }
    }
}

impl AppState {
    pub fn new(token_infos: Vec<TokenInfo>) -> Self {
        Self {
            total_requests: 0,
            active_requests: 0,
            request_logs: Vec::new(),
            token_infos,
        }
    }

    pub fn update_token_infos(&mut self, token_infos: Vec<TokenInfo>) {
        self.token_infos = token_infos;
    }
}

// 模型定义
#[derive(Serialize, Clone)]
pub struct Model {
    pub id: String,
    pub created: i64,
    pub object: String,
    pub owned_by: String,
}

// impl Model {
//     pub fn is_pesticide(&self) -> bool {
//         !(self.owned_by.as_str() == CURSOR || self.id.as_str() == "gpt-4o-mini") 
//     }
// }

// 请求日志
#[derive(Serialize, Clone)]
pub struct RequestLog {
    pub timestamp: DateTime<Local>,
    pub model: String,
    pub token_info: TokenInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt: Option<String>,
    pub stream: bool,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

// pub struct PromptList(Option<String>);

// impl PromptList {
//     pub fn to_vec(&self) -> Vec<>
// }

// 聊天请求
#[derive(Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<Message>,
    #[serde(default)]
    pub stream: bool,
}

// 用于存储 token 信息
#[derive(Serialize, Clone)]
pub struct TokenInfo {
    pub token: String,
    pub checksum: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<UserUsageInfo>,
}

// TokenUpdateRequest 结构体
#[derive(Deserialize)]
pub struct TokenUpdateRequest {
    pub tokens: String,
    #[serde(default)]
    pub token_list: Option<String>,
}

// 添加用于接收更新请求的结构体
#[derive(Deserialize)]
pub struct ConfigUpdateRequest {
    #[serde(default)]
    pub action: String, // "get", "update", "reset"
    #[serde(default)]
    pub path: String,
    #[serde(default)]
    pub content: Option<PageContent>, // "default", "text", "html"
    #[serde(default)]
    pub enable_stream_check: Option<bool>,
    #[serde(default)]
    pub include_stop_stream: Option<bool>,
    #[serde(default)]
    pub vision_ability: Option<VisionAbility>,
    #[serde(default)]
    pub enable_slow_pool: Option<bool>,
    #[serde(default)]
    pub enable_all_claude: Option<bool>,
}
