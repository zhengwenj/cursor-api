#![allow(clippy::enum_variant_names, dead_code)]

// Include the generated Protobuf code
// include!(concat!(env!("OUT_DIR"), "/aiserver.v1.rs"));
include!("v1/aiserver.v1.rs");

impl ErrorDetails {
    /// Converts an error to an appropriate HTTP status code.
    ///
    /// This method maps internal error types to standard HTTP status codes based on
    /// the nature of the error, following RESTful API best practices.
    ///
    /// Returns:
    ///   - u16: The HTTP status code corresponding to the error.
    pub fn status_code(error: i32) -> u16 {
        use error_details::Error;
        match Error::try_from(error) {
            Ok(error) => match error {
                // 400 - Bad Request: Client errors that are malformed or invalid
                Error::BadRequest
                | Error::BadModelName
                | Error::SlashEditFileTooLong
                | Error::FileUnsupported
                | Error::ClaudeImageTooLarge
                | Error::ConversationTooLong => 400,

                // 401 - Unauthorized: Authentication related errors
                Error::BadApiKey
                | Error::BadUserApiKey
                | Error::InvalidAuthId
                | Error::AuthTokenNotFound
                | Error::AuthTokenExpired
                | Error::Unauthorized
                | Error::GithubNoUserCredentials => 401,

                // 402 - Payment Required
                Error::UsagePricingRequired | Error::UsagePricingRequiredChangeable => 402,

                // 403 - Forbidden: Permission related errors
                Error::NotLoggedIn
                | Error::NotHighEnoughPermissions
                | Error::AgentRequiresLogin
                | Error::ProUserOnly
                | Error::TaskNoPermissions
                | Error::GithubUserNoAccess
                | Error::GithubAppNoAccess => 403,

                // 404 - Not Found: Resource not found errors
                Error::NotFound
                | Error::UserNotFound
                | Error::TaskUuidNotFound
                | Error::AgentEngineNotFound
                | Error::GitgraphNotFound
                | Error::FileNotFound => 404,

                // 409 - Conflict: Resource state conflicts
                Error::GithubMultipleOwners => 409,

                // 410 - Gone: Resource no longer available
                Error::Deprecated | Error::OutdatedClient => 410,

                // 422 - Unprocessable Entity: Valid request but unable to process
                Error::ApiKeyNotSupported => 422,

                // 429 - Too Many Requests: Rate limiting related errors
                Error::FreeUserRateLimitExceeded
                | Error::ProUserRateLimitExceeded
                | Error::OpenaiRateLimitExceeded
                | Error::OpenaiAccountLimitExceeded
                | Error::GenericRateLimitExceeded
                | Error::Gpt4VisionPreviewRateLimit
                | Error::ApiKeyRateLimit
                | Error::RateLimited
                | Error::RateLimitedChangeable => 429,

                // 499 - Client Closed Request (non-standard but commonly used)
                Error::UserAbortedRequest => 499,

                // 503 - Service Unavailable: Server temporarily unavailable due to overload/maintenance
                Error::FreeUserUsageLimit
                | Error::ProUserUsageLimit
                | Error::ResourceExhausted
                | Error::MaxTokens => 503,

                // 504 - Gateway Timeout
                Error::Timeout => 504,

                // 533 - Upstream Failure (non-standard): The upstream service reported a failure
                Error::Unspecified
                | Error::Openai
                | Error::CustomMessage
                | Error::Debounced
                | Error::RepositoryServiceRepositoryIsNotInitialized => 533,
            },
            // Errors not defined in the upstream enum are treated as true internal server errors
            Err(_) => 500,
        }
    }

