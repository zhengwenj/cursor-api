use super::aiserver::v1::throw_error_check_request::Error as ErrorType;
use reqwest::StatusCode;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ChatError {
    pub error: ErrorBody,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorBody {
    pub code: String,
    pub message: String,
    pub details: Vec<ErrorDetail>,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorDetail {
    #[serde(rename = "type")]
    pub error_type: String,
    pub debug: ErrorDebug,
    pub value: String,
}

#[derive(Serialize, Deserialize)]
pub struct ErrorDebug {
    pub error: String,
    pub details: ErrorDetails,
    #[serde(rename = "isExpected")]
    pub is_expected: bool,
}

impl ErrorDebug {
    // pub fn is_valid(&self) -> bool {
    //     ErrorType::from_str_name(&self.error).is_some()
    // }

    pub fn status_code(&self) -> u16 {
        match ErrorType::from_str_name(&self.error) {
            Some(error) => match error {
                ErrorType::Unspecified => 500,
                ErrorType::BadApiKey
                | ErrorType::BadUserApiKey
                | ErrorType::InvalidAuthId
                | ErrorType::AuthTokenNotFound
                | ErrorType::AuthTokenExpired
                | ErrorType::Unauthorized => 401,
                ErrorType::NotLoggedIn
                | ErrorType::NotHighEnoughPermissions
                | ErrorType::AgentRequiresLogin
                | ErrorType::ProUserOnly
                | ErrorType::TaskNoPermissions => 403,
                ErrorType::NotFound
                | ErrorType::UserNotFound
                | ErrorType::TaskUuidNotFound
                | ErrorType::AgentEngineNotFound
                | ErrorType::GitgraphNotFound
                | ErrorType::FileNotFound => 404,
                ErrorType::FreeUserRateLimitExceeded
                | ErrorType::ProUserRateLimitExceeded
                | ErrorType::OpenaiRateLimitExceeded
                | ErrorType::OpenaiAccountLimitExceeded
                | ErrorType::GenericRateLimitExceeded
                | ErrorType::Gpt4VisionPreviewRateLimit
                | ErrorType::ApiKeyRateLimit => 429,
                ErrorType::BadRequest
                | ErrorType::BadModelName
                | ErrorType::SlashEditFileTooLong
                | ErrorType::FileUnsupported
                | ErrorType::ClaudeImageTooLarge => 400,
                _ => 500,
            },
            None => 500,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct ErrorDetails {
    pub title: String,
    pub detail: String,
    #[serde(rename = "isRetryable")]
    pub is_retryable: bool,
}

use crate::common::models::{ApiStatus, ErrorResponse as CommonErrorResponse};

impl ChatError {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn to_error_response(&self) -> ErrorResponse {
        if self.error.details.is_empty() {
            return ErrorResponse {
                status: 500,
                code: "ERROR_UNKNOWN".to_string(),
                error: None,
            };
        }
        ErrorResponse {
            status: self.error.details[0].debug.status_code(),
            code: self.error.details[0].debug.error.clone(),
            error: Some(Error {
                message: self.error.details[0].debug.details.title.clone(),
                details: self.error.details[0].debug.details.detail.clone(),
                value: self.error.details[0].value.clone(),
            }),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<Error>,
}

#[derive(Serialize)]
pub struct Error {
    pub message: String,
    pub details: String,
    pub value: String,
}

impl ErrorResponse {
    pub fn to_json(&self) -> serde_json::Value {
        serde_json::to_value(self).unwrap()
    }

    pub fn status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
    }

    pub fn native_code(&self) -> String {
        self.code.replace("_", " ").to_lowercase()
    }

    pub fn to_common(self) -> CommonErrorResponse {
        CommonErrorResponse {
            status: ApiStatus::Error,
            code: Some(self.status),
            error: self.error.as_ref().map(|error| error.message.clone()).or(Some(self.code.clone())),
            message: self.error.as_ref().map(|error| error.details.clone()),
        }
    }
}

pub enum StreamError {
    ChatError(ChatError),
    DataLengthLessThan5,
    EmptyMessage,
}

impl std::fmt::Display for StreamError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            StreamError::ChatError(error) => write!(f, "{}", serde_json::to_string(error).unwrap()),
            StreamError::DataLengthLessThan5 => write!(f, "data length less than 5"),
            StreamError::EmptyMessage => write!(f, "empty message"),
        }
    }
}
