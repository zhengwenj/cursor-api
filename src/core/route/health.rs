use crate::{
    app::{
        constant::{
            AUTHORIZATION_BEARER_PREFIX, CONTENT_TYPE_TEXT_HTML_WITH_UTF8,
            CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8, PKG_VERSION, ROUTE_ABOUT_PATH, ROUTE_API_PATH,
            ROUTE_BASIC_CALIBRATION_PATH, ROUTE_BUILD_KEY_PATH, ROUTE_CONFIG_PATH,
            ROUTE_ENV_EXAMPLE_PATH, ROUTE_GET_CHECKSUM, ROUTE_GET_HASH, ROUTE_GET_TIMESTAMP_HEADER,
            ROUTE_HEALTH_PATH, ROUTE_LOGS_PATH, ROUTE_PROXIES_ADD_PATH, ROUTE_PROXIES_DELETE_PATH,
            ROUTE_PROXIES_GET_PATH, ROUTE_PROXIES_PATH, ROUTE_PROXIES_SET_GENERAL_PATH,
            ROUTE_PROXIES_SET_PATH, ROUTE_README_PATH, ROUTE_ROOT_PATH, ROUTE_STATIC_PATH,
            ROUTE_TOKEN_UPGRADE_PATH, ROUTE_TOKENS_ADD_PATH, ROUTE_TOKENS_BY_TAG_GET_PATH,
            ROUTE_TOKENS_DELETE_PATH, ROUTE_TOKENS_GET_PATH, ROUTE_TOKENS_PATH,
            ROUTE_TOKENS_PROFILE_UPDATE_PATH, ROUTE_TOKENS_SET_PATH, ROUTE_TOKENS_STATUS_SET_PATH,
            ROUTE_TOKENS_TAGS_GET_PATH, ROUTE_TOKENS_TAGS_SET_PATH, ROUTE_TOKENS_UPGRADE_PATH,
            ROUTE_USER_INFO_PATH,
        },
        lazy::{
            AUTH_TOKEN, ROUTE_CHAT_PATH, ROUTE_MESSAGES_PATH, ROUTE_MODELS_PATH, get_start_time,
        },
        model::{AppConfig, AppState, PageContent},
    },
    common::model::{
        ApiStatus,
        health::{CpuInfo, HealthCheckResponse, MemoryInfo, SystemInfo, SystemStats},
    },
    core::constant::Models,
};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{
        HeaderMap, StatusCode,
        header::{CONTENT_TYPE, LOCATION},
    },
    response::{IntoResponse, Response},
};
use reqwest::header::AUTHORIZATION;
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};
use tokio::sync::Mutex;

pub async fn handle_root() -> impl IntoResponse {
    match AppConfig::get_page_content(ROUTE_ROOT_PATH).unwrap_or_default() {
        PageContent::Default => Response::builder()
            .status(StatusCode::TEMPORARY_REDIRECT)
            .header(LOCATION, ROUTE_HEALTH_PATH)
            .body(Body::empty())
            .unwrap(),
        PageContent::Text(content) => Response::builder()
            .header(CONTENT_TYPE, CONTENT_TYPE_TEXT_PLAIN_WITH_UTF8)
            .body(Body::from(content))
            .unwrap(),
        PageContent::Html(content) => Response::builder()
            .header(CONTENT_TYPE, CONTENT_TYPE_TEXT_HTML_WITH_UTF8)
            .body(Body::from(content))
            .unwrap(),
    }
}

static ENDPOINTS: std::sync::LazyLock<[&'static str; 34]> = std::sync::LazyLock::new(|| {
    [
        &*ROUTE_CHAT_PATH,
        &*ROUTE_MESSAGES_PATH,
        &*ROUTE_MODELS_PATH,
        ROUTE_TOKENS_PATH,
        ROUTE_TOKENS_GET_PATH,
        ROUTE_TOKENS_SET_PATH,
        ROUTE_TOKENS_ADD_PATH,
        ROUTE_TOKENS_DELETE_PATH,
        ROUTE_TOKENS_TAGS_GET_PATH,
        ROUTE_TOKENS_TAGS_SET_PATH,
        ROUTE_TOKENS_BY_TAG_GET_PATH,
        ROUTE_TOKENS_PROFILE_UPDATE_PATH,
        ROUTE_TOKENS_UPGRADE_PATH,
        ROUTE_TOKENS_STATUS_SET_PATH,
        ROUTE_PROXIES_PATH,
        ROUTE_PROXIES_GET_PATH,
        ROUTE_PROXIES_SET_PATH,
        ROUTE_PROXIES_ADD_PATH,
        ROUTE_PROXIES_DELETE_PATH,
        ROUTE_PROXIES_SET_GENERAL_PATH,
        ROUTE_LOGS_PATH,
        ROUTE_ENV_EXAMPLE_PATH,
        ROUTE_CONFIG_PATH,
        ROUTE_STATIC_PATH,
        ROUTE_ABOUT_PATH,
        ROUTE_README_PATH,
        ROUTE_API_PATH,
        ROUTE_GET_HASH,
        ROUTE_GET_CHECKSUM,
        ROUTE_GET_TIMESTAMP_HEADER,
        ROUTE_BASIC_CALIBRATION_PATH,
        ROUTE_USER_INFO_PATH,
        ROUTE_BUILD_KEY_PATH,
        ROUTE_TOKEN_UPGRADE_PATH,
    ]
});

pub async fn handle_health(
    State(state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
) -> Json<HealthCheckResponse> {
    let start_time = get_start_time();
    let uptime = (chrono::Local::now() - start_time).num_seconds();

    // 先检查 headers 是否包含有效的认证信息
    let stats = if headers
        .get(AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .is_some_and(|token| token == AUTH_TOKEN.as_str())
    {
        // 只有在需要系统信息时才创建实例
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

        Some(SystemStats {
            started: start_time.to_string(),
            total_requests: state.request_manager.total_requests,
            active_requests: state.request_manager.active_requests,
            system: SystemInfo {
                memory: MemoryInfo {
                    rss: memory, // 物理内存使用量(字节)
                },
                cpu: CpuInfo {
                    usage: cpu_usage, // CPU 使用率(百分比)
                },
            },
        })
    } else {
        None
    };

    Json(HealthCheckResponse {
        status: ApiStatus::Healthy,
        version: PKG_VERSION,
        uptime,
        stats,
        models: Models::ids(),
        endpoints: &*ENDPOINTS,
    })
}
