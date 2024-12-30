use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use bytes::Bytes;
use chrono::Local;
use cursor_api::{
    app::{
        client::build_client,
        constant::*,
        models::*,
        token::{
            get_user_info, get_user_usage, handle_get_tokeninfo, handle_update_tokeninfo,
            handle_update_tokeninfo_post, load_tokens,
        },
        utils::{parse_bool_from_env, parse_string_from_env},
    },
    chat::{error::StreamError, stream::parse_stream_data},
};
use cursor_api::{chat::stream::StreamMessage, message::*};
use futures::{Stream, StreamExt};
use std::{
    convert::Infallible,
    sync::{atomic::AtomicBool, Arc},
};
use std::{
    pin::Pin,
    sync::atomic::{AtomicUsize, Ordering},
};
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;
use uuid::Uuid;

// 支持的模型列表
mod models;
use models::AVAILABLE_MODELS;

// 自定义错误类型
enum ChatError {
    ModelNotSupported(String),
    EmptyMessages,
    NoTokens,
    RequestFailed(String),
    Unauthorized,
}

impl ChatError {
    fn to_json(&self) -> serde_json::Value {
        let (code, message) = match self {
            ChatError::ModelNotSupported(model) => (
                "model_not_supported",
                format!("Model '{}' is not supported", model),
            ),
            ChatError::EmptyMessages => (
                "empty_messages",
                "Message array cannot be empty".to_string(),
            ),
            ChatError::NoTokens => ("no_tokens", "No available tokens".to_string()),
            ChatError::RequestFailed(err) => ("request_failed", format!("Request failed: {}", err)),
            ChatError::Unauthorized => ("unauthorized", "Invalid authorization token".to_string()),
        };

        serde_json::json!({
            "error": {
                "code": code,
                MESSAGE: message
            }
        })
    }
}

