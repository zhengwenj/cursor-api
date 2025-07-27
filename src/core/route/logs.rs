use crate::{
    app::{
        constant::{
            AUTHORIZATION_BEARER_PREFIX, ERR_LOG_TOKEN_NOT_FOUND, HEADER_VALUE_TEXT_HTML_UTF8,
            ROUTE_LOGS_PATH,
        },
        lazy::AUTH_TOKEN,
        model::{AppConfig, AppState, DateTime, ExtToken, LogStatus, RequestLog, TokenKey},
    },
    common::model::{ApiStatus, userinfo::MembershipType},
    core::config::parse_dynamic_token,
};
use ahash::{HashMap, HashSet};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{
        HeaderMap, StatusCode,
        header::{AUTHORIZATION, CONTENT_TYPE},
    },
    response::Response,
};
use std::sync::{Arc, atomic::Ordering};

// 日志处理
pub async fn handle_logs() -> Response {
    AppConfig::get_page_content(ROUTE_LOGS_PATH)
        .unwrap_or_default()
        .into_response(|| {
            Response::builder()
                .header(CONTENT_TYPE, HEADER_VALUE_TEXT_HTML_UTF8)
                .body(Body::from(include_str!("../../../static/logs.min.html")))
        })
}

#[derive(::serde::Deserialize, Default)]
pub struct LogsQueryParams {
    // 分页与排序控制
    pub limit: Option<usize>,  // 返回记录数量限制
    pub offset: Option<usize>, // 起始位置偏移量
    pub reverse: Option<bool>, // 反向排序，默认false（从旧到新）

    // 时间范围过滤
    pub from_date: Option<DateTime>, // 开始日期时间
    pub to_date: Option<DateTime>,   // 结束日期时间

    // 用户标识过滤
    pub user_id: Option<String>,         // 按用户ID精确匹配
    pub email: Option<String>,           // 按用户邮箱过滤（部分匹配）
    pub membership_type: Option<String>, // 按会员类型过滤

    // 核心业务过滤
    pub status: Option<String>,              // 按状态过滤
    pub model: Option<String>,               // 按模型名称过滤（部分匹配）
    pub include_models: Option<Vec<String>>, // 包含特定模型
    pub exclude_models: Option<Vec<String>>, // 排除特定模型

    // 请求特征过滤
    pub stream: Option<bool>,    // 是否为流式请求
    pub has_chain: Option<bool>, // 是否包含对话链

    // 错误相关过滤
    pub has_error: Option<bool>, // 是否包含错误
    pub error: Option<String>,   // 按错误过滤（部分匹配）

    // 性能指标过滤
    pub min_total_time: Option<f64>, // 最小总耗时（秒）
    pub max_total_time: Option<f64>, // 最大总耗时（秒）
    pub min_tokens: Option<i32>,     // 最小token数
    pub max_tokens: Option<i32>,     // 最大token数
}

#[derive(::serde::Deserialize)]
pub struct LogsRequest {
    #[serde(default)]
    pub query: LogsQueryParams,
}

