use crate::{
    app::{
        constant::{
            BUILD_TIMESTAMP, IS_DEBUG, IS_PRERELEASE, PKG_NAME, PKG_VERSION, ROUTE_ABOUT_PATH,
            ROUTE_API_PATH, ROUTE_BUILD_KEY_PATH, ROUTE_CONFIG_PATH, ROUTE_CONFIG_VERSION_GET_PATH,
            ROUTE_CPP_CONFIG_PATH, ROUTE_CPP_MODELS_PATH, ROUTE_CPP_STREAM_PATH,
            ROUTE_ENV_EXAMPLE_PATH, ROUTE_FILE_SYNC_PATH, ROUTE_FILE_UPLOAD_PATH,
            ROUTE_GEN_CHECKSUM, ROUTE_GEN_HASH, ROUTE_GEN_UUID, ROUTE_GET_TIMESTAMP_HEADER,
            ROUTE_HEALTH_PATH, ROUTE_LOGS_GET_PATH, ROUTE_LOGS_PATH, ROUTE_LOGS_TOKENS_GET_PATH,
            ROUTE_PROXIES_ADD_PATH, ROUTE_PROXIES_DELETE_PATH, ROUTE_PROXIES_GET_PATH,
            ROUTE_PROXIES_PATH, ROUTE_PROXIES_SET_GENERAL_PATH, ROUTE_PROXIES_SET_PATH,
            ROUTE_README_PATH, ROUTE_ROOT_PATH, ROUTE_STATIC_PATH, ROUTE_TOKENS_ADD_PATH,
            ROUTE_TOKENS_CONFIG_VERSION_UPDATE_PATH, ROUTE_TOKENS_DELETE_PATH,
            ROUTE_TOKENS_GET_PATH, ROUTE_TOKENS_PATH, ROUTE_TOKENS_PROFILE_UPDATE_PATH,
            ROUTE_TOKENS_PROXY_SET_PATH, ROUTE_TOKENS_REFRESH_PATH, ROUTE_TOKENS_SET_PATH,
            ROUTE_TOKENS_STATUS_SET_PATH, ROUTE_TOKENS_TIMEZONE_SET_PATH,
        },
        lazy::get_start_time,
        model::{AppConfig, AppState, DateTime},
    },
    common::model::{
        ApiStatus,
        health::{
            BuildInfo, Capabilities, CpuInfo, HealthCheckResponse, MemoryInfo, RequestStats,
            RuntimeStats, ServiceInfo, SystemStats,
        },
    },
    core::constant::Models,
};
use axum::{
    Json,
    body::Body,
    extract::State,
    http::{StatusCode, header::LOCATION},
    response::Response,
};
use std::sync::Arc;
use sysinfo::{CpuRefreshKind, MemoryRefreshKind, RefreshKind, System};

pub async fn handle_root() -> Response {
    AppConfig::get_page_content(ROUTE_ROOT_PATH)
        .unwrap_or_default()
        .into_response(|| {
            Response::builder()
                .status(StatusCode::TEMPORARY_REDIRECT)
                .header(LOCATION, ROUTE_HEALTH_PATH)
                .body(Body::empty())
        })
}

