use ::reqwest::{
    Client, RequestBuilder,
    header::{
        ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, AUTHORIZATION, CACHE_CONTROL, CONNECTION,
        CONTENT_LENGTH, CONTENT_TYPE, COOKIE, DNT, HOST, ORIGIN, PRAGMA, REFERER, TE, USER_AGENT,
    },
};

use crate::{
    app::{
        constant::{
            AMZN_TRACE_ID, AUTHORIZATION_BEARER_PREFIX, CLIENT_KEY, CONNECT_ACCEPT_ENCODING,
            CONNECT_CONTENT_ENCODING, CONNECT_ES, CONNECT_PROTO, CONNECT_PROTOCOL_VERSION, CORS,
            CURSOR_API2_HOST, CURSOR_CHECKSUM, CURSOR_CLIENT_VERSION, CURSOR_CONFIG_VERSION,
            CURSOR_HOST, CURSOR_REFERER_URL, CURSOR_STREAMING, CURSOR_TIMEZONE, EMPTY, ENCODING,
            ENCODINGS, FALSE, FS_CLIENT_KEY, GHOST_MODE, HEADER_VALUE_ACCEPT, JSON, KEEP_ALIVE,
            LANGUAGE, NEW_ONBOARDING_COMPLETED, NO_CACHE, NONE, ONE, PRIORITY, PROTO, PROXY_HOST,
            REQUEST_ID, SAME_ORIGIN, SEC_FETCH_DEST, SEC_FETCH_MODE, SEC_FETCH_SITE, SEC_GPC,
            SESSION_ID, TRAILERS, TRUE, U_EQ_0, UA, VSCODE_ORIGIN, cursor_client_version,
            header_value_ua_cursor_latest,
        },
        lazy::{
            PRI_REVERSE_PROXY_HOST, PUB_REVERSE_PROXY_HOST, USE_PRI_REVERSE_PROXY,
            USE_PUB_REVERSE_PROXY, sessions_url, stripe_url, token_poll_url, token_refresh_url,
            token_upgrade_url, usage_api_url, user_api_url,
        },
        model::ExtToken,
    },
    common::utils::StringBuilder,
};

trait RequestBuilderExt: Sized {
    fn opt_header<K, V>(self, key: K, value: Option<V>) -> Self
    where
        http::HeaderName: TryFrom<K>,
        <http::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::HeaderValue: TryFrom<V>,
        <http::HeaderValue as TryFrom<V>>::Error: Into<http::Error>;

    fn opt_header_map<K, I, V, F: FnOnce(I) -> V>(self, key: K, value: Option<I>, f: F) -> Self
    where
        http::HeaderName: TryFrom<K>,
        <http::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::HeaderValue: TryFrom<V>,
        <http::HeaderValue as TryFrom<V>>::Error: Into<http::Error>;

    fn header_if<K, V>(self, key: K, value: V, condition: bool) -> Self
    where
        http::HeaderName: TryFrom<K>,
        <http::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::HeaderValue: TryFrom<V>,
        <http::HeaderValue as TryFrom<V>>::Error: Into<http::Error>;
}

impl RequestBuilderExt for RequestBuilder {
    #[inline]
    fn opt_header<K, V>(self, key: K, value: Option<V>) -> Self
    where
        http::HeaderName: TryFrom<K>,
        <http::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::HeaderValue: TryFrom<V>,
        <http::HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        if let Some(value) = value {
            self.header(key, value)
        } else {
            self
        }
    }

    #[inline]
    fn opt_header_map<K, I, V, F: FnOnce(I) -> V>(self, key: K, value: Option<I>, f: F) -> Self
    where
        http::HeaderName: TryFrom<K>,
        <http::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::HeaderValue: TryFrom<V>,
        <http::HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        if let Some(value) = value {
            self.header(key, f(value))
        } else {
            self
        }
    }

    #[inline]
    fn header_if<K, V>(self, key: K, value: V, condition: bool) -> Self
    where
        http::HeaderName: TryFrom<K>,
        <http::HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        http::HeaderValue: TryFrom<V>,
        <http::HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        if condition {
            self.header(key, value)
        } else {
            self
        }
    }
}

