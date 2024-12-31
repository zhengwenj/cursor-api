use super::{
    constant::*,
    models::{AppState, TokenInfo, TokenUpdateRequest},
    statics::*,
    utils::{generate_checksum, generate_hash, i32_to_u32},
};
use crate::{chat::aiserver::v1::GetUserInfoResponse, common::models::{ApiStatus, NormalResponseNoData}};
use axum::http::HeaderMap;
use axum::{
    extract::{Query, State},
    Json,
};
use image::EncodableLayout;
use prost::Message;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

// 规范化文件内容并写入
fn normalize_and_write(content: &str, file_path: &str) -> String {
    let normalized = content.replace("\r\n", "\n");
    if normalized != content {
        if let Err(e) = std::fs::write(file_path, &normalized) {
            eprintln!("警告: 无法更新规范化的文件: {}", e);
        }
    }
    normalized
}

// 解析token和别名
fn parse_token_alias(token_part: &str, line: &str) -> Option<(String, Option<String>)> {
    match token_part.split("::").collect::<Vec<_>>() {
        parts if parts.len() == 1 => Some((parts[0].to_string(), None)),
        parts if parts.len() == 2 => Some((parts[1].to_string(), Some(parts[0].to_string()))),
        _ => {
            eprintln!("警告: 忽略无效的行: {}", line);
            None
        }
    }
}

// Token 加载函数
pub fn load_tokens() -> Vec<TokenInfo> {
    let token_file = get_token_file();
    let token_list_file = get_token_list_file();

    // 确保文件存在
    for file in [&token_file, &token_list_file] {
        if !std::path::Path::new(file).exists() {
            if let Err(e) = std::fs::write(file, EMPTY_STRING) {
                eprintln!("警告: 无法创建文件 '{}': {}", file, e);
            }
        }
    }

    // 读取和规范化 token 文件
    let token_entries = match std::fs::read_to_string(&token_file) {
        Ok(content) => {
            let normalized = normalize_and_write(&content, &token_file);
            normalized
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        return None;
                    }
                    parse_token_alias(line, line)
                })
                .collect::<Vec<_>>()
        }
        Err(e) => {
            eprintln!("警告: 无法读取token文件 '{}': {}", token_file, e);
            Vec::new()
        }
    };

    // 读取和规范化 token-list 文件
    let mut token_map: std::collections::HashMap<String, (String, Option<String>)> =
        match std::fs::read_to_string(&token_list_file) {
            Ok(content) => {
                let normalized = normalize_and_write(&content, &token_list_file);
                normalized
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            return None;
                        }

                        let parts: Vec<&str> = line.split(',').collect();
                        match parts[..] {
                            [token_part, checksum] => {
                                let (token, alias) = parse_token_alias(token_part, line)?;
                                Some((token, (checksum.to_string(), alias)))
                            }
                            _ => {
                                eprintln!("警告: 忽略无效的token-list行: {}", line);
                                None
                            }
                        }
                    })
                    .collect()
            }
            Err(e) => {
                eprintln!("警告: 无法读取token-list文件: {}", e);
                std::collections::HashMap::new()
            }
        };

    // 更新或添加新token
    for (token, alias) in token_entries {
        if let Some((_, existing_alias)) = token_map.get(&token) {
            // 只在alias不同时更新已存在的token
            if alias != *existing_alias {
                if let Some((checksum, _)) = token_map.get(&token) {
                    token_map.insert(token.clone(), (checksum.clone(), alias));
                }
            }
        } else {
            // 为新token生成checksum
            let checksum = generate_checksum(&generate_hash(), Some(&generate_hash()));
            token_map.insert(token, (checksum, alias));
        }
    }

    // 更新 token-list 文件
    let token_list_content = token_map
        .iter()
        .map(|(token, (checksum, alias))| {
            if let Some(alias) = alias {
                format!("{}::{},{}", alias, token, checksum)
            } else {
                format!("{},{}", token, checksum)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if let Err(e) = std::fs::write(&token_list_file, token_list_content) {
        eprintln!("警告: 无法更新token-list文件: {}", e);
    }

    // 转换为 TokenInfo vector
    token_map
        .into_iter()
        .map(|(token, (checksum, alias))| TokenInfo {
            token,
            checksum,
            alias,
            usage: None,
        })
        .collect()
}

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
    let auth_token = get_auth_token();
    let token_file = get_token_file();
    let token_list_file = get_token_list_file();

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
        token_file: token_file.clone(),
        token_list_file: token_list_file.clone(),
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
    let auth_token = get_auth_token();
    let token_file = get_token_file();
    let token_list_file = get_token_list_file();

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
        token_file: token_file.clone(),
        token_list_file: token_list_file.clone(),
        tokens: None,
        tokens_count: Some(token_infos_len),
        token_list: None,
        message: Some("Token files have been updated and reloaded".to_string()),
    }))
}

#[derive(Deserialize)]
pub struct GetUserInfoQuery {
    alias: String,
}

pub async fn get_user_info(
    State(state): State<Arc<Mutex<AppState>>>,
    Query(query): Query<GetUserInfoQuery>,
) -> Json<GetUserInfo> {
    let token_infos = &state.lock().await.token_infos;
    let token_info = token_infos
        .iter()
        .find(|token_info| token_info.alias == Some(query.alias.clone()));

    let (auth_token, checksum) = match token_info {
        Some(token_info) => (token_info.token.clone(), token_info.checksum.clone()),
        None => return Json(GetUserInfo::Error("No data".to_string())),
    };

    match get_user_usage(&auth_token, &checksum).await {
        Some(usage) => Json(GetUserInfo::Usage(usage)),
        None => Json(GetUserInfo::Error("No data".to_string())),
    }
}

pub async fn get_user_usage(auth_token: &str, checksum: &str) -> Option<UserUsageInfo> {
    // 构建请求客户端
    let client = super::client::build_client(auth_token, checksum, CURSOR_API2_GET_USER_INFO);
    let response = client
        .body(Vec::new())
        .send()
        .await
        .ok()?
        .bytes()
        .await
        .ok()?;
    let user_info = GetUserInfoResponse::decode(response.as_bytes()).ok()?;

    user_info.usage.map(|user_usage| UserUsageInfo {
        fast_requests: i32_to_u32(user_usage.gpt4_requests),
        max_fast_requests: i32_to_u32(user_usage.gpt4_max_requests),
    })
}

#[derive(Serialize)]
pub enum GetUserInfo {
    #[serde(rename = "usage")]
    Usage(UserUsageInfo),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(Serialize, Clone)]
pub struct UserUsageInfo {
    pub fast_requests: u32,
    pub max_fast_requests: u32,
}