#[tokio::main]
async fn main() {
    // 设置自定义 panic hook
    std::panic::set_hook(Box::new(|info| {
        // std::env::set_var("RUST_BACKTRACE", "1");
        if let Some(msg) = info.payload().downcast_ref::<String>() {
            eprintln!("{}", msg);
        } else if let Some(msg) = info.payload().downcast_ref::<&str>() {
            eprintln!("{}", msg);
        }
    }));

    // 加载环境变量
    dotenvy::dotenv().ok();

    // 初始化全局配置
    AppConfig::init(
        parse_bool_from_env("ENABLE_STREAM_CHECK", true),
        parse_bool_from_env("INCLUDE_STOP_REASON_STREAM", true),
        VisionAbility::from_str(parse_string_from_env("VISION_ABILITY", "base64").as_str())
            .unwrap_or_default(),
        parse_bool_from_env("ENABLE_SLOW_POOL", false),
        parse_bool_from_env("PASS_ANY_CLAUDE", false),
        std::env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set"),
        parse_string_from_env("TOKEN_FILE", ".token"),
        parse_string_from_env("TOKEN_LIST_FILE", ".token-list"),
        parse_string_from_env("ROUTE_PREFIX", ""),
    );

    // 加载 tokens
    let token_infos = load_tokens();

    // 初始化应用状态
    let state = Arc::new(Mutex::new(AppState::new(token_infos)));

    let route_prefix = AppConfig::get_route_prefix();

    // 设置路由
    let app = Router::new()
        .route(ROUTER_ROOT_PATH, get(handle_root))
        .route(ROUTER_HEALTH_PATH, get(handle_health))
        .route(ROUTER_TOKENINFO_PATH, get(handle_tokeninfo_page))
        .route(&format!("{}/v1/models", route_prefix), get(handle_models))
        .route(ROUTER_GET_CHECKSUM, get(handle_get_checksum))
        .route(ROUTER_GET_USER_INFO_PATH, get(get_user_info))
        .route(ROUTER_UPDATE_TOKENINFO_PATH, get(handle_update_tokeninfo))
        .route(ROUTER_GET_TOKENINFO_PATH, post(handle_get_tokeninfo))
        .route(
            ROUTER_UPDATE_TOKENINFO_PATH,
            post(handle_update_tokeninfo_post),
        )
        .route(
            &format!("{}/v1/chat/completions", route_prefix),
            post(handle_chat),
        )
        .route(ROUTER_LOGS_PATH, get(handle_logs))
        .route(ROUTER_LOGS_PATH, post(handle_logs_post))
        .route(ROUTER_ENV_EXAMPLE_PATH, get(handle_env_example))
        .route(ROUTER_CONFIG_PATH, get(handle_config_page))
        .route(ROUTER_CONFIG_PATH, post(handle_config_update))
        .route("/static/:path", get(handle_static))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // 启动服务器
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("0.0.0.0:{}", port);
    println!("服务器运行在端口 {}", port);
    println!("当前版本: v{}", PKG_VERSION);
    if !std::env::args().any(|arg| arg == "--no-instruction") {
        println!(include_str!("../start_instruction"));
    }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 根路由处理
async fn handle_root() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTER_ROOT_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header("Location", ROUTER_HEALTH_PATH)
            .body(Body::empty())
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

async fn handle_health(State(state): State<Arc<Mutex<AppState>>>) -> Json<serde_json::Value> {
    let start_time = APP_CONFIG.read().unwrap().start_time;
    let route_prefix = AppConfig::get_route_prefix();

    // 创建系统信息实例，只监控 CPU 和内存
    let mut sys = System::new_with_specifics(
        RefreshKind::nothing()
            .with_memory(MemoryRefreshKind::everything())
            .with_cpu(CpuRefreshKind::everything()),
    );

    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

    // 刷新 CPU 和内存信息
    sys.refresh_memory();
    sys.refresh_cpu_usage();

    let pid = std::process::id() as usize;
    let process = sys.process(pid.into());

    // 获取内存信息
    let memory = process.map(|p| p.memory()).unwrap_or(0);

    // 获取 CPU 使用率
    let cpu_usage = sys.global_cpu_usage();

    let state = state.lock().await;
    let uptime = (Local::now() - start_time).num_seconds();

    Json(serde_json::json!({
        STATUS: "healthy",
        "version": PKG_VERSION,
        "uptime": uptime,
        "stats": {
            "started": start_time,
            "totalRequests": state.total_requests,
            "activeRequests": state.active_requests,
            "system": {
                "memory": {
                    "rss": memory  // 物理内存使用量(字节)
                },
                "cpu": {
                    "usage": cpu_usage  // CPU 使用率(百分比)
                }
            }
        },
        "models": AVAILABLE_MODELS.iter().map(|m| &m.id).collect::<Vec<_>>(),
        "endpoints": [
            &format!("{}/v1/chat/completions", route_prefix),
            &format!("{}/v1/models", route_prefix),
            ROUTER_GET_CHECKSUM,
            ROUTER_TOKENINFO_PATH,
            ROUTER_UPDATE_TOKENINFO_PATH,
            ROUTER_GET_TOKENINFO_PATH,
            ROUTER_LOGS_PATH,
            ROUTER_GET_USER_INFO_PATH,
            ROUTER_ENV_EXAMPLE_PATH,
            ROUTER_CONFIG_PATH,
            "/static"
        ]
    }))
}

async fn handle_tokeninfo_page() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTER_TOKENINFO_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(include_str!("../static/tokeninfo.min.html").to_string())
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

// 模型列表处理
async fn handle_models() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "object": "list",
        "data": AVAILABLE_MODELS.to_vec()
    }))
}

async fn handle_get_checksum() -> Json<serde_json::Value> {
    let checksum = cursor_api::generate_checksum(
        &cursor_api::generate_hash(),
        Some(&cursor_api::generate_hash()),
    );
    Json(serde_json::json!({
        "checksum": checksum
    }))
}