#[inline]
fn get_client_and_host<'a>(
    client: &Client,
    method: http::Method,
    url: &'a str,
    is_pri: bool,
    real_host: &'a str,
) -> (RequestBuilder, &'a str) {
    if is_pri && *USE_PRI_REVERSE_PROXY {
        (
            client.request(method, url).header(PROXY_HOST, real_host),
            &PRI_REVERSE_PROXY_HOST,
        )
    } else if !is_pri && *USE_PUB_REVERSE_PROXY {
        (
            client.request(method, url).header(PROXY_HOST, real_host),
            &PUB_REVERSE_PROXY_HOST,
        )
    } else {
        (client.request(method, url), real_host)
    }
}

pub(crate) struct AiServiceRequest<'a> {
    pub(crate) ext_token: ExtToken,
    pub(crate) fs_client_key: Option<http::HeaderValue>,
    pub(crate) url: &'a str,
    pub(crate) is_stream: bool,
    pub(crate) trace_id: Option<[u8; 36]>,
    pub(crate) is_pri: bool,
    pub(crate) cookie: Option<http::HeaderValue>,
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
pub fn build_client_request(req: AiServiceRequest) -> RequestBuilder {
    let (builder, host) = get_client_and_host(
        &req.ext_token.get_client(),
        http::Method::POST,
        req.url,
        req.is_pri,
        CURSOR_API2_HOST,
    );

    let mut buf = [0u8; 137];

    builder
        .version(http::Version::HTTP_2)
        .header(HOST, host)
        .header_if(ACCEPT_ENCODING, ENCODING, !req.is_stream)
        .header(AUTHORIZATION, {
            let mut v = __unwrap!(http::HeaderValue::from_str(
                &StringBuilder::with_capacity(2)
                    .append(AUTHORIZATION_BEARER_PREFIX)
                    .append(req.ext_token.primary_token.as_str())
                    .build()
            ));
            v.set_sensitive(true);
            v
        })
        .header_if(CONNECT_ACCEPT_ENCODING, ENCODING, req.is_stream)
        .header_if(CONNECT_CONTENT_ENCODING, ENCODING, req.is_stream)
        .header(CONNECT_PROTOCOL_VERSION, ONE)
        .header(
            CONTENT_TYPE,
            if req.is_stream { CONNECT_PROTO } else { PROTO },
        )
        .header(COOKIE, req.cookie.unwrap_or(NONE))
        .header(USER_AGENT, CONNECT_ES)
        .opt_header_map(AMZN_TRACE_ID, req.trace_id, |v| {
            const PREFIX: &[u8; 5] = b"Root=";
            unsafe {
                ::core::ptr::copy_nonoverlapping(PREFIX.as_ptr(), buf.as_mut_ptr(), 5);
                ::core::ptr::copy_nonoverlapping(v.as_ptr(), buf.as_mut_ptr().add(5), 36);
            }
            __unwrap!(http::HeaderValue::from_bytes(buf.get_unchecked(..41)))
        })
        .header(
            CLIENT_KEY,
            __unwrap!(http::HeaderValue::from_bytes({
                req.ext_token
                    .client_key
                    .to_str(&mut *(buf.as_mut_ptr() as *mut [u8; 64]));
                buf.get_unchecked(..64)
            })),
        )
        .header(
            CURSOR_CHECKSUM,
            __unwrap!(http::HeaderValue::from_bytes({
                req.ext_token.checksum.to_str(&mut buf);
                &buf
            })),
        )
        .header(CURSOR_CLIENT_VERSION, cursor_client_version())
        .opt_header_map(CURSOR_CONFIG_VERSION, req.ext_token.config_version, |v| {
            v.hyphenated()
                .encode_lower(unsafe { &mut *(buf.as_mut_ptr() as *mut [u8; 36]) });
            __unwrap!(http::HeaderValue::from_bytes(buf.get_unchecked(..36)))
        })
        .header(CURSOR_STREAMING, TRUE)
        .header(CURSOR_TIMEZONE, req.ext_token.timezone_name())
        .opt_header(FS_CLIENT_KEY, req.fs_client_key)
        .header(GHOST_MODE, TRUE)
        .header(NEW_ONBOARDING_COMPLETED, FALSE)
        .opt_header_map(REQUEST_ID, req.trace_id, |v| {
            __unwrap!(http::HeaderValue::from_bytes(&v))
        })
        .header(SESSION_ID, {
            req.ext_token
                .session_id
                .hyphenated()
                .encode_lower(unsafe { &mut *(buf.as_mut_ptr() as *mut [u8; 36]) });
            __unwrap!(http::HeaderValue::from_bytes(buf.get_unchecked(..36)))
        })
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
    let (builder, host) = get_client_and_host(
        client,
        http::Method::GET,
        stripe_url(is_pri),
        is_pri,
        CURSOR_API2_HOST,
    );

    builder
        .version(http::Version::HTTP_2)
        .header(HOST, host)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(AUTHORIZATION, {
            let mut v = __unwrap!(http::HeaderValue::from_str(
                &StringBuilder::with_capacity(2)
                    .append(AUTHORIZATION_BEARER_PREFIX)
                    .append(auth_token)
                    .build()
            ));
            v.set_sensitive(true);
            v
        })
        .header(GHOST_MODE, TRUE)
        .header(NEW_ONBOARDING_COMPLETED, FALSE)
        .header(USER_AGENT, header_value_ua_cursor_latest())
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .header(ORIGIN, VSCODE_ORIGIN)
    // .header(SEC_CH_UA, NOT_A_BRAND)
    // .header(SEC_CH_UA_MOBILE, MOBILE_NO)
    // .header(SEC_CH_UA_PLATFORM, WINDOWS)
    // .header(SEC_FETCH_SITE, CROSS_SITE)
    // .header(SEC_FETCH_MODE, CORS)
    // .header(SEC_FETCH_DEST, EMPTY)
    // .header(SEC_GPC, ONE)
    // .header(CONNECTION, KEEP_ALIVE)
    // .header(PRAGMA, NO_CACHE)
    // .header(CACHE_CONTROL, NO_CACHE)
    // .header(TE, TRAILERS)
    // .header(PRIORITY, U_EQ_0)
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
    let (builder, host) = get_client_and_host(
        client,
        http::Method::GET,
        usage_api_url(is_pri),
        is_pri,
        CURSOR_HOST,
    );

    builder
        .version(http::Version::HTTP_11)
        .header(HOST, host)
        .header(USER_AGENT, UA)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(REFERER, CURSOR_REFERER_URL)
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
            format_workos_cursor_session_token(user_id, auth_token),
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
    let (builder, host) = get_client_and_host(
        client,
        http::Method::GET,
        user_api_url(is_pri),
        is_pri,
        CURSOR_HOST,
    );

    builder
        .version(http::Version::HTTP_11)
        .header(HOST, host)
        .header(USER_AGENT, UA)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(REFERER, CURSOR_REFERER_URL)
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
            format_workos_cursor_session_token(user_id, auth_token),
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
    let (builder, host) = get_client_and_host(
        client,
        http::Method::POST,
        token_upgrade_url(is_pri),
        is_pri,
        CURSOR_HOST,
    );

    crate::define_typed_constants! {
        &'static str => {
            UUID_PREFIX = "{\"uuid\":\"",
            CHALLENGE_PREFIX = "\",\"challenge\":\"",
            SUFFIX = "\"}",

            REFERER_PREFIX = "https://cursor.com/loginDeepControl?challenge=",
            REFERER_MIDDLE = "&uuid=",
            REFERER_SUFFIX = "&mode=login",
        }
        usize => {
            UUID_LEN = 36,
            CHALLENGE_LEN = 43,

            BODY_CAPACITY = UUID_PREFIX.len() + UUID_LEN + CHALLENGE_PREFIX.len() + CHALLENGE_LEN + SUFFIX.len(),
            REFERER_CAPACITY = REFERER_PREFIX.len() + CHALLENGE_LEN + REFERER_MIDDLE.len() + UUID_LEN + REFERER_SUFFIX.len(),
        }
    }

    // 使用常量预分配空间 - body
    let mut body = String::with_capacity(BODY_CAPACITY);
    body.push_str(UUID_PREFIX);
    body.push_str(uuid);
    body.push_str(CHALLENGE_PREFIX);
    body.push_str(challenge);
    body.push_str(SUFFIX);

    // 使用常量预分配空间 - referer
    let mut referer = String::with_capacity(REFERER_CAPACITY);
    referer.push_str(REFERER_PREFIX);
    referer.push_str(challenge);
    referer.push_str(REFERER_MIDDLE);
    referer.push_str(uuid);
    referer.push_str(REFERER_SUFFIX);

    builder
        .version(http::Version::HTTP_11)
        .header(HOST, host)
        .header(USER_AGENT, UA)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(REFERER, referer)
        .header(CONTENT_TYPE, JSON)
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
            format_workos_cursor_session_token(user_id, auth_token),
        )
        .body(body)
}

