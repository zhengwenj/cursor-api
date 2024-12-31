use super::{
    constant::{DEFAULT_TOKEN_FILE_NAME, DEFAULT_TOKEN_LIST_FILE_NAME, EMPTY_STRING},
    utils::parse_string_from_env,
};
use std::sync::LazyLock;

macro_rules! def_pub_static {
    // 基础版本：直接存储 String
    ($name:ident, $value:expr) => {
        pub static $name: LazyLock<String> = LazyLock::new(|| $value);

        def_pub_static_getter!($name);
    };

    // 环境变量版本
    ($name:ident, env: $env_key:expr, default: $default:expr) => {
        pub static $name: LazyLock<String> =
            LazyLock::new(|| parse_string_from_env($env_key, $default).trim().to_string());

        def_pub_static_getter!($name);
    };
}

macro_rules! def_pub_static_getter {
    ($name:ident) => {
        paste::paste! {
            pub fn [<get_ $name:lower>]() -> String {
                (*$name).clone()
            }
        }
    };
}

def_pub_static!(ROUTE_PREFIX, env: "ROUTE_PREFIX", default: EMPTY_STRING);
def_pub_static!(AUTH_TOKEN, env: "AUTH_TOKEN", default: EMPTY_STRING);
def_pub_static!(TOKEN_FILE, env: "TOKEN_FILE", default: DEFAULT_TOKEN_FILE_NAME);
def_pub_static!(TOKEN_LIST_FILE, env: "TOKEN_LIST_FILE", default: DEFAULT_TOKEN_LIST_FILE_NAME);
def_pub_static!(
    ROUTE_MODELS_PATH,
    format!("{}/v1/models", ROUTE_PREFIX.as_str())
);
def_pub_static!(
    ROUTE_CHAT_PATH,
    format!("{}/v1/chat/completions", ROUTE_PREFIX.as_str())
);

pub static START_TIME: LazyLock<chrono::DateTime<chrono::Local>> =
    LazyLock::new(chrono::Local::now);

pub fn get_start_time() -> chrono::DateTime<chrono::Local> {
    *START_TIME
}
