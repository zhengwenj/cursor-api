pub mod config;
pub mod error;
pub mod health;
pub mod token;
pub mod tri;
pub mod userinfo;

use std::borrow::Cow;

use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiStatus {
    Success,
    Error,
}

#[derive(Serialize)]
pub struct GenericError {
    // status -> 成功 / 失败
    pub status: ApiStatus,
    // HTTP 请求的状态码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<u16>,
    // HTTP 请求的错误码
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Cow<'static, str>>,
    // 错误详情
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<Cow<'static, str>>,
}
