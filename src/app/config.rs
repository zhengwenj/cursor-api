use super::{
    constant::*,
    models::{AppConfig, AppState},
    statics::*,
};
use crate::common::models::{
    config::{ConfigData, ConfigUpdateRequest},
    ApiStatus, ErrorResponse, NormalResponse,
};
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    Json,
};
use std::sync::Arc;
use tokio::sync::Mutex;

pub async fn handle_config_update(
    State(_state): State<Arc<Mutex<AppState>>>,
    headers: HeaderMap,
    Json(request): Json<ConfigUpdateRequest>,
) -> Result<Json<NormalResponse<ConfigData>>, (StatusCode, Json<ErrorResponse>)> {
    let auth_header = headers
        .get(HEADER_NAME_AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
        .ok_or((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                status: ApiStatus::Failed,
                code: Some(401),
                error: Some("未提供认证令牌".to_string()),
                message: None,
            }),
        ))?;

    if auth_header != get_auth_token() {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(ErrorResponse {
                status: ApiStatus::Failed,
                code: Some(401),
                error: Some("无效的认证令牌".to_string()),
                message: None,
            }),
        ));
    }

    match request.action.as_str() {
        "get" => Ok(Json(NormalResponse {
            status: ApiStatus::Success,
            data: Some(ConfigData {
                page_content: AppConfig::get_page_content(&request.path),
                enable_stream_check: AppConfig::get_stream_check(),
                include_stop_stream: AppConfig::get_stop_stream(),
                vision_ability: AppConfig::get_vision_ability(),
                enable_slow_pool: AppConfig::get_slow_pool(),
                enable_all_claude: AppConfig::get_allow_claude(),
                check_usage_models: AppConfig::get_usage_check(),
            }),
            message: None,
        })),

        "update" => {
            // 处理页面内容更新
            if !request.path.is_empty() && request.content.is_some() {
                let content = request.content.unwrap();

                if let Err(e) = AppConfig::update_page_content(&request.path, content) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新页面内容失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 处理 enable_stream_check 更新
            if let Some(enable_stream_check) = request.enable_stream_check {
                if let Err(e) = AppConfig::update_stream_check(enable_stream_check) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新 enable_stream_check 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 处理 include_stop_stream 更新
            if let Some(include_stop_stream) = request.include_stop_stream {
                if let Err(e) = AppConfig::update_stop_stream(include_stop_stream) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新 include_stop_stream 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 处理 vision_ability 更新
            if let Some(vision_ability) = request.vision_ability {
                if let Err(e) = AppConfig::update_vision_ability(vision_ability) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新 vision_ability 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 处理 enable_slow_pool 更新
            if let Some(enable_slow_pool) = request.enable_slow_pool {
                if let Err(e) = AppConfig::update_slow_pool(enable_slow_pool) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新 enable_slow_pool 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 处理 enable_all_claude 更新
            if let Some(enable_all_claude) = request.enable_all_claude {
                if let Err(e) = AppConfig::update_allow_claude(enable_all_claude) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新 enable_all_claude 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 处理 check_usage_models 更新
            if let Some(check_usage_models) = request.check_usage_models {
                if let Err(e) = AppConfig::update_usage_check(check_usage_models) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("更新 check_usage_models 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            Ok(Json(NormalResponse {
                status: ApiStatus::Success,
                data: None,
                message: Some("配置已更新".to_string()),
            }))
        }

        "reset" => {
            // 重置页面内容
            if !request.path.is_empty() {
                if let Err(e) = AppConfig::reset_page_content(&request.path) {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置页面内容失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 重置 enable_stream_check
            if request.enable_stream_check.is_some() {
                if let Err(e) = AppConfig::reset_stream_check() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置 enable_stream_check 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 重置 include_stop_stream
            if request.include_stop_stream.is_some() {
                if let Err(e) = AppConfig::reset_stop_stream() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置 include_stop_stream 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 重置 vision_ability
            if request.vision_ability.is_some() {
                if let Err(e) = AppConfig::reset_vision_ability() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置 vision_ability 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 重置 enable_slow_pool
            if request.enable_slow_pool.is_some() {
                if let Err(e) = AppConfig::reset_slow_pool() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置 enable_slow_pool 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 重置 enable_all_claude
            if request.enable_all_claude.is_some() {
                if let Err(e) = AppConfig::reset_allow_claude() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置 enable_slow_pool 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }

            // 重置 check_usage_models
            if request.check_usage_models.is_some() {
                if let Err(e) = AppConfig::reset_usage_check() {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ErrorResponse {
                            status: ApiStatus::Failed,
                            code: Some(500),
                            error: Some(format!("重置 check_usage_models 失败: {}", e)),
                            message: None,
                        }),
                    ));
                }
            }
            Ok(Json(NormalResponse {
                status: ApiStatus::Success,
                data: None,
                message: Some("配置已重置".to_string()),
            }))
        }

        _ => Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                status: ApiStatus::Failed,
                code: Some(400),
                error: Some("无效的操作类型".to_string()),
                message: None,
            }),
        )),
    }
}