const ENDPOINTS: &'static [&'static str] = &[
    "{}/v1/chat/completions",
    "{}/v1/messages",
    "{}/v1/models",
    "{}/raw/models",
    ROUTE_TOKENS_PATH,
    ROUTE_TOKENS_GET_PATH,
    ROUTE_TOKENS_SET_PATH,
    ROUTE_TOKENS_ADD_PATH,
    ROUTE_TOKENS_DELETE_PATH,
    ROUTE_TOKENS_PROFILE_UPDATE_PATH,
    ROUTE_TOKENS_CONFIG_VERSION_UPDATE_PATH,
    ROUTE_TOKENS_REFRESH_PATH,
    ROUTE_TOKENS_STATUS_SET_PATH,
    ROUTE_TOKENS_PROXY_SET_PATH,
    ROUTE_TOKENS_TIMEZONE_SET_PATH,
    ROUTE_PROXIES_PATH,
    ROUTE_PROXIES_GET_PATH,
    ROUTE_PROXIES_SET_PATH,
    ROUTE_PROXIES_ADD_PATH,
    ROUTE_PROXIES_DELETE_PATH,
    ROUTE_PROXIES_SET_GENERAL_PATH,
    ROUTE_LOGS_PATH,
    ROUTE_LOGS_GET_PATH,
    ROUTE_LOGS_TOKENS_GET_PATH,
    ROUTE_ENV_EXAMPLE_PATH,
    ROUTE_CONFIG_PATH,
    ROUTE_STATIC_PATH,
    ROUTE_ABOUT_PATH,
    ROUTE_README_PATH,
    ROUTE_API_PATH,
    ROUTE_GEN_UUID,
    ROUTE_GEN_HASH,
    ROUTE_GEN_CHECKSUM,
    ROUTE_GET_TIMESTAMP_HEADER,
    // ROUTE_BASIC_CALIBRATION_PATH,
    // ROUTE_USER_INFO_PATH,
    ROUTE_BUILD_KEY_PATH,
    ROUTE_CONFIG_VERSION_GET_PATH,
    // ROUTE_TOKEN_UPGRADE_PATH,
    ROUTE_CPP_CONFIG_PATH,
    ROUTE_CPP_MODELS_PATH,
    ROUTE_FILE_UPLOAD_PATH,
    ROUTE_FILE_SYNC_PATH,
    ROUTE_CPP_STREAM_PATH,
];

pub async fn handle_health(State(state): State<Arc<AppState>>) -> Json<HealthCheckResponse> {
    let system = {
        let mut sys = System::new_with_specifics(
            RefreshKind::nothing()
                .with_memory(MemoryRefreshKind::everything())
                .with_cpu(CpuRefreshKind::everything()),
        );

        std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);

        // 刷新系统信息
        sys.refresh_memory();
        sys.refresh_cpu_usage();

        let pid = std::process::id() as usize;
        let process = sys.process(pid.into());

        // 获取程序内存使用量和系统总内存
        let memory_used = process.map(|p| p.memory()).unwrap_or(0);
        let total_memory = sys.total_memory();
        let available_memory = sys.available_memory();

        // 计算内存使用比例(百分比)
        let memory_percentage = if total_memory > 0 {
            (memory_used as f32 / total_memory as f32) * 100.0
        } else {
            0.0
        };

        // 获取 CPU 使用率
        let cpu_usage = sys.global_cpu_usage();

        // 获取负载平均值
        let load_avg = {
            let load = System::load_average();
            [load.one, load.five, load.fifteen]
        };

        SystemStats {
            memory: MemoryInfo {
                used_bytes: memory_used,                 // 当前进程使用的内存
                used_percentage: memory_percentage,      // 当前进程内存占系统总内存的百分比
                available_bytes: Some(available_memory), // 系统可用内存
            },
            cpu: CpuInfo {
                usage_percentage: cpu_usage, // 系统整体 CPU 使用率
                load_average: load_avg,      // 系统负载平均值
            },
        }
    };

    Json(HealthCheckResponse {
        status: ApiStatus::Success,
        service: ServiceInfo {
            name: PKG_NAME,
            version: PKG_VERSION,
            is_debug: *crate::app::lazy::log::DEBUG,
            build: BuildInfo {
                #[cfg(feature = "__preview")]
                version: crate::app::constant::BUILD_VERSION,
                timestamp: BUILD_TIMESTAMP,
                is_debug: IS_DEBUG,
                is_prerelease: IS_PRERELEASE,
            },
        },
        runtime: {
            let started_at = get_start_time();
            RuntimeStats {
                started_at: DateTime::from_naive(started_at),
                uptime_seconds: (DateTime::naive_now() - *started_at).num_seconds(),
                requests: RequestStats {
                    total: state
                        .total_requests
                        .load(::core::sync::atomic::Ordering::Relaxed),
                    active: state
                        .active_requests
                        .load(::core::sync::atomic::Ordering::Relaxed),
                    errors: state
                        .error_requests
                        .load(::core::sync::atomic::Ordering::Relaxed),
                },
            }
        },
        system,
        capabilities: Capabilities {
            models: Models::ids(),
            endpoints: ENDPOINTS,
            features: &[
                #[cfg(feature = "__preview")]
                "preview",
                #[cfg(feature = "__compat")]
                "compat",
            ],
        },
    })
}
