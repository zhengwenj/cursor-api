use crate::app::{
    constant::{
        CONTENT_TYPE_CONNECT_PROTO, CONTENT_TYPE_PROTO, CURSOR_API2_HOST, CURSOR_HOST,
        CURSOR_SETTINGS_URL, HEADER_NAME_GHOST_MODE, TRUE,
    },
    lazy::{
        PRI_REVERSE_PROXY_HOST, PUB_REVERSE_PROXY_HOST, USE_PRI_REVERSE_PROXY,
        USE_PUB_REVERSE_PROXY, cursor_api2_stripe_url, cursor_token_poll_url,
        cursor_token_upgrade_url, cursor_usage_api_url, cursor_user_api_url,
    },
};
use reqwest::{
    Client, RequestBuilder,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL, CONNECTION, CONTENT_LENGTH,
        CONTENT_TYPE, COOKIE, DNT, HOST, ORIGIN, PRAGMA, REFERER, TE, TRANSFER_ENCODING,
        USER_AGENT,
    },
};

macro_rules! def_const {
    ($name:ident, $value:expr) => {
        const $name: &'static str = $value;
    };
}

def_const!(SEC_FETCH_DEST, "sec-fetch-dest");
def_const!(SEC_FETCH_MODE, "sec-fetch-mode");
def_const!(SEC_FETCH_SITE, "sec-fetch-site");
def_const!(SEC_GPC, "sec-gpc");
def_const!(PRIORITY, "priority");

def_const!(ONE, "1");
def_const!(ENCODINGS, "gzip,br");
def_const!(VALUE_ACCEPT, "*/*");
def_const!(VALUE_LANGUAGE, "en-US");
def_const!(EMPTY, "empty");
def_const!(CORS, "cors");
def_const!(NO_CACHE, "no-cache");
def_const!(
    UA_WIN,
    "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36"
);
def_const!(SAME_ORIGIN, "same-origin");
def_const!(KEEP_ALIVE, "keep-alive");
def_const!(TRAILERS, "trailers");
def_const!(U_EQ_4, "u=4");
def_const!(U_EQ_0, "u=0");

def_const!(PROXY_HOST, "x-co");

#[inline]
fn get_client_and_host_post<'a>(
    client: &Client,
    url: &'a str,
    is_pri: bool,
    real_host: &'a str,
) -> (RequestBuilder, &'a str) {
    if is_pri && *USE_PRI_REVERSE_PROXY {
        (
            client.post(url).header(PROXY_HOST, real_host),
            PRI_REVERSE_PROXY_HOST.as_str(),
        )
    } else if !is_pri && *USE_PUB_REVERSE_PROXY {
        (
            client.post(url).header(PROXY_HOST, real_host),
            PUB_REVERSE_PROXY_HOST.as_str(),
        )
    } else {
        (client.post(url), real_host)
    }
}

pub(crate) struct AiServiceRequest<'a> {
    pub(crate) client: Client,
    pub(crate) auth_token: &'a str,
    pub(crate) checksum: &'a str,
    pub(crate) client_key: &'a str,
    pub(crate) url: &'a str,
    pub(crate) is_stream: bool,
    pub(crate) timezone: &'static str,
    pub(crate) trace_id: &'a str,
    pub(crate) is_pri: bool,
}

/// 返回预构建的 Cursor API 客户端
///
/// # 参数
///
/// * `auth_token` - 授权令牌
/// * `checksum` - 校验和
/// * `endpoint` - API 端点路径
///
/// # 返回
///
/// * `reqwest::RequestBuilder` - 配置好的请求构建器
pub fn build_request(req: AiServiceRequest) -> RequestBuilder {
    let (client, host) =
        get_client_and_host_post(&req.client, req.url, req.is_pri, CURSOR_API2_HOST);

    client
        .header(
            CONTENT_TYPE,
            if req.is_stream {
                CONTENT_TYPE_CONNECT_PROTO
            } else {
                CONTENT_TYPE_PROTO
            },
        )
        .bearer_auth(req.auth_token)
        .header("connect-accept-encoding", ENCODINGS)
        .header("connect-protocol-version", ONE)
        .header(USER_AGENT, "connect-es/1.6.1")
        .header("x-amzn-trace-id", format!("Root={}", req.trace_id))
        .header("x-client-key", req.client_key)
        .header("x-cursor-checksum", req.checksum)
        .header("x-cursor-client-version", "0.42.5")
        .header("x-cursor-timezone", req.timezone)
        .header(HEADER_NAME_GHOST_MODE, TRUE)
        .header("x-request-id", req.trace_id)
        .header(HOST, host)
        .header(CONNECTION, KEEP_ALIVE)
        .header(TRANSFER_ENCODING, "chunked")
}