pub fn build_token_poll_request(
    client: &Client,
    uuid: &str,
    verifier: &str,
    is_pri: bool,
) -> RequestBuilder {
    let (builder, host) = get_client_and_host(
        client,
        http::Method::GET,
        token_poll_url(is_pri),
        is_pri,
        CURSOR_API2_HOST,
    );

    builder
        .version(http::Version::HTTP_11)
        .header(HOST, host)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(USER_AGENT, header_value_ua_cursor_latest())
        .header(ORIGIN, VSCODE_ORIGIN)
        .header(GHOST_MODE, TRUE)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .query(&[("uuid", uuid), ("verifier", verifier)])
}

pub fn build_token_refresh_request(client: &Client, is_pri: bool, body: Vec<u8>) -> RequestBuilder {
    let (builder, host) = get_client_and_host(
        client,
        http::Method::POST,
        token_refresh_url(is_pri),
        is_pri,
        CURSOR_API2_HOST,
    );

    builder
        .header(HOST, host)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(CONTENT_TYPE, JSON)
        .header(CONTENT_LENGTH, body.len())
        .header(USER_AGENT, header_value_ua_cursor_latest())
        .header(ORIGIN, VSCODE_ORIGIN)
        .header(GHOST_MODE, TRUE)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .body(body)
}

