pub mod log;

use super::{
    constant::{
        CURSOR_API2_HOST, CURSOR_API4_HOST, CURSOR_GCPP_ASIA_HOST, CURSOR_GCPP_EU_HOST,
        CURSOR_GCPP_US_HOST, CURSOR_HOST, EMPTY_STRING, HTTPS_PREFIX,
    },
    model::{DateTime, GcppHost},
};
use crate::common::utils::{parse_bool_from_env, parse_string_from_env, parse_usize_from_env};
use std::{
    borrow::Cow,
    path::PathBuf,
    sync::{LazyLock, OnceLock},
};

macro_rules! def_pub_static {
    // 基础版本：直接存储 String
    ($name:ident, $value:expr) => {
        pub static $name: LazyLock<String> = LazyLock::new(|| $value);
    };

    // 环境变量版本
    ($name:ident,env: $env_key:expr,default: $default:expr) => {
        pub static $name: LazyLock<Cow<'static, str>> =
            LazyLock::new(|| parse_string_from_env($env_key, $default));
    };
}

def_pub_static!(AUTH_TOKEN, env: "AUTH_TOKEN", default: EMPTY_STRING);

static START_TIME: OnceLock<chrono::NaiveDateTime> = OnceLock::new();

#[inline]
pub fn get_start_time() -> &'static chrono::NaiveDateTime {
    START_TIME.get_or_init(DateTime::naive_now)
}

pub static GENERAL_TIMEZONE: LazyLock<chrono_tz::Tz> = LazyLock::new(|| {
    use std::str::FromStr as _;
    let tz = parse_string_from_env("GENERAL_TIMEZONE", EMPTY_STRING);
    if tz.is_empty() {
        __eprintln!(
            "未配置时区，请在环境变量GENERAL_TIMEZONE中设置，格式如'Asia/Shanghai'\n将使用默认时区: Asia/Shanghai"
        );
        return chrono_tz::Tz::Asia__Shanghai;
    }
    match chrono_tz::Tz::from_str(&tz) {
        Ok(tz) => tz,
        Err(e) => {
            eprintln!("无法解析时区 '{tz}': {e}\n将使用默认时区: Asia/Shanghai");
            chrono_tz::Tz::Asia__Shanghai
        }
    }
});

def_pub_static!(DEFAULT_INSTRUCTIONS, env: "DEFAULT_INSTRUCTIONS", default: "Respond in Chinese by default\n<|END_USER|>\n\n<|BEGIN_ASSISTANT|>\n\n\nYour will\n<|END_ASSISTANT|>\n\n<|BEGIN_USER|>\n\n\nThe current date is {{currentDateTime}}");

pub fn get_default_instructions(now_with_tz: chrono::DateTime<chrono_tz::Tz>) -> String {
    DEFAULT_INSTRUCTIONS.replace(
        "{{currentDateTime}}",
        &now_with_tz.to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
    )
}

pub static GENERAL_GCPP_HOST: LazyLock<GcppHost> = LazyLock::new(|| {
    let gcpp_host = parse_string_from_env("GENERAL_GCPP_HOST", EMPTY_STRING);
    let gcpp_host = gcpp_host.trim();
    if gcpp_host.is_empty() {
        __eprintln!(
            "未配置默认代码补全区域，请在环境变量GENERAL_GCPP_HOST中设置，格式如'Asia'\n将使用默认区域: Asia"
        );
        return GcppHost::Asia;
    }
    match GcppHost::from_str(gcpp_host) {
        Some(gcpp_host) => gcpp_host,
        None => {
            eprintln!("无法解析区域 '{gcpp_host}'\n将使用默认区域: Asia");
            GcppHost::Asia
        }
    }
});

def_pub_static!(PRI_REVERSE_PROXY_HOST, env: "PRI_REVERSE_PROXY_HOST", default: EMPTY_STRING);

def_pub_static!(PUB_REVERSE_PROXY_HOST, env: "PUB_REVERSE_PROXY_HOST", default: EMPTY_STRING);

const DEFAULT_KEY_PREFIX: &str = "sk-";

def_pub_static!(KEY_PREFIX, env: "KEY_PREFIX", default: DEFAULT_KEY_PREFIX);