#[inline]
fn get_client_and_host<'a>(
    client: &Client,
    url: &'a str,
    is_pri: bool,
    real_host: &'a str,
) -> (RequestBuilder, &'a str) {
    if is_pri && *USE_PRI_REVERSE_PROXY {
        (
            client.get(url).header(PROXY_HOST, real_host),
            PRI_REVERSE_PROXY_HOST.as_str(),
        )
    } else if !is_pri && *USE_PUB_REVERSE_PROXY {
        (
            client.get(url).header(PROXY_HOST, real_host),
            PUB_REVERSE_PROXY_HOST.as_str(),
        )
    } else {
        (client.get(url), real_host)
    }
}

/// 返回预构建的获取 Stripe 账户信息的 Cursor API 客户端
///
/// # 参数
///
/// * `auth_token` - 授权令牌
///
/// # 返回
///
/// * `reqwest::RequestBuilder` - 配置好的请求构建器
pub fn build_profile_request(client: &Client, auth_token: &str, is_pri: bool) -> RequestBuilder {
    let (client, host) = get_client_and_host(
        client,
        cursor_api2_stripe_url(is_pri),
        is_pri,
        CURSOR_API2_HOST,
    );

    client
        .header(HOST, host)
        .header("sec-ch-ua", "\"Not-A.Brand\";v=\"99\", \"Chromium\";v=\"124\"")
        .header(HEADER_NAME_GHOST_MODE, TRUE)
        .header("sec-ch-ua-mobile", "?0")
        .bearer_auth(auth_token)
        .header(
            USER_AGENT,
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Cursor/0.42.5 Chrome/124.0.6367.243 Electron/30.4.0 Safari/537.36",
        )
        .header("sec-ch-ua-platform", "\"Windows\"")
        .header(ACCEPT, VALUE_ACCEPT)
        .header(ORIGIN, "vscode-file://vscode-app")
        .header(SEC_FETCH_SITE, "cross-site")
        .header(SEC_FETCH_MODE, CORS)
        .header(SEC_FETCH_DEST, EMPTY)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(ACCEPT_LANGUAGE, VALUE_LANGUAGE)
        .header(PRIORITY, "u=1, i")
}

