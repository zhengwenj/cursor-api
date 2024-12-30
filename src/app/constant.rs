pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_DESCRIPTION: &str = env!("CARGO_PKG_DESCRIPTION");
pub const PKG_AUTHORS: &str = env!("CARGO_PKG_AUTHORS");
pub const PKG_REPOSITORY: &str = env!("CARGO_PKG_REPOSITORY");

pub const ROUTER_ROOT_PATH: &str = "/";
pub const ROUTER_HEALTH_PATH: &str = "/health";
pub const ROUTER_GET_CHECKSUM: &str = "/get-checksum";
pub const ROUTER_GET_USER_INFO_PATH: &str = "/get-user-info";
pub const ROUTER_LOGS_PATH: &str = "/logs";
pub const ROUTER_CONFIG_PATH: &str = "/config";
pub const ROUTER_TOKENINFO_PATH: &str = "/tokeninfo";
pub const ROUTER_GET_TOKENINFO_PATH: &str = "/get-tokeninfo";
pub const ROUTER_UPDATE_TOKENINFO_PATH: &str = "/update-tokeninfo";
pub const ROUTER_ENV_EXAMPLE_PATH: &str = "/env-example";
pub const ROUTER_SHARED_STYLES_PATH: &str = "/static/shared-styles.css";
pub const ROUTER_SHARED_JS_PATH: &str = "/static/shared.js";

pub const STATUS: &str = "status";
pub const MESSAGE: &str = "message";
pub const ERROR: &str = "error";

pub const TOKEN_FILE: &str = "token_file";
pub const TOKEN_LIST_FILE: &str = "token_list_file";
pub const TOKENS: &str = "tokens";
pub const TOKEN_LIST: &str = "token_list";

pub const STATUS_SUCCESS: &str = "success";
pub const STATUS_FAILED: &str = "failed";

pub const HEADER_NAME_CONTENT_TYPE: &str = "content-type";
pub const HEADER_NAME_AUTHORIZATION: &str = "Authorization";

pub const CONTENT_TYPE_PROTO: &str = "application/proto";
pub const CONTENT_TYPE_CONNECT_PROTO: &str = "application/connect+proto";
pub const CONTENT_TYPE_TEXT_HTML_WITH_UTF8: &str = "text/html;charset=utf-8";
pub const CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8: &str = "text/plain;charset=utf-8";

pub const AUTHORIZATION_BEARER_PREFIX: &str = "Bearer ";

pub const OBJECT_CHAT_COMPLETION: &str = "chat.completion";
pub const OBJECT_CHAT_COMPLETION_CHUNK: &str = "chat.completion.chunk";

pub const CURSOR_API2_HOST: &str = "api2.cursor.sh";
pub const CURSOR_API2_BASE_URL: &str = "https://api2.cursor.sh/aiserver.v1.AiService/";

pub const CURSOR_API2_STREAM_CHAT: &str = "StreamChat";
pub const CURSOR_API2_GET_USER_INFO: &str = "GetUserInfo";

pub const FINISH_REASON_STOP: &str = "stop";

pub const LONG_CONTEXT_MODELS: [&str; 4] = [
    "gpt-4o-128k",
    "gemini-1.5-flash-500k",
    "claude-3-haiku-200k",
    "claude-3-5-sonnet-200k",
];

pub const MODEL_OBJECT: &str = "model";
pub const ANTHROPIC: &str = "anthropic";
pub const CURSOR: &str = "cursor";
pub const GOOGLE: &str = "google";
pub const OPENAI: &str = "openai";