pub fn build_proto_web_request(
    client: &Client,
    user_id: &str,
    auth_token: &str,
    url: fn(bool) -> &'static str,
    is_pri: bool,
    body: bytes::Bytes,
) -> RequestBuilder {
    let (builder, host) =
        get_client_and_host(client, http::Method::POST, url(is_pri), is_pri, CURSOR_HOST);

    builder
        .version(http::Version::HTTP_11)
        .header(HOST, host)
        .header(USER_AGENT, UA)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(CONTENT_TYPE, JSON)
        .header(CONTENT_LENGTH, body.len())
        .header(REFERER, CURSOR_REFERER_URL)
        .header(DNT, ONE)
        .header(SEC_GPC, ONE)
        .header(SEC_FETCH_DEST, EMPTY)
        .header(SEC_FETCH_MODE, CORS)
        .header(SEC_FETCH_SITE, SAME_ORIGIN)
        .header(CONNECTION, KEEP_ALIVE)
        .header(PRIORITY, U_EQ_0)
        .header(PRAGMA, NO_CACHE)
        .header(CACHE_CONTROL, NO_CACHE)
        .header(TE, TRAILERS)
        .header(
            COOKIE,
            format_workos_cursor_session_token(user_id, auth_token),
        )
        .body(body)
}

pub fn build_sessions_request(
    client: &Client,
    user_id: &str,
    auth_token: &str,
    is_pri: bool,
) -> RequestBuilder {
    let (builder, host) = get_client_and_host(
        client,
        http::Method::GET,
        sessions_url(is_pri),
        is_pri,
        CURSOR_HOST,
    );

    builder
        .version(http::Version::HTTP_11)
        .header(HOST, host)
        .header(USER_AGENT, UA)
        .header(ACCEPT, HEADER_VALUE_ACCEPT)
        .header(ACCEPT_LANGUAGE, LANGUAGE)
        .header(ACCEPT_ENCODING, ENCODINGS)
        .header(REFERER, CURSOR_REFERER_URL)
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
            format_workos_cursor_session_token(user_id, auth_token),
        )
}

#[inline]
fn format_workos_cursor_session_token(user_id: &str, auth_token: &str) -> String {
    crate::define_typed_constants! {
        &'static str => {
            TOKEN_PREFIX = "WorkosCursorSessionToken=",
            SEPARATOR = "%3A%3A",
        }
        usize => {
            USER_ID_LEN = 31,
            PREFIX_AND_USER_ID_AND_SEPARATOR = TOKEN_PREFIX.len() + USER_ID_LEN + SEPARATOR.len(),
        }
    }

    // 预分配足够的空间: TOKEN_PREFIX + user_id + SEPARATOR + auth_token
    let mut result = String::with_capacity(PREFIX_AND_USER_ID_AND_SEPARATOR + auth_token.len());

    result.push_str(TOKEN_PREFIX);
    result.push_str(user_id);
    result.push_str(SEPARATOR);
    result.push_str(auth_token);

    result
}