/// 返回预构建的获取使用情况的 Cursor API 客户端
///
/// # 参数
///
/// * `user_id` - 用户 ID
/// * `auth_token` - 授权令牌
///
/// # 返回
///
/// * `reqwest::RequestBuilder` - 配置好的请求构建器
pub fn build_usage_request(
    client: &Client,
    user_id: &str,
    auth_token: &str,
    is_pri: bool,
) -> RequestBuilder {
    let (client, host) =
        get_client_and_host(client, cursor_usage_api_url(is_pri), is_pri, CURSOR_HOST);

    client
        .header(HOST, host)
        .header(USER_AGENT, UA_WIN)
        .header(ACCEPT, VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, VALUE_LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(REFERER, CURSOR_SETTINGS_URL)
        .header(DNT, ONE)
        .header(SEC_GPC, ONE)
        .header(SEC_FETCH_DEST, EMPTY)
        .header(SEC_FETCH_MODE, CORS)
        .header(SEC_FETCH_SITE, SAME_ORIGIN)
        .header(CONNECTION, KEEP_ALIVE)
        .header(PRAGMA, NO_CACHE)
        .header(CACHE_CONTROL, NO_CACHE)
        .header(TE, TRAILERS)
        .header(PRIORITY, U_EQ_4)
        .header(
            COOKIE,
            format!("WorkosCursorSessionToken={user_id}%3A%3A{auth_token}"),
        )
        .query(&[("user", user_id)])
}

/// 返回预构建的获取用户信息的 Cursor API 客户端
///
/// # 参数
///
/// * `user_id` - 用户 ID
/// * `auth_token` - 授权令牌
///
/// # 返回
///
/// * `reqwest::RequestBuilder` - 配置好的请求构建器
pub fn build_userinfo_request(
    client: &Client,
    user_id: &str,
    auth_token: &str,
    is_pri: bool,
) -> RequestBuilder {
    let (client, host) =
        get_client_and_host(client, cursor_user_api_url(is_pri), is_pri, CURSOR_HOST);

    client
        .header(HOST, host)
        .header(USER_AGENT, UA_WIN)
        .header(ACCEPT, VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, VALUE_LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(REFERER, CURSOR_SETTINGS_URL)
        .header(DNT, ONE)
        .header(SEC_GPC, ONE)
        .header(SEC_FETCH_DEST, EMPTY)
        .header(SEC_FETCH_MODE, CORS)
        .header(SEC_FETCH_SITE, SAME_ORIGIN)
        .header(CONNECTION, KEEP_ALIVE)
        .header(PRAGMA, NO_CACHE)
        .header(CACHE_CONTROL, NO_CACHE)
        .header(TE, TRAILERS)
        .header(PRIORITY, U_EQ_4)
        .header(
            COOKIE,
            format!("WorkosCursorSessionToken={user_id}%3A%3A{auth_token}"),
        )
}

pub fn build_token_upgrade_request(
    client: &Client,
    uuid: &str,
    challenge: &str,
    user_id: &str,
    auth_token: &str,
    is_pri: bool,
) -> RequestBuilder {
    let (client, host) = get_client_and_host_post(
        client,
        cursor_token_upgrade_url(is_pri),
        is_pri,
        CURSOR_HOST,
    );

    let body = format!("{{\"uuid\":\"{uuid}\",\"challenge\":\"{challenge}\"}}");

    client
        .header(HOST, host)
        .header(USER_AGENT, UA_WIN)
        .header(ACCEPT, VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, VALUE_LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(
            REFERER,
            format!(
                "https://cursor.com/loginDeepControl?challenge={challenge}&uuid={uuid}&mode=login"
            ),
        )
        .header(CONTENT_TYPE, "application/json")
        .header(CONTENT_LENGTH, body.len())
        .header(DNT, ONE)
        .header(SEC_GPC, ONE)
        .header(SEC_FETCH_DEST, EMPTY)
        .header(SEC_FETCH_MODE, CORS)
        .header(SEC_FETCH_SITE, SAME_ORIGIN)
        .header(CONNECTION, KEEP_ALIVE)
        .header(PRAGMA, NO_CACHE)
        .header(CACHE_CONTROL, NO_CACHE)
        .header(TE, TRAILERS)
        .header(PRIORITY, U_EQ_0)
        .header(
            COOKIE,
            format!("WorkosCursorSessionToken={user_id}%3A%3A{auth_token}"),
        )
        .body(body)
}

pub fn build_token_poll_request(
    client: &Client,
    uuid: &str,
    verifier: &str,
    is_pri: bool,
) -> RequestBuilder {
    let (client, host) = get_client_and_host(
        client,
        cursor_token_poll_url(is_pri),
        is_pri,
        CURSOR_API2_HOST,
    );
    client
        .header(HOST, host)
        .header(ACCEPT_ENCODING, "gzip, deflate")
        .header(ACCEPT_LANGUAGE, "en-US") 
        .header(USER_AGENT, "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Cursor/0.48.2 Chrome/132.0.6834.210 Electron/34.3.4 Safari/537.36")
        .header(ORIGIN, "vscode-file://vscode-app")
        .header(HEADER_NAME_GHOST_MODE, TRUE)
        .header(ACCEPT, "*/*")
        .query(&[("uuid", uuid), ("verifier", verifier)])
}
