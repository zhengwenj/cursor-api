use crate::{
    app::{
        constant::{
            AUTHORIZATION_BEARER_PREFIX, CONTENT_TYPE_TEXT_HTML_WITH_UTF8,
            CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8, HEADER_NAME_AUTHORIZATION, HEADER_NAME_CONTENT_TYPE,
            ROUTE_TOKENINFO_PATH,
        },
        model::{AppConfig, AppState, PageContent, TokenUpdateRequest},
        lazy::{AUTH_TOKEN, TOKEN_FILE, TOKEN_LIST_FILE},
    },
    common::{
        models::{ApiStatus, NormalResponseNoData},
        utils::{generate_checksum, generate_hash, tokens::load_tokens},
    },
};
use axum::{
    extract::State,
    http::HeaderMap,
    response::{IntoResponse, Response},
    Json,
};
use reqwest::StatusCode;
use serde::Serialize;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Serialize)]
pub struct ChecksumResponse {
    pub checksum: String,
}

pub async fn handle_get_checksum() -> Json<ChecksumResponse> {
    let checksum = generate_checksum(&generate_hash(), Some(&generate_hash()));
    Json(ChecksumResponse { checksum })
}

// 更新 TokenInfo 处理
pub async fn handle_update_tokeninfo(
    State(state): State<Arc<Mutex<AppState>>>,
) -> Json<NormalResponseNoData> {
    // 重新加载 tokens
    let token_infos = load_tokens();

    // 更新应用状态
    {
        let mut state = state.lock().await;
        state.token_infos = token_infos;
    }

    Json(NormalResponseNoData {
        status: ApiStatus::Success,
        message: Some("Token list has been reloaded".to_string()),
    })
}

// 获取 TokenInfo 处理
pub async fn handle_get_tokeninfo(
    State(_state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
) -> Result<Json<TokenInfoResponse>, StatusCode> {
    let auth_token = AUTH_TOKEN.as_str();
    let token_file = TOKEN_FILE.as_str();
    let token_list_file = TOKEN_LIST_FILE.as_str();

    // 验证 AUTH_TOKEN
    let auth_header = headers
        .get(HEADER_NAME_AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if auth_header != auth_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 读取文件内容
    let tokens = std::fs::read_to_string(&token_file).unwrap_or_else(|_| String::new());
    let token_list = std::fs::read_to_string(&token_list_file).unwrap_or_else(|_| String::new());

    Ok(Json(TokenInfoResponse {
        status: ApiStatus::Success,
        token_file: token_file.to_string(),
        token_list_file: token_list_file.to_string(),
        tokens: Some(tokens.clone()),
        tokens_count: Some(tokens.len()),
        token_list: Some(token_list),
        message: None,
    }))
}

#[derive(Serialize)]
pub struct TokenInfoResponse {
    pub status: ApiStatus,
    pub token_file: String,
    pub token_list_file: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokens_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_list: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

pub async fn handle_update_tokeninfo_post(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    Json(request): Json<TokenUpdateRequest>,
) -> Result<Json<TokenInfoResponse>, StatusCode> {
    let auth_token = AUTH_TOKEN.as_str();
    let token_file = TOKEN_FILE.as_str();
    let token_list_file = TOKEN_LIST_FILE.as_str();

    // 验证 AUTH_TOKEN
    let auth_header = headers
        .get(HEADER_NAME_AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if auth_header != auth_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // 写入 .token 文件
    std::fs::write(&token_file, &request.tokens).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // 如果提供了 token_list，则写入
    if let Some(token_list) = request.token_list {
        std::fs::write(&token_list_file, token_list)
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // 重新加载 tokens
    let token_infos = load_tokens();
    let token_infos_len = token_infos.len();

    // 更新应用状态
    {
        let mut state = state.lock().await;
        state.token_infos = token_infos;
    }

    Ok(Json(TokenInfoResponse {
        status: ApiStatus::Success,
        token_file: token_file.to_string(),
        token_list_file: token_list_file.to_string(),
        tokens: None,
        tokens_count: Some(token_infos_len),
        token_list: None,
        message: Some("Token files have been updated and reloaded".to_string()),
    }))
}

pub async fn handle_tokeninfo_page() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_TOKENINFO_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(include_str!("../../../static/tokeninfo.min.html").to_string())
            .unwrap(),
        PageContent::Text(content) => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8)
            .body(content.clone())
            .unwrap(),
        PageContent::Html(content) => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(content.clone())
            .unwrap(),
    }
}