// pub static TOKEN_DELIMITER: LazyLock<char> = LazyLock::new(|| {
//     let delimiter = parse_ascii_char_from_env("TOKEN_DELIMITER", COMMA);
//     if delimiter.is_ascii_alphabetic()
//         || delimiter.is_ascii_digit()
//         || delimiter == '/'
//         || delimiter == '-'
//         || delimiter == '_'
//     {
//         COMMA
//     } else {
//         delimiter
//     }
// });

// pub static USE_COMMA_DELIMITER: LazyLock<bool> = LazyLock::new(|| {
//     let enable = parse_bool_from_env("USE_COMMA_DELIMITER", true);
//     if enable && *TOKEN_DELIMITER == COMMA {
//         false
//     } else {
//         enable
//     }
// });

pub static USE_PRI_REVERSE_PROXY: LazyLock<bool> =
    LazyLock::new(|| !PRI_REVERSE_PROXY_HOST.is_empty());

pub static USE_PUB_REVERSE_PROXY: LazyLock<bool> =
    LazyLock::new(|| !PUB_REVERSE_PROXY_HOST.is_empty());

macro_rules! def_cursor_api_url {
    // 单个API URL定义
    ($name:ident, $api_host:ident, $path:expr) => {
        #[doc = $path]
        pub fn $name(is_pri: bool) -> &'static str {
            static URL_PRI: OnceLock<String> = OnceLock::new();
            static URL_PUB: OnceLock<String> = OnceLock::new();

            if is_pri {
                URL_PRI.get_or_init(|| {
                    let host = if *USE_PRI_REVERSE_PROXY {
                        &PRI_REVERSE_PROXY_HOST
                    } else {
                        $api_host
                    };
                    let mut url = String::with_capacity(HTTPS_PREFIX.len() + host.len() + $path.len());
                    url.push_str(HTTPS_PREFIX);
                    url.push_str(host);
                    url.push_str($path);
                    url
                })
            } else {
                URL_PUB.get_or_init(|| {
                    let host = if *USE_PUB_REVERSE_PROXY {
                        &PUB_REVERSE_PROXY_HOST
                    } else {
                        $api_host
                    };
                    let mut url = String::with_capacity(HTTPS_PREFIX.len() + host.len() + $path.len());
                    url.push_str(HTTPS_PREFIX);
                    url.push_str(host);
                    url.push_str($path);
                    url
                })
            }
        }
    };

    // 批量API URL定义
    ([$($name:ident),+ $(,)?], $api_host:ident, [$($path:expr),+ $(,)?]) => {
        $(
            def_cursor_api_url!($name, $api_host, $path);
        )+
    };
}

// API2 HOST 相关API
def_cursor_api_url!(
    [
        chat_url,
        chat_models_url,
        stripe_url,
        token_poll_url,
        token_refresh_url,
        server_config_url,
        is_on_new_pricing_url,
        sessions_url
    ],
    CURSOR_API2_HOST,
    [
        "/aiserver.v1.ChatService/StreamUnifiedChatWithTools",
        "/aiserver.v1.AiService/AvailableModels",
        "/auth/full_stripe_profile",
        "/auth/poll",
        "/oauth/token",
        "/aiserver.v1.ServerConfigService/GetServerConfig",
        "/api/dashboard/is-on-new-pricing",
        "/api/auth/sessions"
    ]
);

// CURSOR HOST 相关API
def_cursor_api_url!(
    [
        usage_api_url,
        user_api_url,
        token_upgrade_url,
        teams_url,
        aggregated_usage_events_url,
        filtered_usage_events_url
    ],
    CURSOR_HOST,
    [
        "/api/usage",
        "/api/auth/me",
        "/api/auth/loginDeepCallbackControl",
        "/api/dashboard/teams",
        "/api/dashboard/get-aggregated-usage-events",
        "/api/dashboard/get-filtered-usage-events"
    ]
);

// API4 HOST 相关API
def_cursor_api_url!(
    cpp_config_url,
    CURSOR_API4_HOST,
    "/aiserver.v1.AiService/CppConfig"
);

// API2 HOST CPP相关API
def_cursor_api_url!(
    cpp_models_url,
    CURSOR_API2_HOST,
    "/aiserver.v1.CppService/AvailableModels"
);

