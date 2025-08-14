#![allow(internal_features)]
#![feature(
    addr_parse_ascii,
    cold_path,
    hasher_prefixfree_extras,
    const_trait_impl,
    const_default,
    core_intrinsics,
    associated_type_defaults,
    sized_type_properties
)]
#![allow(clippy::redundant_static_lifetimes)]

#[macro_use]
extern crate cursor_api;

mod app;
mod common;
mod core;
mod leak;
mod natural_args;

use ::axum::{
    Router, middleware,
    routing::{get, post},
};
use ::tokio::signal;
use ::tower_http::{cors::CorsLayer, limit::RequestBodyLimitLayer};

use app::{
    config::handle_config_update,
    constant::{
        EMPTY_STRING, EXE_NAME, ROUTE_ABOUT_PATH, ROUTE_API_PATH, ROUTE_BUILD_KEY_PATH,
        ROUTE_CONFIG_PATH, ROUTE_CONFIG_VERSION_GET_PATH, ROUTE_CPP_CONFIG_PATH,
        ROUTE_CPP_MODELS_PATH, ROUTE_CPP_STREAM_PATH, ROUTE_ENV_EXAMPLE_PATH, ROUTE_FILE_SYNC_PATH,
        ROUTE_FILE_UPLOAD_PATH, ROUTE_GEN_CHECKSUM, ROUTE_GEN_HASH, ROUTE_GEN_UUID,
        ROUTE_GET_TIMESTAMP_HEADER, ROUTE_HEALTH_PATH, ROUTE_LOGS_GET_PATH, ROUTE_LOGS_PATH,
        ROUTE_LOGS_TOKENS_GET_PATH, ROUTE_PROXIES_ADD_PATH, ROUTE_PROXIES_DELETE_PATH,
        ROUTE_PROXIES_GET_PATH, ROUTE_PROXIES_PATH, ROUTE_PROXIES_SET_GENERAL_PATH,
        ROUTE_PROXIES_SET_PATH, ROUTE_README_PATH, ROUTE_ROOT_PATH, ROUTE_STATIC_PATH,
        ROUTE_TOKENS_ADD_PATH, ROUTE_TOKENS_ALIAS_SET_PATH,
        ROUTE_TOKENS_CONFIG_VERSION_UPDATE_PATH, ROUTE_TOKENS_DELETE_PATH, ROUTE_TOKENS_GET_PATH,
        ROUTE_TOKENS_PATH, ROUTE_TOKENS_PROFILE_UPDATE_PATH, ROUTE_TOKENS_PROXY_SET_PATH,
        ROUTE_TOKENS_REFRESH_PATH, ROUTE_TOKENS_SET_PATH, ROUTE_TOKENS_STATUS_SET_PATH,
        ROUTE_TOKENS_TIMEZONE_SET_PATH, VERSION,
    },
    lazy::AUTH_TOKEN,
    model::{AppConfig, AppState},
};
use common::utils::parse_from_env;
use core::{
    middleware::{admin_auth_middleware, cpp_auth_middleware, v1_auth_middleware},
    route::{
        handle_about, handle_add_proxy, handle_add_tokens, handle_api_page, handle_build_key,
        handle_build_key_page, handle_config_page, handle_delete_proxies, handle_delete_tokens,
        handle_env_example, handle_gen_checksum, handle_gen_hash, handle_gen_uuid,
        handle_get_config_version, handle_get_logs, handle_get_logs_tokens, handle_get_proxies,
        handle_get_timestamp_header, handle_get_tokens, handle_health, handle_logs, handle_options,
        handle_proxies_page, handle_readme, handle_refresh_tokens, handle_root,
        handle_set_general_proxy, handle_set_proxies, handle_set_tokens, handle_set_tokens_alias,
        handle_set_tokens_proxy, handle_set_tokens_status, handle_set_tokens_timezone,
        handle_static, handle_tokens_page, handle_update_tokens_config_version,
        handle_update_tokens_profile,
    },
    service::{
        cpp::{
            handle_cpp_config, handle_cpp_models, handle_stream_cpp, handle_sync_file,
            handle_upload_file,
        },
        handle_chat_completions, handle_messages, handle_models, handle_raw_models,
    },
};
use natural_args::{DEFAULT_LISTEN_HOST, ENV_HOST, ENV_PORT};