    /// Returns the snake_case string representation of the error type.
    ///
    /// This method maps error variants to their snake_case string names,
    /// useful for logging, debugging, or API responses.
    ///
    /// Returns:
    ///   - &'static str: The snake_case name of the error type.
    pub fn r#type(error: i32) -> &'static str {
        use error_details::Error;
        match Error::try_from(error) {
            Ok(error) => match error {
                Error::Unspecified => "unspecified",
                Error::BadApiKey => "bad_api_key",
                Error::BadUserApiKey => "bad_user_api_key",
                Error::NotLoggedIn => "not_logged_in",
                Error::InvalidAuthId => "invalid_auth_id",
                Error::NotHighEnoughPermissions => "not_high_enough_permissions",
                Error::AgentRequiresLogin => "agent_requires_login",
                Error::BadModelName => "bad_model_name",
                Error::NotFound => "not_found",
                Error::Deprecated => "deprecated",
                Error::UserNotFound => "user_not_found",
                Error::FreeUserRateLimitExceeded => "free_user_rate_limit_exceeded",
                Error::ProUserRateLimitExceeded => "pro_user_rate_limit_exceeded",
                Error::FreeUserUsageLimit => "free_user_usage_limit",
                Error::ProUserUsageLimit => "pro_user_usage_limit",
                Error::ResourceExhausted => "resource_exhausted",
                Error::AuthTokenNotFound => "auth_token_not_found",
                Error::AuthTokenExpired => "auth_token_expired",
                Error::Openai => "openai",
                Error::OpenaiRateLimitExceeded => "openai_rate_limit_exceeded",
                Error::OpenaiAccountLimitExceeded => "openai_account_limit_exceeded",
                Error::TaskUuidNotFound => "task_uuid_not_found",
                Error::TaskNoPermissions => "task_no_permissions",
                Error::AgentEngineNotFound => "agent_engine_not_found",
                Error::MaxTokens => "max_tokens",
                Error::ProUserOnly => "pro_user_only",
                Error::ApiKeyNotSupported => "api_key_not_supported",
                Error::UserAbortedRequest => "user_aborted_request",
                Error::Timeout => "timeout",
                Error::GenericRateLimitExceeded => "generic_rate_limit_exceeded",
                Error::SlashEditFileTooLong => "slash_edit_file_too_long",
                Error::FileUnsupported => "file_unsupported",
                Error::Gpt4VisionPreviewRateLimit => "gpt4_vision_preview_rate_limit",
                Error::CustomMessage => "custom_message",
                Error::OutdatedClient => "outdated_client",
                Error::ClaudeImageTooLarge => "claude_image_too_large",
                Error::GitgraphNotFound => "gitgraph_not_found",
                Error::FileNotFound => "file_not_found",
                Error::ApiKeyRateLimit => "api_key_rate_limit",
                Error::Debounced => "debounced",
                Error::BadRequest => "bad_request",
                Error::RepositoryServiceRepositoryIsNotInitialized =>
                    "repository_service_repository_is_not_initialized",
                Error::Unauthorized => "unauthorized",
                Error::ConversationTooLong => "conversation_too_long",
                Error::UsagePricingRequired => "usage_pricing_required",
                Error::UsagePricingRequiredChangeable => "usage_pricing_required_changeable",
                Error::GithubNoUserCredentials => "github_no_user_credentials",
                Error::GithubUserNoAccess => "github_user_no_access",
                Error::GithubAppNoAccess => "github_app_no_access",
                Error::GithubMultipleOwners => "github_multiple_owners",
                Error::RateLimited => "rate_limited",
                Error::RateLimitedChangeable => "rate_limited_changeable",
            },
            Err(_) => crate::app::constant::UNKNOWN, // Default for unknown error types
        }
    }
}

impl CustomErrorDetails {
    #[inline]
    pub fn add(&mut self, rhs: Self) {
        #[inline(always)]
        fn add_string(a: &mut String, b: String) {
            a.reserve(b.len() + 1);
            a.push('&');
            a.push_str(&b);
        }
        add_string(&mut self.title, rhs.title);
        add_string(&mut self.detail, rhs.detail);
        // self.buttons.extend(rhs.buttons);
        self.additional_info.extend(rhs.additional_info);
    }
}
