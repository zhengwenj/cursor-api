/// .aiserver.v1.CursorPosition
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CursorPosition {
    #[prost(int32, tag = "1")]
    pub line: i32,
    #[prost(int32, tag = "2")]
    pub column: i32,
}
/// .aiserver.v1.EnvironmentInfo
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct EnvironmentInfo {
    #[prost(string, optional, tag = "1")]
    pub exthost_platform: Option<String>,
    #[prost(string, optional, tag = "2")]
    pub exthost_arch: Option<String>,
    #[prost(string, optional, tag = "3")]
    pub exthost_release: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub exthost_shell: Option<String>,
    #[prost(string, optional, tag = "5")]
    pub local_timestamp: Option<String>,
    #[prost(string, repeated, tag = "6")]
    pub workspace_uris: Vec<String>,
}
/// .aiserver.v1.SimpleRange
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SimpleRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub start_column: i32,
    #[prost(int32, tag = "3")]
    pub end_line_number_inclusive: i32,
    #[prost(int32, tag = "4")]
    pub end_column: i32,
}
/// .aiserver.v1.LineRange
#[derive(
    ::serde::Deserialize, ::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message,
)]
pub struct LineRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub end_line_number_inclusive: i32,
}
/// .aiserver.v1.CursorRange
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CursorRange {
    #[prost(message, optional, tag = "1")]
    pub start_position: Option<CursorPosition>,
    #[prost(message, optional, tag = "2")]
    pub end_position: Option<CursorPosition>,
}
/// .aiserver.v1.DetailedLine
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DetailedLine {
    #[prost(string, tag = "1")]
    pub text: String,
    #[prost(float, tag = "2")]
    pub line_number: f32,
    #[prost(bool, tag = "3")]
    pub is_signature: bool,
}
/// .aiserver.v1.CodeBlock
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CodeBlock {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(string, optional, tag = "2")]
    pub file_contents: Option<String>,
    #[prost(int32, optional, tag = "9")]
    pub file_contents_length: Option<i32>,
    #[prost(message, optional, tag = "3")]
    pub range: Option<CursorRange>,
    #[prost(string, tag = "4")]
    pub contents: String,
    #[prost(message, optional, tag = "5")]
    pub signatures: Option<code_block::Signatures>,
    #[prost(string, optional, tag = "6")]
    pub override_contents: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub original_contents: Option<String>,
    #[prost(message, repeated, tag = "8")]
    pub detailed_lines: Vec<DetailedLine>,
    #[prost(message, optional, tag = "10")]
    pub file_git_context: Option<FileGit>,
}
/// Nested message and enum types in `CodeBlock`.
pub mod code_block {
    /// .aiserver.v1.CodeBlock.Signatures
    #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    pub struct Signatures {
        #[prost(message, repeated, tag = "1")]
        pub ranges: Vec<super::CursorRange>,
    }
}
/// .aiserver.v1.GitCommit
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GitCommit {
    #[prost(string, tag = "1")]
    pub commit: String,
    #[prost(string, tag = "2")]
    pub author: String,
    #[prost(string, tag = "3")]
    pub date: String,
    #[prost(string, tag = "4")]
    pub message: String,
}
/// .aiserver.v1.FileGit
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FileGit {
    #[prost(message, repeated, tag = "1")]
    pub commits: Vec<GitCommit>,
}
/// .aiserver.v1.Diagnostic
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Diagnostic {}
/// Nested message and enum types in `Diagnostic`.
pub mod diagnostic {
    /// .aiserver.v1.Diagnostic.RelatedInformation
    #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct RelatedInformation {
        #[prost(string, tag = "1")]
        pub message: String,
        #[prost(message, optional, tag = "2")]
        pub range: Option<super::CursorRange>,
    }
    /// .aiserver.v1.Diagnostic.DiagnosticSeverity
    #[derive(
        ::serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[serde(rename_all = "snake_case")]
    #[repr(i32)]
    pub enum DiagnosticSeverity {
        Unspecified = 0,
        Error = 1,
        Warning = 2,
        Information = 3,
        Hint = 4,
    }
    pub mod diagnostic_severity {
        pub mod option {
            #[inline]
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                unsafe {
                    ::core::intrinsics::transmute_unchecked(<Option<
                        super::super::DiagnosticSeverity,
                    > as ::serde::Deserialize>::deserialize(
                        deserializer
                    ))
                }
            }
        }
    }
}
/// .aiserver.v1.CurrentFileInfo
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CurrentFileInfo {
    #[prost(int32, tag = "9")]
    pub contents_start_at_line: i32,
    #[prost(message, optional, tag = "3")]
    pub cursor_position: Option<CursorPosition>,
    #[prost(int32, tag = "8")]
    pub total_number_of_lines: i32,
    #[prost(message, optional, tag = "6")]
    pub selection: Option<CursorRange>,
}
/// .aiserver.v1.AzureState
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AzureState {
    #[prost(string, tag = "1")]
    pub api_key: String,
    #[prost(string, tag = "2")]
    pub base_url: String,
    #[prost(string, tag = "3")]
    pub deployment: String,
    #[prost(bool, tag = "4")]
    pub use_azure: bool,
}
/// .aiserver.v1.ModelDetails
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ModelDetails {
    #[prost(string, optional, tag = "1")]
    pub model_name: Option<String>,
    #[prost(message, optional, tag = "4")]
    pub azure_state: Option<AzureState>,
    #[prost(bool, optional, tag = "5")]
    pub enable_slow_pool: Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub max_mode: Option<bool>,
}
/// .aiserver.v1.LinterError
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LinterError {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(message, optional, tag = "2")]
    pub range: Option<CursorRange>,
    #[prost(string, optional, tag = "3")]
    pub source: Option<String>,
    #[prost(message, repeated, tag = "4")]
    pub related_information: Vec<diagnostic::RelatedInformation>,
    #[serde(with = "diagnostic::diagnostic_severity::option")]
    #[prost(enumeration = "diagnostic::DiagnosticSeverity", optional, tag = "5")]
    pub severity: Option<i32>,
}
/// .aiserver.v1.LinterErrors
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LinterErrors {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(message, repeated, tag = "2")]
    pub errors: Vec<LinterError>,
    #[prost(string, tag = "3")]
    pub file_contents: String,
}
/// .aiserver.v1.CursorRule
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CursorRule {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(string, optional, tag = "3")]
    pub body: Option<String>,
    #[prost(bool, optional, tag = "4")]
    pub is_from_glob: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub always_apply: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub attach_to_background_agents: Option<bool>,
}
/// .aiserver.v1.ExplicitContext
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExplicitContext {
    #[prost(string, tag = "1")]
    pub context: String,
    #[prost(string, optional, tag = "2")]
    pub repo_context: Option<String>,
    #[prost(message, repeated, tag = "3")]
    pub rules: Vec<CursorRule>,
    #[prost(string, optional, tag = "4")]
    pub mode_specific_context: Option<String>,
}
/// .aiserver.v1.ErrorDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
    #[prost(enumeration = "error_details::Error", tag = "1")]
    pub error: i32,
    #[prost(message, optional, tag = "2")]
    pub details: Option<CustomErrorDetails>,
    #[prost(bool, optional, tag = "3")]
    pub is_expected: Option<bool>,
}
/// Nested message and enum types in `ErrorDetails`.
pub mod error_details {
    /// .aiserver.v1.ErrorDetails.Error
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Error {
        Unspecified = 0,
        BadApiKey = 1,
        BadUserApiKey = 42,
        NotLoggedIn = 2,
        InvalidAuthId = 3,
        NotHighEnoughPermissions = 4,
        AgentRequiresLogin = 18,
        BadModelName = 5,
        NotFound = 39,
        Deprecated = 40,
        UserNotFound = 6,
        FreeUserRateLimitExceeded = 7,
        ProUserRateLimitExceeded = 8,
        FreeUserUsageLimit = 9,
        ProUserUsageLimit = 10,
        ResourceExhausted = 41,
        AuthTokenNotFound = 11,
        AuthTokenExpired = 12,
        Openai = 13,
        OpenaiRateLimitExceeded = 14,
        OpenaiAccountLimitExceeded = 15,
        TaskUuidNotFound = 16,
        TaskNoPermissions = 17,
        AgentEngineNotFound = 19,
        MaxTokens = 20,
        ProUserOnly = 23,
        ApiKeyNotSupported = 24,
        UserAbortedRequest = 21,
        Timeout = 25,
        GenericRateLimitExceeded = 22,
        SlashEditFileTooLong = 26,
        FileUnsupported = 27,
        Gpt4VisionPreviewRateLimit = 28,
        CustomMessage = 29,
        OutdatedClient = 30,
        ClaudeImageTooLarge = 31,
        GitgraphNotFound = 32,
        FileNotFound = 33,
        ApiKeyRateLimit = 34,
        Debounced = 35,
        BadRequest = 36,
        RepositoryServiceRepositoryIsNotInitialized = 37,
        Unauthorized = 38,
        ConversationTooLong = 43,
        UsagePricingRequired = 44,
        UsagePricingRequiredChangeable = 45,
        GithubNoUserCredentials = 46,
        GithubUserNoAccess = 47,
        GithubAppNoAccess = 48,
        GithubMultipleOwners = 49,
        RateLimited = 50,
        RateLimitedChangeable = 51,
    }
}
/// .aiserver.v1.CustomErrorDetails
#[derive(::serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CustomErrorDetails {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub detail: String,
    #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty")]
    #[prost(map = "string, string", tag = "7")]
    pub additional_info: ::std::collections::HashMap<String, String>,
}
/// .aiserver.v1.ImageProto
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ImageProto {
    #[prost(bytes = "vec", tag = "1")]
    pub data: Vec<u8>,
    #[prost(message, optional, tag = "2")]
    pub dimension: Option<image_proto::Dimension>,
    #[prost(string, tag = "3")]
    pub uuid: String,
}
/// Nested message and enum types in `ImageProto`.
pub mod image_proto {
    /// .aiserver.v1.ImageProto.Dimension
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Dimension {
        #[prost(int32, tag = "1")]
        pub width: i32,
        #[prost(int32, tag = "2")]
        pub height: i32,
    }
}
/// .aiserver.v1.ComposerExternalLink
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ComposerExternalLink {
    #[prost(string, tag = "1")]
    pub url: String,
    #[prost(string, tag = "2")]
    pub uuid: String,
}
/// .aiserver.v1.CodeChunk
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CodeChunk {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(int32, tag = "2")]
    pub start_line_number: i32,
    #[prost(string, repeated, tag = "3")]
    pub lines: Vec<String>,
    #[prost(enumeration = "code_chunk::SummarizationStrategy", optional, tag = "4")]
    pub summarization_strategy: Option<i32>,
    #[prost(string, tag = "5")]
    pub language_identifier: String,
    #[prost(enumeration = "code_chunk::Intent", optional, tag = "6")]
    pub intent: Option<i32>,
    #[prost(bool, optional, tag = "7")]
    pub is_final_version: Option<bool>,
    #[prost(bool, optional, tag = "8")]
    pub is_first_version: Option<bool>,
}
/// Nested message and enum types in `CodeChunk`.
pub mod code_chunk {
    /// .aiserver.v1.CodeChunk.Intent
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Intent {
        Unspecified = 0,
        ComposerFile = 1,
        CompressedComposerFile = 2,
    }
    /// .aiserver.v1.CodeChunk.SummarizationStrategy
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SummarizationStrategy {
        NoneUnspecified = 0,
        Summarized = 1,
        Embedded = 2,
    }
}
/// .aiserver.v1.LspSubgraphPosition
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LspSubgraphPosition {
    #[prost(int32, tag = "1")]
    pub line: i32,
    #[prost(int32, tag = "2")]
    pub character: i32,
}
/// .aiserver.v1.LspSubgraphRange
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LspSubgraphRange {
    #[prost(int32, tag = "1")]
    pub start_line: i32,
    #[prost(int32, tag = "2")]
    pub start_character: i32,
    #[prost(int32, tag = "3")]
    pub end_line: i32,
    #[prost(int32, tag = "4")]
    pub end_character: i32,
}
/// .aiserver.v1.LspSubgraphContextItem
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LspSubgraphContextItem {
    #[prost(string, optional, tag = "1")]
    pub uri: Option<String>,
    #[prost(string, tag = "2")]
    pub r#type: String,
    #[prost(string, tag = "3")]
    pub content: String,
    #[prost(message, optional, tag = "4")]
    pub range: Option<LspSubgraphRange>,
}
/// .aiserver.v1.LspSubgraphFullContext
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LspSubgraphFullContext {
    #[prost(string, tag = "1")]
    pub uri: String,
    #[prost(string, tag = "2")]
    pub symbol_name: String,
    #[prost(message, repeated, tag = "3")]
    pub positions: Vec<LspSubgraphPosition>,
    #[prost(message, repeated, tag = "4")]
    pub context_items: Vec<LspSubgraphContextItem>,
    #[prost(float, tag = "5")]
    pub score: f32,
}
/// .aiserver.v1.FSUploadFileRequest
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FsUploadFileRequest {
    #[prost(string, tag = "1")]
    pub uuid: String,
    #[prost(string, tag = "2")]
    pub relative_workspace_path: String,
    #[prost(string, tag = "3")]
    pub contents: String,
    #[prost(int32, tag = "4")]
    pub model_version: i32,
    #[prost(string, optional, tag = "5")]
    pub sha256_hash: Option<String>,
}
/// .aiserver.v1.FSUploadFileResponse
#[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FsUploadFileResponse {
    #[serde(with = "fs_upload_error_type")]
    #[prost(enumeration = "FsUploadErrorType", tag = "1")]
    pub error: i32,
}
/// .aiserver.v1.FilesyncUpdateWithModelVersion
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FilesyncUpdateWithModelVersion {
    #[prost(int32, tag = "1")]
    pub model_version: i32,
    #[prost(string, tag = "2")]
    pub relative_workspace_path: String,
    #[prost(message, repeated, tag = "3")]
    pub updates: Vec<SingleUpdateRequest>,
    #[prost(int32, tag = "4")]
    pub expected_file_length: i32,
}
/// .aiserver.v1.SingleUpdateRequest
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SingleUpdateRequest {
    #[prost(int32, tag = "1")]
    pub start_position: i32,
    #[prost(int32, tag = "2")]
    pub end_position: i32,
    #[prost(int32, tag = "3")]
    pub change_length: i32,
    #[prost(string, tag = "4")]
    pub replaced_string: String,
    #[prost(message, optional, tag = "5")]
    pub range: Option<SimpleRange>,
}
/// .aiserver.v1.FSSyncFileRequest
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct FsSyncFileRequest {
    #[prost(string, tag = "1")]
    pub uuid: String,
    #[prost(string, tag = "2")]
    pub relative_workspace_path: String,
    #[prost(int32, tag = "3")]
    pub model_version: i32,
    #[prost(message, repeated, tag = "4")]
    pub filesync_updates: Vec<FilesyncUpdateWithModelVersion>,
    #[prost(string, tag = "5")]
    pub sha256_hash: String,
}
/// .aiserver.v1.FSSyncFileResponse
#[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FsSyncFileResponse {
    #[serde(with = "fs_sync_error_type")]
    #[prost(enumeration = "FsSyncErrorType", tag = "1")]
    pub error: i32,
}
/// .aiserver.v1.CodeResult
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CodeResult {
    #[prost(message, optional, tag = "1")]
    pub code_block: Option<CodeBlock>,
    #[prost(float, tag = "2")]
    pub score: f32,
}
/// .aiserver.v1.CppIntentInfo
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CppIntentInfo {
    #[prost(string, tag = "1")]
    pub source: String,
}
/// .aiserver.v1.LspSuggestion
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LspSuggestion {
    #[prost(string, tag = "1")]
    pub label: String,
}
/// .aiserver.v1.LspSuggestedItems
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LspSuggestedItems {
    #[prost(message, repeated, tag = "1")]
    pub suggestions: Vec<LspSuggestion>,
}
/// .aiserver.v1.StreamCppRequest
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct StreamCppRequest {
    #[prost(message, optional, tag = "1")]
    pub current_file: Option<CurrentFileInfo>,
    #[prost(string, repeated, tag = "2")]
    pub diff_history: Vec<String>,
    #[prost(string, optional, tag = "3")]
    pub model_name: Option<String>,
    #[prost(message, optional, tag = "4")]
    pub linter_errors: Option<LinterErrors>,
    #[prost(message, repeated, tag = "13")]
    pub context_items: Vec<CppContextItem>,
    #[prost(string, repeated, tag = "5")]
    pub diff_history_keys: Vec<String>,
    #[prost(bool, optional, tag = "6")]
    pub give_debug_output: Option<bool>,
    #[prost(message, repeated, tag = "7")]
    pub file_diff_histories: Vec<CppFileDiffHistory>,
    #[prost(message, repeated, tag = "8")]
    pub merged_diff_histories: Vec<CppFileDiffHistory>,
    #[prost(message, repeated, tag = "9")]
    pub block_diff_patches: Vec<BlockDiffPatch>,
    #[prost(bool, optional, tag = "10")]
    pub is_nightly: Option<bool>,
    #[prost(bool, optional, tag = "11")]
    pub is_debug: Option<bool>,
    #[prost(bool, optional, tag = "12")]
    pub immediately_ack: Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub enable_more_context: Option<bool>,
    #[prost(message, repeated, tag = "14")]
    pub parameter_hints: Vec<CppParameterHint>,
    #[prost(message, repeated, tag = "15")]
    pub lsp_contexts: Vec<LspSubgraphFullContext>,
    #[prost(message, optional, tag = "16")]
    pub cpp_intent_info: Option<CppIntentInfo>,
    #[prost(string, optional, tag = "18")]
    pub workspace_id: Option<String>,
    #[prost(message, repeated, tag = "19")]
    pub additional_files: Vec<AdditionalFile>,
    #[serde(with = "stream_cpp_request::control_token::option")]
    #[prost(enumeration = "stream_cpp_request::ControlToken", optional, tag = "20")]
    pub control_token: Option<i32>,
    #[prost(double, optional, tag = "21")]
    pub client_time: Option<f64>,
    #[prost(message, repeated, tag = "22")]
    pub filesync_updates: Vec<FilesyncUpdateWithModelVersion>,
    #[prost(double, tag = "23")]
    pub time_since_request_start: f64,
    #[prost(double, tag = "24")]
    pub time_at_request_send: f64,
    #[prost(double, optional, tag = "25")]
    pub client_timezone_offset: Option<f64>,
    #[prost(message, optional, tag = "26")]
    pub lsp_suggested_items: Option<LspSuggestedItems>,
    #[prost(bool, optional, tag = "27")]
    pub supports_cpt: Option<bool>,
    #[prost(bool, optional, tag = "28")]
    pub supports_crlf_cpt: Option<bool>,
    #[prost(message, repeated, tag = "29")]
    pub code_results: Vec<CodeResult>,
}
/// Nested message and enum types in `StreamCppRequest`.
pub mod stream_cpp_request {
    /// .aiserver.v1.StreamCppRequest.ControlToken
    #[derive(
        ::serde::Deserialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[serde(rename_all = "snake_case")]
    #[repr(i32)]
    pub enum ControlToken {
        Unspecified = 0,
        Quiet = 1,
        Loud = 2,
        Op = 3,
    }
    pub mod control_token {
        pub mod option {
            #[inline]
            pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
            where
                D: ::serde::Deserializer<'de>,
            {
                <Option<super::super::ControlToken> as ::serde::Deserialize>::deserialize(
                    deserializer,
                )
                .map(|opt| opt.map(|val| val as i32))
            }
        }
    }
}
/// .aiserver.v1.StreamCppResponse
#[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct StreamCppResponse {
    #[prost(string, tag = "1")]
    pub text: String,
    #[prost(int32, optional, tag = "2")]
    pub suggestion_start_line: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub suggestion_confidence: Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub done_stream: Option<bool>,
    #[prost(string, optional, tag = "5")]
    pub debug_model_output: Option<String>,
    #[prost(string, optional, tag = "6")]
    pub debug_model_input: Option<String>,
    #[prost(string, optional, tag = "7")]
    pub debug_stream_time: Option<String>,
    #[prost(string, optional, tag = "8")]
    pub debug_total_time: Option<String>,
    #[prost(string, optional, tag = "9")]
    pub debug_ttft_time: Option<String>,
    #[prost(string, optional, tag = "10")]
    pub debug_server_timing: Option<String>,
    #[prost(message, optional, tag = "11")]
    pub range_to_replace: Option<LineRange>,
    #[prost(message, optional, tag = "12")]
    pub cursor_prediction_target: Option<stream_cpp_response::CursorPredictionTarget>,
    #[prost(bool, optional, tag = "13")]
    pub done_edit: Option<bool>,
    #[prost(message, optional, tag = "14")]
    pub model_info: Option<stream_cpp_response::ModelInfo>,
    #[prost(bool, optional, tag = "15")]
    pub begin_edit: Option<bool>,
    #[prost(bool, optional, tag = "16")]
    pub should_remove_leading_eol: Option<bool>,
    #[prost(string, optional, tag = "17")]
    pub binding_id: Option<String>,
}
/// Nested message and enum types in `StreamCppResponse`.
pub mod stream_cpp_response {
    /// .aiserver.v1.StreamCppResponse.CursorPredictionTarget
    #[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CursorPredictionTarget {
        #[prost(string, tag = "1")]
        pub relative_path: String,
        #[prost(int32, tag = "2")]
        pub line_number_one_indexed: i32,
        #[prost(string, tag = "3")]
        pub expected_content: String,
        #[prost(bool, tag = "4")]
        pub should_retrigger_cpp: bool,
    }
    /// .aiserver.v1.StreamCppResponse.ModelInfo
    #[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ModelInfo {
        #[prost(bool, tag = "1")]
        pub is_fused_cursor_prediction_model: bool,
        #[prost(bool, tag = "2")]
        pub is_multidiff_model: bool,
    }
}
/// .aiserver.v1.CppConfigRequest
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CppConfigRequest {
    #[prost(bool, optional, tag = "1")]
    pub is_nightly: Option<bool>,
    #[prost(string, tag = "2")]
    pub model: String,
    #[prost(bool, optional, tag = "3")]
    pub supports_cpt: Option<bool>,
}
/// .aiserver.v1.CppConfigResponse
#[derive(::serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CppConfigResponse {
    #[prost(int32, optional, tag = "1")]
    pub above_radius: Option<i32>,
    #[prost(int32, optional, tag = "2")]
    pub below_radius: Option<i32>,
    #[prost(message, optional, tag = "4")]
    pub merge_behavior: Option<cpp_config_response::MergeBehavior>,
    #[prost(bool, optional, tag = "5")]
    pub is_on: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_ghost_text: Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub should_let_user_enable_cpp_even_if_not_pro: Option<bool>,
    #[serde(with = "cpp_config_response::heuristic")]
    #[prost(enumeration = "cpp_config_response::Heuristic", repeated, tag = "8")]
    pub heuristics: Vec<i32>,
    #[prost(string, repeated, tag = "9")]
    pub exclude_recently_viewed_files_patterns: Vec<String>,
    #[prost(bool, tag = "10")]
    pub enable_rvf_tracking: bool,
    #[prost(int32, tag = "11")]
    pub global_debounce_duration_millis: i32,
    #[prost(int32, tag = "12")]
    pub client_debounce_duration_millis: i32,
    #[prost(string, tag = "13")]
    pub cpp_url: String,
    #[prost(bool, tag = "14")]
    pub use_whitespace_diff_history: bool,
    #[prost(message, optional, tag = "15")]
    pub import_prediction_config: Option<cpp_config_response::ImportPredictionConfig>,
    #[prost(bool, tag = "16")]
    pub enable_filesync_debounce_skipping: bool,
    #[prost(float, tag = "17")]
    pub check_filesync_hash_percent: f32,
    #[prost(string, tag = "18")]
    pub geo_cpp_backend_url: String,
    #[prost(message, optional, tag = "19")]
    pub recently_rejected_edit_thresholds:
        Option<cpp_config_response::RecentlyRejectedEditThresholds>,
    #[prost(bool, tag = "20")]
    pub is_fused_cursor_prediction_model: bool,
    #[prost(bool, tag = "21")]
    pub include_unchanged_lines: bool,
    #[prost(bool, tag = "22")]
    pub should_fetch_rvf_text: bool,
    #[prost(int32, optional, tag = "23")]
    pub max_number_of_cleared_suggestions_since_last_accept: Option<i32>,
    #[prost(message, optional, tag = "24")]
    pub suggestion_hint_config: Option<cpp_config_response::SuggestionHintConfig>,
    #[prost(bool, tag = "25")]
    pub allows_tab_chunks: bool,
    #[prost(int32, optional, tag = "26")]
    pub tab_context_refresh_debounce_ms: Option<i32>,
    #[prost(int32, optional, tag = "27")]
    pub tab_context_refresh_editor_change_debounce_ms: Option<i32>,
}
/// Nested message and enum types in `CppConfigResponse`.
pub mod cpp_config_response {
    /// .aiserver.v1.CppConfigResponse.ImportPredictionConfig
    #[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ImportPredictionConfig {
        #[prost(bool, tag = "1")]
        pub is_disabled_by_backend: bool,
        #[prost(bool, tag = "2")]
        pub should_turn_on_automatically: bool,
        #[prost(bool, tag = "3")]
        pub python_enabled: bool,
    }
    /// .aiserver.v1.CppConfigResponse.MergeBehavior
    #[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct MergeBehavior {
        #[prost(string, tag = "1")]
        pub r#type: String,
        #[prost(int32, optional, tag = "2")]
        pub limit: Option<i32>,
        #[prost(int32, optional, tag = "3")]
        pub radius: Option<i32>,
    }
    /// .aiserver.v1.CppConfigResponse.RecentlyRejectedEditThresholds
    #[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct RecentlyRejectedEditThresholds {
        #[prost(int32, tag = "1")]
        pub hard_reject_threshold: i32,
        #[prost(int32, tag = "2")]
        pub soft_reject_threshold: i32,
    }
    /// .aiserver.v1.CppConfigResponse.SuggestionHintConfig
    #[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SuggestionHintConfig {
        #[prost(string, repeated, tag = "1")]
        pub important_lsp_extensions: Vec<String>,
        #[prost(string, repeated, tag = "2")]
        pub enabled_for_path_extensions: Vec<String>,
    }
    /// .aiserver.v1.CppConfigResponse.Heuristic
    #[derive(
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[serde(rename_all = "snake_case")]
    #[repr(i32)]
    pub enum Heuristic {
        Unspecified = 0,
        LotsOfAddedText = 1,
        DuplicatingLineAfterSuggestion = 2,
        DuplicatingMultipleLinesAfterSuggestion = 3,
        RevertingUserChange = 4,
        OutputExtendsBeyondRangeAndIsRepeated = 5,
        SuggestingRecentlyRejectedEdit = 6,
    }
    pub mod heuristic {
        #[inline]
        pub fn serialize<S>(value: &[i32], serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Vec<super::Heuristic> as ::serde::Serialize>::serialize(
                &value
                    .iter()
                    .map(|val| super::Heuristic::try_from(*val).unwrap_or_default())
                    .collect(),
                serializer,
            )
        }
    }
}
/// .aiserver.v1.AdditionalFile
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct AdditionalFile {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(bool, tag = "2")]
    pub is_open: bool,
    #[prost(string, repeated, tag = "3")]
    pub visible_range_content: Vec<String>,
    #[prost(double, optional, tag = "4")]
    pub last_viewed_at: Option<f64>,
    #[prost(int32, repeated, tag = "5")]
    pub start_line_number_one_indexed: Vec<i32>,
    #[prost(message, repeated, tag = "6")]
    pub visible_ranges: Vec<LineRange>,
}
/// .aiserver.v1.AvailableCppModelsResponse
#[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AvailableCppModelsResponse {
    #[prost(string, repeated, tag = "1")]
    pub models: Vec<String>,
    #[prost(string, optional, tag = "2")]
    pub default_model: Option<String>,
}
/// .aiserver.v1.CppFileDiffHistory
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CppFileDiffHistory {
    #[prost(string, tag = "1")]
    pub file_name: String,
    #[prost(string, repeated, tag = "2")]
    pub diff_history: Vec<String>,
    #[prost(double, repeated, tag = "3")]
    pub diff_history_timestamps: Vec<f64>,
}
/// .aiserver.v1.CppContextItem
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CppContextItem {
    #[prost(string, tag = "1")]
    pub contents: String,
    #[prost(string, optional, tag = "2")]
    pub symbol: Option<String>,
    #[prost(string, tag = "3")]
    pub relative_workspace_path: String,
    #[prost(float, tag = "4")]
    pub score: f32,
}
/// .aiserver.v1.CppParameterHint
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CppParameterHint {
    #[prost(string, tag = "1")]
    pub label: String,
    #[prost(string, optional, tag = "2")]
    pub documentation: Option<String>,
}
/// .aiserver.v1.IRange
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct IRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub start_column: i32,
    #[prost(int32, tag = "3")]
    pub end_line_number: i32,
    #[prost(int32, tag = "4")]
    pub end_column: i32,
}
/// .aiserver.v1.BlockDiffPatch
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct BlockDiffPatch {
    #[prost(message, optional, tag = "1")]
    pub start_model_window: Option<block_diff_patch::ModelWindow>,
    #[prost(message, repeated, tag = "3")]
    pub changes: Vec<block_diff_patch::Change>,
    #[prost(string, tag = "4")]
    pub relative_path: String,
    #[prost(string, tag = "7")]
    pub model_uuid: String,
    #[prost(int32, tag = "5")]
    pub start_from_change_index: i32,
}
/// Nested message and enum types in `BlockDiffPatch`.
pub mod block_diff_patch {
    /// .aiserver.v1.BlockDiffPatch.Change
    #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Change {
        #[prost(string, tag = "1")]
        pub text: String,
        #[prost(message, optional, tag = "2")]
        pub range: Option<super::IRange>,
    }
    /// .aiserver.v1.BlockDiffPatch.ModelWindow
    #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ModelWindow {
        #[prost(string, repeated, tag = "1")]
        pub lines: Vec<String>,
        #[prost(int32, tag = "2")]
        pub start_line_number: i32,
        #[prost(int32, tag = "3")]
        pub end_line_number: i32,
    }
}
/// .aiserver.v1.FetchRulesParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FetchRulesParams {
    #[prost(string, repeated, tag = "1")]
    pub rule_names: Vec<String>,
}
/// .aiserver.v1.FetchRulesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRulesResult {
    #[prost(message, repeated, tag = "1")]
    pub rules: Vec<CursorRule>,
}
/// .aiserver.v1.ToolResultError
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ToolResultError {
    #[prost(string, tag = "1")]
    pub client_visible_error_message: String,
    #[prost(string, tag = "2")]
    pub model_visible_error_message: String,
    #[prost(string, optional, tag = "3")]
    pub actual_error_message_only_send_from_client_to_server_never_the_other_way_around_because_that_may_be_a_security_risk:
        Option<String>,
    #[prost(oneof = "tool_result_error::ErrorDetails", tags = "5, 6")]
    pub error_details: Option<tool_result_error::ErrorDetails>,
}
/// Nested message and enum types in `ToolResultError`.
pub mod tool_result_error {
    /// .aiserver.v1.ToolResultError.EditFileError
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct EditFileError {
        #[prost(int32, tag = "1")]
        pub num_lines_in_file_before_edit: i32,
    }
    /// .aiserver.v1.ToolResultError.SearchReplaceError
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SearchReplaceError {
        #[prost(int32, tag = "1")]
        pub num_lines_in_file_before_edit: i32,
    }
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Oneof)]
    pub enum ErrorDetails {
        #[prost(message, tag = "5")]
        EditFileErrorDetails(EditFileError),
        #[prost(message, tag = "6")]
        SearchReplaceErrorDetails(SearchReplaceError),
    }
}
/// .aiserver.v1.ClientSideToolV2Call
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSideToolV2Call {
    #[prost(enumeration = "ClientSideToolV2", tag = "1")]
    pub tool: i32,
    #[prost(string, tag = "3")]
    pub tool_call_id: String,
    #[prost(double, optional, tag = "6")]
    pub timeout_ms: Option<f64>,
    #[prost(string, tag = "9")]
    pub name: String,
    #[prost(bool, tag = "14")]
    pub is_streaming: bool,
    #[prost(bool, tag = "15")]
    pub is_last_message: bool,
    #[prost(bool, tag = "51")]
    pub internal: bool,
    #[prost(string, tag = "10")]
    pub raw_args: String,
    #[prost(uint32, optional, tag = "48")]
    pub tool_index: Option<u32>,
    #[prost(string, optional, tag = "49")]
    pub model_call_id: Option<String>,
    #[prost(oneof = "client_side_tool_v2_call::Params", tags = "24, 26, 27, 33")]
    pub params: Option<client_side_tool_v2_call::Params>,
}
/// Nested message and enum types in `ClientSideToolV2Call`.
pub mod client_side_tool_v2_call {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Params {
        #[prost(message, tag = "24")]
        FetchRulesParams(super::FetchRulesParams),
        #[prost(message, tag = "26")]
        WebSearchParams(super::WebSearchParams),
        #[prost(message, tag = "27")]
        McpParams(super::McpParams),
        #[prost(message, tag = "33")]
        KnowledgeBaseParams(super::KnowledgeBaseParams),
    }
}
/// .aiserver.v1.ClientSideToolV2Result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientSideToolV2Result {
    #[prost(enumeration = "ClientSideToolV2", tag = "1")]
    pub tool: i32,
    #[prost(string, tag = "35")]
    pub tool_call_id: String,
    #[prost(message, optional, tag = "8")]
    pub error: Option<ToolResultError>,
    #[prost(string, optional, tag = "48")]
    pub model_call_id: Option<String>,
    #[prost(uint32, optional, tag = "49")]
    pub tool_index: Option<u32>,
    #[prost(message, optional, tag = "50")]
    pub attachments: Option<ToolResultAttachments>,
    #[prost(oneof = "client_side_tool_v2_result::Result", tags = "25, 27, 28, 34")]
    pub result: Option<client_side_tool_v2_result::Result>,
}
/// Nested message and enum types in `ClientSideToolV2Result`.
pub mod client_side_tool_v2_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
        #[prost(message, tag = "25")]
        FetchRulesResult(super::FetchRulesResult),
        #[prost(message, tag = "27")]
        WebSearchResult(super::WebSearchResult),
        #[prost(message, tag = "28")]
        McpResult(super::McpResult),
        #[prost(message, tag = "34")]
        KnowledgeBaseResult(super::KnowledgeBaseResult),
    }
}
/// .aiserver.v1.NudgeMessage
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct NudgeMessage {
    #[prost(string, tag = "1")]
    pub raw_message: String,
}
/// .aiserver.v1.ToolResultAttachments
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolResultAttachments {
    #[prost(message, repeated, tag = "1")]
    pub original_todos: Vec<TodoItem>,
    #[prost(message, repeated, tag = "2")]
    pub updated_todos: Vec<TodoItem>,
    #[prost(message, repeated, tag = "3")]
    pub nudge_messages: Vec<NudgeMessage>,
    #[prost(bool, tag = "4")]
    pub should_show_todo_write_reminder: bool,
    #[prost(enumeration = "tool_result_attachments::TodoReminderType", tag = "5")]
    pub todo_reminder_type: i32,
    #[prost(message, optional, tag = "6")]
    pub discovery_budget_reminder: Option<tool_result_attachments::DiscoveryBudgetReminder>,
}
/// Nested message and enum types in `ToolResultAttachments`.
pub mod tool_result_attachments {
    /// .aiserver.v1.ToolResultAttachments.DiscoveryBudgetReminder
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DiscoveryBudgetReminder {
        #[prost(int32, tag = "1")]
        pub discovery_rounds_remaining: i32,
        #[prost(string, optional, tag = "2")]
        pub discovery_effort: Option<String>,
    }
    /// .aiserver.v1.ToolResultAttachments.TodoReminderType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TodoReminderType {
        Unspecified = 0,
        Every10Turns = 1,
        AfterEdit = 2,
    }
}
/// .aiserver.v1.WebSearchParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WebSearchParams {
    #[prost(string, tag = "1")]
    pub search_term: String,
}
/// .aiserver.v1.WebSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebSearchResult {
    #[prost(message, repeated, tag = "1")]
    pub references: Vec<web_search_result::WebReference>,
    #[prost(bool, optional, tag = "2")]
    pub is_final: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub rejected: Option<bool>,
}
/// Nested message and enum types in `WebSearchResult`.
pub mod web_search_result {
    /// .aiserver.v1.WebSearchResult.WebReference
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct WebReference {
        #[prost(string, tag = "1")]
        pub title: String,
        #[prost(string, tag = "2")]
        pub url: String,
        #[prost(string, tag = "3")]
        pub chunk: String,
    }
}
/// .aiserver.v1.MCPParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct McpParams {
    #[prost(message, repeated, tag = "1")]
    pub tools: Vec<mcp_params::Tool>,
}
/// Nested message and enum types in `MCPParams`.
pub mod mcp_params {
    /// .aiserver.v1.MCPParams.Tool
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Tool {
        #[prost(string, tag = "1")]
        pub name: String,
        #[prost(string, tag = "2")]
        pub description: String,
        #[prost(string, tag = "3")]
        pub parameters: String,
        #[prost(string, tag = "4")]
        pub server_name: String,
    }
}
/// .aiserver.v1.MCPResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct McpResult {
    #[prost(string, tag = "1")]
    pub selected_tool: String,
    #[prost(string, tag = "2")]
    pub result: String,
}
/// .aiserver.v1.KnowledgeBaseParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct KnowledgeBaseParams {
    #[prost(string, tag = "1")]
    pub knowledge_to_store: String,
    #[prost(string, tag = "2")]
    pub title: String,
    #[prost(string, optional, tag = "3")]
    pub existing_knowledge_id: Option<String>,
    #[prost(string, optional, tag = "4")]
    pub action: Option<String>,
}
/// .aiserver.v1.KnowledgeBaseResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct KnowledgeBaseResult {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(string, tag = "2")]
    pub confirmation_message: String,
    #[prost(string, tag = "3")]
    pub id: String,
}
/// .aiserver.v1.TodoItem
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TodoItem {
    #[prost(string, tag = "1")]
    pub content: String,
    #[prost(string, tag = "2")]
    pub status: String,
    #[prost(string, tag = "3")]
    pub id: String,
    #[prost(string, repeated, tag = "4")]
    pub dependencies: Vec<String>,
}
/// .aiserver.v1.ComposerCapabilityRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposerCapabilityRequest {
    #[prost(
        enumeration = "composer_capability_request::ComposerCapabilityType",
        tag = "1"
    )]
    pub r#type: i32,
    #[prost(
        oneof = "composer_capability_request::Data",
        tags = "2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14"
    )]
    pub data: Option<composer_capability_request::Data>,
}
/// Nested message and enum types in `ComposerCapabilityRequest`.
pub mod composer_capability_request {
    /// .aiserver.v1.ComposerCapabilityRequest.ToolSchema
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToolSchema {
        #[prost(enumeration = "ToolType", tag = "1")]
        pub r#type: i32,
        #[prost(string, tag = "2")]
        pub name: String,
        #[prost(map = "string, message", tag = "3")]
        pub properties: ::std::collections::HashMap<String, SchemaProperty>,
        #[prost(string, repeated, tag = "4")]
        pub required: Vec<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.SchemaProperty
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct SchemaProperty {
        #[prost(string, tag = "1")]
        pub r#type: String,
        #[prost(string, optional, tag = "2")]
        pub description: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.LoopOnLintsCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct LoopOnLintsCapability {
        #[prost(message, repeated, tag = "1")]
        pub linter_errors: Vec<super::LinterErrors>,
        #[prost(string, optional, tag = "2")]
        pub custom_instructions: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.LoopOnTestsCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct LoopOnTestsCapability {
        #[prost(string, repeated, tag = "1")]
        pub test_names: Vec<String>,
        #[prost(string, optional, tag = "2")]
        pub custom_instructions: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.MegaPlannerCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct MegaPlannerCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.LoopOnCommandCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct LoopOnCommandCapability {
        #[prost(string, tag = "1")]
        pub command: String,
        #[prost(string, optional, tag = "2")]
        pub custom_instructions: Option<String>,
        #[prost(string, optional, tag = "3")]
        pub output: Option<String>,
        #[prost(int32, optional, tag = "4")]
        pub exit_code: Option<i32>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.ToolCallCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToolCallCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
        #[prost(message, repeated, tag = "2")]
        pub tool_schemas: Vec<ToolSchema>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct DiffReviewCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
        #[prost(message, repeated, tag = "2")]
        pub diffs: Vec<diff_review_capability::SimpleFileDiff>,
    }
    /// Nested message and enum types in `DiffReviewCapability`.
    pub mod diff_review_capability {
        /// .aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability.SimpleFileDiff
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct SimpleFileDiff {
            #[prost(string, tag = "1")]
            pub relative_workspace_path: String,
            #[prost(message, repeated, tag = "3")]
            pub chunks: Vec<simple_file_diff::Chunk>,
        }
        /// Nested message and enum types in `SimpleFileDiff`.
        pub mod simple_file_diff {
            /// .aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability.SimpleFileDiff.Chunk
            #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
            pub struct Chunk {
                #[prost(string, repeated, tag = "1")]
                pub old_lines: Vec<String>,
                #[prost(string, repeated, tag = "2")]
                pub new_lines: Vec<String>,
                #[prost(message, optional, tag = "3")]
                pub old_range: Option<super::super::super::LineRange>,
                #[prost(message, optional, tag = "4")]
                pub new_range: Option<super::super::super::LineRange>,
            }
        }
    }
    /// .aiserver.v1.ComposerCapabilityRequest.DecomposerCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct DecomposerCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.ContextPickingCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContextPickingCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
        #[prost(string, repeated, tag = "2")]
        pub potential_context_files: Vec<String>,
        #[prost(message, repeated, tag = "3")]
        pub potential_context_code_chunks: Vec<super::CodeChunk>,
        #[prost(string, repeated, tag = "4")]
        pub files_in_context: Vec<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.EditTrailCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct EditTrailCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.AutoContextCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct AutoContextCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
        #[prost(string, repeated, tag = "2")]
        pub additional_files: Vec<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.ContextPlannerCapability
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ContextPlannerCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
        #[prost(message, repeated, tag = "2")]
        pub attached_code_chunks: Vec<super::CodeChunk>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.RememberThisCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct RememberThisCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
        #[prost(string, tag = "2")]
        pub memory: String,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.CursorRulesCapability
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CursorRulesCapability {
        #[prost(string, optional, tag = "1")]
        pub custom_instructions: Option<String>,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.ComposerCapabilityType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ComposerCapabilityType {
        Unspecified = 0,
        LoopOnLints = 1,
        LoopOnTests = 2,
        MegaPlanner = 3,
        LoopOnCommand = 4,
        ToolCall = 5,
        DiffReview = 6,
        ContextPicking = 7,
        EditTrail = 8,
        AutoContext = 9,
        ContextPlanner = 10,
        DiffHistory = 11,
        RememberThis = 12,
        Decomposer = 13,
        UsesCodebase = 14,
        ToolFormer = 15,
        CursorRules = 16,
        TokenCounter = 17,
        UsageData = 18,
        Chimes = 19,
        CodeDecayTracker = 20,
        BackgroundComposer = 21,
        Summarization = 22,
        AiCodeTracking = 23,
        Queuing = 24,
        Memories = 25,
        RcpLogs = 26,
        KnowledgeFetch = 27,
        SlackIntegration = 28,
        SubComposer = 29,
        Thinking = 30,
        ContextWindow = 31,
        OnlineMetrics = 32,
    }
    /// .aiserver.v1.ComposerCapabilityRequest.ToolType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ToolType {
        Unspecified = 0,
        AddFileToContext = 1,
        Iterate = 3,
        RemoveFileFromContext = 4,
        SemanticSearchCodebase = 5,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "2")]
        LoopOnLints(LoopOnLintsCapability),
        #[prost(message, tag = "3")]
        LoopOnTests(LoopOnTestsCapability),
        #[prost(message, tag = "4")]
        MegaPlanner(MegaPlannerCapability),
        #[prost(message, tag = "5")]
        LoopOnCommand(LoopOnCommandCapability),
        #[prost(message, tag = "6")]
        ToolCall(ToolCallCapability),
        #[prost(message, tag = "7")]
        DiffReview(DiffReviewCapability),
        #[prost(message, tag = "8")]
        ContextPicking(ContextPickingCapability),
        #[prost(message, tag = "9")]
        EditTrail(EditTrailCapability),
        #[prost(message, tag = "10")]
        AutoContext(AutoContextCapability),
        #[prost(message, tag = "11")]
        ContextPlanner(ContextPlannerCapability),
        #[prost(message, tag = "12")]
        RememberThis(RememberThisCapability),
        #[prost(message, tag = "13")]
        Decomposer(DecomposerCapability),
        #[prost(message, tag = "14")]
        CursorRules(CursorRulesCapability),
    }
}
/// .aiserver.v1.StreamUnifiedChatRequestWithTools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatRequestWithTools {
    #[prost(
        oneof = "stream_unified_chat_request_with_tools::Request",
        tags = "1, 2"
    )]
    pub request: Option<stream_unified_chat_request_with_tools::Request>,
}
/// Nested message and enum types in `StreamUnifiedChatRequestWithTools`.
pub mod stream_unified_chat_request_with_tools {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Request {
        #[prost(message, tag = "1")]
        StreamUnifiedChatRequest(::prost::alloc::boxed::Box<super::StreamUnifiedChatRequest>),
        #[prost(message, tag = "2")]
        ClientSideToolV2Result(::prost::alloc::boxed::Box<super::ClientSideToolV2Result>),
    }
}
/// .aiserver.v1.StreamStart
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct StreamStart {
    #[prost(string, tag = "1")]
    pub padding: String,
}
/// .aiserver.v1.StreamUnifiedChatResponseWithTools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatResponseWithTools {
    #[prost(
        oneof = "stream_unified_chat_response_with_tools::Response",
        tags = "1, 2, 5"
    )]
    pub response: Option<stream_unified_chat_response_with_tools::Response>,
}
/// Nested message and enum types in `StreamUnifiedChatResponseWithTools`.
pub mod stream_unified_chat_response_with_tools {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Response {
        #[prost(message, tag = "1")]
        ClientSideToolV2Call(::prost::alloc::boxed::Box<super::ClientSideToolV2Call>),
        #[prost(message, tag = "2")]
        StreamUnifiedChatResponse(::prost::alloc::boxed::Box<super::StreamUnifiedChatResponse>),
        #[prost(message, tag = "5")]
        StreamStart(super::StreamStart),
    }
}
/// .aiserver.v1.WebCitation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebCitation {
    #[prost(message, repeated, tag = "1")]
    pub references: Vec<WebReference>,
}
/// .aiserver.v1.WebReference
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WebReference {
    #[prost(string, tag = "2")]
    pub title: String,
    #[prost(string, tag = "1")]
    pub url: String,
    #[prost(string, tag = "3")]
    pub chunk: String,
}
/// .aiserver.v1.StreamUnifiedChatRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatRequest {
    #[prost(message, repeated, tag = "1")]
    pub conversation: Vec<ConversationMessage>,
    #[prost(message, repeated, tag = "30")]
    pub full_conversation_headers_only: Vec<ConversationMessageHeader>,
    #[prost(message, optional, tag = "3")]
    pub explicit_context: Option<ExplicitContext>,
    #[prost(message, optional, tag = "5")]
    pub model_details: Option<ModelDetails>,
    #[prost(string, optional, tag = "8")]
    pub use_web: Option<String>,
    #[prost(message, repeated, tag = "9")]
    pub external_links: Vec<ComposerExternalLink>,
    #[prost(bool, optional, tag = "13")]
    pub should_cache: Option<bool>,
    #[prost(message, optional, tag = "15")]
    pub current_file: Option<CurrentFileInfo>,
    #[prost(bool, optional, tag = "17")]
    pub use_reference_composer_diff_prompt: Option<bool>,
    #[prost(bool, optional, tag = "19")]
    pub use_new_compression_scheme: Option<bool>,
    #[prost(bool, tag = "22")]
    pub is_chat: bool,
    #[prost(string, tag = "23")]
    pub conversation_id: String,
    #[prost(message, optional, tag = "26")]
    pub environment_info: Option<EnvironmentInfo>,
    #[prost(bool, tag = "27")]
    pub is_agentic: bool,
    #[prost(enumeration = "ClientSideToolV2", repeated, tag = "29")]
    pub supported_tools: Vec<i32>,
    #[prost(message, repeated, tag = "34")]
    pub mcp_tools: Vec<mcp_params::Tool>,
    #[prost(bool, optional, tag = "35")]
    pub use_full_inputs_context: Option<bool>,
    #[prost(bool, optional, tag = "36")]
    pub is_resume: Option<bool>,
    #[prost(bool, optional, tag = "37")]
    pub allow_model_fallbacks: Option<bool>,
    #[prost(int32, optional, tag = "38")]
    pub number_of_times_shown_fallback_model_warning: Option<i32>,
    #[prost(
        enumeration = "stream_unified_chat_request::UnifiedMode",
        optional,
        tag = "46"
    )]
    pub unified_mode: Option<i32>,
    #[prost(enumeration = "ClientSideToolV2", repeated, tag = "47")]
    pub tools_requiring_accepted_return: Vec<i32>,
    #[prost(bool, optional, tag = "48")]
    pub should_disable_tools: Option<bool>,
    #[prost(
        enumeration = "stream_unified_chat_request::ThinkingLevel",
        optional,
        tag = "49"
    )]
    pub thinking_level: Option<i32>,
    #[prost(bool, optional, tag = "51")]
    pub uses_rules: Option<bool>,
    #[prost(bool, optional, tag = "53")]
    pub mode_uses_auto_apply: Option<bool>,
    #[prost(string, optional, tag = "54")]
    pub unified_mode_name: Option<String>,
}
/// Nested message and enum types in `StreamUnifiedChatRequest`.
pub mod stream_unified_chat_request {
    /// .aiserver.v1.StreamUnifiedChatRequest.UnifiedMode
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum UnifiedMode {
        Unspecified = 0,
        Chat = 1,
        Agent = 2,
        Edit = 3,
        Custom = 4,
    }
    /// .aiserver.v1.StreamUnifiedChatRequest.ThinkingLevel
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ThinkingLevel {
        Unspecified = 0,
        Medium = 1,
        High = 2,
    }
}
/// .aiserver.v1.StreamUnifiedChatResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatResponse {
    #[prost(string, tag = "1")]
    pub text: String,
    #[prost(string, optional, tag = "5")]
    pub filled_prompt: Option<String>,
    #[prost(message, optional, tag = "11")]
    pub web_citation: Option<WebCitation>,
    #[prost(message, optional, tag = "25")]
    pub thinking: Option<conversation_message::Thinking>,
}
/// .aiserver.v1.ConversationMessageHeader
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ConversationMessageHeader {
    #[prost(string, tag = "1")]
    pub bubble_id: String,
    #[prost(string, optional, tag = "2")]
    pub server_bubble_id: Option<String>,
    #[prost(enumeration = "conversation_message::MessageType", tag = "3")]
    pub r#type: i32,
}
/// .aiserver.v1.ConversationMessage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationMessage {
    #[prost(string, tag = "1")]
    pub text: String,
    #[prost(enumeration = "conversation_message::MessageType", tag = "2")]
    pub r#type: i32,
    #[prost(message, repeated, tag = "10")]
    pub images: Vec<ImageProto>,
    #[prost(string, tag = "13")]
    pub bubble_id: String,
    #[prost(string, optional, tag = "32")]
    pub server_bubble_id: Option<String>,
    #[prost(message, repeated, tag = "18")]
    pub tool_results: Vec<conversation_message::ToolResult>,
    #[prost(bool, optional, tag = "20")]
    pub is_capability_iteration: Option<bool>,
    #[prost(message, repeated, tag = "21")]
    pub capabilities: Vec<ComposerCapabilityRequest>,
    #[prost(bool, tag = "29")]
    pub is_agentic: bool,
    #[prost(message, repeated, tag = "36")]
    pub web_references: Vec<WebReference>,
    #[prost(message, optional, tag = "45")]
    pub thinking: Option<conversation_message::Thinking>,
    #[prost(message, repeated, tag = "46")]
    pub all_thinking_blocks: Vec<conversation_message::Thinking>,
    #[prost(
        enumeration = "stream_unified_chat_request::UnifiedMode",
        optional,
        tag = "47"
    )]
    pub unified_mode: Option<i32>,
    #[prost(enumeration = "ClientSideToolV2", repeated, tag = "51")]
    pub supported_tools: Vec<i32>,
    #[prost(message, repeated, tag = "62")]
    pub external_links: Vec<ComposerExternalLink>,
    #[prost(bool, optional, tag = "63")]
    pub use_web: Option<bool>,
    #[prost(bool, optional, tag = "67")]
    pub is_simple_looping_message: Option<bool>,
}
/// Nested message and enum types in `ConversationMessage`.
pub mod conversation_message {
    /// .aiserver.v1.ConversationMessage.CodeChunk
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CodeChunk {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: String,
        #[prost(int32, tag = "2")]
        pub start_line_number: i32,
        #[prost(string, repeated, tag = "3")]
        pub lines: Vec<String>,
        #[prost(enumeration = "code_chunk::SummarizationStrategy", optional, tag = "4")]
        pub summarization_strategy: Option<i32>,
        #[prost(string, tag = "5")]
        pub language_identifier: String,
        #[prost(enumeration = "code_chunk::Intent", optional, tag = "6")]
        pub intent: Option<i32>,
        #[prost(bool, optional, tag = "7")]
        pub is_final_version: Option<bool>,
        #[prost(bool, optional, tag = "8")]
        pub is_first_version: Option<bool>,
        #[prost(bool, optional, tag = "9")]
        pub contents_are_missing: Option<bool>,
        #[prost(bool, optional, tag = "10")]
        pub is_only_included_from_folder: Option<bool>,
        #[prost(message, optional, tag = "11")]
        pub code_chunk_git_context: Option<code_chunk::CodeChunkGitContext>,
    }
    /// Nested message and enum types in `CodeChunk`.
    pub mod code_chunk {
        /// .aiserver.v1.ConversationMessage.CodeChunk.CodeChunkGitContext
        #[derive(Clone, PartialEq, ::prost::Message)]
        pub struct CodeChunkGitContext {
            #[prost(message, repeated, tag = "1")]
            pub git_info: Vec<code_chunk_git_context::CodeChunkGitInfo>,
        }
        /// Nested message and enum types in `CodeChunkGitContext`.
        pub mod code_chunk_git_context {
            /// .aiserver.v1.ConversationMessage.CodeChunk.CodeChunkGitContext.CodeChunkGitInfo
            #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
            pub struct CodeChunkGitInfo {
                #[prost(string, tag = "1")]
                pub commit: String,
                #[prost(string, tag = "2")]
                pub author: String,
                #[prost(string, tag = "3")]
                pub date: String,
                #[prost(string, tag = "4")]
                pub message: String,
            }
        }
        /// .aiserver.v1.ConversationMessage.CodeChunk.Intent
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum Intent {
            Unspecified = 0,
            ComposerFile = 1,
            CompressedComposerFile = 2,
            RecentlyViewedFile = 3,
            Outline = 4,
            MentionedFile = 5,
            CodeSelection = 6,
            AiEditedFile = 7,
            VisibleFile = 8,
            TerminalSelection = 9,
        }
        /// .aiserver.v1.ConversationMessage.CodeChunk.SummarizationStrategy
        #[derive(
            Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
        )]
        #[repr(i32)]
        pub enum SummarizationStrategy {
            NoneUnspecified = 0,
            Summarized = 1,
            Embedded = 2,
        }
    }
    /// .aiserver.v1.ConversationMessage.ToolResult
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ToolResult {
        #[prost(string, tag = "1")]
        pub tool_call_id: String,
        #[prost(string, tag = "2")]
        pub tool_name: String,
        #[prost(uint32, tag = "3")]
        pub tool_index: u32,
        #[prost(string, optional, tag = "12")]
        pub model_call_id: Option<String>,
        #[prost(string, tag = "4")]
        pub args: String,
        #[prost(string, tag = "5")]
        pub raw_args: String,
        #[prost(message, repeated, tag = "6")]
        pub attached_code_chunks: Vec<CodeChunk>,
        #[prost(string, optional, tag = "7")]
        pub content: Option<String>,
        #[prost(message, optional, tag = "8")]
        pub result: Option<super::ClientSideToolV2Result>,
        #[prost(message, optional, tag = "9")]
        pub error: Option<super::ToolResultError>,
        #[prost(message, repeated, tag = "10")]
        pub images: Vec<super::ImageProto>,
        #[prost(message, optional, tag = "11")]
        pub tool_call: Option<super::ClientSideToolV2Call>,
    }
    /// .aiserver.v1.ConversationMessage.Thinking
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct Thinking {
        #[prost(string, tag = "1")]
        pub text: String,
        #[prost(string, tag = "2")]
        pub signature: String,
        #[prost(string, tag = "3")]
        pub redacted_thinking: String,
        #[prost(bool, tag = "4")]
        pub is_last_thinking_chunk: bool,
    }
    /// .aiserver.v1.ConversationMessage.MessageType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum MessageType {
        Unspecified = 0,
        Human = 1,
        Ai = 2,
    }
}
/// .aiserver.v1.UsageEventDisplay
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UsageEventDisplay {
    #[prost(int64, tag = "1")]
    pub timestamp: i64,
    #[prost(string, tag = "2")]
    pub model: String,
    #[serde(with = "usage_event_kind")]
    #[prost(enumeration = "UsageEventKind", tag = "3")]
    pub kind: i32,
    #[serde(
        alias = "customSubscriptionName",
        skip_serializing_if = "Option::is_none"
    )]
    #[prost(string, optional, tag = "4")]
    pub custom_subscription_name: Option<String>,
    #[serde(alias = "maxMode", default)]
    #[prost(bool, tag = "5")]
    pub max_mode: bool,
    #[serde(alias = "requestsCosts", default)]
    #[prost(float, tag = "6")]
    pub requests_costs: f32,
    #[serde(alias = "usageBasedCosts", skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "7")]
    pub usage_based_costs: Option<String>,
    #[serde(alias = "isTokenBasedCall", skip_serializing_if = "Option::is_none")]
    #[prost(bool, optional, tag = "8")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage", skip_serializing_if = "Option::is_none")]
    #[prost(message, optional, tag = "9")]
    pub token_usage: Option<TokenUsage>,
    #[serde(alias = "owningUser", skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "10")]
    pub owning_user: Option<String>,
    #[serde(alias = "owningTeam", skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "11")]
    pub owning_team: Option<String>,
    #[serde(alias = "userEmail", skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "12")]
    pub user_email: Option<String>,
}
/// .aiserver.v1.TokenUsage
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, Copy, PartialEq, ::prost::Message)]
pub struct TokenUsage {
    #[serde(alias = "inputTokens", default)]
    #[prost(int32, tag = "1")]
    pub input_tokens: i32,
    #[serde(alias = "outputTokens", default)]
    #[prost(int32, tag = "2")]
    pub output_tokens: i32,
    #[serde(alias = "cacheWriteTokens", default)]
    #[prost(int32, tag = "3")]
    pub cache_write_tokens: i32,
    #[serde(alias = "cacheReadTokens", default)]
    #[prost(int32, tag = "4")]
    pub cache_read_tokens: i32,
    #[serde(alias = "totalCents", default)]
    #[prost(float, tag = "5")]
    pub total_cents: f32,
}
/// .aiserver.v1.AvailableModelsRequest
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AvailableModelsRequest {
    #[serde(default)]
    #[prost(bool, tag = "1")]
    pub is_nightly: bool,
    #[serde(default)]
    #[prost(bool, tag = "2")]
    pub include_long_context_models: bool,
    #[serde(default)]
    #[prost(bool, tag = "3")]
    pub exclude_max_named_models: bool,
    #[serde(default)]
    #[prost(string, repeated, tag = "4")]
    pub additional_model_names: Vec<String>,
}
/// .aiserver.v1.AvailableModelsResponse
#[derive(::serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AvailableModelsResponse {
    #[prost(message, repeated, tag = "2")]
    pub models: Vec<available_models_response::AvailableModel>,
}
/// Nested message and enum types in `AvailableModelsResponse`.
pub mod available_models_response {
    /// .aiserver.v1.AvailableModelsResponse.TooltipData
    #[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct TooltipData {
        #[prost(string, tag = "1")]
        pub primary_text: String,
        #[prost(string, tag = "2")]
        pub secondary_text: String,
        #[prost(bool, tag = "3")]
        pub secondary_warning_text: bool,
        #[prost(string, tag = "4")]
        pub icon: String,
        #[prost(string, tag = "5")]
        pub tertiary_text: String,
        #[prost(string, tag = "6")]
        pub tertiary_text_url: String,
        #[prost(string, optional, tag = "7")]
        pub markdown_content: Option<String>,
    }
    /// .aiserver.v1.AvailableModelsResponse.AvailableModel
    #[derive(::serde::Serialize, Clone, PartialEq, ::prost::Message)]
    pub struct AvailableModel {
        #[prost(string, tag = "1")]
        pub name: String,
        #[prost(bool, tag = "2")]
        pub default_on: bool,
        #[prost(bool, optional, tag = "3")]
        pub is_long_context_only: Option<bool>,
        #[prost(bool, optional, tag = "4")]
        pub is_chat_only: Option<bool>,
        #[prost(bool, optional, tag = "5")]
        pub supports_agent: Option<bool>,
        #[serde(with = "degradation_status")]
        #[prost(enumeration = "DegradationStatus", optional, tag = "6")]
        pub degradation_status: Option<i32>,
        #[prost(double, optional, tag = "7")]
        pub price: Option<f64>,
        #[prost(message, optional, tag = "8")]
        pub tooltip_data: Option<TooltipData>,
        #[prost(message, optional, tag = "20")]
        pub tooltip_data_for_max_mode: Option<TooltipData>,
        #[prost(bool, optional, tag = "9")]
        pub supports_thinking: Option<bool>,
        #[prost(bool, optional, tag = "10")]
        pub supports_images: Option<bool>,
        #[prost(bool, optional, tag = "11")]
        pub supports_auto_context: Option<bool>,
        #[prost(int32, optional, tag = "12")]
        pub auto_context_max_tokens: Option<i32>,
        #[prost(int32, optional, tag = "13")]
        pub auto_context_extended_max_tokens: Option<i32>,
        #[prost(bool, optional, tag = "14")]
        pub supports_max_mode: Option<bool>,
        #[prost(bool, optional, tag = "19")]
        pub supports_non_max_mode: Option<bool>,
        #[prost(int32, optional, tag = "15")]
        pub context_token_limit: Option<i32>,
        #[prost(int32, optional, tag = "16")]
        pub context_token_limit_for_max_mode: Option<i32>,
        #[prost(string, optional, tag = "17")]
        pub client_display_name: Option<String>,
        #[prost(string, optional, tag = "18")]
        pub server_model_name: Option<String>,
        #[prost(bool, optional, tag = "21")]
        pub is_recommended_for_background_composer: Option<bool>,
    }
    /// .aiserver.v1.AvailableModelsResponse.DegradationStatus
    #[derive(
        ::serde::Serialize,
        Clone,
        Copy,
        Debug,
        PartialEq,
        Eq,
        Hash,
        PartialOrd,
        Ord,
        ::prost::Enumeration,
    )]
    #[serde(rename_all = "snake_case")]
    #[repr(i32)]
    pub enum DegradationStatus {
        Unspecified = 0,
        Degraded = 1,
        Disabled = 2,
    }
    pub mod degradation_status {
        #[inline]
        pub fn serialize<S>(value: &Option<i32>, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: ::serde::Serializer,
        {
            <Option<super::DegradationStatus> as ::serde::Serialize>::serialize(
                &value.map(|val| super::DegradationStatus::try_from(val).unwrap_or_default()),
                serializer,
            )
        }
    }
}
/// .aiserver.v1.GetFilteredUsageEventsRequest
#[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
#[serde(rename_all = "camelCase")]
pub struct GetFilteredUsageEventsRequest {
    #[prost(int32, tag = "1")]
    pub team_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag = "2")]
    pub start_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag = "3")]
    pub end_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag = "4")]
    pub user_id: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(string, optional, tag = "5")]
    pub model_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag = "6")]
    pub page: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag = "7")]
    pub page_size: Option<i32>,
}
/// .aiserver.v1.GetFilteredUsageEventsResponse
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
#[serde(rename_all = "camelCase")]
pub struct GetFilteredUsageEventsResponse {
    #[serde(default)]
    #[prost(int32, tag = "2")]
    pub total_usage_events_count: i32,
    #[serde(default)]
    #[prost(message, repeated, tag = "3")]
    pub usage_events_display: Vec<UsageEventDisplay>,
}
/// .aiserver.v1.GetAggregatedUsageEventsRequest
#[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
#[serde(rename_all = "camelCase")]
pub struct GetAggregatedUsageEventsRequest {
    #[prost(int32, tag = "1")]
    pub team_id: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag = "2")]
    pub start_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int64, optional, tag = "3")]
    pub end_date: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[prost(int32, optional, tag = "4")]
    pub user_id: Option<i32>,
}
/// .aiserver.v1.GetAggregatedUsageEventsResponse
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
#[serde(rename_all = "camelCase")]
pub struct GetAggregatedUsageEventsResponse {
    #[serde(default)]
    #[prost(message, repeated, tag = "1")]
    pub aggregations: Vec<get_aggregated_usage_events_response::ModelUsageAggregation>,
    #[serde(default)]
    #[prost(int64, tag = "2")]
    pub total_input_tokens: i64,
    #[serde(default)]
    #[prost(int64, tag = "3")]
    pub total_output_tokens: i64,
    #[serde(default)]
    #[prost(int64, tag = "4")]
    pub total_cache_write_tokens: i64,
    #[serde(default)]
    #[prost(int64, tag = "5")]
    pub total_cache_read_tokens: i64,
    #[serde(default)]
    #[prost(double, tag = "6")]
    pub total_cost_cents: f64,
    #[serde(default)]
    #[prost(double, tag = "7")]
    pub percent_of_burst_used: f64,
}
/// Nested message and enum types in `GetAggregatedUsageEventsResponse`.
pub mod get_aggregated_usage_events_response {
    /// .aiserver.v1.GetAggregatedUsageEventsResponse.ModelUsageAggregation
    #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
    #[serde(rename_all = "camelCase")]
    pub struct ModelUsageAggregation {
        #[serde(default)]
        #[prost(string, tag = "1")]
        pub model_intent: String,
        #[serde(default)]
        #[prost(int64, tag = "2")]
        pub input_tokens: i64,
        #[serde(default)]
        #[prost(int64, tag = "3")]
        pub output_tokens: i64,
        #[serde(default)]
        #[prost(int64, tag = "4")]
        pub cache_write_tokens: i64,
        #[serde(default)]
        #[prost(int64, tag = "5")]
        pub cache_read_tokens: i64,
        #[serde(default)]
        #[prost(double, tag = "6")]
        pub total_cents: f64,
    }
}
/// .aiserver.v1.GetServerConfigResponse
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GetServerConfigResponse {
    #[prost(string, tag = "6")]
    pub config_version: String,
}
/// .aiserver.v1.FSUploadErrorType
#[derive(
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum FsUploadErrorType {
    Unspecified = 0,
    NonExistant = 1,
    HashMismatch = 2,
}
pub mod fs_upload_error_type {
    #[inline]
    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <super::FsUploadErrorType as ::serde::Serialize>::serialize(
            &super::FsUploadErrorType::try_from(*value).unwrap_or_default(),
            serializer,
        )
    }
}
/// .aiserver.v1.FSSyncErrorType
#[derive(
    ::serde::Serialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum FsSyncErrorType {
    Unspecified = 0,
    NonExistant = 1,
    HashMismatch = 2,
}
pub mod fs_sync_error_type {
    #[inline]
    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <super::FsSyncErrorType as ::serde::Serialize>::serialize(
            &super::FsSyncErrorType::try_from(*value)
                .unwrap_or(super::FsSyncErrorType::Unspecified),
            serializer,
        )
    }
}
/// .aiserver.v1.ClientSideToolV2
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ClientSideToolV2 {
    Unspecified = 0,
    ReadSemsearchFiles = 1,
    RipgrepSearch = 3,
    ReadFile = 5,
    ListDir = 6,
    EditFile = 7,
    FileSearch = 8,
    SemanticSearchFull = 9,
    DeleteFile = 11,
    Reapply = 12,
    RunTerminalCommandV2 = 15,
    FetchRules = 16,
    WebSearch = 18,
    Mcp = 19,
    SearchSymbols = 23,
    BackgroundComposerFollowup = 24,
    KnowledgeBase = 25,
    FetchPullRequest = 26,
    DeepSearch = 27,
    CreateDiagram = 28,
    FixLints = 29,
    ReadLints = 30,
    GoToDefinition = 31,
    Task = 32,
    AwaitTask = 33,
    TodoRead = 34,
    TodoWrite = 35,
    EditFileV2 = 38,
    ListDirV2 = 39,
    ReadFileV2 = 40,
    RipgrepRawSearch = 41,
    GlobFileSearch = 42,
}
/// .aiserver.v1.UsageEventKind
#[derive(
    ::serde::Serialize,
    ::serde::Deserialize,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    ::prost::Enumeration,
)]
#[serde(rename_all = "snake_case")]
#[repr(i32)]
pub enum UsageEventKind {
    #[serde(alias = "USAGE_EVENT_KIND_UNSPECIFIED")]
    Unspecified = 0,
    #[serde(alias = "USAGE_EVENT_KIND_USAGE_BASED")]
    UsageBased = 1,
    #[serde(alias = "USAGE_EVENT_KIND_USER_API_KEY")]
    UserApiKey = 2,
    #[serde(alias = "USAGE_EVENT_KIND_INCLUDED_IN_PRO")]
    IncludedInPro = 3,
    #[serde(alias = "USAGE_EVENT_KIND_INCLUDED_IN_BUSINESS")]
    IncludedInBusiness = 4,
    #[serde(alias = "USAGE_EVENT_KIND_ERRORED_NOT_CHARGED")]
    ErroredNotCharged = 5,
    #[serde(alias = "USAGE_EVENT_KIND_ABORTED_NOT_CHARGED")]
    AbortedNotCharged = 6,
    #[serde(alias = "USAGE_EVENT_KIND_CUSTOM_SUBSCRIPTION")]
    CustomSubscription = 7,
    #[serde(alias = "USAGE_EVENT_KIND_INCLUDED_IN_PRO_PLUS")]
    IncludedInProPlus = 8,
    #[serde(alias = "USAGE_EVENT_KIND_INCLUDED_IN_ULTRA")]
    IncludedInUltra = 9,
}
pub mod usage_event_kind {
    #[inline]
    pub fn serialize<S>(value: &i32, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        <super::UsageEventKind as ::serde::Serialize>::serialize(
            &super::UsageEventKind::try_from(*value).unwrap_or_default(),
            serializer,
        )
    }
    #[inline]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        unsafe {
            ::core::intrinsics::transmute_unchecked(
                <super::UsageEventKind as ::serde::Deserialize>::deserialize(deserializer),
            )
        }
    }
}
