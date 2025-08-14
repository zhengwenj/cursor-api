use ::axum::{
    Json,
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
};
use ::prost::Message as _;

use crate::{
    app::{
        constant::AUTHORIZATION_BEARER_PREFIX,
        lazy::{AUTH_TOKEN, KEY_PREFIX},
        model::{
            AppConfig, BuildKeyRequest, BuildKeyResponse, ExtToken, GetConfigVersionRequest,
            GetConfigVersionResponse, Token, UsageCheckModelType,
        },
    },
    common::utils::{to_base64, token_to_tokeninfo},
    core::{
        config::{KeyConfig, key_config},
        constant::ERR_NODATA,
    },
};

// 常量定义
const ERROR_UNAUTHORIZED: &str = "Unauthorized";
// const ERROR_NO_AUTH_TOKEN: &str = "未提供授权令牌";
// const ERROR_INVALID_TOKEN: &str = "无效令牌或无效校验和";
// const SUCCESS_CALIBRATION: &str = "校验成功";

// #[derive(::serde::Deserialize)]
// pub struct BasicCalibrationRequest {
//     key: Option<String>,
// }

// #[derive(::serde::Serialize)]
// pub struct BasicCalibrationResponse {
//     status: ApiStatus,
//     message: &'static str,
//     keys: Option<[String; 3]>,
//     bundle: Option<ExtToken>,
// }

// pub async fn handle_basic_calibration(
//     State(state): State<Arc<AppState>>,
//     headers: HeaderMap,
//     Json(request): Json<BasicCalibrationRequest>,
// ) -> Json<BasicCalibrationResponse> {
//     // 验证认证令牌
//     if headers
//         .get(AUTHORIZATION)
//         .and_then(|h| h.to_str().ok())
//         .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX))
//         .is_none_or(|h| !AppConfig::calibrate_token_eq(h) && h != *AUTH_TOKEN)
//     {
//         return Json(BasicCalibrationResponse {
//             status: ApiStatus::Error,
//             message: ERROR_NO_AUTH_TOKEN,
//             keys: None,
//             bundle: None,
//         });
//     }

//     let key = match request.key {
//         Some(key) => key,
//         None => {
//             return Json(BasicCalibrationResponse {
//                 status: ApiStatus::Error,
//                 message: ERROR_NO_AUTH_TOKEN,
//                 keys: None,
//                 bundle: None,
//             });
//         }
//     };

//     let ext_token = if let Some(key) = TokenKey::from_string(&key)
//         && let Some(bundle) = state.log_manager_lock().await.tokens().get(&key).cloned()
//     {
//         bundle
//     } else if AppConfig::get_dynamic_key()
//         && let Some(ext_token) = parse_dynamic_token(&key)
//             .and_then(|key_config| key_config.token_info)
//             .and_then(tokeninfo_to_token)
//     {
//         ext_token
//     } else {
//         return Json(BasicCalibrationResponse {
//             status: ApiStatus::Error,
//             message: ERROR_INVALID_TOKEN,
//             keys: None,
//             bundle: None,
//         });
//     };

//     // 返回校验结果
//     Json(BasicCalibrationResponse {
//         status: ApiStatus::Success,
//         message: SUCCESS_CALIBRATION,
//         keys: Some([token_to_tokeninfo(
//             *ext_token.token.raw(),
//             ext_token.checksum,
//             ext_token.client_key,
//             ext_token.config_version,
//             ext_token.session_id,
//             ext_token.proxy,
//             ext_token.timezone,
//             ext_token.gcpp_host.map(|v| v as i32),
//         )]),
//         bundle: Some(ext_token),
//     })
// }

pub async fn handle_build_key(
    headers: HeaderMap,
    Json(request): Json<BuildKeyRequest>,
) -> (StatusCode, Json<BuildKeyResponse>) {
    // 验证认证令牌
    if AppConfig::is_share() {
        let auth_header = headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX));

        if auth_header.is_none_or(|h| !AppConfig::share_token_eq(h) && h != *AUTH_TOKEN) {
            return (
                StatusCode::UNAUTHORIZED,
                Json(BuildKeyResponse::Error(ERROR_UNAUTHORIZED)),
            );
        }
    }

    let token_key = request.token.key();
    let token_info = token_to_tokeninfo(
        request.token,
        request.checksum,
        request.client_key,
        request.config_version,
        request.session_id,
        request.proxy_name,
        request.timezone,
        request.gcpp_host.map(|v| v as i32),
    );

    // 构建 proto 消息
    let key_config = KeyConfig {
        token_info: Some(token_info),
        secret: request.secret.map(|s| {
            use sha2::Digest as _;
            sha2::Sha256::new()
                .chain_update(s.as_bytes())
                .finalize()
                .to_vec()
        }),
        disable_vision: request.disable_vision,
        enable_slow_pool: request.enable_slow_pool,
        include_web_references: request.include_web_references,
        usage_check_models: if let Some(usage_check_models) = request.usage_check_models {
            Some(key_config::UsageCheckModel {
                r#type: match usage_check_models.model_type {
                    UsageCheckModelType::Default =>
                        key_config::usage_check_model::Type::Default as i32,
                    UsageCheckModelType::Disabled =>
                        key_config::usage_check_model::Type::Disabled as i32,
                    UsageCheckModelType::All => key_config::usage_check_model::Type::All as i32,
                    UsageCheckModelType::Custom =>
                        key_config::usage_check_model::Type::Custom as i32,
                },
                model_ids: if matches!(usage_check_models.model_type, UsageCheckModelType::Custom) {
                    usage_check_models
                        .model_ids
                        .iter()
                        .map(|s| s.to_string())
                        .collect()
                } else {
                    Vec::new()
                },
            })
        } else {
            None
        },
    };

    // 序列化
    let encoded = key_config.encode_to_vec();

    use crate::common::utils::string_builder;
    let key = string_builder::StringBuilder::with_capacity(2)
        .append(&**KEY_PREFIX)
        .append(to_base64(&encoded))
        .build();

    (
        StatusCode::OK,
        Json(BuildKeyResponse::Keys([
            key,
            token_key.to_string(),
            token_key.to_string2(),
        ])),
    )
}

pub async fn handle_get_config_version(
    headers: HeaderMap,
    Json(request): Json<GetConfigVersionRequest>,
) -> (StatusCode, Json<GetConfigVersionResponse>) {
    // 验证认证令牌
    if AppConfig::is_share() {
        let auth_header = headers
            .get(AUTHORIZATION)
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix(AUTHORIZATION_BEARER_PREFIX));

        if auth_header.is_none_or(|h| !AppConfig::share_token_eq(h) && h != *AUTH_TOKEN) {
            return (
                StatusCode::UNAUTHORIZED,
                Json(GetConfigVersionResponse::Error(ERROR_UNAUTHORIZED)),
            );
        }
    }

    let token = ExtToken {
        primary_token: Token::new(request.token, None),
        secondary_token: None,
        checksum: request.checksum,
        client_key: request.client_key,
        config_version: None,
        session_id: request.session_id,
        proxy: request.proxy_name,
        timezone: request.timezone.and_then(|s| {
            use ::core::str::FromStr as _;
            chrono_tz::Tz::from_str(&s).ok()
        }),
        gcpp_host: request.gcpp_host,
        user: None,
    };

    match crate::common::utils::get_server_config(token, false).await {
        Some(cv) => (
            StatusCode::OK,
            Json(GetConfigVersionResponse::ConfigVersion(cv)),
        ),
        None => (
            StatusCode::FORBIDDEN,
            Json(GetConfigVersionResponse::Error(ERR_NODATA)),
        ),
    }
}
