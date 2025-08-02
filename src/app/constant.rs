mod header;
pub mod status {
    #[allow(dead_code)]
    pub struct StatusCode(pub u16);

    /// 533 Upstream Failure
    /// [A non-standard code. Indicates the server, while acting as a gateway or proxy,
    /// received a response from an upstream service that constituted a failure. Unlike
    /// 502 (Bad Gateway), which implies an invalid or unparseable response, this code
    /// suggests the upstream service itself reported an error (e.g., returned a 5xx status).]
    pub const UPSTREAM_FAILURE: ::http::StatusCode =
        unsafe { ::core::intrinsics::transmute_unchecked(StatusCode(533)) };
}

pub use header::*;
pub use status::UPSTREAM_FAILURE;

#[macro_export]
macro_rules! def_pub_const {
    // 单个常量定义
    // ($name:ident, $value:expr) => {
    //     pub const $name: &'static str = $value;
    // };

    // 批量常量定义
    ($($(#[$meta:meta])* $name:ident => $value:expr),+ $(,)?) => {
        $(
            $(#[$meta])*
            pub const $name: &'static str = $value;
        )+
    };
}

#[macro_export]
macro_rules! define_typed_constants {
    // 递归情况：处理一个类型块，然后继续处理剩余的
    (
        $vis:vis $ty:ty => {
            $(
                $(#[$attr:meta])*
                $name:ident = $value:expr
            ),* $(,)?
        }
        $($rest:tt)*
    ) => {
        $(
            $(#[$attr])*
            $vis const $name: $ty = $value;
        )*

        // 递归处理剩余的类型块
        $crate::define_typed_constants! {
            $($rest)*
        }
    };

    // 基础情况：没有更多内容时停止
    () => {};
}

pub const COMMA: char = ',';

pub use crate::common::build::{BUILD_TIMESTAMP, IS_DEBUG, IS_PRERELEASE, VERSION};

#[cfg(feature = "__preview")]
pub use crate::common::build::BUILD_VERSION;

// Package related constants
def_pub_const!(
    PKG_VERSION => env!("CARGO_PKG_VERSION"),
    PKG_NAME => env!("CARGO_PKG_NAME"),
    #[cfg(windows)]
    EXE_NAME => concat!(env!("CARGO_PKG_NAME"), ".exe"),
    #[cfg(not(windows))]
    EXE_NAME => PKG_NAME
    // PKG_DESCRIPTION => env!("CARGO_PKG_DESCRIPTION"),
    // PKG_AUTHORS => env!("CARGO_PKG_AUTHORS"),
    // PKG_REPOSITORY => env!("CARGO_PKG_REPOSITORY")
);

// Basic string constants
def_pub_const!(
    EMPTY_STRING => "",
    COMMA_STRING => ",",
    UNKNOWN => "unknown",
    TYPE => "type",
    ERROR => "error"
);

// Route related constants
def_pub_const!(
    ROUTE_ROOT_PATH => "/",
    ROUTE_HEALTH_PATH => "/health",
    ROUTE_GEN_UUID => "/gen-uuid",
    ROUTE_GEN_HASH => "/gen-hash",
    ROUTE_GEN_CHECKSUM => "/gen-checksum",
    ROUTE_GEN_TOKEN => "/gen-token",
    ROUTE_GET_TIMESTAMP_HEADER => "/get-tsheader",
    // ROUTE_USER_INFO_PATH => "/userinfo",
    ROUTE_API_PATH => "/api",
    ROUTE_LOGS_PATH => "/logs",
    ROUTE_LOGS_GET_PATH => "/logs/get",
    ROUTE_LOGS_TOKENS_GET_PATH => "/logs/tokens/get",
    ROUTE_CONFIG_PATH => "/config",
    ROUTE_TOKENS_PATH => "/tokens",
    ROUTE_TOKENS_GET_PATH => "/tokens/get",
    ROUTE_TOKENS_SET_PATH => "/tokens/set",
    ROUTE_TOKENS_ADD_PATH => "/tokens/add",
    ROUTE_TOKENS_DELETE_PATH => "/tokens/del",
    ROUTE_TOKENS_ALIAS_SET_PATH => "/tokens/alias/set",
    ROUTE_TOKENS_PROFILE_UPDATE_PATH => "/tokens/profile/update",
    ROUTE_TOKENS_CONFIG_VERSION_UPDATE_PATH => "/tokens/config-version/update",
    ROUTE_TOKENS_REFRESH_PATH => "/tokens/refresh",
    ROUTE_TOKENS_STATUS_SET_PATH => "/tokens/status/set",
    ROUTE_TOKENS_PROXY_SET_PATH => "/tokens/proxy/set",
    ROUTE_TOKENS_TIMEZONE_SET_PATH => "/tokens/timezone/set",
    ROUTE_PROXIES_PATH => "/proxies",
    ROUTE_PROXIES_GET_PATH => "/proxies/get",
    ROUTE_PROXIES_SET_PATH => "/proxies/set",
    ROUTE_PROXIES_ADD_PATH => "/proxies/add",
    ROUTE_PROXIES_DELETE_PATH => "/proxies/del",
    ROUTE_PROXIES_SET_GENERAL_PATH => "/proxies/set-general",
    ROUTE_ENV_EXAMPLE_PATH => "/env-example",
    ROUTE_STATIC_PATH => "/static/{path}",
    ROUTE_SHARED_STYLES_PATH => "/static/shared-styles.css",
    ROUTE_SHARED_JS_PATH => "/static/shared.js",
    ROUTE_ABOUT_PATH => "/about",
    ROUTE_README_PATH => "/readme",
    ROUTE_BUILD_KEY_PATH => "/build-key",
    ROUTE_CONFIG_VERSION_GET_PATH => "/config-version/get",
    ROUTE_CPP_CONFIG_PATH => "/cpp/config",
    ROUTE_CPP_MODELS_PATH => "/cpp/models",
    ROUTE_FILE_UPLOAD_PATH => "/file/upload",
    ROUTE_FILE_SYNC_PATH => "/file/sync",
    ROUTE_CPP_STREAM_PATH => "/cpp/stream",
);

// Status constants
def_pub_const!(
    STATUS_PENDING => "pending",
    STATUS_SUCCESS => "success",
    STATUS_FAILURE => "failure"
);

// Authorization constants
def_pub_const!(
    AUTHORIZATION_BEARER_PREFIX => "Bearer "
);

// Cursor related constants
def_pub_const!(
    CURSOR_API2_HOST => "api2.cursor.sh",
    CURSOR_HOST => "cursor.com",
    CURSOR_API4_HOST => "api4.cursor.sh",
    CURSOR_GCPP_ASIA_HOST => "us-asia.gcpp.cursor.sh",
    CURSOR_GCPP_EU_HOST => "us-eu.gcpp.cursor.sh",
    CURSOR_GCPP_US_HOST => "us-only.gcpp.cursor.sh"
);

// Object type constants
def_pub_const!(
    OBJECT_CHAT_COMPLETION => "chat.completion",
    OBJECT_CHAT_COMPLETION_CHUNK => "chat.completion.chunk",
    CHATCMPL_PREFIX => "chatcmpl-",
    MSG01_PREFIX => "msg_01",
    // TOOLU01_PREFIX => "toolu_01",
    // OBJECT_TEXT_COMPLETION => "text_completion"
);

// def_pub_const!(
//     CURSOR_API2_STREAM_CHAT => "StreamChat",
//     CURSOR_API2_GET_USER_INFO => "GetUserInfo"
// );

// Finish reason constants
def_pub_const!(
    FINISH_REASON_STOP => "stop"
);

// Error message constants
def_pub_const!(
    ERR_STREAM_RESPONSE => "Empty stream response",
    ERR_RESPONSE_RECEIVED => "Empty response received",
    ERR_LOG_TOKEN_NOT_FOUND => "日志对应的token必须存在 - 数据一致性错误",
    INVALID_STREAM => "invalid_stream"
);

// def_pub_const!(ERR_CHECKSUM_NO_GOOD => "checksum no good");

def_pub_const!(
    HEADER_B64 => "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.",
    ISSUER => "https://authentication.cursor.sh",
    SCOPE => "openid profile email offline_access",
    AUDIENCE => "https://cursor.com",
    TYPE_SESSION => "session",
    TYPE_WEB => "web"
);

def_pub_const!(
    ASIA => "Asia",
    EU => "EU",
    US => "US"
);

def_pub_const!(
    UNNAMED => "unnamed",
    UNNAMED_PATTERN => "unnamed-"
);

def_pub_const!(
    HTTPS_PREFIX => "https://"
);

def_pub_const! {
    DEFAULT_THINKING_TAG => "think",
    DEFAULT_THINKING_TAG_OPEN => "<think>",
    DEFAULT_THINKING_TAG_CLOSE => "</think>"
}

static mut THINKING_TAG_OPEN: *const str = DEFAULT_THINKING_TAG_OPEN;
static mut THINKING_TAG_CLOSE: *const str = DEFAULT_THINKING_TAG_CLOSE;

#[deny(unused)]
pub fn init_thinking_tags() {
    unsafe {
        // 避免重复初始化
        if THINKING_TAG_OPEN as *const u8 != (DEFAULT_THINKING_TAG_OPEN as *const str) as *const u8
        {
            return;
        }

        let tag = crate::common::utils::parse_string_from_env("THINKING_TAG", DEFAULT_THINKING_TAG);

        if tag == DEFAULT_THINKING_TAG {
            return;
        }

        // 检查标签长度限制
        const MAX_TAG_LEN: usize = 16;
        let tag_len = tag.len();
        if tag_len > MAX_TAG_LEN - 3 {
            __eprintln!("Warning: THINKING_TAG too long, using default");
            return;
        }

        let mut buf = [0u8; MAX_TAG_LEN];
        let tag_bytes = tag.as_bytes();

        // 构建开始标签 <tag>
        buf[0] = b'<';
        ::core::ptr::copy_nonoverlapping(tag_bytes.as_ptr(), buf.as_mut_ptr().add(1), tag_len);
        *buf.get_unchecked_mut(tag_len + 1) = b'>';
        let open_len = tag_len + 2;

        // 分配开始标签
        let open_layout = ::core::alloc::Layout::array::<u8>(open_len).unwrap();
        let open_ptr = ::std::alloc::alloc(open_layout);
        if open_ptr.is_null() {
            ::std::alloc::handle_alloc_error(open_layout);
        }
        ::core::ptr::copy_nonoverlapping(buf.as_ptr(), open_ptr, open_len);
        THINKING_TAG_OPEN =
            ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(open_ptr, open_len));

        // 构建结束标签 </tag>
        buf[1] = b'/';
        ::core::ptr::copy_nonoverlapping(tag_bytes.as_ptr(), buf.as_mut_ptr().add(2), tag_len);
        *buf.get_unchecked_mut(tag_len + 2) = b'>';
        let close_len = tag_len + 3;

        // 分配结束标签
        let close_layout = ::core::alloc::Layout::array::<u8>(close_len).unwrap();
        let close_ptr = ::std::alloc::alloc(close_layout);
        if close_ptr.is_null() {
            ::std::alloc::handle_alloc_error(close_layout);
        }
        ::core::ptr::copy_nonoverlapping(buf.as_ptr(), close_ptr, close_len);
        THINKING_TAG_CLOSE =
            ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(close_ptr, close_len));
    }
}

#[inline(always)]
pub fn get_thinking_tag_open() -> &'static str { unsafe { &*THINKING_TAG_OPEN } }

#[inline(always)]
pub fn get_thinking_tag_close() -> &'static str { unsafe { &*THINKING_TAG_CLOSE } }