// 日志处理
async fn handle_logs() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTER_LOGS_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(Body::from(
                include_str!("../static/logs.min.html").to_string(),
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

async fn handle_logs_post(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let auth_token = AppConfig::get_auth_token();

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
    Ok(Json(serde_json::json!({
        "total": state.request_logs.len(),
        "logs": state.request_logs,
        "timestamp": Local::now(),
        STATUS: STATUS_SUCCESS
    })))
}

async fn handle_env_example() -> impl IntoResponse {
    Response::builder()
        .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8)
        .body(include_str!("../.env.example").to_string())
        .unwrap()
}

// 聊天处理函数的签名
async fn handle_chat(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    Json(request): Json<ChatRequest>,
) -> Result<Response<Body>, (StatusCode, Json<serde_json::Value>)> {
    let allow_claude = AppConfig::get_allow_claude();

    // 验证模型是否支持
    let model_supported = AVAILABLE_MODELS.iter().any(|m| m.id == request.model);

    if !(model_supported || allow_claude && request.model.starts_with("claude")) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ChatError::ModelNotSupported(request.model).to_json()),
        ));
    }

    let request_time = Local::now();

    // 验证请求
    if request.messages.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ChatError::EmptyMessages.to_json()),
        ));
    }

    // 获取并处理认证令牌
    let auth_token = headers
        .get(axum::http::header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ChatError::Unauthorized.to_json()),
        ))?;

    // 验证 AuthToken
    if auth_token != AppConfig::get_auth_token() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ChatError::Unauthorized.to_json()),
        ));
    }

    // 完整的令牌处理逻辑和对应的 checksum
    let (auth_token, checksum, alias) = {
        static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);
        let state_guard = state.lock().await;
        let token_infos = &state_guard.token_infos;

        if token_infos.is_empty() {
            return Err((
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ChatError::NoTokens.to_json()),
            ));
        }

        let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::SeqCst) % token_infos.len();
        let token_info = &token_infos[index];
        (
            token_info.token.clone(),
            token_info.checksum.clone(),
            token_info.alias.clone(),
        )
    };

    // 更新请求日志
    {
        let state_clone = state.clone();
        let mut state = state.lock().await;
        state.total_requests += 1;
        state.active_requests += 1;

        // 创建一个后台任务来获取使用情况
        let auth_token_clone = auth_token.clone();
        let checksum_clone = checksum.clone();

        tokio::spawn(async move {
            let usage = get_user_usage(&auth_token_clone, &checksum_clone).await;
            let mut state = state_clone.lock().await;
            // 根据时间戳找到对应的日志
            if let Some(log) = state
                .request_logs
                .iter_mut()
                .find(|log| log.timestamp == request_time)
            {
                log.token_info.usage = usage;
            }
        });

        state.request_logs.push(RequestLog {
            timestamp: request_time,
            model: request.model.clone(),
            token_info: TokenInfo {
                token: auth_token.clone(),
                checksum: checksum.clone(),
                alias: alias.clone(),
                usage: None,
            },
            prompt: None,
            stream: request.stream,
            status: "pending".to_string(),
            error: None,
        });

        if state.request_logs.len() > 100 {
            state.request_logs.remove(0);
        }
    }

    // 将消息转换为hex格式
    let hex_data = cursor_api::encode_chat_message(request.messages, &request.model)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(
                    ChatError::RequestFailed("Failed to encode chat message".to_string()).to_json(),
                ),
            )
        })?;

    // 构建请求客户端
    let client = build_client(&auth_token, &checksum, CURSOR_API2_STREAM_CHAT);
    let response = client.body(hex_data).send().await;

    // 处理请求结果
    let response = match response {
        Ok(resp) => {
            // 更新请求日志为成功
            {
                let mut state = state.lock().await;
                state.request_logs.last_mut().unwrap().status = STATUS_SUCCESS.to_string();
            }
            resp
        }
        Err(e) => {
            // 更新请求日志为失败
            {
                let mut state = state.lock().await;
                if let Some(last_log) = state.request_logs.last_mut() {
                    last_log.status = STATUS_FAILED.to_string();
                    last_log.error = Some(e.to_string());
                }
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChatError::RequestFailed(e.to_string()).to_json()),
            ));
        }
    };

    // 释放活动请求计数
    {
        let mut state = state.lock().await;
        state.active_requests -= 1;
    }

    if request.stream {
        let response_id = format!("chatcmpl-{}", Uuid::new_v4().simple());
        let full_text = Arc::new(Mutex::new(String::with_capacity(1024)));
        let is_start = Arc::new(AtomicBool::new(true));

        let stream = {
            // 创建新的 stream
            let mut stream = response.bytes_stream();

            let enable_stream_check = AppConfig::get_stream_check();

            if enable_stream_check {
                // 检查第一个 chunk
                match stream.next().await {
                    Some(first_chunk) => {
                        let chunk = first_chunk.map_err(|e| {
                            let error_message = format!("Failed to read response chunk: {}", e);
                            // 理论上，若程序正常，必定成功，因为前面判断过了
                            (
                                StatusCode::INTERNAL_SERVER_ERROR,
                                Json(ChatError::RequestFailed(error_message).to_json()),
                            )
                        })?;

                        match parse_stream_data(&chunk) {
                            Err(StreamError::ChatError(error)) => {
                                let error_respone = error.to_error_response();
                                // 更新请求日志为失败
                                {
                                    let mut state = state.lock().await;
                                    if let Some(last_log) = state.request_logs.last_mut() {
                                        last_log.status = STATUS_FAILED.to_string();
                                        last_log.error = Some(error_respone.native_code());
                                    }
                                }
                                return Err((
                                    error_respone.status_code(),
                                    Json(error_respone.to_json()),
                                ));
                            }
                            Ok(_) | Err(_) => {
                                // 创建一个包含第一个 chunk 的 stream
                                Box::pin(
                                    futures::stream::once(async move { Ok(chunk) }).chain(stream),
                                )
                                    as Pin<
                                        Box<
                                            dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send,
                                        >,
                                    >
                            }
                        }
                    }
                    None => {
                        // Box::pin(stream)
                        //     as Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>
                        // 更新请求日志为失败
                        {
                            let mut state = state.lock().await;
                            if let Some(last_log) = state.request_logs.last_mut() {
                                last_log.status = STATUS_FAILED.to_string();
                                last_log.error = Some("Empty stream response".to_string());
                            }
                        }
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ChatError::RequestFailed("Empty stream response".to_string())
                                    .to_json(),
                            ),
                        ));
                    }
                }
            } else {
                Box::pin(stream)
                    as Pin<Box<dyn Stream<Item = Result<Bytes, reqwest::Error>> + Send>>
            }
        }
        .then(move |chunk| {
            let response_id = response_id.clone();
            let model = request.model.clone();
            let is_start = is_start.clone();
            let full_text = full_text.clone();
            let state = state.clone();

            async move {
                let chunk = chunk.unwrap_or_default();
                match parse_stream_data(&chunk) {
                    Ok(StreamMessage::Content(texts)) => {
                        let mut response_data = String::new();

                        for text in texts {
                            let mut text_guard = full_text.lock().await;
                            text_guard.push_str(&text);
                            let is_first = is_start.load(Ordering::SeqCst);

                            let response = ChatResponse {
                                id: response_id.clone(),
                                object: OBJECT_CHAT_COMPLETION_CHUNK.to_string(),
                                created: chrono::Utc::now().timestamp(),
                                model: if is_first { Some(model.clone()) } else { None },
                                choices: vec![Choice {
                                    index: 0,
                                    message: None,
                                    delta: Some(Delta {
                                        role: if is_first {
                                            is_start.store(false, Ordering::SeqCst);
                                            Some(Role::Assistant)
                                        } else {
                                            None
                                        },
                                        content: Some(text),
                                    }),
                                    finish_reason: None,
                                }],
                                usage: None,
                            };

                            response_data.push_str(&format!(
                                "data: {}\n\n",
                                serde_json::to_string(&response).unwrap()
                            ));
                        }

                        Ok::<_, Infallible>(Bytes::from(response_data))
                    }
                    Ok(StreamMessage::StreamStart) => {
                        // 发送初始响应，包含模型信息
                        let response = ChatResponse {
                            id: response_id.clone(),
                            object: OBJECT_CHAT_COMPLETION_CHUNK.to_string(),
                            created: chrono::Utc::now().timestamp(),
                            model: {
                                is_start.store(true, Ordering::SeqCst);
                                Some(model.clone())
                            },
                            choices: vec![Choice {
                                index: 0,
                                message: None,
                                delta: Some(Delta {
                                    role: Some(Role::Assistant),
                                    content: Some(String::new()),
                                }),
                                finish_reason: None,
                            }],
                            usage: None,
                        };

                        Ok(Bytes::from(format!(
                            "data: {}\n\n",
                            serde_json::to_string(&response).unwrap()
                        )))
                    }
                    Ok(StreamMessage::StreamEnd) => {
                        // 根据配置决定是否发送最后的 finish_reason
                        let include_finish_reason =
                            parse_bool_from_env("INCLUDE_STOP_FINISH_REASON_STREAM", true);

                        if include_finish_reason {
                            let response = ChatResponse {
                                id: response_id.clone(),
                                object: OBJECT_CHAT_COMPLETION_CHUNK.to_string(),
                                created: chrono::Utc::now().timestamp(),
                                model: None,
                                choices: vec![Choice {
                                    index: 0,
                                    message: None,
                                    delta: Some(Delta {
                                        role: None,
                                        content: None,
                                    }),
                                    finish_reason: Some(FINISH_REASON_STOP.to_string()),
                                }],
                                usage: None,
                            };
                            Ok(Bytes::from(format!(
                                "data: {}\n\ndata: [DONE]\n\n",
                                serde_json::to_string(&response).unwrap()
                            )))
                        } else {
                            Ok(Bytes::from("data: [DONE]\n\n"))
                        }
                    }
                    Ok(StreamMessage::Debug(debug_prompt)) => {
                        if let Ok(mut state) = state.try_lock() {
                            if let Some(last_log) = state.request_logs.last_mut() {
                                last_log.prompt = Some(debug_prompt.clone());
                            }
                        }
                        Ok(Bytes::new())
                    }
                    Err(StreamError::ChatError(error)) => {
                        eprintln!("Stream error occurred: {}", error.to_json());
                        Ok(Bytes::new())
                    }
                    Err(e) => {
                        eprintln!("[警告] Stream error: {}", e);
                        Ok(Bytes::new())
                    }
                }
            }
        });

        Ok(Response::builder()
            .header("Cache-Control", "no-cache")
            .header("Connection", "keep-alive")
            .header(HEADER_NAME_CONTENT_TYPE, "text/event-stream")
            .body(Body::from_stream(stream))
            .unwrap())
    } else {
        // 非流式响应
        let mut full_text = String::with_capacity(1024); // 预分配合适的容量
        let mut stream = response.bytes_stream();
        let mut prompt = None;

        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ChatError::RequestFailed(format!("Failed to read response chunk: {}", e))
                            .to_json(),
                    ),
                )
            })?;

            match parse_stream_data(&chunk) {
                Ok(StreamMessage::Content(texts)) => {
                    for text in texts {
                        full_text.push_str(&text);
                    }
                }
                Ok(StreamMessage::Debug(debug_prompt)) => {
                    prompt = Some(debug_prompt);
                }
                Ok(StreamMessage::StreamStart) | Ok(StreamMessage::StreamEnd) => {}
                Err(StreamError::ChatError(error)) => {
                    return Err((
                        StatusCode::from_u16(error.error.details[0].debug.status_code())
                            .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR),
                        Json(error.to_json()),
                    ));
                }
                Err(_) => continue,
            }
        }

        // 检查响应是否为空
        if full_text.is_empty() {
            // 更新请求日志为失败
            {
                let mut state = state.lock().await;
                if let Some(last_log) = state.request_logs.last_mut() {
                    last_log.status = STATUS_FAILED.to_string();
                    last_log.error = Some("Empty response received".to_string());
                    if let Some(p) = prompt {
                        last_log.prompt = Some(p);
                    }
                }
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChatError::RequestFailed("Empty response received".to_string()).to_json()),
            ));
        }

        // 更新请求日志提示词
        {
            let mut state = state.lock().await;
            if let Some(last_log) = state.request_logs.last_mut() {
                last_log.prompt = prompt;
            }
        }

        let response_data = ChatResponse {
            id: format!("chatcmpl-{}", Uuid::new_v4().simple()),
            object: OBJECT_CHAT_COMPLETION.to_string(),
            created: chrono::Utc::now().timestamp(),
            model: Some(request.model),
            choices: vec![Choice {
                index: 0,
                message: Some(Message {
                    role: Role::Assistant,
                    content: MessageContent::Text(full_text),
                }),
                delta: None,
                finish_reason: Some(FINISH_REASON_STOP.to_string()),
            }],
            usage: Some(Usage {
                prompt_tokens: 0,
                completion_tokens: 0,
                total_tokens: 0,
            }),
        };

        Ok(Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, "application/json")
            .body(Body::from(serde_json::to_string(&response_data).unwrap()))
            .unwrap())
    }
}

