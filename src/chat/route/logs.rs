use crate::{
    app::{
        constant::{
            AUTHORIZATION_BEARER_PREFIX, CONTENT_TYPE_TEXT_HTML_WITH_UTF8,
            CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8, HEADER_NAME_AUTHORIZATION, HEADER_NAME_CONTENT_TYPE,
            ROUTE_LOGS_PATH,
        },
        model::{AppConfig, AppState, PageContent, RequestLog},
        lazy::AUTH_TOKEN,
    },
    common::models::ApiStatus,
};
use axum::{
    body::Body,
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    Json,
};
use chrono::Local;
use std::sync::Arc;
use tokio::sync::Mutex;

// 日志处理
pub async fn handle_logs() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_LOGS_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(Body::from(
                include_str!("../../../static/logs.min.html").to_string(),
            ))
            .unwrap(),
        PageContent::Text(content) => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8)
            .body(Body::from(content.clone()))
            .unwrap(),
        PageContent::Html(content) => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(Body::from(content.clone()))
            .unwrap(),
    }
}

pub async fn handle_logs_post(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
) -> Result<Json<LogsResponse>, StatusCode> {
    let auth_token = AUTH_TOKEN.as_str();

    // 验证 AUTH_TOKEN
    let auth_header = headers
        .get(HEADER_NAME_AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if auth_header != auth_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let state = state.lock().await;
    Ok(Json(LogsResponse {
        status: ApiStatus::Success,
        total: state.request_logs.len(),
        logs: state.request_logs.clone(),
        timestamp: Local::now().to_string(),
    }))
}

#[derive(serde::Serialize)]
pub struct LogsResponse {
    pub status: ApiStatus,
    pub total: usize,
    pub logs: Vec<RequestLog>,
    pub timestamp: String,
}