// GCPP ASIA HOST 相关API
def_cursor_api_url!(
    [
        asia_upload_file_url,
        asia_sync_file_url,
        asia_stream_cpp_url,
        // asia_next_cursor_prediction_url
    ],
    CURSOR_GCPP_ASIA_HOST,
    [
        "/aiserver.v1.FileSyncService/FSUploadFile",
        "/aiserver.v1.FileSyncService/FSSyncFile",
        "/aiserver.v1.AiService/StreamCpp",
        // "/aiserver.v1.AiService/StreamNextCursorPrediction"
    ]
);

// GCPP EU HOST 相关API
def_cursor_api_url!(
    [
        eu_upload_file_url,
        eu_sync_file_url,
        eu_stream_cpp_url,
        // eu_next_cursor_prediction_url
    ],
    CURSOR_GCPP_EU_HOST,
    [
        "/aiserver.v1.FileSyncService/FSUploadFile",
        "/aiserver.v1.FileSyncService/FSSyncFile",
        "/aiserver.v1.AiService/StreamCpp",
        // "/aiserver.v1.AiService/StreamNextCursorPrediction"
    ]
);

// GCPP US HOST 相关API
def_cursor_api_url!(
    [
        us_upload_file_url,
        us_sync_file_url,
        us_stream_cpp_url,
        // us_next_cursor_prediction_url
    ],
    CURSOR_GCPP_US_HOST,
    [
        "/aiserver.v1.FileSyncService/FSUploadFile",
        "/aiserver.v1.FileSyncService/FSSyncFile",
        "/aiserver.v1.AiService/StreamCpp",
        // "/aiserver.v1.AiService/StreamNextCursorPrediction"
    ]
);

static DATA_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    let data_dir = parse_string_from_env("DATA_DIR", "data");
    let path = std::env::current_exe()
        .ok()
        .and_then(|exe_path| exe_path.parent().map(|p| p.to_path_buf()))
        .unwrap_or_else(|| PathBuf::from("."))
        .join(&*data_dir);
    if !path.exists() {
        std::fs::create_dir_all(&path).expect("无法创建数据目录");
    }
    path
});

pub static STATIC_DIR: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("static"));

pub(super) static CONFIG_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("config.bin"));

pub(super) static LOGS_FILE_PATH: LazyLock<PathBuf> = LazyLock::new(|| DATA_DIR.join("logs.bin"));

pub(super) static TOKENS_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("tokens.bin"));

pub(super) static PROXIES_FILE_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| DATA_DIR.join("proxies.bin"));

// TCP 和超时相关常量
const DEFAULT_TCP_KEEPALIVE: usize = 90;
const MAX_TCP_KEEPALIVE: u64 = 600;

pub static TCP_KEEPALIVE: LazyLock<u64> = LazyLock::new(|| {
    let keepalive = parse_usize_from_env("TCP_KEEPALIVE", DEFAULT_TCP_KEEPALIVE);
    u64::try_from(keepalive)
        .map(|t| t.min(MAX_TCP_KEEPALIVE))
        .unwrap_or(DEFAULT_TCP_KEEPALIVE as u64)
});

const DEFAULT_SERVICE_TIMEOUT: usize = 30;
const MAX_SERVICE_TIMEOUT: u64 = 600;

pub static SERVICE_TIMEOUT: LazyLock<u64> = LazyLock::new(|| {
    let timeout = parse_usize_from_env("SERVICE_TIMEOUT", DEFAULT_SERVICE_TIMEOUT);
    u64::try_from(timeout)
        .map(|t| t.min(MAX_SERVICE_TIMEOUT))
        .unwrap_or(DEFAULT_SERVICE_TIMEOUT as u64)
});

pub static REAL_USAGE: LazyLock<bool> = LazyLock::new(|| parse_bool_from_env("REAL_USAGE", true));

// pub static TOKEN_VALIDITY_RANGE: LazyLock<TokenValidityRange> = LazyLock::new(|| {
//     let short = if let Ok(Ok(validity)) = std::env::var("TOKEN_SHORT_VALIDITY")
//         .as_deref()
//         .map(ValidityRange::from_str)
//     {
//         validity
//     } else {
//         ValidityRange::new(5400, 5400)
//     };
//     let long = if let Ok(Ok(validity)) = std::env::var("TOKEN_LONG_VALIDITY")
//         .as_deref()
//         .map(ValidityRange::from_str)
//     {
//         validity
//     } else {
//         ValidityRange::new(5184000, 5184000)
//     };
//     TokenValidityRange::new(short, long)
// });