// 配置页面处理函数
async fn handle_config_page() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTER_CONFIG_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(include_str!("../static/config.min.html").to_string())
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

// 配置更新处理函数
async fn handle_config_update(
    State(_state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    Json(request): Json<ConfigUpdateRequest>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    // 验证 AUTH_TOKEN
    let auth_token = AppConfig::get_auth_token();

    let auth_header = headers
        .get(HEADER_NAME_AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "未提供认证令牌"
            })),
        ))?;

    if auth_header != auth_token {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "无效的认证令牌"
            })),
        ));
    }

    match request.action.as_str() {
        "get" => Ok(Json(serde_json::json!({
            STATUS: STATUS_SUCCESS,
            "data": {
                "page_content": AppConfig::get_page_content(&request.path),
                "enable_stream_check": AppConfig::get_stream_check(),
                "include_stop_stream": AppConfig::get_stop_stream(),
                "vision_ability": AppConfig::get_vision_ability(),
                "enable_slow_pool": AppConfig::get_slow_pool(),
                "enable_all_claude": AppConfig::get_allow_claude(),
            }
        }))),

        "update" => {
            // 处理页面内容更新
            if !request.path.is_empty() && request.content.is_some() {
                let content = request.content.unwrap();

                if let Err(e) = AppConfig::update_page_content(&request.path, content) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("更新页面内容失败: {}", e)
                        })),
                    ));
                }
            }

            // 处理 enable_stream_check 更新
            if let Some(enable_stream_check) = request.enable_stream_check {
                if let Err(e) = AppConfig::update_stream_check(enable_stream_check) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("更新 enable_stream_check 失败: {}", e)
                        })),
                    ));
                }
            }

            // 处理 include_stop_stream 更新
            if let Some(include_stop_stream) = request.include_stop_stream {
                if let Err(e) = AppConfig::update_stop_stream(include_stop_stream) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("更新 include_stop_stream 失败: {}", e)
                        })),
                    ));
                }
            }

            // 处理 vision_ability 更新
            if let Some(vision_ability) = request.vision_ability {
                if let Err(e) = AppConfig::update_vision_ability(vision_ability) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("更新 vision_ability 失败: {}", e)
                        })),
                    ));
                }
            }

            // 处理 enable_slow_pool 更新
            if let Some(enable_slow_pool) = request.enable_slow_pool {
                if let Err(e) = AppConfig::update_slow_pool(enable_slow_pool) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("更新 enable_slow_pool 失败: {}", e)
                        })),
                    ));
                }
            }

            // 处理 enable_all_claude 更新
            if let Some(enable_all_claude) = request.enable_all_claude {
                if let Err(e) = AppConfig::update_allow_claude(enable_all_claude) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("更新 enable_all_claude 失败: {}", e)
                        })),
                    ));
                }
            }

            Ok(Json(serde_json::json!({
                STATUS: STATUS_SUCCESS,
                MESSAGE: "配置已更新"
            })))
        }

        "reset" => {
            // 重置页面内容
            if !request.path.is_empty() {
                if let Err(e) = AppConfig::reset_page_content(&request.path) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("重置页面内容失败: {}", e)
                        })),
                    ));
                }
            }

            // 重置 enable_stream_check
            if request.enable_stream_check.is_some() {
                if let Err(e) = AppConfig::reset_stream_check() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("重置 enable_stream_check 失败: {}", e)
                        })),
                    ));
                }
            }

            // 重置 include_stop_stream
            if request.include_stop_stream.is_some() {
                if let Err(e) = AppConfig::reset_stop_stream() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("重置 include_stop_stream 失败: {}", e)
                        })),
                    ));
                }
            }

            // 重置 vision_ability
            if request.vision_ability.is_some() {
                if let Err(e) = AppConfig::reset_vision_ability() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("重置 vision_ability 失败: {}", e)
                        })),
                    ));
                }
            }

            // 重置 enable_slow_pool
            if request.enable_slow_pool.is_some() {
                if let Err(e) = AppConfig::reset_slow_pool() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("重置 enable_slow_pool 失败: {}", e)
                        })),
                    ));
                }
            }

            // 重置 enable_all_claude
            if request.enable_all_claude.is_some() {
                if let Err(e) = AppConfig::reset_allow_claude() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(serde_json::json!({
                            "error": format!("重置 enable_slow_pool 失败: {}", e)
                        })),
                    ));
                }
            }
            Ok(Json(serde_json::json!({
                STATUS: STATUS_SUCCESS,
                MESSAGE: "配置已重置"
            })))
        }

        _ => Err((
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "无效的操作类型"
            })),
        )),
    }
}