pub async fn handle_get_logs(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(request): Json<LogsRequest>,
) -> Result<Json<LogsResponse>, StatusCode> {
    // 获取认证头
    let auth_token = headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_token = if auth_token != *AUTH_TOKEN {
        Some(if let Some(token_key) = TokenKey::from_string(auth_token) {
            token_key
        } else {
            parse_dynamic_token(auth_token)
                .and_then(|key_config| key_config.token_info)
                .and_then(|info| info.token)
                .and_then(|t| t.into_raw())
                .ok_or(StatusCode::UNAUTHORIZED)?
                .key()
        })
    } else {
        None
    };

    // 如果状态存在但无效，直接返回空结果
    if let Some(status) = &request.query.status
        && LogStatus::from_str_name(status).is_none()
    {
        return Ok(Json(LogsResponse {
            status: ApiStatus::Success,
            total: 0,
            active: None,
            error: None,
            logs: Vec::new(),
            timestamp: DateTime::now(),
        }));
    }

    // 如果会员类型存在但无效，直接返回空结果
    let membership_enum = if let Some(membership_type) = &request.query.membership_type {
        match MembershipType::from_str(membership_type) {
            Some(m) => Some(m),
            None => {
                return Ok(Json(LogsResponse {
                    status: ApiStatus::Success,
                    total: 0,
                    active: None,
                    error: None,
                    logs: Vec::new(),
                    timestamp: DateTime::now(),
                }));
            }
        }
    } else {
        None
    };

    // 如果user_id存在但无效，直接返回空结果
    let parsed_user_id = if let Some(user_id) = &request.query.user_id {
        match user_id.parse() {
            Ok(id) => Some(id),
            Err(_) => {
                return Err(StatusCode::BAD_REQUEST);
            }
        }
    } else {
        None
    };

    // 准备日志数据
    let log_manager = state.log_manager_lock().await;
    let tokens = log_manager.tokens();
    let mut iterator = Box::new(log_manager.logs().iter()) as Box<dyn Iterator<Item = &RequestLog>>;

    let (active, error) = if let Some(token_key) = user_token {
        iterator = Box::new(iterator.filter(move |log| log.token_info.key == token_key));
        (None, None)
    } else {
        (
            Some(state.active_requests.load(Ordering::Relaxed)),
            Some(state.error_requests.load(Ordering::Relaxed)),
        )
    };

    // 时间范围过滤
    if let Some(from_date) = request.query.from_date {
        iterator = Box::new(iterator.filter(move |log| log.timestamp >= from_date));
    }

    if let Some(to_date) = request.query.to_date {
        iterator = Box::new(iterator.filter(move |log| log.timestamp <= to_date));
    }

    // 用户标识过滤
    if let Some(user_id) = parsed_user_id {
        iterator = Box::new(iterator.filter(move |log| {
            tokens
                .get(&log.token_info.key)
                .expect(ERR_LOG_TOKEN_NOT_FOUND)
                .primary_token
                .raw()
                .subject
                .id
                == user_id
        }));
    }

    if let Some(email) = &request.query.email {
        iterator = Box::new(iterator.filter(move |log| {
            tokens
                .get(&log.token_info.key)
                .expect(ERR_LOG_TOKEN_NOT_FOUND)
                .user
                .as_ref()
                .map(|user| user.email.contains(email))
                .unwrap_or(false)
        }));
    }

    if let Some(membership_type) = membership_enum {
        iterator = Box::new(iterator.filter(move |log| {
            log.token_info
                .stripe
                .as_ref()
                .map(|p| p.membership_type == membership_type)
                .unwrap_or(false)
        }));
    }

    // 核心业务过滤
    if let Some(status) = &request.query.status {
        iterator = Box::new(iterator.filter(move |log| log.status.as_str_name() == status));
    }

    if let Some(model) = &request.query.model {
        iterator = Box::new(iterator.filter(move |log| log.model.contains(model)));
    }

    if let Some(include_models) = &request.query.include_models {
        iterator =
            Box::new(iterator.filter(move |log| include_models.iter().any(|m| log.model == *m)));
    }

    if let Some(exclude_models) = &request.query.exclude_models {
        iterator =
            Box::new(iterator.filter(move |log| !exclude_models.iter().any(|m| log.model == *m)));
    }

    // 请求特征过滤
    if let Some(stream) = request.query.stream {
        iterator = Box::new(iterator.filter(move |log| log.stream == stream));
    }

    if let Some(has_chain) = request.query.has_chain {
        iterator = Box::new(iterator.filter(move |log| log.chain.is_some() == has_chain));
    }

    // 错误相关过滤
    if let Some(has_error) = request.query.has_error {
        iterator = Box::new(iterator.filter(move |log| log.error.is_some() == has_error));
    }

    if let Some(error) = &request.query.error {
        iterator = Box::new(iterator.filter(move |log| log.error.contains(error)));
    }

    // 性能指标过滤
    if let Some(min_time) = request.query.min_total_time {
        iterator = Box::new(iterator.filter(move |log| log.timing.total >= min_time));
    }

    if let Some(max_time) = request.query.max_total_time {
        iterator = Box::new(iterator.filter(move |log| log.timing.total <= max_time));
    }

    if let Some(min_tokens) = request.query.min_tokens {
        iterator = Box::new(iterator.filter(move |log| {
            log.chain
                .as_ref()
                .and_then(|c| c.usage)
                .map(|u| (u.input + u.output) >= min_tokens)
                .unwrap_or(false)
        }));
    }

    if let Some(max_tokens) = request.query.max_tokens {
        iterator = Box::new(iterator.filter(move |log| {
            log.chain
                .as_ref()
                .and_then(|c| c.usage)
                .map(|u| (u.input + u.output) <= max_tokens)
                .unwrap_or(false)
        }));
    }

    // 收集过滤后的日志引用
    let filtered_logs: Vec<_> = iterator.cloned().collect();
    let total = filtered_logs.len() as u64;

    // 应用分页（根据reverse参数决定迭代方向）
    let result_logs: Vec<RequestLog> = if request.query.reverse.unwrap_or(false) {
        filtered_logs
            .into_iter()
            .rev()
            .skip(request.query.offset.unwrap_or(0))
            .take(request.query.limit.unwrap_or(usize::MAX))
            .collect()
    } else {
        filtered_logs
            .into_iter()
            .skip(request.query.offset.unwrap_or(0))
            .take(request.query.limit.unwrap_or(usize::MAX))
            .collect()
    };

    drop(log_manager);

    Ok(Json(LogsResponse {
        status: ApiStatus::Success,
        total,
        active,
        error,
        logs: result_logs,
        timestamp: DateTime::now(),
    }))
}

