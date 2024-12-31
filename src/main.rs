use axum::{
    body::Body,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Json, Router,
};
use chrono::Local;
use cursor_api::{
    app::{
        config::handle_config_update,
        constant::*,
        models::*,
        statics::*,
        token::{
            get_user_info, handle_get_checksum, handle_get_tokeninfo, handle_update_tokeninfo,
            handle_update_tokeninfo_post, load_tokens,
        },
        utils::{parse_bool_from_env, parse_string_from_env},
    },
    chat::{
        constant::AVAILABLE_MODELS,
        service::{handle_chat, handle_models},
    },
    common::models::{
        health::{CpuInfo, HealthCheckResponse, MemoryInfo, SystemInfo, SystemStats},
        ApiStatus,
    },
};
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::sync::Mutex;
use tower_http::cors::CorsLayer;

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

    if get_auth_token() == EMPTY_STRING {
        panic!("AUTH_TOKEN must be set")
    };

    // 初始化全局配置
    AppConfig::init(
        parse_bool_from_env("ENABLE_STREAM_CHECK", true),
        parse_bool_from_env("INCLUDE_STOP_REASON_STREAM", true),
        VisionAbility::from_str(&parse_string_from_env("VISION_ABILITY", EMPTY_STRING)),
        parse_bool_from_env("ENABLE_SLOW_POOL", false),
        parse_bool_from_env("PASS_ANY_CLAUDE", false),
    );

    // 加载 tokens
    let token_infos = load_tokens();

    // 初始化应用状态
    let state = Arc::new(Mutex::new(AppState::new(token_infos)));

    // 设置路由
    let app = Router::new()
        .route(ROUTE_ROOT_PATH, get(handle_root))
        .route(ROUTE_HEALTH_PATH, get(handle_health))
        .route(ROUTE_TOKENINFO_PATH, get(handle_tokeninfo_page))
        .route(ROUTE_MODELS_PATH.as_str(), get(handle_models))
        .route(ROUTE_GET_CHECKSUM, get(handle_get_checksum))
        .route(ROUTE_GET_USER_INFO_PATH, get(get_user_info))
        .route(ROUTE_UPDATE_TOKENINFO_PATH, get(handle_update_tokeninfo))
        .route(ROUTE_GET_TOKENINFO_PATH, post(handle_get_tokeninfo))
        .route(
            ROUTE_UPDATE_TOKENINFO_PATH,
            post(handle_update_tokeninfo_post),
        )
        .route(ROUTE_CHAT_PATH.as_str(), post(handle_chat))
        .route(ROUTE_LOGS_PATH, get(handle_logs))
        .route(ROUTE_LOGS_PATH, post(handle_logs_post))
        .route(ROUTE_ENV_EXAMPLE_PATH, get(handle_env_example))
        .route(ROUTE_CONFIG_PATH, get(handle_config_page))
        .route(ROUTE_CONFIG_PATH, post(handle_config_update))
        .route(ROUTE_STATIC_PATH, get(handle_static))
        .route(ROUTE_ABOUT_PATH, get(handle_about))
        .route(ROUTE_README_PATH, get(handle_readme))
        .layer(CorsLayer::permissive())
        .with_state(state);

    // 启动服务器
    let port = parse_string_from_env("PORT", "3000");
    let addr = format!("0.0.0.0:{}", port);
    println!("服务器运行在端口 {}", port);
    println!("当前版本: v{}", PKG_VERSION);
    // if !std::env::args().any(|arg| arg == "--no-instruction") {
    //     println!(include_str!("../start_instruction"));
    // }

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// 根路由处理
async fn handle_root() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_ROOT_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header(HEADER_NAME_LOCATION, ROUTE_HEALTH_PATH)
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

async fn handle_health(State(state): State<Arc<Mutex<AppState>>>) -> Json<HealthCheckResponse> {
    let start_time = get_start_time();

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

    Json(HealthCheckResponse {
        status: ApiStatus::Healthy,
        version: PKG_VERSION,
        uptime,
        stats: SystemStats {
            started: start_time.to_string(),
            total_requests: state.total_requests,
            active_requests: state.active_requests,
            system: SystemInfo {
                memory: MemoryInfo {
                    rss: memory, // 物理内存使用量(字节)
                },
                cpu: CpuInfo {
                    usage: cpu_usage, // CPU 使用率(百分比)
                },
            },
        },
        models: AVAILABLE_MODELS.iter().map(|m| m.id).collect::<Vec<_>>(),
        endpoints: vec![
            ROUTE_CHAT_PATH.as_str(),
            ROUTE_MODELS_PATH.as_str(),
            ROUTE_GET_CHECKSUM,
            ROUTE_TOKENINFO_PATH,
            ROUTE_UPDATE_TOKENINFO_PATH,
            ROUTE_GET_TOKENINFO_PATH,
            ROUTE_LOGS_PATH,
            ROUTE_GET_USER_INFO_PATH,
            ROUTE_ENV_EXAMPLE_PATH,
            ROUTE_CONFIG_PATH,
            ROUTE_STATIC_PATH,
            ROUTE_ABOUT_PATH,
            ROUTE_README_PATH

        ],
    })
}

async fn handle_tokeninfo_page() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_TOKENINFO_PATH).unwrap_or_default() {
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

// 日志处理
async fn handle_logs() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_LOGS_PATH).unwrap_or_default() {
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
) -> Result<Json<LogsResponse>, StatusCode> {
    let auth_token = get_auth_token();

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
struct LogsResponse {
    status: ApiStatus,
    total: usize,
    logs: Vec<RequestLog>,
    timestamp: String,
}

async fn handle_env_example() -> impl IntoResponse {
    Response::builder()
        .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8)
        .body(include_str!("../.env.example").to_string())
        .unwrap()
}

// 配置页面处理函数
async fn handle_config_page() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_CONFIG_PATH).unwrap_or_default() {
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

async fn handle_static(Path(path): Path<String>) -> impl IntoResponse {
    match path.as_str() {
        "shared-styles.css" => {
            match AppConfig::get_page_content(ROUTE_SHARED_STYLES_PATH).unwrap_or_default() {
                PageContent::Default => Response::builder()
                    .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_CSS_WITH_UTF8)
                    .body(include_str!("../static/shared-styles.min.css").to_string())
                    .unwrap(),
                PageContent::Text(content) | PageContent::Html(content) => Response::builder()
                    .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_CSS_WITH_UTF8)
                    .body(content.clone())
                    .unwrap(),
            }
        }
        "shared.js" => {
            match AppConfig::get_page_content(ROUTE_SHARED_JS_PATH).unwrap_or_default() {
                PageContent::Default => Response::builder()
                    .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_JS_WITH_UTF8)
                    .body(include_str!("../static/shared.min.js").to_string())
                    .unwrap(),
                PageContent::Text(content) | PageContent::Html(content) => Response::builder()
                    .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_JS_WITH_UTF8)
                    .body(content.clone())
                    .unwrap(),
            }
        }
        _ => Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body("Not found".to_string())
            .unwrap(),
    }
}

async fn handle_about() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_ABOUT_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .header(HEADER_NAME_CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(include_str!("../static/readme.min.html").to_string())
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

async fn handle_readme() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_README_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header(HEADER_NAME_LOCATION, ROUTE_ABOUT_PATH)
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