async fn handle_static(Path(path): Path<String>) -> impl IntoResponse {
    match path.as_str() {
        "shared-styles.css" => {
            match AppConfig::get_page_content(ROUTER_SHARED_STYLES_PATH).unwrap_or_default() {
                PageContent::Default => Response::builder()
                    .header(HEADER_NAME_CONTENT_TYPE, "text/css;charset=utf-8")
                    .body(include_str!("../static/shared-styles.min.css").to_string())
                    .unwrap(),
                PageContent::Text(content) | PageContent::Html(content) => Response::builder()
                    .header(HEADER_NAME_CONTENT_TYPE, "text/css;charset=utf-8")
                    .body(content.clone())
                    .unwrap(),
            }
        }
        "shared.js" => match AppConfig::get_page_content(ROUTER_SHARED_JS_PATH).unwrap_or_default()
        {
            PageContent::Default => Response::builder()
                .header(HEADER_NAME_CONTENT_TYPE, "text/javascript;charset=utf-8")
                .body(include_str!("../static/shared.min.js").to_string())
                .unwrap(),
            PageContent::Text(content) | PageContent::Html(content) => Response::builder()
                .header(HEADER_NAME_CONTENT_TYPE, "text/javascript;charset=utf-8")
                .body(content.clone())
                .unwrap(),
        },
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found".to_string())
            .unwrap(),
    }
}
