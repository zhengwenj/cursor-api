use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};

use crate::{
    app::{
        constant::{API_KEY, AUTHORIZATION_BEARER_PREFIX},
        lazy::AUTH_TOKEN,
        model::{AppConfig, AppState, TokenKey},
    },
    common::{model::error::ChatError, utils::tokeninfo_to_token},
    core::config::{KeyConfig, parse_dynamic_token},
};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{Request, StatusCode, header::AUTHORIZATION},
    middleware::Next,
    response::{IntoResponse, Response},
};

#[inline]
pub fn auth(headers: &http::HeaderMap) -> Option<&str> {
    if let Some(val) = headers.get(API_KEY)
        && let Ok(s) = val.to_str()
    {
        return Some(s);
    }
    if let Some(val) = headers.get(AUTHORIZATION)
        && let Ok(s) = val.to_str()
    {
        return s.strip_prefix(AUTHORIZATION_BEARER_PREFIX);
    }
    None
}

// 管理员认证中间件函数
pub async fn admin_auth_middleware(request: Request<Body>, next: Next) -> Response {
    if let Some(token) = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        && token == *AUTH_TOKEN
    {
        return next.run(request).await;
    };

    (
        StatusCode::UNAUTHORIZED,
        Json(ChatError::Unauthorized.to_generic()),
    )
        .into_response()
}

pub async fn v1_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let auth_token = match auth(request.headers()) {
        Some(v) => v,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ChatError::Unauthorized.to_generic()),
            )
                .into_response();
        }
    };

    let mut current_config = KeyConfig::new_with_global();

    // 获取token信息
    let v = {
        // 管理员Token
        if let Some(part) = auth_token.strip_prefix(&**AUTH_TOKEN) {
            let token_manager = state.token_manager.read().await;

            let token_info = if part.is_empty() {
                let token_infos: Vec<_> = token_manager
                    .tokens()
                    .iter()
                    .flatten()
                    .filter(|t| t.is_enabled())
                    .collect();

                if token_infos.is_empty() {
                    return (
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(ChatError::NoTokens.to_generic()),
                    )
                        .into_response();
                }

                static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);

                let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::AcqRel) % token_infos.len();
                token_infos[index]
            } else if let Some(alias) = part.strip_prefix('-') {
                if !token_manager.alias_map().contains_key(alias) {
                    return StatusCode::NOT_FOUND.into_response();
                }
                if let Some(token_info) = token_manager.get_by_alias(alias) {
                    token_info
                } else {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(ChatError::Unauthorized.to_generic()),
                    )
                        .into_response();
                }
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                )
                    .into_response();
            };
            (token_info.bundle.clone_without_user(), true)
        }
        // 共享Token
        else if AppConfig::is_share() && AppConfig::share_token_eq(auth_token) {
            let token_manager = state.token_manager.read().await;
            let token_infos: Vec<_> = token_manager
                .tokens()
                .iter()
                .flatten()
                .filter(|t| t.is_enabled())
                .collect();

            if token_infos.is_empty() {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ChatError::NoTokens.to_generic()),
                )
                    .into_response();
            }

            static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);

            let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::AcqRel) % token_infos.len();
            let token_info = token_infos[index];
            (token_info.bundle.clone_without_user(), true)
        }
        // 普通用户Token
        else if let Some(key) = TokenKey::from_string(auth_token) {
            let log_manager = state.log_manager_lock().await;
            if let Some(bundle) = log_manager.tokens().get(&key) {
                (bundle.clone_without_user(), false)
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                )
                    .into_response();
            }
        }
        // 动态密钥
        else if AppConfig::get_dynamic_key() {
            if let Some(ext_token) = parse_dynamic_token(auth_token)
                .and_then(|key_config| {
                    key_config.copy_without_auth_token(&mut current_config);
                    key_config.token_info
                })
                .and_then(tokeninfo_to_token)
            {
                (ext_token, false)
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ChatError::Unauthorized.to_generic()),
            )
                .into_response();
        }
    };

    request.extensions_mut().insert(v);
    request.extensions_mut().insert(current_config);

    next.run(request).await
}

pub async fn cpp_auth_middleware(
    State(state): State<Arc<AppState>>,
    mut request: Request<Body>,
    next: Next,
) -> Response {
    let auth_token = match auth(request.headers()) {
        Some(v) => v,
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ChatError::Unauthorized.to_generic()),
            )
                .into_response();
        }
    };

    // 获取token信息
    let v = {
        // 管理员Token
        if let Some(part) = auth_token.strip_prefix(&**AUTH_TOKEN) {
            let token_manager = state.token_manager.read().await;

            let token_info = if part.is_empty() {
                let token_infos: Vec<_> = token_manager
                    .tokens()
                    .iter()
                    .flatten()
                    .filter(|t| t.is_enabled())
                    .collect();

                if token_infos.is_empty() {
                    return (
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(ChatError::NoTokens.to_generic()),
                    )
                        .into_response();
                }

                static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);

                let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::AcqRel) % token_infos.len();
                token_infos[index]
            } else if let Some(alias) = part.strip_prefix('-') {
                if !token_manager.alias_map().contains_key(alias) {
                    return StatusCode::NOT_FOUND.into_response();
                }
                if let Some(token_info) = token_manager.get_by_alias(alias) {
                    token_info
                } else {
                    return (
                        StatusCode::UNAUTHORIZED,
                        Json(ChatError::Unauthorized.to_generic()),
                    )
                        .into_response();
                }
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                )
                    .into_response();
            };
            (token_info.bundle.clone_without_user(), true)
        }
        // 共享Token
        else if AppConfig::is_share() && AppConfig::share_token_eq(auth_token) {
            let token_manager = state.token_manager.read().await;
            let token_infos: Vec<_> = token_manager
                .tokens()
                .iter()
                .flatten()
                .filter(|t| t.is_enabled())
                .collect();

            if token_infos.is_empty() {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ChatError::NoTokens.to_generic()),
                )
                    .into_response();
            }

            static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);

            let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::AcqRel) % token_infos.len();
            let token_info = token_infos[index];
            (token_info.bundle.clone_without_user(), true)
        }
        // 普通用户Token
        else if let Some(key) = TokenKey::from_string(auth_token) {
            let log_manager = state.log_manager_lock().await;
            if let Some(bundle) = log_manager.tokens().get(&key) {
                (bundle.clone_without_user(), false)
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                )
                    .into_response();
            }
        }
        // 动态密钥
        else if AppConfig::get_dynamic_key() {
            if let Some(ext_token) = parse_dynamic_token(auth_token)
                .and_then(|key_config| key_config.token_info)
                .and_then(tokeninfo_to_token)
            {
                (ext_token, false)
            } else {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                )
                    .into_response();
            }
        } else {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ChatError::Unauthorized.to_generic()),
            )
                .into_response();
        }
    };

    request.extensions_mut().insert(v);

    next.run(request).await
}