#[tokio::main]
async fn main() {
    // 设置自定义 panic hook
    #[cfg(not(debug_assertions))]
    ::std::panic::set_hook(Box::new(|info| {
        __cold_path!(); // panic 是异常路径
        // std::env::set_var("RUST_BACKTRACE", "1");
        if let Some(msg) = info.payload().downcast_ref::<String>() {
            __eprintln!(msg);
        } else if let Some(msg) = info.payload().downcast_ref::<&str>() {
            __eprintln!(msg);
        }
    }));

    // 加载环境变量
    {
        let current_exe = __unwrap_panic!(std::env::current_exe());
        let file_name = __unwrap_panic!(
            current_exe
                .file_name()
                .and_then(|s| s.to_str())
                .ok_or("filename")
        );
        let expect = __unwrap_panic!(current_exe.parent().ok_or("parent"))
            .join(EXE_NAME)
            .is_file();

        if file_name != EXE_NAME {
            if expect {
                println!(
                    "Oh, I see you already have a {EXE_NAME} sitting there. Multiple versions? How adventurous of you!"
                )
            } else {
                println!("{file_name}? Really? *{EXE_NAME}* was literally right there!");
            };
        }

        // 处理自然语言参数
        natural_args::process_args(file_name);
    }

    // tracing_subscriber::fmt::init();

    if AUTH_TOKEN.is_empty() {
        __cold_path!();
        __eprintln!("AUTH_TOKEN must be set\n");
        std::process::exit(1);
    };

    // 初始化全局配置
    AppConfig::init();

    // 初始化应用状态
    let state = std::sync::Arc::new(__unwrap_panic!(AppState::load().await));

    // 尝试加载保存的配置
    if let Err(e) = AppConfig::load() {
        __cold_path!(); // 配置加载失败是错误路径
        eprintln!("加载保存的配置失败: {e}");
    }

    // 创建一个克隆用于后台任务
    let state_for_reload = state.clone();

    // 启动后台任务在每个整1000秒时更新 checksum
    tokio::spawn(async move {
        use crate::app::model::TimestampHeader;
        let state = state_for_reload;
        let mut counter = 29u8;

        loop {
            let now = common::utils::now_secs();
            let current_kilo = now / 1000;

            // 更新为当前千秒
            TimestampHeader::update_global_with(current_kilo);

            // 等待到下一个千秒
            let wait_duration = (current_kilo + 1) * 1000 - now;
            ::tokio::time::sleep(::core::time::Duration::from_secs(wait_duration)).await;

            // 每30次循环才更新一次client_key
            counter += 1;
            if counter >= 30 {
                state.update_client_key().await;
                counter = 0;
            }
        }
    });

    // 创建一个克隆用于信号处理
    let state_for_shutdown = state.clone();

    // 设置关闭信号处理
    let shutdown_signal = async move {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        {
            let terminate = async {
                signal::unix::signal(signal::unix::SignalKind::terminate())
                    .expect("failed to install signal handler")
                    .recv()
                    .await;
            };

            tokio::select! {
                _ = ctrl_c => {},
                _ = terminate => {},
            }
        }

        #[cfg(not(unix))]
        {
            ctrl_c.await;
        }

        __println!("正在关闭服务器...");

        // 保存配置
        if let Err(e) = AppConfig::save() {
            __cold_path!(); // 配置保存失败是错误路径
            eprintln!("保存配置失败: {e}");
        } else {
            __println!("配置已保存");
        }

        // 保存状态
        if let Err(e) = state_for_shutdown.save().await {
            __cold_path!(); // 状态保存失败是错误路径
            eprintln!("保存状态失败: {e}");
        } else {
            __println!("状态已保存");
        }

        app::lazy::log::flush_all_debug_logs().await;
    };

    // 设置路由
    let app = {
        let (
            route_raw_models_path,
            route_models_path,
            route_chat_completions_path,
            route_messages_path,
        ) = {
            define_typed_constants! {
                &'static str => {
                    RAW_MODELS_PATH = "/raw/models",
                    MODELS_PATH = "/v1/models",
                    CHAT_COMPLETIONS_PATH = "/v1/chat/completions",
                    MESSAGES_PATH = "/v1/messages",
                }
            }
            use ::std::borrow::Cow;

            let route_prefix = parse_from_env("ROUTE_PREFIX", EMPTY_STRING);
            if route_prefix.is_empty() {
                (
                    Cow::Borrowed(RAW_MODELS_PATH),
                    Cow::Borrowed(MODELS_PATH),
                    Cow::Borrowed(CHAT_COMPLETIONS_PATH),
                    Cow::Borrowed(MESSAGES_PATH),
                )
            } else {
                #[inline]
                fn make_route(route_prefix: &str, path: &'static str) -> Cow<'static, str> {
                    let mut route_path = String::with_capacity(path.len() + route_prefix.len());
                    route_path.push_str(route_prefix);
                    route_path.push_str(path);
                    Cow::Owned(route_path)
                }

                (
                    make_route(&route_prefix, RAW_MODELS_PATH),
                    make_route(&route_prefix, MODELS_PATH),
                    make_route(&route_prefix, CHAT_COMPLETIONS_PATH),
                    make_route(&route_prefix, MESSAGES_PATH),
                )
            }
        };
        Router::new()
            .without_v07_checks()
            .route(ROUTE_ROOT_PATH, get(handle_root))
            .route(ROUTE_HEALTH_PATH, get(handle_health))
            .route(ROUTE_TOKENS_PATH, get(handle_tokens_page))
            .route(ROUTE_PROXIES_PATH, get(handle_proxies_page))
            .merge(
                Router::new()
                    .without_v07_checks()
                    .route(ROUTE_TOKENS_GET_PATH, post(handle_get_tokens))
                    .route(ROUTE_TOKENS_SET_PATH, post(handle_set_tokens))
                    .route(ROUTE_TOKENS_ADD_PATH, post(handle_add_tokens))
                    .route(ROUTE_TOKENS_DELETE_PATH, post(handle_delete_tokens))
                    .route(ROUTE_TOKENS_ALIAS_SET_PATH, post(handle_set_tokens_alias))
                    .route(
                        ROUTE_TOKENS_PROFILE_UPDATE_PATH,
                        post(handle_update_tokens_profile),
                    )
                    .route(
                        ROUTE_TOKENS_CONFIG_VERSION_UPDATE_PATH,
                        post(handle_update_tokens_config_version),
                    )
                    .route(ROUTE_TOKENS_REFRESH_PATH, post(handle_refresh_tokens))
                    .route(ROUTE_TOKENS_STATUS_SET_PATH, post(handle_set_tokens_status))
                    .route(ROUTE_TOKENS_PROXY_SET_PATH, post(handle_set_tokens_proxy))
                    .route(
                        ROUTE_TOKENS_TIMEZONE_SET_PATH,
                        post(handle_set_tokens_timezone),
                    )
                    .route(ROUTE_PROXIES_GET_PATH, post(handle_get_proxies))
                    .route(ROUTE_PROXIES_SET_PATH, post(handle_set_proxies))
                    .route(ROUTE_PROXIES_ADD_PATH, post(handle_add_proxy))
                    .route(ROUTE_PROXIES_DELETE_PATH, post(handle_delete_proxies))
                    .route(
                        ROUTE_PROXIES_SET_GENERAL_PATH,
                        post(handle_set_general_proxy),
                    )
                    .route_layer(middleware::from_fn(admin_auth_middleware)),
            )
            .merge(
                Router::new()
                    .without_v07_checks()
                    .route(ROUTE_CPP_CONFIG_PATH, post(handle_cpp_config))
                    .route(ROUTE_CPP_MODELS_PATH, post(handle_cpp_models))
                    .route(ROUTE_FILE_UPLOAD_PATH, post(handle_upload_file))
                    .route(ROUTE_FILE_SYNC_PATH, post(handle_sync_file))
                    .route(ROUTE_CPP_STREAM_PATH, post(handle_stream_cpp))
                    .route_layer(middleware::from_fn_with_state(
                        state.clone(),
                        cpp_auth_middleware,
                    )),
            )
            .route(&route_raw_models_path, get(handle_raw_models))
            .route(
                &route_models_path,
                get(handle_models).options(handle_options),
            )
            .route(
                &route_messages_path,
                post(handle_messages)
                    .route_layer(middleware::from_fn_with_state(
                        state.clone(),
                        v1_auth_middleware,
                    ))
                    .options(handle_options),
            )
            .route(
                &route_chat_completions_path,
                post(handle_chat_completions)
                    .route_layer(middleware::from_fn_with_state(
                        state.clone(),
                        v1_auth_middleware,
                    ))
                    .options(handle_options),
            )
            .route(ROUTE_LOGS_PATH, get(handle_logs))
            .route(ROUTE_LOGS_GET_PATH, post(handle_get_logs))
            .route(ROUTE_LOGS_TOKENS_GET_PATH, post(handle_get_logs_tokens))
            .route(ROUTE_ENV_EXAMPLE_PATH, get(handle_env_example))
            .route(
                ROUTE_CONFIG_PATH,
                get(handle_config_page).post(handle_config_update),
            )
            .route(
                ROUTE_STATIC_PATH,
                get(handle_static).options(handle_options),
            )
            .route(ROUTE_ABOUT_PATH, get(handle_about))
            .route(ROUTE_README_PATH, get(handle_readme))
            .route(ROUTE_API_PATH, get(handle_api_page))
            .route(ROUTE_GEN_UUID, get(handle_gen_uuid))
            .route(ROUTE_GEN_HASH, get(handle_gen_hash))
            .route(ROUTE_GEN_CHECKSUM, get(handle_gen_checksum))
            .route(ROUTE_GET_TIMESTAMP_HEADER, get(handle_get_timestamp_header))
            // .route(ROUTE_BASIC_CALIBRATION_PATH, post(handle_basic_calibration))
            // .route(ROUTE_USER_INFO_PATH, post(handle_user_info))
            .route(
                ROUTE_BUILD_KEY_PATH,
                get(handle_build_key_page).post(handle_build_key),
            )
            .route(
                ROUTE_CONFIG_VERSION_GET_PATH,
                post(handle_get_config_version),
            )
            // .route(ROUTE_TOKEN_UPGRADE_PATH, post(handle_token_upgrade))
            .layer(RequestBodyLimitLayer::new(parse_from_env(
                "REQUEST_BODY_LIMIT",
                2_000_000,
            )))
            .layer(CorsLayer::permissive())
            .with_state(state)
    };

    // 启动服务器
    let listener = {
        use std::net::{IpAddr, Ipv4Addr, SocketAddr};
        let port = {
            std::env::var(ENV_PORT)
                .ok()
                .and_then(|v| v.trim().parse().ok())
                .unwrap_or(3000)
        };
        let addr = SocketAddr::new(
            IpAddr::parse_ascii(parse_from_env(ENV_HOST, DEFAULT_LISTEN_HOST).as_bytes())
                .unwrap_or_else(|e| {
                    __cold_path!(); // IP解析失败是错误路径
                    eprintln!("无法解析IP: {e}");
                    IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0))
                }),
            port,
        );
        println!("服务器运行在 {addr}");
        tokio::net::TcpListener::bind(addr)
            .await
            .unwrap_or_else(|e| {
                __cold_path!();
                eprintln!("无法绑定到地址 {addr}: {e}");
                std::process::exit(1);
            })
    };
    println!("当前版本: v{VERSION}");
    #[cfg(feature = "__preview")]
    {
        __println!("当前是测试版，有问题及时反馈哦~");
    }
    common::time::print_project_age();
    common::time::print_build_age();

    let start_time = app::lazy::get_start_time();
    let server = axum::serve(listener, app);
    tokio::select! {
        result = server => {
            if let Err(e) = result {
                __cold_path!(); // 服务器错误是异常路径
                eprintln!("服务器错误: {e}");
            }
        }
        _ = shutdown_signal => {
            println!(
                "运行时间: {}",
                common::utils::duration_fmt::human(__unwrap!(
                    app::model::DateTime::naive_now()
                        .signed_duration_since(*start_time)
                        .to_std()
                ))
                .format(common::utils::duration_fmt::DurationFormat::Random)
                .language(common::utils::duration_fmt::Language::Random)
            );
            common::time::print_project_age();
            common::time::print_build_age();
            __println!("服务器已关闭");
        }
    }
}