#[derive(::serde::Serialize)]
pub struct LogsResponse {
    pub status: ApiStatus,
    pub total: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<u64>,
    pub logs: Vec<RequestLog>,
    pub timestamp: DateTime,
}

pub async fn handle_get_logs_tokens(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(keys): Json<HashSet<String>>,
) -> Result<Json<LogsTokensResponse>, StatusCode> {
    // 获取认证头
    let auth_token = headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_token = if auth_token != *AUTH_TOKEN {
        Some(if let Some(token_key) = TokenKey::from_string(auth_token) {
            token_key
        } else {
            parse_dynamic_token(auth_token)
                .and_then(|key_config| key_config.token_info)
                .and_then(|info| info.token)
                .and_then(|t| t.into_raw())
                .ok_or(StatusCode::UNAUTHORIZED)?
                .key()
        })
    } else {
        None
    };

    if let Some(token_key) = user_token {
        let mut iter = keys.into_iter();
        let key = iter.next();
        if let Some(key_str) = key
            && iter.next().is_none()
        {
            match TokenKey::from_string(&key_str) {
                Some(key) if key == token_key => {
                    let result = state
                        .log_manager_lock()
                        .await
                        .get_token(&token_key)
                        .cloned();
                    Ok(Json(LogsTokensResponse {
                        status: ApiStatus::Success,
                        tokens: HashMap::from_iter([(key_str, result)]),
                        total: 1,
                        timestamp: DateTime::now(),
                    }))
                }
                Some(_) => Err(StatusCode::UNAUTHORIZED),
                None => Err(StatusCode::BAD_REQUEST),
            }
        } else {
            Err(StatusCode::UNAUTHORIZED)
        }
    } else {
        let keys: Vec<_> = keys
            .into_iter()
            .filter_map(|s| TokenKey::from_string(&s).map(|key| (s, key)))
            .collect();
        let len = keys.len();
        let mut map = HashMap::with_capacity_and_hasher(len, ::ahash::RandomState::new());
        let log_manager = state.log_manager_lock().await;
        for (s, key) in keys {
            let value = log_manager.get_token(&key).cloned();
            map.insert(s, value);
        }
        Ok(Json(LogsTokensResponse {
            status: ApiStatus::Success,
            tokens: map,
            total: len as u64,
            timestamp: DateTime::now(),
        }))
    }
}

#[derive(::serde::Serialize)]
pub struct LogsTokensResponse {
    pub status: ApiStatus,
    pub tokens: HashMap<String, Option<ExtToken>>,
    pub total: u64,
    pub timestamp: DateTime,
}
