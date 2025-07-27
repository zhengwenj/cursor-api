/// aiserver.v1.CursorPosition
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CursorPosition {
  #[prost(int32, tag = "1")]
  pub line: i32,
  #[prost(int32, tag = "2")]
  pub column: i32,
}
/// aiserver.v1.VscodeOSStatistics
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VscodeOsStatistics {
  #[prost(double, tag = "1")]
  pub totalmem: f64,
  #[prost(double, tag = "2")]
  pub freemem: f64,
  #[prost(double, repeated, tag = "3")]
  pub loadavg: Vec<f64>,
}
/// aiserver.v1.VscodeOSProperties
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VscodeOsProperties {
  #[prost(string, tag = "1")]
  pub r#type: String,
  #[prost(string, tag = "2")]
  pub release: String,
  #[prost(string, tag = "3")]
  pub arch: String,
  #[prost(string, tag = "4")]
  pub platform: String,
  #[prost(message, repeated, tag = "5")]
  pub cpus: Vec<VscodeCpuProperties>,
}
/// aiserver.v1.VscodeCPUProperties
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VscodeCpuProperties {
  #[prost(string, tag = "1")]
  pub model: String,
  #[prost(double, tag = "2")]
  pub speed: f64,
}
/// aiserver.v1.EnvironmentInfo
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
/// aiserver.v1.SimplestRange
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SimplestRange {
  #[prost(int32, tag = "1")]
  pub start_line: i32,
  #[prost(int32, tag = "2")]
  pub end_line_inclusive: i32,
}
/// aiserver.v1.GitDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GitDiff {
  #[prost(message, repeated, tag = "1")]
  pub diffs: Vec<FileDiff>,
  #[prost(enumeration = "git_diff::DiffType", tag = "2")]
  pub diff_type: i32,
}
/// Nested message and enum types in `GitDiff`.
pub mod git_diff {
  /// aiserver.v1.GitDiff.DiffType
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum DiffType {
    Unspecified = 0,
    DiffToHead = 1,
    DiffFromBranchToMain = 2,
  }
}
/// aiserver.v1.FileDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileDiff {
  #[prost(int32, tag = "4")]
  pub added: i32,
  #[prost(int32, tag = "5")]
  pub removed: i32,
  #[prost(string, tag = "1")]
  pub from: String,
  #[prost(string, tag = "2")]
  pub to: String,
  #[prost(message, repeated, tag = "3")]
  pub chunks: Vec<file_diff::Chunk>,
  #[prost(string, optional, tag = "6")]
  pub before_file_contents: Option<String>,
  #[prost(string, optional, tag = "7")]
  pub after_file_contents: Option<String>,
}
/// Nested message and enum types in `FileDiff`.
pub mod file_diff {
  /// aiserver.v1.FileDiff.Chunk
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct Chunk {
    #[prost(string, tag = "1")]
    pub content: String,
    #[prost(string, repeated, tag = "2")]
    pub lines: Vec<String>,
    #[prost(int32, tag = "3")]
    pub old_start: i32,
    #[prost(int32, tag = "4")]
    pub old_lines: i32,
    #[prost(int32, tag = "5")]
    pub new_start: i32,
    #[prost(int32, tag = "6")]
    pub new_lines: i32,
  }
}
/// aiserver.v1.SimpleRange
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
/// aiserver.v1.LineRange
#[derive(
  ::serde::Deserialize, ::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message,
)]
pub struct LineRange {
  #[prost(int32, tag = "1")]
  pub start_line_number: i32,
  #[prost(int32, tag = "2")]
  pub end_line_number_inclusive: i32,
}
/// aiserver.v1.CursorRange
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CursorRange {
  #[prost(message, optional, tag = "1")]
  pub start_position: Option<CursorPosition>,
  #[prost(message, optional, tag = "2")]
  pub end_position: Option<CursorPosition>,
}
/// aiserver.v1.DetailedLine
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DetailedLine {
  #[prost(string, tag = "1")]
  pub text: String,
  #[prost(float, tag = "2")]
  pub line_number: f32,
  #[prost(bool, tag = "3")]
  pub is_signature: bool,
}
/// aiserver.v1.CodeBlock
#[derive(Clone, PartialEq, ::prost::Message)]
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
  /// aiserver.v1.CodeBlock.Signatures
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct Signatures {
    #[prost(message, repeated, tag = "1")]
    pub ranges: Vec<super::CursorRange>,
  }
}
/// aiserver.v1.GitCommit
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
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
/// aiserver.v1.FileGit
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FileGit {
  #[prost(message, repeated, tag = "1")]
  pub commits: Vec<GitCommit>,
}
/// aiserver.v1.File
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct File {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, tag = "2")]
  pub contents: String,
  #[prost(message, optional, tag = "3")]
  pub file_git_context: Option<FileGit>,
}
/// aiserver.v1.Diagnostic
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct Diagnostic {
  #[prost(string, tag = "1")]
  pub message: String,
  #[prost(message, optional, tag = "2")]
  pub range: Option<CursorRange>,
  #[serde(with = "diagnostic::diagnostic_severity")]
  #[prost(enumeration = "diagnostic::DiagnosticSeverity", tag = "3")]
  pub severity: i32,
  #[prost(message, repeated, tag = "4")]
  pub related_information: Vec<diagnostic::RelatedInformation>,
}
/// Nested message and enum types in `Diagnostic`.
pub mod diagnostic {
  /// aiserver.v1.Diagnostic.RelatedInformation
  #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct RelatedInformation {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(message, optional, tag = "2")]
    pub range: Option<super::CursorRange>,
  }
  /// aiserver.v1.Diagnostic.DiagnosticSeverity
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
    #[inline]
    pub fn deserialize<'de, D>(deserializer: D) -> Result<i32, D::Error>
    where
      D: ::serde::Deserializer<'de>,
    {
      <super::DiagnosticSeverity as ::serde::Deserialize>::deserialize(deserializer)
        .map(|val| val as i32)
    }
    pub mod option {
      #[inline]
      pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<i32>, D::Error>
      where
        D: ::serde::Deserializer<'de>,
      {
        <Option<super::super::DiagnosticSeverity> as ::serde::Deserialize>::deserialize(
          deserializer,
        )
        .map(|opt| opt.map(|val| val as i32))
      }
    }
  }
}
/// aiserver.v1.BM25Chunk
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Bm25Chunk {
  #[prost(string, tag = "1")]
  pub content: String,
  #[prost(message, optional, tag = "2")]
  pub range: Option<SimplestRange>,
  #[prost(int32, tag = "3")]
  pub score: i32,
  #[prost(string, tag = "4")]
  pub relative_path: String,
}
/// aiserver.v1.CurrentFileInfo
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CurrentFileInfo {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, tag = "2")]
  pub contents: String,
  #[prost(bool, tag = "18")]
  pub rely_on_filesync: bool,
  #[prost(string, optional, tag = "17")]
  pub sha_256_hash: Option<String>,
  #[prost(message, repeated, tag = "10")]
  pub top_chunks: Vec<Bm25Chunk>,
  #[prost(int32, tag = "9")]
  pub contents_start_at_line: i32,
  #[prost(message, optional, tag = "3")]
  pub cursor_position: Option<CursorPosition>,
  #[prost(message, repeated, tag = "4")]
  pub dataframes: Vec<DataframeInfo>,
  #[prost(int32, tag = "8")]
  pub total_number_of_lines: i32,
  #[prost(string, tag = "5")]
  pub language_id: String,
  #[prost(message, optional, tag = "6")]
  pub selection: Option<CursorRange>,
  #[prost(int32, optional, tag = "11")]
  pub alternative_version_id: Option<i32>,
  #[prost(message, repeated, tag = "7")]
  pub diagnostics: Vec<Diagnostic>,
  #[prost(int32, optional, tag = "14")]
  pub file_version: Option<i32>,
  #[prost(int32, repeated, tag = "15")]
  pub cell_start_lines: Vec<i32>,
  #[prost(string, tag = "19")]
  pub workspace_root_path: String,
  #[prost(string, optional, tag = "20")]
  pub line_ending: Option<String>,
}
/// aiserver.v1.AzureState
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
/// aiserver.v1.BedrockState
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct BedrockState {
  #[prost(string, tag = "1")]
  pub access_key: String,
  #[prost(string, tag = "2")]
  pub secret_key: String,
  #[prost(string, tag = "3")]
  pub region: String,
  #[prost(bool, tag = "4")]
  pub use_bedrock: bool,
  #[prost(string, tag = "5")]
  pub session_token: String,
}
/// aiserver.v1.ModelDetails
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ModelDetails {
  #[prost(string, optional, tag = "1")]
  pub model_name: Option<String>,
  #[prost(string, optional, tag = "2")]
  pub api_key: Option<String>,
  #[prost(bool, optional, tag = "3")]
  pub enable_ghost_mode: Option<bool>,
  #[prost(message, optional, tag = "4")]
  pub azure_state: Option<AzureState>,
  #[prost(bool, optional, tag = "5")]
  pub enable_slow_pool: Option<bool>,
  #[prost(string, optional, tag = "6")]
  pub openai_api_base_url: Option<String>,
  #[prost(message, optional, tag = "7")]
  pub bedrock_state: Option<BedrockState>,
  #[prost(bool, optional, tag = "8")]
  pub max_mode: Option<bool>,
}
/// aiserver.v1.DataframeInfo
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct DataframeInfo {
  #[prost(string, tag = "1")]
  pub name: String,
  #[prost(string, tag = "2")]
  pub shape: String,
  #[prost(int32, tag = "3")]
  pub data_dimensionality: i32,
  #[prost(message, repeated, tag = "6")]
  pub columns: Vec<dataframe_info::Column>,
  #[prost(int32, tag = "7")]
  pub row_count: i32,
  #[prost(string, tag = "8")]
  pub index_column: String,
}
/// Nested message and enum types in `DataframeInfo`.
pub mod dataframe_info {
  /// aiserver.v1.DataframeInfo.Column
  #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct Column {
    #[prost(string, tag = "1")]
    pub key: String,
    #[prost(string, tag = "2")]
    pub r#type: String,
  }
}
/// aiserver.v1.LinterError
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
/// aiserver.v1.LinterErrors
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LinterErrors {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(message, repeated, tag = "2")]
  pub errors: Vec<LinterError>,
  #[prost(string, tag = "3")]
  pub file_contents: String,
}
/// aiserver.v1.LinterErrorsWithoutFileContents
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LinterErrorsWithoutFileContents {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(message, repeated, tag = "2")]
  pub errors: Vec<LinterError>,
}
/// aiserver.v1.CursorRule
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
/// aiserver.v1.ProjectLayout
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectLayout {
  #[prost(string, tag = "1")]
  pub root_path: String,
  #[prost(message, optional, tag = "2")]
  pub content: Option<ProjectLayoutDirectoryContent>,
}
/// aiserver.v1.ProjectLayoutDirectoryContent
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectLayoutDirectoryContent {
  #[prost(message, repeated, tag = "1")]
  pub directories: Vec<ProjectLayoutDirectory>,
  #[prost(message, repeated, tag = "2")]
  pub files: Vec<ProjectLayoutFile>,
  #[prost(int32, optional, tag = "3")]
  pub total_files: Option<i32>,
  #[prost(int32, optional, tag = "4")]
  pub total_subfolders: Option<i32>,
  #[prost(message, repeated, tag = "5")]
  pub hidden_files: Vec<ProjectLayoutFile>,
}
/// aiserver.v1.ProjectLayoutDirectory
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProjectLayoutDirectory {
  #[prost(string, tag = "1")]
  pub name: String,
  #[prost(message, optional, tag = "2")]
  pub content: Option<ProjectLayoutDirectoryContent>,
}
/// aiserver.v1.ProjectLayoutFile
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ProjectLayoutFile {
  #[prost(string, tag = "1")]
  pub name: String,
}
/// aiserver.v1.ExplicitContext
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
/// aiserver.v1.DocumentSymbol
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentSymbol {
  #[prost(string, tag = "1")]
  pub name: String,
  #[prost(string, tag = "2")]
  pub detail: String,
  #[prost(enumeration = "document_symbol::SymbolKind", tag = "3")]
  pub kind: i32,
  #[prost(string, tag = "5")]
  pub container_name: String,
  #[prost(message, optional, tag = "6")]
  pub range: Option<document_symbol::Range>,
  #[prost(message, optional, tag = "7")]
  pub selection_range: Option<document_symbol::Range>,
  #[prost(message, repeated, tag = "8")]
  pub children: Vec<DocumentSymbol>,
}
/// Nested message and enum types in `DocumentSymbol`.
pub mod document_symbol {
  /// aiserver.v1.DocumentSymbol.Range
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct Range {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub start_column: i32,
    #[prost(int32, tag = "3")]
    pub end_line_number: i32,
    #[prost(int32, tag = "4")]
    pub end_column: i32,
  }
  /// aiserver.v1.DocumentSymbol.SymbolKind
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum SymbolKind {
    Unspecified = 0,
    File = 1,
    Module = 2,
    Namespace = 3,
    Package = 4,
    Class = 5,
    Method = 6,
    Property = 7,
    Field = 8,
    Constructor = 9,
    Enum = 10,
    Interface = 11,
    Function = 12,
    Variable = 13,
    Constant = 14,
    String = 15,
    Number = 16,
    Boolean = 17,
    Array = 18,
    Object = 19,
    Key = 20,
    Null = 21,
    EnumMember = 22,
    Struct = 23,
    Event = 24,
    Operator = 25,
    TypeParameter = 26,
  }
}
/// aiserver.v1.ErrorDetails
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ErrorDetails {
  #[prost(enumeration = "error_details::Error", tag = "1")]
  pub error: i32,
  #[prost(message, optional, tag = "2")]
  pub details: Option<CustomErrorDetails>,
  // #[prost(bool, optional, tag = "3")]
  // pub is_expected: Option<bool>,
}
/// Nested message and enum types in `ErrorDetails`.
pub mod error_details {
  /// aiserver.v1.ErrorDetails.Error
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
/// aiserver.v1.CustomErrorDetails
#[derive(::serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct CustomErrorDetails {
  #[prost(string, tag = "1")]
  pub title: String,
  #[prost(string, tag = "2")]
  pub detail: String,
  // #[prost(bool, optional, tag = "3")]
  // pub allow_command_links_potentially_unsafe_please_only_use_for_handwritten_trusted_markdown: Option<bool>,
  // #[prost(bool, optional, tag = "4")]
  // pub is_retryable: Option<bool>,
  // #[prost(bool, optional, tag = "5")]
  // pub show_request_id: Option<bool>,
  // #[prost(bool, optional, tag = "6")]
  // pub should_show_immediate_error: Option<bool>,
  // #[prost(message, repeated, tag = "8")]
  // pub buttons: Vec<ErrorButton>,
  #[serde(skip_serializing_if = "::std::collections::HashMap::is_empty")]
  #[prost(map = "string, string", tag = "7")]
  pub additional_info: ::std::collections::HashMap<String, String>,
}
// /// aiserver.v1.ErrorButton
// #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
// pub struct ErrorButton {
//   #[prost(string, tag = "1")]
//   pub label: String,
//   #[prost(oneof = "error_button::Action", tags = "2, 3, 4, 5")]
//   pub action: Option<error_button::Action>,
// }
// /// Nested message and enum types in `ErrorButton`.
// pub mod error_button {
//   #[derive(Clone, PartialEq, Eq, Hash, ::prost::Oneof)]
//   pub enum Action {
//     #[prost(message, tag = "2")]
//     Upgrade(super::UpgradeAction),
//     #[prost(message, tag = "3")]
//     SwitchModel(super::SwitchModelAction),
//     #[prost(message, tag = "4")]
//     ConfigureSpendLimit(super::ConfigureSpendLimitAction),
//     #[prost(message, tag = "5")]
//     Url(super::UrlAction),
//   }
// }
// /// aiserver.v1.UpgradeAction
// #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
// pub struct UpgradeAction {
//   #[prost(string, tag = "1")]
//   pub membership_to_upgrade_to: String,
// }
// /// aiserver.v1.SwitchModelAction
// #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
// pub struct SwitchModelAction {}
// /// aiserver.v1.ConfigureSpendLimitAction
// #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
// pub struct ConfigureSpendLimitAction {
//   #[prost(string, tag = "1")]
//   pub confirm_label: String,
// }
// /// aiserver.v1.UrlAction
// #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
// pub struct UrlAction {
//   #[prost(string, tag = "1")]
//   pub url: String,
// }
/// aiserver.v1.ImageProto
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ImageProto {
  #[prost(bytes = "vec", tag = "1")]
  pub data: Vec<u8>,
  #[prost(message, optional, tag = "2")]
  pub dimension: Option<image_proto::Dimension>,
  #[prost(string, tag = "3")]
  pub uuid: String,
  // #[prost(string, optional, tag = "4")]
  // pub task_specific_description: Option<String>,
}
/// Nested message and enum types in `ImageProto`.
pub mod image_proto {
  /// aiserver.v1.ImageProto.Dimension
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct Dimension {
    #[prost(int32, tag = "1")]
    pub width: i32,
    #[prost(int32, tag = "2")]
    pub height: i32,
  }
}
/// aiserver.v1.ChatQuote
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ChatQuote {
  #[prost(string, tag = "1")]
  pub markdown: String,
  #[prost(string, tag = "2")]
  pub bubble_id: String,
  #[prost(int32, tag = "3")]
  pub section_index: i32,
}
/// aiserver.v1.ComposerExternalLink
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ComposerExternalLink {
  #[prost(string, tag = "1")]
  pub url: String,
  #[prost(string, tag = "2")]
  pub uuid: String,
}
/// aiserver.v1.CodeChunk
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
  /// aiserver.v1.CodeChunk.Intent
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum Intent {
    Unspecified = 0,
    ComposerFile = 1,
    CompressedComposerFile = 2,
  }
  /// aiserver.v1.CodeChunk.SummarizationStrategy
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum SummarizationStrategy {
    NoneUnspecified = 0,
    Summarized = 1,
    Embedded = 2,
  }
}
/// aiserver.v1.RCPCallFrame
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct RcpCallFrame {
  #[prost(string, optional, tag = "1")]
  pub function_name: Option<String>,
  #[prost(string, optional, tag = "2")]
  pub url: Option<String>,
  #[prost(int32, optional, tag = "3")]
  pub line_number: Option<i32>,
  #[prost(int32, optional, tag = "4")]
  pub column_number: Option<i32>,
}
/// aiserver.v1.RCPStackTrace
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcpStackTrace {
  #[prost(message, repeated, tag = "1")]
  pub call_frames: Vec<RcpCallFrame>,
  #[prost(string, optional, tag = "2")]
  pub raw_stack_trace: Option<String>,
}
/// aiserver.v1.RCPLogEntry
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RcpLogEntry {
  #[prost(string, tag = "1")]
  pub message: String,
  #[prost(double, tag = "2")]
  pub timestamp: f64,
  #[prost(string, tag = "3")]
  pub level: String,
  #[prost(string, tag = "4")]
  pub client_name: String,
  #[prost(string, tag = "5")]
  pub session_id: String,
  #[prost(message, optional, tag = "6")]
  pub stack_trace: Option<RcpStackTrace>,
  #[prost(string, optional, tag = "7")]
  pub object_data_json: Option<String>,
}
/// aiserver.v1.RCPUIElementPicked
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct RcpuiElementPicked {
  #[prost(string, tag = "1")]
  pub element: String,
  #[prost(string, tag = "2")]
  pub xpath: String,
  #[prost(string, tag = "3")]
  pub text_content: String,
  #[prost(string, tag = "4")]
  pub extra: String,
  #[prost(string, optional, tag = "5")]
  pub component: Option<String>,
  #[prost(string, optional, tag = "6")]
  pub component_props_json: Option<String>,
}
/// aiserver.v1.LspSubgraphPosition
#[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LspSubgraphPosition {
  #[prost(int32, tag = "1")]
  pub line: i32,
  #[prost(int32, tag = "2")]
  pub character: i32,
}
/// aiserver.v1.LspSubgraphRange
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
/// aiserver.v1.LspSubgraphContextItem
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
/// aiserver.v1.LspSubgraphFullContext
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
/// aiserver.v1.FSUploadFileRequest
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
/// aiserver.v1.FSUploadFileResponse
#[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FsUploadFileResponse {
  #[serde(with = "fs_upload_error_type")]
  #[prost(enumeration = "FsUploadErrorType", tag = "1")]
  pub error: i32,
}
/// aiserver.v1.FilesyncUpdateWithModelVersion
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
/// aiserver.v1.SingleUpdateRequest
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
/// aiserver.v1.FSSyncFileRequest
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
/// aiserver.v1.FSSyncFileResponse
#[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FsSyncFileResponse {
  #[serde(with = "fs_sync_error_type")]
  #[prost(enumeration = "FsSyncErrorType", tag = "1")]
  pub error: i32,
}
/// aiserver.v1.CppIntentInfo
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CppIntentInfo {
  #[prost(string, tag = "1")]
  pub source: String,
}
/// aiserver.v1.LspSuggestion
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct LspSuggestion {
  #[prost(string, tag = "1")]
  pub label: String,
}
/// aiserver.v1.LspSuggestedItems
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct LspSuggestedItems {
  #[prost(message, repeated, tag = "1")]
  pub suggestions: Vec<LspSuggestion>,
}
/// aiserver.v1.StreamCppRequest
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
}
/// Nested message and enum types in `StreamCppRequest`.
pub mod stream_cpp_request {
  /// aiserver.v1.StreamCppRequest.ControlToken
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
        <Option<super::super::ControlToken> as ::serde::Deserialize>::deserialize(deserializer)
          .map(|opt| opt.map(|val| val as i32))
      }
    }
  }
}
/// aiserver.v1.StreamCppResponse
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
  /// aiserver.v1.StreamCppResponse.CursorPredictionTarget
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
  /// aiserver.v1.StreamCppResponse.ModelInfo
  #[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ModelInfo {
    #[prost(bool, tag = "1")]
    pub is_fused_cursor_prediction_model: bool,
    #[prost(bool, tag = "2")]
    pub is_multidiff_model: bool,
  }
}
/// aiserver.v1.CppConfigRequest
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CppConfigRequest {
  #[prost(bool, optional, tag = "1")]
  pub is_nightly: Option<bool>,
  #[prost(string, tag = "2")]
  pub model: String,
  #[prost(bool, optional, tag = "3")]
  pub supports_cpt: Option<bool>,
}
/// aiserver.v1.CppConfigResponse
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
}
/// Nested message and enum types in `CppConfigResponse`.
pub mod cpp_config_response {
  /// aiserver.v1.CppConfigResponse.ImportPredictionConfig
  #[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ImportPredictionConfig {
    #[prost(bool, tag = "1")]
    pub is_disabled_by_backend: bool,
    #[prost(bool, tag = "2")]
    pub should_turn_on_automatically: bool,
    #[prost(bool, tag = "3")]
    pub python_enabled: bool,
  }
  /// aiserver.v1.CppConfigResponse.MergeBehavior
  #[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct MergeBehavior {
    #[prost(string, tag = "1")]
    pub r#type: String,
    #[prost(int32, optional, tag = "2")]
    pub limit: Option<i32>,
    #[prost(int32, optional, tag = "3")]
    pub radius: Option<i32>,
  }
  /// aiserver.v1.CppConfigResponse.RecentlyRejectedEditThresholds
  #[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct RecentlyRejectedEditThresholds {
    #[prost(int32, tag = "1")]
    pub hard_reject_threshold: i32,
    #[prost(int32, tag = "2")]
    pub soft_reject_threshold: i32,
  }
  /// aiserver.v1.CppConfigResponse.SuggestionHintConfig
  #[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct SuggestionHintConfig {
    #[prost(string, repeated, tag = "1")]
    pub important_lsp_extensions: Vec<String>,
    #[prost(string, repeated, tag = "2")]
    pub enabled_for_path_extensions: Vec<String>,
  }
  /// aiserver.v1.CppConfigResponse.Heuristic
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
/// aiserver.v1.AdditionalFile
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
/// aiserver.v1.AvailableCppModelsResponse
#[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AvailableCppModelsResponse {
  #[prost(string, repeated, tag = "1")]
  pub models: Vec<String>,
  #[prost(string, optional, tag = "2")]
  pub default_model: Option<String>,
}
/// aiserver.v1.CppFileDiffHistory
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct CppFileDiffHistory {
  #[prost(string, tag = "1")]
  pub file_name: String,
  #[prost(string, repeated, tag = "2")]
  pub diff_history: Vec<String>,
  #[prost(double, repeated, tag = "3")]
  pub diff_history_timestamps: Vec<f64>,
}
/// aiserver.v1.CppContextItem
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
/// aiserver.v1.CppParameterHint
#[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CppParameterHint {
  #[prost(string, tag = "1")]
  pub label: String,
  #[prost(string, optional, tag = "2")]
  pub documentation: Option<String>,
}
/// aiserver.v1.IRange
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
/// aiserver.v1.BlockDiffPatch
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
  /// aiserver.v1.BlockDiffPatch.Change
  #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct Change {
    #[prost(string, tag = "1")]
    pub text: String,
    #[prost(message, optional, tag = "2")]
    pub range: Option<super::IRange>,
  }
  /// aiserver.v1.BlockDiffPatch.ModelWindow
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
/// aiserver.v1.QueryOnlyRepoAccess
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct QueryOnlyRepoAccess {
  #[prost(string, tag = "1")]
  pub owner_auth_id: String,
  #[prost(string, tag = "2")]
  pub access_token: String,
  #[prost(string, tag = "3")]
  pub user_repo_owner: String,
  #[prost(string, tag = "4")]
  pub user_repo_name: String,
}
/// aiserver.v1.CodeResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CodeResult {
  #[prost(message, optional, tag = "1")]
  pub code_block: Option<CodeBlock>,
  #[prost(float, tag = "2")]
  pub score: f32,
}
/// aiserver.v1.RepositoryInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepositoryInfo {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, repeated, tag = "2")]
  pub remote_urls: Vec<String>,
  #[prost(string, repeated, tag = "3")]
  pub remote_names: Vec<String>,
  #[prost(string, tag = "4")]
  pub repo_name: String,
  #[prost(string, tag = "5")]
  pub repo_owner: String,
  #[prost(bool, tag = "6")]
  pub is_tracked: bool,
  #[prost(bool, tag = "7")]
  pub is_local: bool,
  #[prost(int32, optional, tag = "8")]
  pub num_files: Option<i32>,
  #[prost(double, optional, tag = "9")]
  pub orthogonal_transform_seed: Option<f64>,
  #[prost(enumeration = "EmbeddingModel", optional, tag = "10")]
  pub preferred_embedding_model: Option<i32>,
  #[prost(string, tag = "11")]
  pub workspace_uri: String,
  #[prost(enumeration = "DatabaseProvider", optional, tag = "12")]
  pub preferred_db_provider: Option<i32>,
}
/// aiserver.v1.ReapplyParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReapplyParams {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
}
/// aiserver.v1.ReapplyResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReapplyResult {
  #[prost(message, optional, tag = "1")]
  pub diff: Option<edit_file_result::FileDiff>,
  #[prost(bool, tag = "2")]
  pub is_applied: bool,
  #[prost(bool, tag = "3")]
  pub apply_failed: bool,
  #[prost(message, repeated, tag = "4")]
  pub linter_errors: Vec<LinterError>,
  #[prost(bool, optional, tag = "5")]
  pub rejected: Option<bool>,
}
/// aiserver.v1.FetchRulesParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FetchRulesParams {
  #[prost(string, repeated, tag = "1")]
  pub rule_names: Vec<String>,
}
/// aiserver.v1.FetchRulesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchRulesResult {
  #[prost(message, repeated, tag = "1")]
  pub rules: Vec<CursorRule>,
}
/// aiserver.v1.ReapplyStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReapplyStream {}
/// aiserver.v1.ToolResultError
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
  /// aiserver.v1.ToolResultError.EditFileError
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct EditFileError {
    #[prost(int32, tag = "1")]
    pub num_lines_in_file_before_edit: i32,
  }
  /// aiserver.v1.ToolResultError.SearchReplaceError
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
/// aiserver.v1.ClientSideToolV2Call
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
  #[prost(string, tag = "10")]
  pub raw_args: String,
  #[prost(uint32, optional, tag = "48")]
  pub tool_index: Option<u32>,
  #[prost(string, optional, tag = "49")]
  pub model_call_id: Option<String>,
  #[prost(
    oneof = "client_side_tool_v2_call::Params",
    tags = "2, 5, 8, 12, 13, 16, 17, 19, 20, 23, 24, 26, 27, 31, 41, 32, 33, 34, 35, 36, 37, 38, 42, 43, 44, 45, 50"
  )]
  pub params: Option<client_side_tool_v2_call::Params>,
}
/// Nested message and enum types in `ClientSideToolV2Call`.
pub mod client_side_tool_v2_call {
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Params {
    #[prost(message, tag = "2")]
    ReadSemsearchFilesParams(super::ReadSemsearchFilesParams),
    #[prost(message, tag = "5")]
    RipgrepSearchParams(super::RipgrepSearchParams),
    #[prost(message, tag = "8")]
    ReadFileParams(super::ReadFileParams),
    #[prost(message, tag = "12")]
    ListDirParams(super::ListDirParams),
    #[prost(message, tag = "13")]
    EditFileParams(super::EditFileParams),
    #[prost(message, tag = "16")]
    FileSearchParams(super::ToolCallFileSearchParams),
    #[prost(message, tag = "17")]
    SemanticSearchFullParams(super::SemanticSearchFullParams),
    #[prost(message, tag = "19")]
    DeleteFileParams(super::DeleteFileParams),
    #[prost(message, tag = "20")]
    ReapplyParams(super::ReapplyParams),
    #[prost(message, tag = "23")]
    RunTerminalCommandV2Params(super::RunTerminalCommandV2Params),
    #[prost(message, tag = "24")]
    FetchRulesParams(super::FetchRulesParams),
    #[prost(message, tag = "26")]
    WebSearchParams(super::WebSearchParams),
    #[prost(message, tag = "27")]
    McpParams(super::McpParams),
    #[prost(message, tag = "31")]
    SearchSymbolsParams(super::SearchSymbolsParams),
    #[prost(message, tag = "41")]
    GotodefParams(super::GotodefParams),
    #[prost(message, tag = "32")]
    BackgroundComposerFollowupParams(super::BackgroundComposerFollowupParams),
    #[prost(message, tag = "33")]
    KnowledgeBaseParams(super::KnowledgeBaseParams),
    #[prost(message, tag = "34")]
    FetchPullRequestParams(super::FetchPullRequestParams),
    #[prost(message, tag = "35")]
    DeepSearchParams(super::DeepSearchParams),
    #[prost(message, tag = "36")]
    CreateDiagramParams(super::CreateDiagramParams),
    #[prost(message, tag = "37")]
    FixLintsParams(super::FixLintsParams),
    #[prost(message, tag = "38")]
    ReadLintsParams(super::ReadLintsParams),
    #[prost(message, tag = "42")]
    TaskParams(super::TaskParams),
    #[prost(message, tag = "43")]
    AwaitTaskParams(super::AwaitTaskParams),
    #[prost(message, tag = "44")]
    TodoReadParams(super::TodoReadParams),
    #[prost(message, tag = "45")]
    TodoWriteParams(super::TodoWriteParams),
    #[prost(message, tag = "50")]
    EditFileV2Params(super::EditFileV2Params),
  }
}
/// aiserver.v1.ClientSideToolV2Result
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
  #[prost(
    oneof = "client_side_tool_v2_result::Result",
    tags = "2, 4, 6, 9, 10, 11, 18, 20, 21, 24, 25, 27, 28, 32, 33, 34, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 51"
  )]
  pub result: Option<client_side_tool_v2_result::Result>,
}
/// Nested message and enum types in `ClientSideToolV2Result`.
pub mod client_side_tool_v2_result {
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Result {
    #[prost(message, tag = "2")]
    ReadSemsearchFilesResult(super::ReadSemsearchFilesResult),
    #[prost(message, tag = "4")]
    RipgrepSearchResult(super::RipgrepSearchResult),
    #[prost(message, tag = "6")]
    ReadFileResult(super::ReadFileResult),
    #[prost(message, tag = "9")]
    ListDirResult(super::ListDirResult),
    #[prost(message, tag = "10")]
    EditFileResult(super::EditFileResult),
    #[prost(message, tag = "11")]
    FileSearchResult(super::ToolCallFileSearchResult),
    #[prost(message, tag = "18")]
    SemanticSearchFullResult(super::SemanticSearchFullResult),
    #[prost(message, tag = "20")]
    DeleteFileResult(super::DeleteFileResult),
    #[prost(message, tag = "21")]
    ReapplyResult(super::ReapplyResult),
    #[prost(message, tag = "24")]
    RunTerminalCommandV2Result(super::RunTerminalCommandV2Result),
    #[prost(message, tag = "25")]
    FetchRulesResult(super::FetchRulesResult),
    #[prost(message, tag = "27")]
    WebSearchResult(super::WebSearchResult),
    #[prost(message, tag = "28")]
    McpResult(super::McpResult),
    #[prost(message, tag = "32")]
    SearchSymbolsResult(super::SearchSymbolsResult),
    #[prost(message, tag = "33")]
    BackgroundComposerFollowupResult(super::BackgroundComposerFollowupResult),
    #[prost(message, tag = "34")]
    KnowledgeBaseResult(super::KnowledgeBaseResult),
    #[prost(message, tag = "36")]
    FetchPullRequestResult(super::FetchPullRequestResult),
    #[prost(message, tag = "37")]
    DeepSearchResult(super::DeepSearchResult),
    #[prost(message, tag = "38")]
    CreateDiagramResult(super::CreateDiagramResult),
    #[prost(message, tag = "39")]
    FixLintsResult(super::FixLintsResult),
    #[prost(message, tag = "40")]
    ReadLintsResult(super::ReadLintsResult),
    #[prost(message, tag = "41")]
    GotodefResult(super::GotodefResult),
    #[prost(message, tag = "42")]
    TaskResult(super::TaskResult),
    #[prost(message, tag = "43")]
    AwaitTaskResult(super::AwaitTaskResult),
    #[prost(message, tag = "44")]
    TodoReadResult(super::TodoReadResult),
    #[prost(message, tag = "45")]
    TodoWriteResult(super::TodoWriteResult),
    #[prost(message, tag = "51")]
    EditFileV2Result(super::EditFileV2Result),
  }
}
/// aiserver.v1.NudgeMessage
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct NudgeMessage {
  #[prost(string, tag = "1")]
  pub raw_message: String,
}
/// aiserver.v1.ToolResultAttachments
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
}
/// aiserver.v1.StreamedBackPartialToolCall
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct StreamedBackPartialToolCall {
  #[prost(enumeration = "ClientSideToolV2", tag = "1")]
  pub tool: i32,
  #[prost(string, tag = "2")]
  pub tool_call_id: String,
  #[prost(string, tag = "3")]
  pub name: String,
  #[prost(uint32, optional, tag = "4")]
  pub tool_index: Option<u32>,
  #[prost(string, optional, tag = "5")]
  pub model_call_id: Option<String>,
}
/// aiserver.v1.StreamedBackToolCall
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamedBackToolCall {
  #[prost(enumeration = "ClientSideToolV2", tag = "1")]
  pub tool: i32,
  #[prost(string, tag = "2")]
  pub tool_call_id: String,
  #[prost(string, tag = "8")]
  pub name: String,
  #[prost(string, tag = "9")]
  pub raw_args: String,
  #[prost(message, optional, tag = "10")]
  pub error: Option<ToolResultError>,
  #[prost(uint32, optional, tag = "50")]
  pub tool_index: Option<u32>,
  #[prost(string, optional, tag = "51")]
  pub model_call_id: Option<String>,
  #[prost(
    oneof = "streamed_back_tool_call::Params",
    tags = "3, 5, 7, 12, 13, 14, 19, 21, 22, 25, 26, 28, 29, 33, 41, 34, 35, 36, 37, 38, 39, 40, 42, 43, 44, 45, 52"
  )]
  pub params: Option<streamed_back_tool_call::Params>,
}
/// Nested message and enum types in `StreamedBackToolCall`.
pub mod streamed_back_tool_call {
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Params {
    #[prost(message, tag = "3")]
    ReadSemsearchFilesStream(super::ReadSemsearchFilesStream),
    #[prost(message, tag = "5")]
    RipgrepSearchStream(super::RipgrepSearchStream),
    #[prost(message, tag = "7")]
    ReadFileStream(super::ReadFileStream),
    #[prost(message, tag = "12")]
    ListDirStream(super::ListDirStream),
    #[prost(message, tag = "13")]
    EditFileStream(super::EditFileStream),
    #[prost(message, tag = "14")]
    FileSearchStream(super::ToolCallFileSearchStream),
    #[prost(message, tag = "19")]
    SemanticSearchFullStream(super::SemanticSearchFullStream),
    #[prost(message, tag = "21")]
    DeleteFileStream(super::DeleteFileStream),
    #[prost(message, tag = "22")]
    ReapplyStream(super::ReapplyStream),
    #[prost(message, tag = "25")]
    RunTerminalCommandV2Stream(super::RunTerminalCommandV2Stream),
    #[prost(message, tag = "26")]
    FetchRulesStream(super::FetchRulesStream),
    #[prost(message, tag = "28")]
    WebSearchStream(super::WebSearchStream),
    #[prost(message, tag = "29")]
    McpStream(super::McpStream),
    #[prost(message, tag = "33")]
    SearchSymbolsStream(super::SearchSymbolsStream),
    #[prost(message, tag = "41")]
    GotodefStream(super::GotodefStream),
    #[prost(message, tag = "34")]
    BackgroundComposerFollowupStream(super::BackgroundComposerFollowupStream),
    #[prost(message, tag = "35")]
    KnowledgeBaseStream(super::KnowledgeBaseStream),
    #[prost(message, tag = "36")]
    FetchPullRequestStream(super::FetchPullRequestStream),
    #[prost(message, tag = "37")]
    DeepSearchStream(super::DeepSearchStream),
    #[prost(message, tag = "38")]
    CreateDiagramStream(super::CreateDiagramStream),
    #[prost(message, tag = "39")]
    FixLintsStream(super::FixLintsStream),
    #[prost(message, tag = "40")]
    ReadLintsStream(super::ReadLintsStream),
    #[prost(message, tag = "42")]
    TaskStream(super::TaskStream),
    #[prost(message, tag = "43")]
    AwaitTaskStream(super::AwaitTaskStream),
    #[prost(message, tag = "44")]
    TodoReadStream(super::TodoReadStream),
    #[prost(message, tag = "45")]
    TodoWriteStream(super::TodoWriteStream),
    #[prost(message, tag = "52")]
    EditFileV2Stream(super::EditFileV2Stream),
  }
}
/// aiserver.v1.EditFileV2Params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFileV2Params {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, optional, tag = "2")]
  pub contents_after_edit: Option<String>,
  #[prost(bool, optional, tag = "3")]
  pub waiting_for_file_contents: Option<bool>,
  #[prost(bool, tag = "6")]
  pub should_send_back_linter_errors: bool,
  #[prost(message, optional, tag = "7")]
  pub diff: Option<edit_file_result::FileDiff>,
  #[prost(string, tag = "8")]
  pub result_for_model: String,
  #[prost(oneof = "edit_file_v2_params::StreamingEdit", tags = "4, 5")]
  pub streaming_edit: Option<edit_file_v2_params::StreamingEdit>,
}
/// Nested message and enum types in `EditFileV2Params`.
pub mod edit_file_v2_params {
  /// aiserver.v1.EditFileV2Params.StreamingEditText
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct StreamingEditText {
    #[prost(string, tag = "1")]
    pub text: String,
  }
  /// aiserver.v1.EditFileV2Params.StreamingEditCode
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct StreamingEditCode {
    #[prost(string, tag = "1")]
    pub code: String,
  }
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Oneof)]
  pub enum StreamingEdit {
    #[prost(message, tag = "4")]
    Text(StreamingEditText),
    #[prost(message, tag = "5")]
    Code(StreamingEditCode),
  }
}
/// aiserver.v1.EditFileV2Result
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFileV2Result {
  #[prost(string, optional, tag = "1")]
  pub contents_before_edit: Option<String>,
  #[prost(string, optional, tag = "9")]
  pub eol_sequence: Option<String>,
  #[prost(bool, tag = "2")]
  pub file_was_created: bool,
  #[prost(message, optional, tag = "3")]
  pub diff: Option<edit_file_result::FileDiff>,
  #[prost(bool, optional, tag = "4")]
  pub rejected: Option<bool>,
  #[prost(message, repeated, tag = "5")]
  pub linter_errors: Vec<LinterError>,
  #[prost(bool, tag = "6")]
  pub sent_back_linter_errors: bool,
  #[prost(bool, tag = "8")]
  pub should_auto_fix_lints: bool,
  #[prost(message, optional, tag = "7")]
  pub human_review_v2: Option<HumanReview>,
  #[prost(string, tag = "10")]
  pub result_for_model: String,
}
/// aiserver.v1.EditFileV2Stream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct EditFileV2Stream {}
/// aiserver.v1.EditFileParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFileParams {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, tag = "2")]
  pub language: String,
  #[prost(bool, tag = "4")]
  pub blocking: bool,
  #[prost(string, tag = "3")]
  pub contents: String,
  #[prost(string, optional, tag = "5")]
  pub instructions: Option<String>,
  #[prost(bool, optional, tag = "12")]
  pub should_edit_file_fail_for_large_files: Option<bool>,
  #[prost(string, optional, tag = "6")]
  pub old_string: Option<String>,
  #[prost(string, optional, tag = "7")]
  pub new_string: Option<String>,
  #[prost(bool, optional, tag = "8")]
  pub allow_multiple_matches: Option<bool>,
  #[prost(bool, optional, tag = "10")]
  pub use_whitespace_insensitive_fallback: Option<bool>,
  #[prost(bool, optional, tag = "11")]
  pub use_did_you_mean_fuzzy_match: Option<bool>,
  #[prost(bool, optional, tag = "16")]
  pub gracefully_handle_recoverable_errors: Option<bool>,
  #[prost(message, repeated, tag = "9")]
  pub line_ranges: Vec<LineRange>,
  #[prost(int32, optional, tag = "13")]
  pub notebook_cell_idx: Option<i32>,
  #[prost(bool, optional, tag = "14")]
  pub is_new_cell: Option<bool>,
  #[prost(string, optional, tag = "15")]
  pub cell_language: Option<String>,
  #[prost(string, optional, tag = "17")]
  pub edit_category: Option<String>,
  #[prost(bool, optional, tag = "18")]
  pub should_eagerly_process_lints: Option<bool>,
}
/// aiserver.v1.EditFileResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct EditFileResult {
  #[prost(message, optional, tag = "1")]
  pub diff: Option<edit_file_result::FileDiff>,
  #[prost(bool, tag = "2")]
  pub is_applied: bool,
  #[prost(bool, tag = "3")]
  pub apply_failed: bool,
  #[prost(message, repeated, tag = "4")]
  pub linter_errors: Vec<LinterError>,
  #[prost(bool, optional, tag = "5")]
  pub rejected: Option<bool>,
  #[prost(int32, optional, tag = "6")]
  pub num_matches: Option<i32>,
  #[prost(bool, optional, tag = "7")]
  pub whitespace_insensitive_fallback_found_match: Option<bool>,
  #[prost(bool, optional, tag = "8")]
  pub no_match_found_in_line_ranges: Option<bool>,
  #[prost(message, optional, tag = "11")]
  pub recoverable_error: Option<edit_file_result::RecoverableError>,
  #[prost(int32, optional, tag = "9")]
  pub num_lines_in_file: Option<i32>,
  #[prost(bool, optional, tag = "10")]
  pub is_subagent_edit: Option<bool>,
  #[prost(bool, optional, tag = "12")]
  pub diff_became_no_op_due_to_on_save_fixes: Option<bool>,
  #[prost(message, optional, tag = "13")]
  pub human_review: Option<edit_file_result::EditFileHumanReview>,
  #[prost(message, optional, tag = "14")]
  pub human_feedback: Option<edit_file_result::HumanFeedback>,
  #[prost(bool, optional, tag = "15")]
  pub should_eagerly_process_lints: Option<bool>,
  #[prost(message, optional, tag = "16")]
  pub human_review_v2: Option<HumanReview>,
  #[prost(bool, optional, tag = "17")]
  pub were_all_new_linter_errors_resolved_by_this_edit: Option<bool>,
}
/// Nested message and enum types in `EditFileResult`.
pub mod edit_file_result {
  /// aiserver.v1.EditFileResult.FileDiff
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct FileDiff {
    #[prost(message, repeated, tag = "1")]
    pub chunks: Vec<file_diff::ChunkDiff>,
    #[prost(enumeration = "file_diff::Editor", tag = "2")]
    pub editor: i32,
    #[prost(bool, tag = "3")]
    pub hit_timeout: bool,
  }
  /// Nested message and enum types in `FileDiff`.
  pub mod file_diff {
    /// aiserver.v1.EditFileResult.FileDiff.ChunkDiff
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ChunkDiff {
      #[prost(string, tag = "1")]
      pub diff_string: String,
      #[prost(int32, tag = "2")]
      pub old_start: i32,
      #[prost(int32, tag = "3")]
      pub new_start: i32,
      #[prost(int32, tag = "4")]
      pub old_lines: i32,
      #[prost(int32, tag = "5")]
      pub new_lines: i32,
      #[prost(int32, tag = "6")]
      pub lines_removed: i32,
      #[prost(int32, tag = "7")]
      pub lines_added: i32,
    }
    /// aiserver.v1.EditFileResult.FileDiff.Editor
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Editor {
      Unspecified = 0,
      Ai = 1,
      Human = 2,
    }
  }
  /// aiserver.v1.EditFileResult.RecoverableError
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct RecoverableError {
    #[prost(enumeration = "recoverable_error::RecoverableErrorType", tag = "1")]
    pub error_type: i32,
    #[prost(string, tag = "2")]
    pub model_message: String,
  }
  /// Nested message and enum types in `RecoverableError`.
  pub mod recoverable_error {
    /// aiserver.v1.EditFileResult.RecoverableError.RecoverableErrorType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum RecoverableErrorType {
      Unspecified = 0,
      SearchStringNotFound = 1,
      AmbiguousSearchString = 2,
    }
  }
  /// aiserver.v1.EditFileResult.EditFileHumanReview
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct EditFileHumanReview {
    #[prost(bool, tag = "1")]
    pub is_edit_accepted: bool,
    #[prost(string, tag = "2")]
    pub text_result: String,
    #[prost(bool, tag = "3")]
    pub stop_and_get_new_user_input: bool,
  }
  /// aiserver.v1.EditFileResult.HumanFeedback
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct HumanFeedback {
    #[prost(string, tag = "1")]
    pub selected_option: String,
    #[prost(string, tag = "2")]
    pub feedback_text: String,
    #[prost(bool, tag = "3")]
    pub submit_feedback_as_new_message: bool,
    #[prost(string, tag = "4")]
    pub bubble_id: String,
  }
}
/// aiserver.v1.HumanReview
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct HumanReview {
  #[prost(string, tag = "1")]
  pub selected_option: String,
  #[prost(string, tag = "2")]
  pub feedback_text: String,
  #[prost(bool, tag = "3")]
  pub submit_feedback_as_new_message: bool,
  #[prost(string, tag = "4")]
  pub bubble_id: String,
}
/// aiserver.v1.EditFileStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct EditFileStream {}
/// aiserver.v1.ToolCallFileSearchParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ToolCallFileSearchParams {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.ToolCallFileSearchStream
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ToolCallFileSearchStream {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.ToolCallFileSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolCallFileSearchResult {
  #[prost(message, repeated, tag = "1")]
  pub files: Vec<tool_call_file_search_result::File>,
  #[prost(bool, optional, tag = "2")]
  pub limit_hit: Option<bool>,
  #[prost(int32, tag = "3")]
  pub num_results: i32,
}
/// Nested message and enum types in `ToolCallFileSearchResult`.
pub mod tool_call_file_search_result {
  /// aiserver.v1.ToolCallFileSearchResult.File
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct File {
    #[prost(string, tag = "1")]
    pub uri: String,
  }
}
/// aiserver.v1.ListDirParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ListDirParams {
  #[prost(string, tag = "1")]
  pub directory_path: String,
}
/// aiserver.v1.ListDirResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ListDirResult {
  #[prost(message, repeated, tag = "1")]
  pub files: Vec<list_dir_result::File>,
  #[prost(string, tag = "2")]
  pub directory_relative_workspace_path: String,
}
/// Nested message and enum types in `ListDirResult`.
pub mod list_dir_result {
  /// aiserver.v1.ListDirResult.File
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct File {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(bool, tag = "2")]
    pub is_directory: bool,
    #[prost(int64, optional, tag = "3")]
    pub size: Option<i64>,
    #[prost(message, optional, tag = "4")]
    pub last_modified: Option<::prost_types::Timestamp>,
    #[prost(int32, optional, tag = "5")]
    pub num_children: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub num_lines: Option<i32>,
  }
}
/// aiserver.v1.ListDirStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ListDirStream {}
/// aiserver.v1.ReadFileParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReadFileParams {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(bool, tag = "2")]
  pub read_entire_file: bool,
  #[prost(int32, optional, tag = "3")]
  pub start_line_one_indexed: Option<i32>,
  #[prost(int32, optional, tag = "4")]
  pub end_line_one_indexed_inclusive: Option<i32>,
  #[prost(bool, tag = "5")]
  pub file_is_allowed_to_be_read_entirely: bool,
  #[prost(int32, optional, tag = "6")]
  pub max_lines: Option<i32>,
  #[prost(int32, optional, tag = "7")]
  pub max_chars: Option<i32>,
  #[prost(int32, optional, tag = "8")]
  pub min_lines: Option<i32>,
}
/// aiserver.v1.ReadFileResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadFileResult {
  #[prost(string, tag = "1")]
  pub contents: String,
  #[prost(bool, tag = "2")]
  pub did_downgrade_to_line_range: bool,
  #[prost(bool, tag = "3")]
  pub did_shorten_line_range: bool,
  #[prost(bool, tag = "4")]
  pub did_set_default_line_range: bool,
  #[prost(string, optional, tag = "5")]
  pub full_file_contents: Option<String>,
  #[prost(string, optional, tag = "6")]
  pub outline: Option<String>,
  #[prost(int32, optional, tag = "7")]
  pub start_line_one_indexed: Option<i32>,
  #[prost(int32, optional, tag = "8")]
  pub end_line_one_indexed_inclusive: Option<i32>,
  #[prost(string, tag = "9")]
  pub relative_workspace_path: String,
  #[prost(bool, tag = "10")]
  pub did_shorten_char_range: bool,
  #[prost(bool, optional, tag = "11")]
  pub read_full_file: Option<bool>,
  #[prost(int32, optional, tag = "12")]
  pub total_lines: Option<i32>,
  #[prost(message, repeated, tag = "13")]
  pub matching_cursor_rules: Vec<CursorRule>,
  #[prost(message, optional, tag = "14")]
  pub file_git_context: Option<FileGit>,
}
/// aiserver.v1.ReadFileStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReadFileStream {}
/// aiserver.v1.RipgrepSearchParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RipgrepSearchParams {
  #[prost(message, optional, tag = "1")]
  pub options: Option<ripgrep_search_params::ITextQueryBuilderOptionsProto>,
  #[prost(message, optional, tag = "2")]
  pub pattern_info: Option<ripgrep_search_params::IPatternInfoProto>,
}
/// Nested message and enum types in `RipgrepSearchParams`.
pub mod ripgrep_search_params {
  /// aiserver.v1.RipgrepSearchParams.IPatternInfoProto
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct IPatternInfoProto {
    #[prost(string, tag = "1")]
    pub pattern: String,
    #[prost(bool, optional, tag = "2")]
    pub is_reg_exp: Option<bool>,
    #[prost(bool, optional, tag = "3")]
    pub is_word_match: Option<bool>,
    #[prost(string, optional, tag = "4")]
    pub word_separators: Option<String>,
    #[prost(bool, optional, tag = "5")]
    pub is_multiline: Option<bool>,
    #[prost(bool, optional, tag = "6")]
    pub is_unicode: Option<bool>,
    #[prost(bool, optional, tag = "7")]
    pub is_case_sensitive: Option<bool>,
    #[prost(message, optional, tag = "8")]
    pub notebook_info: Option<i_pattern_info_proto::INotebookPatternInfoProto>,
    #[prost(bool, optional, tag = "9")]
    pub pattern_was_escaped: Option<bool>,
  }
  /// Nested message and enum types in `IPatternInfoProto`.
  pub mod i_pattern_info_proto {
    /// aiserver.v1.RipgrepSearchParams.IPatternInfoProto.INotebookPatternInfoProto
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct INotebookPatternInfoProto {
      #[prost(bool, optional, tag = "1")]
      pub is_in_notebook_markdown_input: Option<bool>,
      #[prost(bool, optional, tag = "2")]
      pub is_in_notebook_markdown_preview: Option<bool>,
      #[prost(bool, optional, tag = "3")]
      pub is_in_notebook_cell_input: Option<bool>,
      #[prost(bool, optional, tag = "4")]
      pub is_in_notebook_cell_output: Option<bool>,
    }
  }
  /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct ITextQueryBuilderOptionsProto {
    #[prost(message, optional, tag = "1")]
    pub preview_options: Option<i_text_query_builder_options_proto::ITextSearchPreviewOptionsProto>,
    #[prost(string, optional, tag = "2")]
    pub file_encoding: Option<String>,
    #[prost(int32, optional, tag = "3")]
    pub surrounding_context: Option<i32>,
    #[prost(bool, optional, tag = "4")]
    pub is_smart_case: Option<bool>,
    #[prost(message, optional, tag = "5")]
    pub notebook_search_config: Option<i_text_query_builder_options_proto::INotebookSearchConfigProto>,
    #[prost(message, optional, tag = "6")]
    pub exclude_pattern: Option<i_text_query_builder_options_proto::ExcludePatternProto>,
    #[prost(message, optional, tag = "7")]
    pub include_pattern: Option<i_text_query_builder_options_proto::ISearchPathPatternBuilderProto>,
    #[prost(bool, optional, tag = "8")]
    pub expand_patterns: Option<bool>,
    #[prost(int32, optional, tag = "9")]
    pub max_results: Option<i32>,
    #[prost(int32, optional, tag = "10")]
    pub max_file_size: Option<i32>,
    #[prost(bool, optional, tag = "11")]
    pub disregard_ignore_files: Option<bool>,
    #[prost(bool, optional, tag = "12")]
    pub disregard_global_ignore_files: Option<bool>,
    #[prost(bool, optional, tag = "13")]
    pub disregard_parent_ignore_files: Option<bool>,
    #[prost(bool, optional, tag = "14")]
    pub disregard_exclude_settings: Option<bool>,
    #[prost(bool, optional, tag = "15")]
    pub disregard_search_exclude_settings: Option<bool>,
    #[prost(bool, optional, tag = "16")]
    pub ignore_symlinks: Option<bool>,
    #[prost(bool, optional, tag = "17")]
    pub only_open_editors: Option<bool>,
    #[prost(bool, optional, tag = "18")]
    pub only_file_scheme: Option<bool>,
    #[prost(string, optional, tag = "19")]
    pub reason: Option<String>,
    #[prost(message, optional, tag = "20")]
    pub extra_file_resources: Option<i_text_query_builder_options_proto::ExtraFileResourcesProto>,
  }
  /// Nested message and enum types in `ITextQueryBuilderOptionsProto`.
  pub mod i_text_query_builder_options_proto {
    /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto.ExtraFileResourcesProto
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ExtraFileResourcesProto {
      #[prost(string, repeated, tag = "1")]
      pub extra_file_resources: Vec<String>,
    }
    /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto.ExcludePatternProto
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ExcludePatternProto {
      #[prost(message, repeated, tag = "1")]
      pub exclude_pattern: Vec<ISearchPatternBuilderProto>,
    }
    /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto.ISearchPatternBuilderProto
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ISearchPatternBuilderProto {
      #[prost(string, optional, tag = "1")]
      pub uri: Option<String>,
      #[prost(message, optional, tag = "2")]
      pub pattern: Option<ISearchPathPatternBuilderProto>,
    }
    /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto.ISearchPathPatternBuilderProto
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ISearchPathPatternBuilderProto {
      #[prost(string, optional, tag = "1")]
      pub pattern: Option<String>,
      #[prost(string, repeated, tag = "2")]
      pub patterns: Vec<String>,
    }
    /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto.ITextSearchPreviewOptionsProto
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct ITextSearchPreviewOptionsProto {
      #[prost(int32, tag = "1")]
      pub match_lines: i32,
      #[prost(int32, tag = "2")]
      pub chars_per_line: i32,
    }
    /// aiserver.v1.RipgrepSearchParams.ITextQueryBuilderOptionsProto.INotebookSearchConfigProto
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct INotebookSearchConfigProto {
      #[prost(bool, tag = "1")]
      pub include_markup_input: bool,
      #[prost(bool, tag = "2")]
      pub include_markup_preview: bool,
      #[prost(bool, tag = "3")]
      pub include_code_input: bool,
      #[prost(bool, tag = "4")]
      pub include_output: bool,
    }
  }
}
/// aiserver.v1.RipgrepSearchResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RipgrepSearchResult {
  #[prost(message, optional, tag = "1")]
  pub internal: Option<RipgrepSearchResultInternal>,
}
/// aiserver.v1.RipgrepSearchResultInternal
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RipgrepSearchResultInternal {
  #[prost(message, repeated, tag = "1")]
  pub results: Vec<ripgrep_search_result_internal::IFileMatch>,
  #[prost(
    enumeration = "ripgrep_search_result_internal::SearchCompletionExitCode",
    optional,
    tag = "2"
  )]
  pub exit: Option<i32>,
  #[prost(bool, optional, tag = "3")]
  pub limit_hit: Option<bool>,
  #[prost(message, repeated, tag = "4")]
  pub messages: Vec<ripgrep_search_result_internal::ITextSearchCompleteMessage>,
  #[prost(oneof = "ripgrep_search_result_internal::Stats", tags = "5, 6")]
  pub stats: Option<ripgrep_search_result_internal::Stats>,
}
/// Nested message and enum types in `RipgrepSearchResultInternal`.
pub mod ripgrep_search_result_internal {
  /// aiserver.v1.RipgrepSearchResultInternal.IFileMatch
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct IFileMatch {
    #[prost(string, tag = "1")]
    pub resource: String,
    #[prost(message, repeated, tag = "2")]
    pub results: Vec<ITextSearchResult>,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchResult
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct ITextSearchResult {
    #[prost(oneof = "i_text_search_result::Result", tags = "1, 2")]
    pub result: Option<i_text_search_result::Result>,
  }
  /// Nested message and enum types in `ITextSearchResult`.
  pub mod i_text_search_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Result {
      #[prost(message, tag = "1")]
      Match(super::ITextSearchMatch),
      #[prost(message, tag = "2")]
      Context(super::ITextSearchContext),
    }
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchMatch
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct ITextSearchMatch {
    #[prost(string, optional, tag = "1")]
    pub uri: Option<String>,
    #[prost(message, repeated, tag = "2")]
    pub range_locations: Vec<ISearchRangeSetPairing>,
    #[prost(string, tag = "3")]
    pub preview_text: String,
    #[prost(int32, optional, tag = "4")]
    pub webview_index: Option<i32>,
    #[prost(string, optional, tag = "5")]
    pub cell_fragment: Option<String>,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchContext
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ITextSearchContext {
    #[prost(string, optional, tag = "1")]
    pub uri: Option<String>,
    #[prost(string, tag = "2")]
    pub text: String,
    #[prost(int32, tag = "3")]
    pub line_number: i32,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ISearchRangeSetPairing
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ISearchRangeSetPairing {
    #[prost(message, optional, tag = "1")]
    pub source: Option<ISearchRange>,
    #[prost(message, optional, tag = "2")]
    pub preview: Option<ISearchRange>,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ISearchRange
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ISearchRange {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub start_column: i32,
    #[prost(int32, tag = "3")]
    pub end_line_number: i32,
    #[prost(int32, tag = "4")]
    pub end_column: i32,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchCompleteMessage
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ITextSearchCompleteMessage {
    #[prost(string, tag = "1")]
    pub text: String,
    #[prost(enumeration = "TextSearchCompleteMessageType", tag = "2")]
    pub r#type: i32,
    #[prost(bool, optional, tag = "3")]
    pub trusted: Option<bool>,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.IFileSearchStats
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct IFileSearchStats {
    #[prost(bool, tag = "1")]
    pub from_cache: bool,
    #[prost(int32, tag = "5")]
    pub result_count: i32,
    #[prost(enumeration = "i_file_search_stats::FileSearchProviderType", tag = "6")]
    pub r#type: i32,
    #[prost(int32, optional, tag = "7")]
    pub sorting_time: Option<i32>,
    #[prost(oneof = "i_file_search_stats::DetailStats", tags = "2, 3, 4")]
    pub detail_stats: Option<i_file_search_stats::DetailStats>,
  }
  /// Nested message and enum types in `IFileSearchStats`.
  pub mod i_file_search_stats {
    /// aiserver.v1.RipgrepSearchResultInternal.IFileSearchStats.FileSearchProviderType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum FileSearchProviderType {
      Unspecified = 0,
      FileSearchProvider = 1,
      SearchProcess = 2,
    }
    #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Oneof)]
    pub enum DetailStats {
      #[prost(message, tag = "2")]
      SearchEngineStats(super::ISearchEngineStats),
      #[prost(message, tag = "3")]
      CachedSearchStats(super::ICachedSearchStats),
      #[prost(message, tag = "4")]
      FileSearchProviderStats(super::IFileSearchProviderStats),
    }
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchStats
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ITextSearchStats {
    #[prost(enumeration = "i_text_search_stats::TextSearchProviderType", tag = "1")]
    pub r#type: i32,
  }
  /// Nested message and enum types in `ITextSearchStats`.
  pub mod i_text_search_stats {
    /// aiserver.v1.RipgrepSearchResultInternal.ITextSearchStats.TextSearchProviderType
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum TextSearchProviderType {
      Unspecified = 0,
      TextSearchProvider = 1,
      SearchProcess = 2,
      AiTextSearchProvider = 3,
    }
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ISearchEngineStats
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ISearchEngineStats {
    #[prost(int32, tag = "1")]
    pub file_walk_time: i32,
    #[prost(int32, tag = "2")]
    pub directories_walked: i32,
    #[prost(int32, tag = "3")]
    pub files_walked: i32,
    #[prost(int32, tag = "4")]
    pub cmd_time: i32,
    #[prost(int32, optional, tag = "5")]
    pub cmd_result_count: Option<i32>,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.ICachedSearchStats
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ICachedSearchStats {
    #[prost(bool, tag = "1")]
    pub cache_was_resolved: bool,
    #[prost(int32, tag = "2")]
    pub cache_lookup_time: i32,
    #[prost(int32, tag = "3")]
    pub cache_filter_time: i32,
    #[prost(int32, tag = "4")]
    pub cache_entry_count: i32,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.IFileSearchProviderStats
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct IFileSearchProviderStats {
    #[prost(int32, tag = "1")]
    pub provider_time: i32,
    #[prost(int32, tag = "2")]
    pub post_process_time: i32,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.TextSearchCompleteMessageType
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum TextSearchCompleteMessageType {
    Unspecified = 0,
    Information = 1,
    Warning = 2,
  }
  /// aiserver.v1.RipgrepSearchResultInternal.SearchCompletionExitCode
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum SearchCompletionExitCode {
    Unspecified = 0,
    Normal = 1,
    NewSearchStarted = 2,
  }
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Oneof)]
  pub enum Stats {
    #[prost(message, tag = "5")]
    FileSearchStats(IFileSearchStats),
    #[prost(message, tag = "6")]
    TextSearchStats(ITextSearchStats),
  }
}
/// aiserver.v1.RipgrepSearchStream
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct RipgrepSearchStream {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.ReadSemsearchFilesParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSemsearchFilesParams {
  #[prost(message, optional, tag = "1")]
  pub repository_info: Option<RepositoryInfo>,
  #[prost(message, repeated, tag = "2")]
  pub code_results: Vec<CodeResult>,
  #[prost(string, tag = "3")]
  pub query: String,
  #[prost(message, repeated, tag = "4")]
  pub pr_references: Vec<PullRequestReference>,
  #[prost(bool, optional, tag = "5")]
  pub pr_search_on: Option<bool>,
}
/// aiserver.v1.MissingFile
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct MissingFile {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(enumeration = "missing_file::MissingReason", tag = "2")]
  pub missing_reason: i32,
  #[prost(int32, optional, tag = "3")]
  pub num_lines: Option<i32>,
}
/// Nested message and enum types in `MissingFile`.
pub mod missing_file {
  /// aiserver.v1.MissingFile.MissingReason
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum MissingReason {
    Unspecified = 0,
    TooLarge = 1,
    NotFound = 2,
  }
}
/// aiserver.v1.Knowledge
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Knowledge {
  #[prost(string, tag = "1")]
  pub knowledge: String,
  #[prost(string, tag = "2")]
  pub title: String,
}
/// aiserver.v1.ToolPullRequestResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ToolPullRequestResult {
  #[prost(string, tag = "1")]
  pub sha: String,
  #[prost(string, tag = "2")]
  pub full_pr_contents: String,
  #[prost(float, tag = "3")]
  pub score: f32,
  #[prost(string, optional, tag = "4")]
  pub title: Option<String>,
  #[prost(string, optional, tag = "5")]
  pub summary: Option<String>,
  #[prost(uint32, optional, tag = "6")]
  pub pr_number: Option<u32>,
  #[prost(string, repeated, tag = "7")]
  pub changed_files: Vec<String>,
  #[prost(string, optional, tag = "8")]
  pub author: Option<String>,
  #[prost(string, optional, tag = "9")]
  pub date: Option<String>,
}
/// aiserver.v1.ReadSemsearchFilesResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadSemsearchFilesResult {
  #[prost(message, repeated, tag = "1")]
  pub code_results: Vec<CodeResult>,
  #[prost(message, repeated, tag = "2")]
  pub all_files: Vec<File>,
  #[prost(message, repeated, tag = "3")]
  pub missing_files: Vec<MissingFile>,
  #[prost(message, repeated, tag = "4")]
  pub knowledge_results: Vec<Knowledge>,
  #[prost(message, repeated, tag = "5")]
  pub pr_results: Vec<ToolPullRequestResult>,
  #[prost(string, optional, tag = "6")]
  pub git_remote_url: Option<String>,
  #[prost(bool, optional, tag = "7")]
  pub pr_hydration_timed_out: Option<bool>,
}
/// aiserver.v1.ReadSemsearchFilesStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReadSemsearchFilesStream {
  #[prost(int32, tag = "1")]
  pub num_files: i32,
}
/// aiserver.v1.SemanticSearchFullParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticSearchFullParams {
  #[prost(message, optional, tag = "1")]
  pub repository_info: Option<RepositoryInfo>,
  #[prost(string, tag = "2")]
  pub query: String,
  #[prost(string, optional, tag = "3")]
  pub include_pattern: Option<String>,
  #[prost(string, optional, tag = "4")]
  pub exclude_pattern: Option<String>,
  #[prost(int32, tag = "5")]
  pub top_k: i32,
  #[prost(message, repeated, tag = "6")]
  pub pr_references: Vec<PullRequestReference>,
  #[prost(bool, optional, tag = "7")]
  pub pr_search_on: Option<bool>,
}
/// aiserver.v1.SemanticSearchFullResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SemanticSearchFullResult {
  #[prost(message, repeated, tag = "1")]
  pub code_results: Vec<CodeResult>,
  #[prost(message, repeated, tag = "2")]
  pub all_files: Vec<File>,
  #[prost(message, repeated, tag = "3")]
  pub missing_files: Vec<MissingFile>,
  #[prost(message, repeated, tag = "4")]
  pub knowledge_results: Vec<Knowledge>,
  #[prost(message, repeated, tag = "5")]
  pub pr_results: Vec<ToolPullRequestResult>,
  #[prost(string, optional, tag = "6")]
  pub git_remote_url: Option<String>,
  #[prost(bool, optional, tag = "7")]
  pub pr_hydration_timed_out: Option<bool>,
}
/// aiserver.v1.SemanticSearchFullStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SemanticSearchFullStream {
  #[prost(int32, tag = "1")]
  pub num_files: i32,
}
/// aiserver.v1.DeleteFileParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeleteFileParams {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
}
/// aiserver.v1.DeleteFileResult
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeleteFileResult {
  #[prost(bool, tag = "1")]
  pub rejected: bool,
  #[prost(bool, tag = "2")]
  pub file_non_existent: bool,
  #[prost(bool, tag = "3")]
  pub file_deleted_successfully: bool,
}
/// aiserver.v1.DeleteFileStream
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeleteFileStream {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
}
/// aiserver.v1.Range
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Range {
  #[prost(int32, tag = "1")]
  pub start_line: i32,
  #[prost(int32, tag = "2")]
  pub start_character: i32,
  #[prost(int32, tag = "3")]
  pub end_line: i32,
  #[prost(int32, tag = "4")]
  pub end_character: i32,
}
/// aiserver.v1.MatchRange
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct MatchRange {
  #[prost(int32, tag = "1")]
  pub start: i32,
  #[prost(int32, tag = "2")]
  pub end: i32,
}
/// aiserver.v1.GotodefParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GotodefParams {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, tag = "2")]
  pub symbol: String,
  #[prost(int32, tag = "3")]
  pub start_line: i32,
  #[prost(int32, tag = "4")]
  pub end_line: i32,
}
/// aiserver.v1.GotodefDefinition
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GotodefDefinition {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, optional, tag = "2")]
  pub fully_qualified_name: Option<String>,
  #[prost(string, optional, tag = "3")]
  pub symbol_kind: Option<String>,
  #[prost(int32, tag = "4")]
  pub start_line: i32,
  #[prost(int32, tag = "5")]
  pub end_line: i32,
  #[prost(string, repeated, tag = "6")]
  pub code_context_lines: Vec<String>,
}
/// aiserver.v1.GotodefResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GotodefResult {
  #[prost(message, repeated, tag = "1")]
  pub definitions: Vec<GotodefDefinition>,
}
/// aiserver.v1.ShellCommandParsingResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ShellCommandParsingResult {
  #[prost(bool, tag = "1")]
  pub parsing_failed: bool,
  #[prost(message, repeated, tag = "2")]
  pub executable_commands: Vec<shell_command_parsing_result::ExecutableCommand>,
  #[prost(bool, tag = "3")]
  pub has_redirects: bool,
  #[prost(bool, tag = "4")]
  pub has_command_substitution: bool,
}
/// Nested message and enum types in `ShellCommandParsingResult`.
pub mod shell_command_parsing_result {
  /// aiserver.v1.ShellCommandParsingResult.ExecutableCommandArg
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ExecutableCommandArg {
    #[prost(string, tag = "1")]
    pub r#type: String,
    #[prost(string, tag = "2")]
    pub value: String,
  }
  /// aiserver.v1.ShellCommandParsingResult.ExecutableCommand
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct ExecutableCommand {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, repeated, tag = "2")]
    pub args: Vec<ExecutableCommandArg>,
    #[prost(string, tag = "3")]
    pub full_text: String,
  }
}
/// aiserver.v1.RunTerminalCommandV2Params
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTerminalCommandV2Params {
  #[prost(string, tag = "1")]
  pub command: String,
  #[prost(string, optional, tag = "2")]
  pub cwd: Option<String>,
  #[prost(bool, optional, tag = "3")]
  pub new_session: Option<bool>,
  #[prost(message, optional, tag = "4")]
  pub options: Option<run_terminal_command_v2_params::ExecutionOptions>,
  #[prost(bool, tag = "5")]
  pub is_background: bool,
  #[prost(bool, tag = "6")]
  pub require_user_approval: bool,
  #[prost(message, optional, tag = "7")]
  pub parsing_result: Option<ShellCommandParsingResult>,
}
/// Nested message and enum types in `RunTerminalCommandV2Params`.
pub mod run_terminal_command_v2_params {
  /// aiserver.v1.RunTerminalCommandV2Params.ExecutionOptions
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ExecutionOptions {
    #[prost(int32, optional, tag = "1")]
    pub timeout: Option<i32>,
    #[prost(bool, optional, tag = "2")]
    pub skip_ai_check: Option<bool>,
    #[prost(int32, optional, tag = "3")]
    pub command_run_timeout_ms: Option<i32>,
    #[prost(int32, optional, tag = "4")]
    pub command_change_check_interval_ms: Option<i32>,
    #[prost(int32, optional, tag = "5")]
    pub ai_finish_check_max_attempts: Option<i32>,
    #[prost(int32, optional, tag = "6")]
    pub ai_finish_check_interval_ms: Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub delayer_interval_ms: Option<i32>,
  }
}
/// aiserver.v1.RunTerminalCommandV2Result
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct RunTerminalCommandV2Result {
  #[prost(string, tag = "1")]
  pub output: String,
  #[prost(int32, tag = "2")]
  pub exit_code: i32,
  #[prost(bool, optional, tag = "3")]
  pub rejected: Option<bool>,
  #[prost(bool, tag = "4")]
  pub popped_out_into_background: bool,
  #[prost(bool, tag = "5")]
  pub is_running_in_background: bool,
  #[prost(bool, tag = "6")]
  pub not_interrupted: bool,
  #[prost(string, tag = "7")]
  pub resulting_working_directory: String,
  #[prost(bool, tag = "8")]
  pub did_user_change: bool,
  #[prost(enumeration = "RunTerminalCommandEndedReason", tag = "9")]
  pub ended_reason: i32,
  #[prost(int32, optional, tag = "10")]
  pub exit_code_v2: Option<i32>,
  #[prost(string, optional, tag = "11")]
  pub updated_command: Option<String>,
  #[prost(string, tag = "12")]
  pub output_raw: String,
  #[prost(message, optional, tag = "13")]
  pub human_review_v2: Option<HumanReview>,
}
/// aiserver.v1.RunTerminalCommandV2Stream
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunTerminalCommandV2Stream {
  #[prost(string, tag = "1")]
  pub command: String,
  #[prost(bool, tag = "2")]
  pub is_background: bool,
  #[prost(message, optional, tag = "7")]
  pub parsing_result: Option<ShellCommandParsingResult>,
}
/// aiserver.v1.FetchRulesStream
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FetchRulesStream {
  #[prost(string, repeated, tag = "1")]
  pub rule_names: Vec<String>,
}
/// aiserver.v1.WebSearchParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WebSearchParams {
  #[prost(string, tag = "1")]
  pub search_term: String,
}
/// aiserver.v1.WebSearchResult
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
  /// aiserver.v1.WebSearchResult.WebReference
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
/// aiserver.v1.WebSearchStream
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WebSearchStream {
  #[prost(string, tag = "1")]
  pub search_term: String,
}
/// aiserver.v1.MCPParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct McpParams {
  #[prost(message, repeated, tag = "1")]
  pub tools: Vec<mcp_params::Tool>,
}
/// Nested message and enum types in `MCPParams`.
pub mod mcp_params {
  /// aiserver.v1.MCPParams.Tool
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
/// aiserver.v1.MCPResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct McpResult {
  #[prost(string, tag = "1")]
  pub selected_tool: String,
  #[prost(string, tag = "2")]
  pub result: String,
}
/// aiserver.v1.MCPStream
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct McpStream {
  #[prost(message, repeated, tag = "1")]
  pub tools: Vec<mcp_params::Tool>,
}
/// aiserver.v1.SearchSymbolsParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SearchSymbolsParams {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.SearchSymbolsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SearchSymbolsResult {
  #[prost(message, repeated, tag = "1")]
  pub matches: Vec<search_symbols_result::SymbolMatch>,
  #[prost(bool, optional, tag = "2")]
  pub rejected: Option<bool>,
}
/// Nested message and enum types in `SearchSymbolsResult`.
pub mod search_symbols_result {
  /// aiserver.v1.SearchSymbolsResult.SymbolMatch
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct SymbolMatch {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub uri: String,
    #[prost(message, optional, tag = "3")]
    pub range: Option<super::Range>,
    #[prost(string, tag = "4")]
    pub secondary_text: String,
    #[prost(message, repeated, tag = "5")]
    pub label_matches: Vec<super::MatchRange>,
    #[prost(message, repeated, tag = "6")]
    pub description_matches: Vec<super::MatchRange>,
    #[prost(double, tag = "7")]
    pub score: f64,
  }
}
/// aiserver.v1.SearchSymbolsStream
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SearchSymbolsStream {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.BackgroundComposerFollowupParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct BackgroundComposerFollowupParams {
  #[prost(string, tag = "1")]
  pub proposed_followup: String,
  #[prost(string, tag = "2")]
  pub bc_id: String,
}
/// aiserver.v1.BackgroundComposerFollowupResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct BackgroundComposerFollowupResult {
  #[prost(string, tag = "1")]
  pub proposed_followup: String,
  #[prost(bool, tag = "2")]
  pub is_sent: bool,
}
/// aiserver.v1.BackgroundComposerFollowupStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct BackgroundComposerFollowupStream {}
/// aiserver.v1.KnowledgeBaseParams
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
/// aiserver.v1.KnowledgeBaseResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct KnowledgeBaseResult {
  #[prost(bool, tag = "1")]
  pub success: bool,
  #[prost(string, tag = "2")]
  pub confirmation_message: String,
  #[prost(string, tag = "3")]
  pub id: String,
}
/// aiserver.v1.KnowledgeBaseStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct KnowledgeBaseStream {}
/// aiserver.v1.FetchPullRequestParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FetchPullRequestParams {
  #[prost(string, tag = "1")]
  pub pull_number_or_commit_hash: String,
  #[prost(string, optional, tag = "2")]
  pub repo: Option<String>,
  #[prost(bool, optional, tag = "3")]
  pub is_github: Option<bool>,
}
/// aiserver.v1.FetchPullRequestResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FetchPullRequestResult {
  #[prost(string, tag = "1")]
  pub content: String,
  #[prost(uint32, tag = "2")]
  pub pr_number: u32,
  #[prost(string, tag = "3")]
  pub title: String,
  #[prost(string, tag = "4")]
  pub body: String,
  #[prost(string, tag = "5")]
  pub author: String,
  #[prost(string, tag = "6")]
  pub date: String,
  #[prost(string, tag = "7")]
  pub diff: String,
  #[prost(string, optional, tag = "8")]
  pub sha: Option<String>,
  #[prost(string, optional, tag = "9")]
  pub external_link: Option<String>,
  #[prost(string, optional, tag = "10")]
  pub url: Option<String>,
  #[prost(message, repeated, tag = "11")]
  pub comments: Vec<IssueComment>,
  #[prost(string, repeated, tag = "12")]
  pub labels: Vec<String>,
  #[prost(string, repeated, tag = "13")]
  pub assignees: Vec<String>,
  #[prost(bool, optional, tag = "14")]
  pub is_issue: Option<bool>,
  #[prost(string, optional, tag = "15")]
  pub state: Option<String>,
  #[prost(bool, optional, tag = "16")]
  pub prompt_connect_github: Option<bool>,
}
/// aiserver.v1.IssueComment
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct IssueComment {
  #[prost(uint32, tag = "1")]
  pub id: u32,
  #[prost(string, tag = "2")]
  pub body: String,
  #[prost(string, optional, tag = "3")]
  pub author: Option<String>,
  #[prost(string, tag = "4")]
  pub created_at: String,
  #[prost(string, tag = "5")]
  pub updated_at: String,
  #[prost(string, optional, tag = "6")]
  pub author_association: Option<String>,
}
/// aiserver.v1.FetchPullRequestStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FetchPullRequestStream {}
/// aiserver.v1.PullRequestReference
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRequestReference {
  #[prost(string, tag = "1")]
  pub sha: String,
  #[prost(float, tag = "2")]
  pub score: f32,
  #[prost(string, optional, tag = "3")]
  pub title: Option<String>,
  #[prost(string, optional, tag = "4")]
  pub summary: Option<String>,
  #[prost(uint32, optional, tag = "5")]
  pub pr_number: Option<u32>,
  #[prost(string, optional, tag = "6")]
  pub author: Option<String>,
  #[prost(string, optional, tag = "7")]
  pub date: Option<String>,
  #[prost(string, repeated, tag = "8")]
  pub changed_files: Vec<String>,
}
/// aiserver.v1.DeepSearchParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeepSearchParams {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.DeepSearchResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeepSearchResult {
  #[prost(bool, tag = "1")]
  pub success: bool,
  #[prost(string, tag = "2")]
  pub result: String,
}
/// aiserver.v1.DeepSearchStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeepSearchStream {}
/// aiserver.v1.CreateDiagramParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CreateDiagramParams {
  #[prost(string, tag = "1")]
  pub content: String,
}
/// aiserver.v1.CreateDiagramResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CreateDiagramResult {
  #[prost(string, optional, tag = "1")]
  pub error: Option<String>,
}
/// aiserver.v1.CreateDiagramStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CreateDiagramStream {}
/// aiserver.v1.FixLintsParams
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FixLintsParams {}
/// aiserver.v1.FixLintsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FixLintsResult {
  #[prost(message, repeated, tag = "1")]
  pub file_results: Vec<fix_lints_result::FileResult>,
}
/// Nested message and enum types in `FixLintsResult`.
pub mod fix_lints_result {
  /// aiserver.v1.FixLintsResult.FileResult
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct FileResult {
    #[prost(string, tag = "1")]
    pub file_path: String,
    #[prost(message, optional, tag = "2")]
    pub diff: Option<super::edit_file_result::FileDiff>,
    #[prost(bool, tag = "3")]
    pub is_applied: bool,
    #[prost(bool, tag = "4")]
    pub apply_failed: bool,
    #[prost(string, optional, tag = "5")]
    pub error: Option<String>,
    #[prost(message, repeated, tag = "6")]
    pub linter_errors: Vec<super::LinterError>,
  }
}
/// aiserver.v1.FixLintsStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FixLintsStream {}
/// aiserver.v1.ReadLintsParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReadLintsParams {
  #[prost(string, tag = "1")]
  pub path: String,
  #[prost(string, repeated, tag = "2")]
  pub paths: Vec<String>,
}
/// aiserver.v1.ReadLintsResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReadLintsResult {
  #[prost(string, tag = "1")]
  pub path: String,
  #[prost(message, repeated, tag = "2")]
  pub linter_errors: Vec<LinterError>,
  #[prost(message, repeated, tag = "3")]
  pub linter_errors_by_file: Vec<LinterErrors>,
}
/// aiserver.v1.ReadLintsStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ReadLintsStream {}
/// aiserver.v1.GotodefStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GotodefStream {}
/// aiserver.v1.TaskParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TaskParams {
  #[prost(string, tag = "1")]
  pub task_description: String,
  #[prost(string, tag = "4")]
  pub task_title: String,
  #[prost(bool, optional, tag = "2")]
  pub r#async: Option<bool>,
  #[prost(string, repeated, tag = "3")]
  pub allowed_write_directories: Vec<String>,
  #[prost(string, optional, tag = "5")]
  pub model_override: Option<String>,
  #[prost(bool, optional, tag = "6")]
  pub max_mode_override: Option<bool>,
  #[prost(bool, optional, tag = "7")]
  pub default_expanded_while_running: Option<bool>,
}
/// aiserver.v1.TaskResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskResult {
  #[prost(oneof = "task_result::Result", tags = "1, 2")]
  pub result: Option<task_result::Result>,
}
/// Nested message and enum types in `TaskResult`.
pub mod task_result {
  /// aiserver.v1.TaskResult.CompletedTaskResult
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct CompletedTaskResult {
    #[prost(string, tag = "1")]
    pub summary: String,
    #[prost(message, repeated, tag = "2")]
    pub file_results: Vec<super::fix_lints_result::FileResult>,
    #[prost(bool, tag = "3")]
    pub user_aborted: bool,
    #[prost(bool, tag = "4")]
    pub subagent_errored: bool,
  }
  /// aiserver.v1.TaskResult.AsyncTaskResult
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct AsyncTaskResult {
    #[prost(string, tag = "1")]
    pub task_id: String,
    #[prost(bool, tag = "2")]
    pub user_aborted: bool,
    #[prost(bool, tag = "3")]
    pub subagent_errored: bool,
  }
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum Result {
    #[prost(message, tag = "1")]
    CompletedTaskResult(CompletedTaskResult),
    #[prost(message, tag = "2")]
    AsyncTaskResult(AsyncTaskResult),
  }
}
/// aiserver.v1.TaskStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TaskStream {}
/// aiserver.v1.AwaitTaskParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AwaitTaskParams {
  #[prost(string, repeated, tag = "1")]
  pub ids: Vec<String>,
}
/// aiserver.v1.AwaitTaskResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AwaitTaskResult {
  #[prost(message, repeated, tag = "1")]
  pub task_results: Vec<await_task_result::TaskResultItem>,
  #[prost(string, repeated, tag = "2")]
  pub missing_task_ids: Vec<String>,
}
/// Nested message and enum types in `AwaitTaskResult`.
pub mod await_task_result {
  /// aiserver.v1.AwaitTaskResult.TaskResultItem
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct TaskResultItem {
    #[prost(string, tag = "1")]
    pub task_id: String,
    #[prost(message, optional, tag = "2")]
    pub result: Option<super::task_result::CompletedTaskResult>,
  }
}
/// aiserver.v1.AwaitTaskStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AwaitTaskStream {}
/// aiserver.v1.TodoReadParams
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TodoReadParams {
  #[prost(bool, tag = "1")]
  pub read: bool,
}
/// aiserver.v1.TodoItem
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
/// aiserver.v1.TodoReadResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TodoReadResult {
  #[prost(message, repeated, tag = "1")]
  pub todos: Vec<TodoItem>,
}
/// aiserver.v1.TodoReadStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TodoReadStream {}
/// aiserver.v1.TodoWriteParams
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TodoWriteParams {
  #[prost(message, repeated, tag = "1")]
  pub todos: Vec<TodoItem>,
  #[prost(bool, tag = "2")]
  pub merge: bool,
}
/// aiserver.v1.TodoWriteResult
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TodoWriteResult {
  #[prost(bool, tag = "1")]
  pub success: bool,
  #[prost(string, repeated, tag = "2")]
  pub ready_task_ids: Vec<String>,
  #[prost(bool, tag = "3")]
  pub needs_in_progress_todos: bool,
  #[prost(message, repeated, tag = "4")]
  pub final_todos: Vec<TodoItem>,
  #[prost(message, repeated, tag = "5")]
  pub initial_todos: Vec<TodoItem>,
  #[prost(bool, tag = "6")]
  pub was_merge: bool,
}
/// aiserver.v1.TodoWriteStream
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TodoWriteStream {}
/// aiserver.v1.GetLintsForChangeResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetLintsForChangeResponse {
  #[prost(message, repeated, tag = "1")]
  pub lints: Vec<get_lints_for_change_response::Lint>,
}
/// Nested message and enum types in `GetLintsForChangeResponse`.
pub mod get_lints_for_change_response {
  /// aiserver.v1.GetLintsForChangeResponse.Lint
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct Lint {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(string, tag = "2")]
    pub severity: String,
    #[prost(string, tag = "3")]
    pub relative_workspace_path: String,
    #[prost(int32, tag = "4")]
    pub start_line_number_one_indexed: i32,
    #[prost(int32, tag = "5")]
    pub start_column_one_indexed: i32,
    #[prost(int32, tag = "6")]
    pub end_line_number_inclusive_one_indexed: i32,
    #[prost(int32, tag = "7")]
    pub end_column_one_indexed: i32,
    #[prost(message, repeated, tag = "9")]
    pub quick_fixes: Vec<lint::QuickFix>,
  }
  /// Nested message and enum types in `Lint`.
  pub mod lint {
    /// aiserver.v1.GetLintsForChangeResponse.Lint.QuickFix
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct QuickFix {
      #[prost(string, tag = "1")]
      pub message: String,
      #[prost(string, tag = "2")]
      pub kind: String,
      #[prost(bool, tag = "3")]
      pub is_preferred: bool,
      #[prost(message, repeated, tag = "4")]
      pub edits: Vec<quick_fix::Edit>,
    }
    /// Nested message and enum types in `QuickFix`.
    pub mod quick_fix {
      /// aiserver.v1.GetLintsForChangeResponse.Lint.QuickFix.Edit
      #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
      pub struct Edit {
        #[prost(string, tag = "1")]
        pub relative_workspace_path: String,
        #[prost(string, tag = "2")]
        pub text: String,
        #[prost(int32, tag = "3")]
        pub start_line_number_one_indexed: i32,
        #[prost(int32, tag = "4")]
        pub start_column_one_indexed: i32,
        #[prost(int32, tag = "5")]
        pub end_line_number_inclusive_one_indexed: i32,
        #[prost(int32, tag = "6")]
        pub end_column_one_indexed: i32,
      }
    }
  }
}
/// aiserver.v1.DocumentationChunk
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentationChunk {
  #[prost(string, tag = "1")]
  pub doc_name: String,
  #[prost(string, tag = "2")]
  pub page_url: String,
  #[prost(string, tag = "3")]
  pub documentation_chunk: String,
  #[prost(float, tag = "4")]
  pub score: f32,
  #[prost(string, tag = "5")]
  pub page_title: String,
}
/// aiserver.v1.ComposerCapabilityRequest
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
  /// aiserver.v1.ComposerCapabilityRequest.ToolSchema
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
  /// aiserver.v1.ComposerCapabilityRequest.SchemaProperty
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct SchemaProperty {
    #[prost(string, tag = "1")]
    pub r#type: String,
    #[prost(string, optional, tag = "2")]
    pub description: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.LoopOnLintsCapability
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct LoopOnLintsCapability {
    #[prost(message, repeated, tag = "1")]
    pub linter_errors: Vec<super::LinterErrors>,
    #[prost(string, optional, tag = "2")]
    pub custom_instructions: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.LoopOnTestsCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct LoopOnTestsCapability {
    #[prost(string, repeated, tag = "1")]
    pub test_names: Vec<String>,
    #[prost(string, optional, tag = "2")]
    pub custom_instructions: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.MegaPlannerCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct MegaPlannerCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.LoopOnCommandCapability
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
  /// aiserver.v1.ComposerCapabilityRequest.ToolCallCapability
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct ToolCallCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
    #[prost(message, repeated, tag = "2")]
    pub tool_schemas: Vec<ToolSchema>,
    #[prost(string, repeated, tag = "3")]
    pub relevant_files: Vec<String>,
    #[prost(string, repeated, tag = "4")]
    pub files_in_context: Vec<String>,
    #[prost(string, repeated, tag = "5")]
    pub semantic_search_files: Vec<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct DiffReviewCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
    #[prost(message, repeated, tag = "2")]
    pub diffs: Vec<diff_review_capability::SimpleFileDiff>,
  }
  /// Nested message and enum types in `DiffReviewCapability`.
  pub mod diff_review_capability {
    /// aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability.SimpleFileDiff
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct SimpleFileDiff {
      #[prost(string, tag = "1")]
      pub relative_workspace_path: String,
      #[prost(message, repeated, tag = "3")]
      pub chunks: Vec<simple_file_diff::Chunk>,
    }
    /// Nested message and enum types in `SimpleFileDiff`.
    pub mod simple_file_diff {
      /// aiserver.v1.ComposerCapabilityRequest.DiffReviewCapability.SimpleFileDiff.Chunk
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
  /// aiserver.v1.ComposerCapabilityRequest.DecomposerCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct DecomposerCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.ContextPickingCapability
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
  /// aiserver.v1.ComposerCapabilityRequest.EditTrailCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct EditTrailCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.AutoContextCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct AutoContextCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
    #[prost(string, repeated, tag = "2")]
    pub additional_files: Vec<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.ContextPlannerCapability
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct ContextPlannerCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
    #[prost(message, repeated, tag = "2")]
    pub attached_code_chunks: Vec<super::CodeChunk>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.RememberThisCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct RememberThisCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
    #[prost(string, tag = "2")]
    pub memory: String,
  }
  /// aiserver.v1.ComposerCapabilityRequest.CursorRulesCapability
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct CursorRulesCapability {
    #[prost(string, optional, tag = "1")]
    pub custom_instructions: Option<String>,
  }
  /// aiserver.v1.ComposerCapabilityRequest.ComposerCapabilityType
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
  }
  /// aiserver.v1.ComposerCapabilityRequest.ToolType
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
/// aiserver.v1.ComposerCapabilityContext
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ComposerCapabilityContext {
  #[prost(oneof = "composer_capability_context::Data", tags = "27, 28")]
  pub data: Option<composer_capability_context::Data>,
}
/// Nested message and enum types in `ComposerCapabilityContext`.
pub mod composer_capability_context {
  /// aiserver.v1.ComposerCapabilityContext.SlackIntegrationContext
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct SlackIntegrationContext {
    #[prost(string, tag = "1")]
    pub thread: String,
  }
  /// aiserver.v1.ComposerCapabilityContext.GithubPRContext
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct GithubPrContext {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub description: String,
    #[prost(string, tag = "3")]
    pub comments: String,
  }
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Oneof)]
  pub enum Data {
    #[prost(message, tag = "27")]
    SlackIntegration(SlackIntegrationContext),
    #[prost(message, tag = "28")]
    GithubPr(GithubPrContext),
  }
}
/// aiserver.v1.StreamUnifiedChatRequestWithTools
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
/// aiserver.v1.UserRules
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserRules {
  #[prost(message, repeated, tag = "1")]
  pub rules: Vec<conversation_message::KnowledgeItem>,
}
/// aiserver.v1.StreamStart
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct StreamStart {
  #[prost(string, tag = "1")]
  pub padding: String,
}
/// aiserver.v1.StreamUnifiedChatResponseWithTools
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatResponseWithTools {
  #[prost(
    oneof = "stream_unified_chat_response_with_tools::Response",
    tags = "1, 2, 3, 4, 5"
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
    #[prost(message, tag = "3")]
    ConversationSummary(super::ConversationSummary),
    #[prost(message, tag = "4")]
    UserRules(super::UserRules),
    #[prost(message, tag = "5")]
    StreamStart(super::StreamStart),
  }
}
/// aiserver.v1.ConversationSummary
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ConversationSummary {
  #[prost(string, tag = "1")]
  pub summary: String,
  #[prost(string, tag = "2")]
  pub truncation_last_bubble_id_inclusive: String,
  #[prost(string, tag = "3")]
  pub client_should_start_sending_from_inclusive_bubble_id: String,
  #[prost(string, tag = "4")]
  pub previous_conversation_summary_bubble_id: String,
  #[prost(bool, tag = "5")]
  pub includes_tool_results: bool,
}
/// aiserver.v1.ContextToRank
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextToRank {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, tag = "2")]
  pub contents: String,
  #[prost(message, optional, tag = "3")]
  pub line_range: Option<LineRange>,
  #[prost(message, optional, tag = "4")]
  pub code_block: Option<CodeBlock>,
}
/// aiserver.v1.RankedContext
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RankedContext {
  #[prost(message, optional, tag = "1")]
  pub context: Option<ContextToRank>,
  #[prost(float, tag = "2")]
  pub score: f32,
}
/// aiserver.v1.DocumentationCitation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DocumentationCitation {
  #[prost(message, repeated, tag = "1")]
  pub chunks: Vec<DocumentationChunk>,
}
/// aiserver.v1.WebCitation
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WebCitation {
  #[prost(message, repeated, tag = "1")]
  pub references: Vec<WebReference>,
}
/// aiserver.v1.WebReference
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct WebReference {
  #[prost(string, tag = "2")]
  pub title: String,
  #[prost(string, tag = "1")]
  pub url: String,
  #[prost(string, tag = "3")]
  pub chunk: String,
}
/// aiserver.v1.DocsReference
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DocsReference {
  #[prost(string, tag = "1")]
  pub title: String,
  #[prost(string, tag = "2")]
  pub url: String,
  #[prost(string, tag = "3")]
  pub chunk: String,
  #[prost(string, tag = "4")]
  pub name: String,
}
/// aiserver.v1.StatusUpdate
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct StatusUpdate {
  #[prost(string, tag = "1")]
  pub message: String,
  #[prost(string, optional, tag = "2")]
  pub metadata: Option<String>,
}
/// aiserver.v1.StatusUpdates
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StatusUpdates {
  #[prost(message, repeated, tag = "1")]
  pub updates: Vec<StatusUpdate>,
}
/// aiserver.v1.ComposerFileDiffHistory
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposerFileDiffHistory {
  #[prost(string, tag = "1")]
  pub file_name: String,
  #[prost(string, repeated, tag = "2")]
  pub diff_history: Vec<String>,
  #[prost(double, repeated, tag = "3")]
  pub diff_history_timestamps: Vec<f64>,
}
/// aiserver.v1.StreamUnifiedChatRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatRequest {
  #[prost(message, repeated, tag = "1")]
  pub conversation: Vec<ConversationMessage>,
  #[prost(message, repeated, tag = "30")]
  pub full_conversation_headers_only: Vec<ConversationMessageHeader>,
  #[prost(bool, optional, tag = "2")]
  pub allow_long_file_scan: Option<bool>,
  #[prost(message, optional, tag = "3")]
  pub explicit_context: Option<ExplicitContext>,
  #[prost(bool, optional, tag = "4")]
  pub can_handle_filenames_after_language_ids: Option<bool>,
  #[prost(message, optional, tag = "5")]
  pub model_details: Option<ModelDetails>,
  #[prost(message, optional, tag = "6")]
  pub linter_errors: Option<LinterErrors>,
  #[prost(string, repeated, tag = "7")]
  pub documentation_identifiers: Vec<String>,
  #[prost(string, optional, tag = "8")]
  pub use_web: Option<String>,
  #[prost(message, repeated, tag = "9")]
  pub external_links: Vec<ComposerExternalLink>,
  #[prost(message, optional, tag = "10")]
  pub project_context: Option<ConversationMessage>,
  #[prost(message, repeated, tag = "11")]
  pub diffs_for_compressing_files: Vec<stream_unified_chat_request::RedDiff>,
  #[prost(bool, optional, tag = "12")]
  pub compress_edits: Option<bool>,
  #[prost(bool, optional, tag = "13")]
  pub should_cache: Option<bool>,
  #[prost(message, repeated, tag = "14")]
  pub multi_file_linter_errors: Vec<LinterErrors>,
  #[prost(message, optional, tag = "15")]
  pub current_file: Option<CurrentFileInfo>,
  #[prost(message, optional, tag = "16")]
  pub recent_edits: Option<stream_unified_chat_request::RecentEdits>,
  #[prost(bool, optional, tag = "17")]
  pub use_reference_composer_diff_prompt: Option<bool>,
  #[prost(message, repeated, tag = "18")]
  pub file_diff_histories: Vec<ComposerFileDiffHistory>,
  #[prost(bool, optional, tag = "19")]
  pub use_new_compression_scheme: Option<bool>,
  #[prost(message, repeated, tag = "21")]
  pub quotes: Vec<ChatQuote>,
  #[prost(message, repeated, tag = "20")]
  pub additional_ranked_context: Vec<RankedContext>,
  #[prost(bool, tag = "22")]
  pub is_chat: bool,
  #[prost(string, tag = "23")]
  pub conversation_id: String,
  #[prost(message, optional, tag = "24")]
  pub repository_info: Option<RepositoryInfo>,
  #[prost(bool, tag = "25")]
  pub repository_info_should_query_staging: bool,
  #[prost(bool, tag = "39")]
  pub repository_info_should_query_prod: bool,
  #[prost(message, optional, tag = "52")]
  pub query_only_repo_access: Option<QueryOnlyRepoAccess>,
  #[prost(string, tag = "44")]
  pub repo_query_auth_token: String,
  #[prost(message, optional, tag = "26")]
  pub environment_info: Option<EnvironmentInfo>,
  #[prost(bool, tag = "27")]
  pub is_agentic: bool,
  #[prost(message, optional, tag = "28")]
  pub conversation_summary: Option<ConversationSummary>,
  #[prost(enumeration = "ClientSideToolV2", repeated, tag = "29")]
  pub supported_tools: Vec<i32>,
  #[prost(bool, tag = "31")]
  pub enable_yolo_mode: bool,
  #[prost(string, tag = "32")]
  pub yolo_prompt: String,
  #[prost(bool, tag = "33")]
  pub use_unified_chat_prompt: bool,
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
  #[prost(string, optional, tag = "40")]
  pub context_bank_session_id: Option<String>,
  #[prost(int32, optional, tag = "41")]
  pub context_bank_version: Option<i32>,
  #[prost(bytes = "vec", optional, tag = "43")]
  pub context_bank_encryption_key: Option<Vec<u8>>,
  #[prost(bool, tag = "45")]
  pub is_headless: bool,
  #[prost(bool, tag = "68")]
  pub is_background_composer: bool,
  #[prost(message, optional, tag = "42")]
  pub uses_codebase_results: Option<stream_unified_chat_request::CodeSearchResult>,
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
  #[prost(bool, optional, tag = "50")]
  pub should_use_chat_prompt: Option<bool>,
  #[prost(string, optional, tag = "55")]
  pub background_composer_id: Option<String>,
  #[prost(bool, optional, tag = "51")]
  pub uses_rules: Option<bool>,
  #[prost(bool, optional, tag = "53")]
  pub mode_uses_auto_apply: Option<bool>,
  #[prost(string, optional, tag = "54")]
  pub unified_mode_name: Option<String>,
  #[prost(bool, optional, tag = "56")]
  pub use_generate_rules_prompt: Option<bool>,
  #[prost(bool, optional, tag = "57")]
  pub edit_tool_supports_search_and_replace: Option<bool>,
  #[prost(message, repeated, tag = "58")]
  pub project_layouts: Vec<ProjectLayout>,
  #[prost(string, optional, tag = "59")]
  pub repository_name_if_unindexed: Option<String>,
  #[prost(double, optional, tag = "60")]
  pub indexing_progress: Option<f64>,
  #[prost(message, optional, tag = "61")]
  pub full_file_cmd_k_options: Option<stream_unified_chat_request::FullFileCmdKOptions>,
  #[prost(string, optional, tag = "62")]
  pub indexing_phase_if_unindexed: Option<String>,
  #[prost(bool, optional, tag = "63")]
  pub use_knowledge_base_prompt: Option<bool>,
  #[prost(int32, optional, tag = "64")]
  pub indexing_num_files_if_unindexed: Option<i32>,
  #[prost(bool, optional, tag = "65")]
  pub supports_mermaid_diagrams: Option<bool>,
  #[prost(message, optional, tag = "66")]
  pub subagent_info: Option<SubagentInfo>,
  #[prost(bool, tag = "67")]
  pub supports_git_index: bool,
  #[prost(bool, tag = "69")]
  pub force_is_not_dev: bool,
  #[prost(bool, optional, tag = "70")]
  pub disable_edit_file_timeout: Option<bool>,
  #[prost(bool, optional, tag = "71")]
  pub should_attach_linter_errors: Option<bool>,
}
/// Nested message and enum types in `StreamUnifiedChatRequest`.
pub mod stream_unified_chat_request {
  /// aiserver.v1.StreamUnifiedChatRequest.RedDiff
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct RedDiff {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(message, repeated, tag = "2")]
    pub red_ranges: Vec<super::SimplestRange>,
    #[prost(message, repeated, tag = "3")]
    pub red_ranges_reversed: Vec<super::SimplestRange>,
    #[prost(string, tag = "4")]
    pub start_hash: String,
    #[prost(string, tag = "5")]
    pub end_hash: String,
  }
  /// aiserver.v1.StreamUnifiedChatRequest.RecentEdits
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct RecentEdits {
    #[prost(message, repeated, tag = "1")]
    pub code_block_info: Vec<recent_edits::CodeBlockInfo>,
    #[prost(message, repeated, tag = "2")]
    pub final_file_values: Vec<recent_edits::FileInfo>,
    #[prost(string, optional, tag = "3")]
    pub edits_belong_to_composer_generation_uuid: Option<String>,
  }
  /// Nested message and enum types in `RecentEdits`.
  pub mod recent_edits {
    /// aiserver.v1.StreamUnifiedChatRequest.RecentEdits.CodeBlockInfo
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct CodeBlockInfo {
      #[prost(string, tag = "1")]
      pub relative_workspace_path: String,
      #[prost(string, optional, tag = "2")]
      pub content_before: Option<String>,
      #[prost(string, optional, tag = "3")]
      pub content_after: Option<String>,
      #[prost(string, optional, tag = "4")]
      pub generation_uuid: Option<String>,
      #[prost(int32, optional, tag = "5")]
      pub version: Option<i32>,
    }
    /// aiserver.v1.StreamUnifiedChatRequest.RecentEdits.FileInfo
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct FileInfo {
      #[prost(string, tag = "1")]
      pub relative_workspace_path: String,
      #[prost(string, tag = "2")]
      pub content: String,
    }
  }
  /// aiserver.v1.StreamUnifiedChatRequest.CodeSearchResult
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct CodeSearchResult {
    #[prost(message, repeated, tag = "1")]
    pub results: Vec<super::CodeResult>,
    #[prost(message, repeated, tag = "2")]
    pub all_files: Vec<super::File>,
  }
  /// aiserver.v1.StreamUnifiedChatRequest.FullFileCmdKOptions
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct FullFileCmdKOptions {
    #[prost(string, tag = "1")]
    pub file_path: String,
  }
  /// aiserver.v1.StreamUnifiedChatRequest.UnifiedMode
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum UnifiedMode {
    Unspecified = 0,
    Chat = 1,
    Agent = 2,
    Edit = 3,
    Custom = 4,
  }
  /// aiserver.v1.StreamUnifiedChatRequest.ThinkingLevel
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum ThinkingLevel {
    Unspecified = 0,
    Medium = 1,
    High = 2,
  }
}
/// aiserver.v1.ContextPiece
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextPiece {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(string, tag = "2")]
  pub content: String,
  #[prost(float, tag = "3")]
  pub score: f32,
}
/// aiserver.v1.ContextPieceUpdate
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ContextPieceUpdate {
  #[prost(message, repeated, tag = "1")]
  pub pieces: Vec<ContextPiece>,
}
/// aiserver.v1.StreamUnifiedChatResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StreamUnifiedChatResponse {
  #[prost(string, tag = "1")]
  pub text: String,
  #[prost(string, optional, tag = "22")]
  pub server_bubble_id: Option<String>,
  #[prost(string, optional, tag = "2")]
  pub debugging_only_chat_prompt: Option<String>,
  #[prost(int32, optional, tag = "3")]
  pub debugging_only_token_count: Option<i32>,
  #[prost(message, optional, tag = "4")]
  pub document_citation: Option<DocumentationCitation>,
  #[prost(string, optional, tag = "5")]
  pub filled_prompt: Option<String>,
  #[prost(bool, optional, tag = "6")]
  pub is_big_file: Option<bool>,
  #[prost(string, optional, tag = "7")]
  pub intermediate_text: Option<String>,
  #[prost(bool, optional, tag = "10")]
  pub is_using_slow_request: Option<bool>,
  #[prost(message, optional, tag = "8")]
  pub chunk_identity: Option<stream_unified_chat_response::ChunkIdentity>,
  #[prost(message, optional, tag = "9")]
  pub docs_reference: Option<DocsReference>,
  #[prost(message, optional, tag = "11")]
  pub web_citation: Option<WebCitation>,
  #[prost(message, optional, tag = "12")]
  pub status_updates: Option<StatusUpdates>,
  #[prost(message, optional, tag = "13")]
  pub tool_call: Option<StreamedBackToolCall>,
  #[prost(bool, optional, tag = "14")]
  pub should_break_ai_message: Option<bool>,
  #[prost(message, optional, tag = "15")]
  pub partial_tool_call: Option<StreamedBackPartialToolCall>,
  #[prost(message, optional, tag = "16")]
  pub final_tool_result: Option<stream_unified_chat_response::FinalToolResult>,
  #[prost(message, optional, tag = "17")]
  pub symbol_link: Option<SymbolLink>,
  #[prost(message, optional, tag = "19")]
  pub file_link: Option<FileLink>,
  #[prost(message, optional, tag = "18")]
  pub conversation_summary: Option<ConversationSummary>,
  #[prost(message, optional, tag = "20")]
  pub service_status_update: Option<ServiceStatusUpdate>,
  #[prost(message, optional, tag = "21")]
  pub viewable_git_context: Option<ViewableGitContext>,
  #[prost(message, optional, tag = "23")]
  pub context_piece_update: Option<ContextPieceUpdate>,
  #[prost(message, optional, tag = "24")]
  pub used_code: Option<stream_unified_chat_response::UsedCode>,
  #[prost(message, optional, tag = "25")]
  pub thinking: Option<conversation_message::Thinking>,
  #[prost(bool, optional, tag = "26")]
  pub stop_using_dsv3_agentic_model: Option<bool>,
  #[prost(string, optional, tag = "27")]
  pub usage_uuid: Option<String>,
  #[prost(message, optional, tag = "28")]
  pub conversation_summary_starter: Option<ConversationSummaryStarter>,
  #[prost(message, optional, tag = "29")]
  pub subagent_return: Option<SubagentReturnCall>,
  #[prost(message, optional, tag = "30")]
  pub context_window_status: Option<ContextWindowStatus>,
  #[prost(message, optional, tag = "31")]
  pub image_description: Option<stream_unified_chat_response::ImageDescription>,
}
/// Nested message and enum types in `StreamUnifiedChatResponse`.
pub mod stream_unified_chat_response {
  /// aiserver.v1.StreamUnifiedChatResponse.UsedCode
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct UsedCode {
    #[prost(message, repeated, tag = "1")]
    pub code_results: Vec<super::CodeResult>,
  }
  /// aiserver.v1.StreamUnifiedChatResponse.ChunkIdentity
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ChunkIdentity {
    #[prost(string, tag = "1")]
    pub file_name: String,
    #[prost(int32, tag = "2")]
    pub start_line: i32,
    #[prost(int32, tag = "3")]
    pub end_line: i32,
    #[prost(string, tag = "4")]
    pub text: String,
    #[prost(enumeration = "super::ChunkType", tag = "5")]
    pub chunk_type: i32,
  }
  /// aiserver.v1.StreamUnifiedChatResponse.FinalToolResult
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct FinalToolResult {
    #[prost(string, tag = "1")]
    pub tool_call_id: String,
    #[prost(message, optional, tag = "2")]
    pub result: Option<super::ClientSideToolV2Result>,
  }
  /// aiserver.v1.StreamUnifiedChatResponse.ImageDescription
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ImageDescription {
    #[prost(string, tag = "1")]
    pub description: String,
    #[prost(string, tag = "2")]
    pub image_uuid: String,
  }
}
/// aiserver.v1.ContextWindowStatus
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ContextWindowStatus {
  #[prost(int32, tag = "1")]
  pub percentage_remaining: i32,
  #[prost(int32, optional, tag = "2")]
  pub tokens_used: Option<i32>,
  #[prost(int32, optional, tag = "3")]
  pub token_limit: Option<i32>,
}
/// aiserver.v1.ConversationSummaryStarter
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ConversationSummaryStarter {
  #[prost(string, tag = "1")]
  pub message: String,
}
/// aiserver.v1.ServiceStatusUpdate
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ServiceStatusUpdate {
  #[prost(string, tag = "1")]
  pub message: String,
  #[prost(string, tag = "2")]
  pub codicon: String,
  #[prost(bool, optional, tag = "3")]
  pub allow_command_links_potentially_unsafe_please_only_use_for_handwritten_trusted_markdown:
    Option<bool>,
  #[prost(string, optional, tag = "4")]
  pub action_to_run_on_status_update: Option<String>,
}
/// aiserver.v1.SymbolLink
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SymbolLink {
  #[prost(string, tag = "1")]
  pub symbol_name: String,
  #[prost(string, tag = "2")]
  pub symbol_search_string: String,
  #[prost(string, tag = "3")]
  pub relative_workspace_path: String,
  #[prost(int32, tag = "4")]
  pub rough_line_number: i32,
}
/// aiserver.v1.FileLink
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FileLink {
  #[prost(string, tag = "1")]
  pub display_name: String,
  #[prost(string, tag = "2")]
  pub relative_workspace_path: String,
}
/// aiserver.v1.RedDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RedDiff {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(message, repeated, tag = "2")]
  pub red_ranges: Vec<SimplestRange>,
  #[prost(message, repeated, tag = "3")]
  pub red_ranges_reversed: Vec<SimplestRange>,
  #[prost(string, tag = "4")]
  pub start_hash: String,
  #[prost(string, tag = "5")]
  pub end_hash: String,
}
/// aiserver.v1.ConversationMessageHeader
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ConversationMessageHeader {
  #[prost(string, tag = "1")]
  pub bubble_id: String,
  #[prost(string, optional, tag = "2")]
  pub server_bubble_id: Option<String>,
  #[prost(enumeration = "conversation_message::MessageType", tag = "3")]
  pub r#type: i32,
}
/// aiserver.v1.DiffFile
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DiffFile {
  #[prost(string, tag = "1")]
  pub file_details: String,
  #[prost(string, tag = "2")]
  pub file_name: String,
}
/// aiserver.v1.ViewableCommitProps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableCommitProps {
  #[prost(string, tag = "1")]
  pub description: String,
  #[prost(string, tag = "2")]
  pub message: String,
  #[prost(message, repeated, tag = "3")]
  pub files: Vec<DiffFile>,
}
/// aiserver.v1.ViewablePRProps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewablePrProps {
  #[prost(string, tag = "1")]
  pub title: String,
  #[prost(string, tag = "2")]
  pub body: String,
  #[prost(message, repeated, tag = "3")]
  pub files: Vec<DiffFile>,
}
/// aiserver.v1.ViewableDiffProps
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableDiffProps {
  #[prost(message, repeated, tag = "1")]
  pub files: Vec<DiffFile>,
  #[prost(string, tag = "2")]
  pub diff_preface: String,
}
/// aiserver.v1.ViewableGitContext
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ViewableGitContext {
  #[prost(message, optional, tag = "1")]
  pub commit_data: Option<ViewableCommitProps>,
  #[prost(message, optional, tag = "2")]
  pub pull_request_data: Option<ViewablePrProps>,
  #[prost(message, repeated, tag = "3")]
  pub diff_data: Vec<ViewableDiffProps>,
}
/// aiserver.v1.ConversationMessage
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConversationMessage {
  #[prost(string, tag = "1")]
  pub text: String,
  #[prost(enumeration = "conversation_message::MessageType", tag = "2")]
  pub r#type: i32,
  #[prost(message, repeated, tag = "3")]
  pub attached_code_chunks: Vec<conversation_message::CodeChunk>,
  #[prost(message, repeated, tag = "4")]
  pub codebase_context_chunks: Vec<CodeBlock>,
  #[prost(message, repeated, tag = "5")]
  pub commits: Vec<Commit>,
  #[prost(message, repeated, tag = "6")]
  pub pull_requests: Vec<PullRequest>,
  #[prost(message, repeated, tag = "7")]
  pub git_diffs: Vec<GitDiff>,
  #[prost(message, repeated, tag = "8")]
  pub assistant_suggested_diffs: Vec<SimpleFileDiff>,
  #[prost(message, repeated, tag = "9")]
  pub interpreter_results: Vec<InterpreterResult>,
  #[prost(message, repeated, tag = "10")]
  pub images: Vec<ImageProto>,
  #[prost(string, repeated, tag = "11")]
  pub attached_folders: Vec<String>,
  #[prost(message, repeated, tag = "12")]
  pub approximate_lint_errors: Vec<conversation_message::ApproximateLintError>,
  #[prost(string, tag = "13")]
  pub bubble_id: String,
  #[prost(string, optional, tag = "32")]
  pub server_bubble_id: Option<String>,
  #[prost(message, repeated, tag = "14")]
  pub attached_folders_new: Vec<FolderInfo>,
  #[prost(message, repeated, tag = "15")]
  pub lints: Vec<conversation_message::Lints>,
  #[prost(message, repeated, tag = "16")]
  pub user_responses_to_suggested_code_blocks: Vec<UserResponseToSuggestedCodeBlock>,
  #[prost(string, repeated, tag = "17")]
  pub relevant_files: Vec<String>,
  #[prost(message, repeated, tag = "18")]
  pub tool_results: Vec<conversation_message::ToolResult>,
  #[prost(message, repeated, tag = "19")]
  pub notepads: Vec<conversation_message::NotepadContext>,
  #[prost(bool, optional, tag = "20")]
  pub is_capability_iteration: Option<bool>,
  #[prost(message, repeated, tag = "21")]
  pub capabilities: Vec<ComposerCapabilityRequest>,
  #[prost(message, repeated, tag = "22")]
  pub edit_trail_contexts: Vec<conversation_message::EditTrailContext>,
  #[prost(message, repeated, tag = "23")]
  pub suggested_code_blocks: Vec<SuggestedCodeBlock>,
  #[prost(message, repeated, tag = "24")]
  pub diffs_for_compressing_files: Vec<RedDiff>,
  #[prost(message, repeated, tag = "25")]
  pub multi_file_linter_errors: Vec<LinterErrorsWithoutFileContents>,
  #[prost(message, repeated, tag = "26")]
  pub diff_histories: Vec<DiffHistoryData>,
  #[prost(message, repeated, tag = "27")]
  pub recently_viewed_files: Vec<conversation_message::CodeChunk>,
  #[prost(message, repeated, tag = "28")]
  pub recent_locations_history: Vec<conversation_message::RecentLocation>,
  #[prost(bool, tag = "29")]
  pub is_agentic: bool,
  #[prost(message, repeated, tag = "30")]
  pub file_diff_trajectories: Vec<ComposerFileDiffHistory>,
  #[prost(message, optional, tag = "31")]
  pub conversation_summary: Option<ConversationSummary>,
  #[prost(bool, tag = "33")]
  pub existed_subsequent_terminal_command: bool,
  #[prost(bool, tag = "34")]
  pub existed_previous_terminal_command: bool,
  #[prost(message, repeated, tag = "35")]
  pub docs_references: Vec<DocsReference>,
  #[prost(message, repeated, tag = "36")]
  pub web_references: Vec<WebReference>,
  #[prost(message, optional, tag = "37")]
  pub git_context: Option<ViewableGitContext>,
  #[prost(message, repeated, tag = "38")]
  pub attached_folders_list_dir_results: Vec<ListDirResult>,
  #[prost(message, optional, tag = "39")]
  pub cached_conversation_summary: Option<ConversationSummary>,
  #[prost(message, repeated, tag = "40")]
  pub human_changes: Vec<conversation_message::HumanChange>,
  #[prost(bool, tag = "41")]
  pub attached_human_changes: bool,
  #[prost(message, repeated, tag = "42")]
  pub summarized_composers: Vec<conversation_message::ComposerContext>,
  #[prost(message, repeated, tag = "43")]
  pub cursor_rules: Vec<CursorRule>,
  #[prost(message, repeated, tag = "44")]
  pub context_pieces: Vec<ContextPiece>,
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
  #[prost(message, repeated, tag = "48")]
  pub diffs_since_last_apply: Vec<conversation_message::DiffSinceLastApply>,
  #[prost(message, repeated, tag = "49")]
  pub deleted_files: Vec<conversation_message::DeletedFile>,
  #[prost(string, optional, tag = "50")]
  pub usage_uuid: Option<String>,
  #[prost(enumeration = "ClientSideToolV2", repeated, tag = "51")]
  pub supported_tools: Vec<i32>,
  #[prost(message, optional, tag = "52")]
  pub current_file_location_data: Option<CurrentFileLocationData>,
  #[prost(bool, optional, tag = "53")]
  pub edit_tool_supports_search_and_replace: Option<bool>,
  #[prost(string, optional, tag = "54")]
  pub last_terminal_cwd: Option<String>,
  #[prost(bool, optional, tag = "55")]
  pub user_explicitly_asked_to_generate_cursor_rules: Option<bool>,
  #[prost(message, repeated, tag = "56")]
  pub console_logs: Vec<RcpLogEntry>,
  #[prost(string, optional, tag = "57")]
  pub rich_text: Option<String>,
  #[prost(message, repeated, tag = "58")]
  pub knowledge_items: Vec<conversation_message::KnowledgeItem>,
  #[prost(message, repeated, tag = "59")]
  pub ui_element_picked: Vec<RcpuiElementPicked>,
  #[prost(bool, optional, tag = "60")]
  pub user_explicitly_asked_to_add_to_knowledge_base: Option<bool>,
  #[prost(message, repeated, tag = "61")]
  pub documentation_selections: Vec<conversation_message::DocumentationSelection>,
  #[prost(message, repeated, tag = "62")]
  pub external_links: Vec<ComposerExternalLink>,
  #[prost(bool, optional, tag = "63")]
  pub use_web: Option<bool>,
  #[prost(message, repeated, tag = "64")]
  pub project_layouts: Vec<ProjectLayout>,
  #[prost(int32, optional, tag = "65")]
  pub thinking_duration_ms: Option<i32>,
  #[prost(message, optional, tag = "66")]
  pub subagent_return: Option<SubagentReturnCall>,
  #[prost(bool, optional, tag = "67")]
  pub is_simple_looping_message: Option<bool>,
  #[prost(message, repeated, tag = "68")]
  pub capability_contexts: Vec<ComposerCapabilityContext>,
  #[prost(string, optional, tag = "69")]
  pub checkpoint_commit_hash: Option<String>,
  #[prost(string, optional, tag = "70")]
  pub git_status_raw: Option<String>,
  #[prost(message, repeated, tag = "71")]
  pub todos: Vec<TodoItem>,
  #[prost(bool, optional, tag = "72")]
  pub is_review_edits_followup: Option<bool>,
  #[prost(message, optional, tag = "73")]
  pub ide_editors_state: Option<conversation_message::IdeEditorsState>,
}
/// Nested message and enum types in `ConversationMessage`.
pub mod conversation_message {
  /// aiserver.v1.ConversationMessage.CodeChunk
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
    /// aiserver.v1.ConversationMessage.CodeChunk.CodeChunkGitContext
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct CodeChunkGitContext {
      #[prost(message, repeated, tag = "1")]
      pub git_info: Vec<code_chunk_git_context::CodeChunkGitInfo>,
    }
    /// Nested message and enum types in `CodeChunkGitContext`.
    pub mod code_chunk_git_context {
      /// aiserver.v1.ConversationMessage.CodeChunk.CodeChunkGitContext.CodeChunkGitInfo
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
    /// aiserver.v1.ConversationMessage.CodeChunk.Intent
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    }
    /// aiserver.v1.ConversationMessage.CodeChunk.SummarizationStrategy
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum SummarizationStrategy {
      NoneUnspecified = 0,
      Summarized = 1,
      Embedded = 2,
    }
  }
  /// aiserver.v1.ConversationMessage.ToolResult
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
  /// aiserver.v1.ConversationMessage.MultiRangeCodeChunk
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct MultiRangeCodeChunk {
    #[prost(message, repeated, tag = "1")]
    pub ranges: Vec<multi_range_code_chunk::RangeWithPriority>,
    #[prost(string, tag = "2")]
    pub content: String,
    #[prost(string, tag = "3")]
    pub relative_workspace_path: String,
  }
  /// Nested message and enum types in `MultiRangeCodeChunk`.
  pub mod multi_range_code_chunk {
    /// aiserver.v1.ConversationMessage.MultiRangeCodeChunk.RangeWithPriority
    #[derive(Clone, Copy, PartialEq, ::prost::Message)]
    pub struct RangeWithPriority {
      #[prost(message, optional, tag = "1")]
      pub range: Option<super::super::SimplestRange>,
      #[prost(double, tag = "2")]
      pub priority: f64,
    }
  }
  /// aiserver.v1.ConversationMessage.NotepadContext
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct NotepadContext {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(string, tag = "2")]
    pub text: String,
    #[prost(message, repeated, tag = "3")]
    pub attached_code_chunks: Vec<CodeChunk>,
    #[prost(string, repeated, tag = "4")]
    pub attached_folders: Vec<String>,
    #[prost(message, repeated, tag = "5")]
    pub commits: Vec<super::Commit>,
    #[prost(message, repeated, tag = "6")]
    pub pull_requests: Vec<super::PullRequest>,
    #[prost(message, repeated, tag = "7")]
    pub git_diffs: Vec<super::GitDiff>,
    #[prost(message, repeated, tag = "8")]
    pub images: Vec<super::ImageProto>,
  }
  /// aiserver.v1.ConversationMessage.ComposerContext
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ComposerContext {
    #[prost(string, tag = "1")]
    pub name: String,
    #[prost(message, optional, tag = "2")]
    pub conversation_summary: Option<super::ConversationSummary>,
  }
  /// aiserver.v1.ConversationMessage.EditLocation
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct EditLocation {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(message, optional, tag = "3")]
    pub range: Option<super::SimplestRange>,
    #[prost(message, optional, tag = "4")]
    pub initial_range: Option<super::SimplestRange>,
    #[prost(string, tag = "5")]
    pub context_lines: String,
    #[prost(string, tag = "6")]
    pub text: String,
    #[prost(message, optional, tag = "7")]
    pub text_range: Option<super::SimplestRange>,
  }
  /// aiserver.v1.ConversationMessage.EditTrailContext
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct EditTrailContext {
    #[prost(string, tag = "1")]
    pub unique_id: String,
    #[prost(message, repeated, tag = "2")]
    pub edit_trail_sorted: Vec<EditLocation>,
  }
  /// aiserver.v1.ConversationMessage.ApproximateLintError
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ApproximateLintError {
    #[prost(string, tag = "1")]
    pub message: String,
    #[prost(string, tag = "2")]
    pub value: String,
    #[prost(int32, tag = "3")]
    pub start_line: i32,
    #[prost(int32, tag = "4")]
    pub end_line: i32,
    #[prost(int32, tag = "5")]
    pub start_column: i32,
    #[prost(int32, tag = "6")]
    pub end_column: i32,
  }
  /// aiserver.v1.ConversationMessage.Lints
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct Lints {
    #[prost(message, optional, tag = "1")]
    pub lints: Option<super::GetLintsForChangeResponse>,
    #[prost(string, tag = "2")]
    pub chat_codeblock_model_value: String,
  }
  /// aiserver.v1.ConversationMessage.RecentLocation
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct RecentLocation {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(int32, tag = "2")]
    pub line_number: i32,
  }
  /// aiserver.v1.ConversationMessage.RenderedDiff
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct RenderedDiff {
    #[prost(int32, tag = "1")]
    pub start_line_number: i32,
    #[prost(int32, tag = "2")]
    pub end_line_number_exclusive: i32,
    #[prost(string, repeated, tag = "3")]
    pub before_context_lines: Vec<String>,
    #[prost(string, repeated, tag = "4")]
    pub removed_lines: Vec<String>,
    #[prost(string, repeated, tag = "5")]
    pub added_lines: Vec<String>,
    #[prost(string, repeated, tag = "6")]
    pub after_context_lines: Vec<String>,
  }
  /// aiserver.v1.ConversationMessage.HumanChange
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct HumanChange {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(message, repeated, tag = "2")]
    pub rendered_diffs: Vec<RenderedDiff>,
  }
  /// aiserver.v1.ConversationMessage.Thinking
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
  /// aiserver.v1.ConversationMessage.DiffSinceLastApply
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct DiffSinceLastApply {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
    #[prost(message, optional, tag = "2")]
    pub diff: Option<super::edit_file_result::FileDiff>,
    #[prost(bool, optional, tag = "4")]
    pub is_accepted: Option<bool>,
    #[prost(bool, optional, tag = "5")]
    pub is_rejected: Option<bool>,
    #[prost(int32, optional, tag = "6")]
    pub last_apply_chained_from_n_human_messages_ago: Option<i32>,
  }
  /// aiserver.v1.ConversationMessage.DeletedFile
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct DeletedFile {
    #[prost(string, tag = "1")]
    pub relative_workspace_path: String,
  }
  /// aiserver.v1.ConversationMessage.KnowledgeItem
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct KnowledgeItem {
    #[prost(string, tag = "1")]
    pub title: String,
    #[prost(string, tag = "2")]
    pub knowledge: String,
    #[prost(string, tag = "3")]
    pub knowledge_id: String,
    #[prost(bool, tag = "4")]
    pub is_generated: bool,
  }
  /// aiserver.v1.ConversationMessage.DocumentationSelection
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct DocumentationSelection {
    #[prost(string, tag = "1")]
    pub doc_id: String,
    #[prost(string, tag = "2")]
    pub name: String,
  }
  /// aiserver.v1.ConversationMessage.IdeEditorsState
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct IdeEditorsState {
    #[prost(bool, tag = "1")]
    pub is_pill_displayed: bool,
    #[prost(string, repeated, tag = "2")]
    pub visible_file_paths: Vec<String>,
    #[prost(string, repeated, tag = "3")]
    pub recently_viewed_file_paths: Vec<String>,
    #[prost(message, repeated, tag = "4")]
    pub visible_files: Vec<ide_editors_state::File>,
    #[prost(message, repeated, tag = "5")]
    pub recently_viewed_files: Vec<ide_editors_state::File>,
  }
  /// Nested message and enum types in `IdeEditorsState`.
  pub mod ide_editors_state {
    /// aiserver.v1.ConversationMessage.IdeEditorsState.File
    #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
    pub struct File {
      #[prost(string, tag = "1")]
      pub relative_path: String,
      #[prost(bool, optional, tag = "2")]
      pub is_currently_focused: Option<bool>,
      #[prost(int32, optional, tag = "3")]
      pub current_line_number: Option<i32>,
      #[prost(string, optional, tag = "4")]
      pub current_line_text: Option<String>,
      #[prost(int32, optional, tag = "5")]
      pub line_count: Option<i32>,
    }
  }
  /// aiserver.v1.ConversationMessage.MessageType
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum MessageType {
    Unspecified = 0,
    Human = 1,
    Ai = 2,
  }
}
/// aiserver.v1.CurrentFileLocationData
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CurrentFileLocationData {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(int32, tag = "2")]
  pub line_number: i32,
  #[prost(string, tag = "3")]
  pub text: String,
}
/// aiserver.v1.FolderInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderInfo {
  #[prost(string, tag = "1")]
  pub relative_path: String,
  #[prost(message, repeated, tag = "2")]
  pub files: Vec<FolderFileInfo>,
}
/// aiserver.v1.FolderFileInfo
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FolderFileInfo {
  #[prost(string, tag = "1")]
  pub relative_path: String,
  #[prost(string, tag = "2")]
  pub content: String,
  #[prost(bool, tag = "3")]
  pub truncated: bool,
  #[prost(float, tag = "4")]
  pub score: f32,
}
/// aiserver.v1.InterpreterResult
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct InterpreterResult {
  #[prost(string, tag = "1")]
  pub output: String,
  #[prost(bool, tag = "2")]
  pub success: bool,
}
/// aiserver.v1.SimpleFileDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SimpleFileDiff {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(message, repeated, tag = "3")]
  pub chunks: Vec<simple_file_diff::Chunk>,
}
/// Nested message and enum types in `SimpleFileDiff`.
pub mod simple_file_diff {
  /// aiserver.v1.SimpleFileDiff.Chunk
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct Chunk {
    #[prost(string, repeated, tag = "1")]
    pub old_lines: Vec<String>,
    #[prost(string, repeated, tag = "2")]
    pub new_lines: Vec<String>,
    #[prost(message, optional, tag = "3")]
    pub old_range: Option<super::LineRange>,
    #[prost(message, optional, tag = "4")]
    pub new_range: Option<super::LineRange>,
  }
}
/// aiserver.v1.Commit
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commit {
  #[prost(string, tag = "1")]
  pub sha: String,
  #[prost(string, tag = "2")]
  pub message: String,
  #[prost(string, tag = "3")]
  pub description: String,
  #[prost(message, repeated, tag = "4")]
  pub diff: Vec<FileDiff>,
  #[prost(string, tag = "5")]
  pub author: String,
  #[prost(string, tag = "6")]
  pub date: String,
}
/// aiserver.v1.PullRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PullRequest {
  #[prost(string, tag = "1")]
  pub title: String,
  #[prost(string, tag = "2")]
  pub body: String,
  #[prost(message, repeated, tag = "3")]
  pub diff: Vec<FileDiff>,
  #[prost(int64, tag = "4")]
  pub id: i64,
  #[prost(int64, tag = "5")]
  pub number: i64,
}
/// aiserver.v1.SuggestedCodeBlock
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SuggestedCodeBlock {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
}
/// aiserver.v1.UserResponseToSuggestedCodeBlock
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserResponseToSuggestedCodeBlock {
  #[prost(
    enumeration = "user_response_to_suggested_code_block::UserResponseType",
    tag = "1"
  )]
  pub user_response_type: i32,
  #[prost(string, tag = "2")]
  pub file_path: String,
  #[prost(message, optional, tag = "3")]
  pub user_modifications_to_suggested_code_blocks: Option<FileDiff>,
}
/// Nested message and enum types in `UserResponseToSuggestedCodeBlock`.
pub mod user_response_to_suggested_code_block {
  /// aiserver.v1.UserResponseToSuggestedCodeBlock.UserResponseType
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum UserResponseType {
    Unspecified = 0,
    Accept = 1,
    Reject = 2,
    Modify = 3,
  }
}
/// aiserver.v1.ComposerFileDiff
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ComposerFileDiff {
  #[prost(message, repeated, tag = "1")]
  pub chunks: Vec<composer_file_diff::ChunkDiff>,
  #[prost(enumeration = "composer_file_diff::Editor", tag = "2")]
  pub editor: i32,
  #[prost(bool, tag = "3")]
  pub hit_timeout: bool,
}
/// Nested message and enum types in `ComposerFileDiff`.
pub mod composer_file_diff {
  /// aiserver.v1.ComposerFileDiff.ChunkDiff
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ChunkDiff {
    #[prost(string, tag = "1")]
    pub diff_string: String,
    #[prost(int32, tag = "2")]
    pub old_start: i32,
    #[prost(int32, tag = "3")]
    pub new_start: i32,
    #[prost(int32, tag = "4")]
    pub old_lines: i32,
    #[prost(int32, tag = "5")]
    pub new_lines: i32,
    #[prost(int32, tag = "6")]
    pub lines_removed: i32,
    #[prost(int32, tag = "7")]
    pub lines_added: i32,
  }
  /// aiserver.v1.ComposerFileDiff.Editor
  #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
  #[repr(i32)]
  pub enum Editor {
    Unspecified = 0,
    Ai = 1,
    Human = 2,
  }
}
/// aiserver.v1.DiffHistoryData
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DiffHistoryData {
  #[prost(string, tag = "1")]
  pub relative_workspace_path: String,
  #[prost(message, repeated, tag = "2")]
  pub diffs: Vec<ComposerFileDiff>,
  #[prost(double, tag = "3")]
  pub timestamp: f64,
  #[prost(string, tag = "4")]
  pub unique_id: String,
  #[prost(message, optional, tag = "5")]
  pub start_to_end_diff: Option<ComposerFileDiff>,
}
/// aiserver.v1.SubagentReturnCall
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubagentReturnCall {
  #[prost(enumeration = "SubagentType", tag = "1")]
  pub subagent_type: i32,
  #[prost(oneof = "subagent_return_call::ReturnValue", tags = "2, 3, 4")]
  pub return_value: Option<subagent_return_call::ReturnValue>,
}
/// Nested message and enum types in `SubagentReturnCall`.
pub mod subagent_return_call {
  #[derive(Clone, PartialEq, ::prost::Oneof)]
  pub enum ReturnValue {
    #[prost(message, tag = "2")]
    DeepSearchReturnValue(super::DeepSearchSubagentReturnValue),
    #[prost(message, tag = "3")]
    FixLintsReturnValue(super::FixLintsSubagentReturnValue),
    #[prost(message, tag = "4")]
    TaskReturnValue(super::TaskSubagentReturnValue),
  }
}
/// aiserver.v1.SubagentInfo
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct SubagentInfo {
  #[prost(enumeration = "SubagentType", tag = "1")]
  pub subagent_type: i32,
  #[prost(string, tag = "2")]
  pub subagent_id: String,
  #[prost(string, optional, tag = "5")]
  pub parent_request_id: Option<String>,
  #[prost(oneof = "subagent_info::Params", tags = "3, 4, 6")]
  pub params: Option<subagent_info::Params>,
}
/// Nested message and enum types in `SubagentInfo`.
pub mod subagent_info {
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Oneof)]
  pub enum Params {
    #[prost(message, tag = "3")]
    DeepSearchParams(super::DeepSearchSubagentParams),
    #[prost(message, tag = "4")]
    FixLintsParams(super::FixLintsSubagentParams),
    #[prost(message, tag = "6")]
    TaskParams(super::TaskSubagentParams),
  }
}
/// aiserver.v1.DeepSearchSubagentParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct DeepSearchSubagentParams {
  #[prost(string, tag = "1")]
  pub query: String,
}
/// aiserver.v1.DeepSearchSubagentReturnValue
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeepSearchSubagentReturnValue {
  #[prost(message, repeated, tag = "1")]
  pub context_items: Vec<deep_search_subagent_return_value::ContextItem>,
}
/// Nested message and enum types in `DeepSearchSubagentReturnValue`.
pub mod deep_search_subagent_return_value {
  /// aiserver.v1.DeepSearchSubagentReturnValue.ContextItem
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ContextItem {
    #[prost(string, tag = "1")]
    pub file: String,
    #[prost(message, optional, tag = "2")]
    pub line_range: Option<super::LineRange>,
    #[prost(string, tag = "3")]
    pub explanation: String,
  }
}
/// aiserver.v1.FixLintsSubagentParams
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FixLintsSubagentParams {}
/// aiserver.v1.FixLintsSubagentReturnValue
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FixLintsSubagentReturnValue {}
/// aiserver.v1.TaskSubagentParams
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TaskSubagentParams {
  #[prost(string, tag = "1")]
  pub task_description: String,
  #[prost(string, repeated, tag = "2")]
  pub allowed_write_directories: Vec<String>,
}
/// aiserver.v1.TaskSubagentReturnValue
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TaskSubagentReturnValue {
  #[prost(string, tag = "1")]
  pub summary: String,
}
/// aiserver.v1.UsageEventDetails
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UsageEventDetails {
  #[serde(alias = "overrideNumRequestsCounted")]
  #[prost(int32, optional, tag = "13")]
  pub override_num_requests_counted: Option<i32>,
  #[serde(flatten)]
  #[prost(
    oneof = "usage_event_details::Feature",
    tags = "1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 14"
  )]
  pub feature: Option<usage_event_details::Feature>,
}
/// Nested message and enum types in `UsageEventDetails`.
pub mod usage_event_details {
  /// aiserver.v1.UsageEventDetails.BugFinderTriggerV1
  #[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct BugFinderTriggerV1 {
    #[serde(alias = "inBackgroundSubsidized")]
    #[prost(bool, tag = "1")]
    pub in_background_subsidized: bool,
    #[serde(alias = "costCents")]
    #[prost(int32, tag = "2")]
    pub cost_cents: i32,
    #[serde(alias = "isFast")]
    #[prost(bool, tag = "3")]
    pub is_fast: bool,
  }
  /// aiserver.v1.UsageEventDetails.BugBot
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct BugBot {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "2")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, tag = "3")]
    pub is_token_based_call: bool,
    #[serde(alias = "maxMode")]
    #[prost(bool, tag = "4")]
    pub max_mode: bool,
    #[serde(flatten)]
    #[prost(oneof = "bug_bot::Discount", tags = "5, 6")]
    pub discount: Option<bug_bot::Discount>,
  }
  /// Nested message and enum types in `BugBot`.
  pub mod bug_bot {
    #[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Oneof)]
    #[serde(rename_all = "snake_case")]
    pub enum Discount {
      #[serde(alias = "noDiscount")]
      #[prost(message, tag = "5")]
      NoDiscount(()),
      #[prost(message, tag = "6")]
      Free(()),
    }
  }
  /// aiserver.v1.UsageEventDetails.Chat
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct Chat {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "3")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "4")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "5")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.FastApply
  #[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct FastApply {
    #[serde(alias = "isOptimistic")]
    #[prost(bool, tag = "1")]
    pub is_optimistic: bool,
    #[serde(alias = "willingToPayExtraForSpeed")]
    #[prost(bool, tag = "2")]
    pub willing_to_pay_extra_for_speed: bool,
  }
  /// aiserver.v1.UsageEventDetails.Composer
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct Composer {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isHeadless")]
    #[prost(bool, optional, tag = "3")]
    pub is_headless: Option<bool>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "4")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "5")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "6")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.ToolCallComposer
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct ToolCallComposer {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isHeadless")]
    #[prost(bool, optional, tag = "3")]
    pub is_headless: Option<bool>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "4")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "5")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "6")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.WarmComposer
  #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct WarmComposer {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "2")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.ContextChat
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct ContextChat {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "3")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "4")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "5")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.CmdK
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct CmdK {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "3")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "4")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "5")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.TerminalCmdK
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct TerminalCmdK {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "3")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "4")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "5")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.AiReviewAcceptedComment
  #[derive(::serde::Deserialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct AiReviewAcceptedComment {}
  /// aiserver.v1.UsageEventDetails.InterpreterChat
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct InterpreterChat {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "overrideNumRequestsCounted")]
    #[prost(int32, optional, tag = "2")]
    pub override_num_requests_counted: Option<i32>,
    #[serde(alias = "isTokenBasedCall")]
    #[prost(bool, optional, tag = "3")]
    pub is_token_based_call: Option<bool>,
    #[serde(alias = "tokenUsage")]
    #[prost(message, optional, tag = "4")]
    pub token_usage: Option<super::TokenUsage>,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "5")]
    pub max_mode: Option<bool>,
  }
  /// aiserver.v1.UsageEventDetails.SlashEdit
  #[derive(::serde::Deserialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct SlashEdit {
    #[serde(alias = "modelIntent")]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "maxMode")]
    #[prost(bool, optional, tag = "2")]
    pub max_mode: Option<bool>,
  }
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Oneof)]
  #[serde(rename_all = "snake_case")]
  pub enum Feature {
    #[prost(message, tag = "1")]
    Chat(Chat),
    #[serde(rename = "contextChat")]
    #[prost(message, tag = "2")]
    ContextChat(ContextChat),
    #[serde(rename = "cmdK")]
    #[prost(message, tag = "3")]
    CmdK(CmdK),
    #[serde(rename = "terminalCmdK")]
    #[prost(message, tag = "4")]
    TerminalCmdK(TerminalCmdK),
    #[serde(rename = "aiReviewAcceptedComment")]
    #[prost(message, tag = "5")]
    AiReviewAcceptedComment(AiReviewAcceptedComment),
    #[serde(rename = "interpreterChat")]
    #[prost(message, tag = "6")]
    InterpreterChat(InterpreterChat),
    #[serde(rename = "slashEdit")]
    #[prost(message, tag = "7")]
    SlashEdit(SlashEdit),
    #[prost(message, tag = "8")]
    Composer(Composer),
    #[serde(rename = "fastApply")]
    #[prost(message, tag = "9")]
    FastApply(FastApply),
    #[serde(rename = "warmComposer")]
    #[prost(message, tag = "10")]
    WarmComposer(WarmComposer),
    #[serde(rename = "bugFinderTriggerV1")]
    #[prost(message, tag = "11")]
    BugFinderTriggerV1(BugFinderTriggerV1),
    #[serde(rename = "toolCallComposer")]
    #[prost(message, tag = "12")]
    ToolCallComposer(ToolCallComposer),
    #[serde(rename = "bugBot")]
    #[prost(message, tag = "14")]
    BugBot(BugBot),
  }
}
/// aiserver.v1.UsageEvent
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UsageEvent {
  #[prost(int64, tag = "1")]
  pub timestamp: i64,
  #[prost(message, optional, tag = "2")]
  pub details: Option<UsageEventDetails>,
  #[serde(alias = "subscriptionProductId")]
  #[prost(string, optional, tag = "3")]
  pub subscription_product_id: Option<String>,
  #[serde(alias = "usagePriceId")]
  #[prost(string, optional, tag = "4")]
  pub usage_price_id: Option<String>,
  #[serde(alias = "isSlow")]
  #[prost(bool, tag = "5")]
  pub is_slow: bool,
  #[prost(string, tag = "6")]
  pub status: String,
  #[serde(alias = "owningUser")]
  #[prost(string, optional, tag = "7")]
  pub owning_user: Option<String>,
  #[serde(alias = "owningTeam")]
  #[prost(string, optional, tag = "8")]
  pub owning_team: Option<String>,
  #[serde(alias = "priceCents")]
  #[prost(float, optional, tag = "9")]
  pub price_cents: Option<f32>,
}
/// aiserver.v1.UsageEventDisplay
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct UsageEventDisplay {
  #[prost(int64, tag = "1")]
  pub timestamp: i64,
  #[prost(string, tag = "2")]
  pub model: String,
  #[serde(with = "usage_event_kind")]
  #[prost(enumeration = "UsageEventKind", tag = "3")]
  pub kind: i32,
  #[serde(alias = "customSubscriptionName")]
  #[prost(string, optional, tag = "4")]
  pub custom_subscription_name: Option<String>,
  #[serde(alias = "maxMode", default)]
  #[prost(bool, tag = "5")]
  pub max_mode: bool,
  #[serde(alias = "requestsCosts", default)]
  #[prost(float, tag = "6")]
  pub requests_costs: f32,
  #[serde(alias = "usageBasedCosts")]
  #[prost(string, optional, tag = "7")]
  pub usage_based_costs: Option<String>,
  #[serde(alias = "isTokenBasedCall")]
  #[prost(bool, optional, tag = "8")]
  pub is_token_based_call: Option<bool>,
  #[serde(alias = "tokenUsage")]
  #[prost(message, optional, tag = "9")]
  pub token_usage: Option<TokenUsage>,
  #[serde(alias = "owningUser")]
  #[prost(string, optional, tag = "10")]
  pub owning_user: Option<String>,
  #[serde(alias = "owningTeam")]
  #[prost(string, optional, tag = "11")]
  pub owning_team: Option<String>,
  #[serde(alias = "userEmail")]
  #[prost(string, optional, tag = "12")]
  pub user_email: Option<String>,
}
/// aiserver.v1.TokenUsage
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
/// aiserver.v1.AvailableModelsRequest
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
/// aiserver.v1.AvailableModelsResponse
#[derive(::serde::Serialize, Clone, PartialEq, ::prost::Message)]
pub struct AvailableModelsResponse {
  #[prost(message, repeated, tag = "2")]
  pub models: Vec<available_models_response::AvailableModel>,
  #[prost(string, repeated, tag = "1")]
  pub model_names: Vec<String>,
}
/// Nested message and enum types in `AvailableModelsResponse`.
pub mod available_models_response {
  /// aiserver.v1.AvailableModelsResponse.TooltipData
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
  /// aiserver.v1.AvailableModelsResponse.AvailableModel
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
  /// aiserver.v1.AvailableModelsResponse.DegradationStatus
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
        &value
          .map(|val| super::DegradationStatus::try_from(val).ok())
          .flatten(),
        serializer,
      )
    }
  }
}
/// aiserver.v1.Team
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct Team {
  #[prost(string, tag = "1")]
  pub name: String,
  #[prost(int32, tag = "2")]
  pub id: i32,
  #[prost(enumeration = "TeamRole", tag = "3")]
  pub role: i32,
  #[prost(int32, tag = "4")]
  pub seats: i32,
  #[prost(bool, tag = "5")]
  pub has_billing: bool,
  #[prost(int32, tag = "6")]
  pub request_quota_per_seat: i32,
  #[prost(bool, tag = "7")]
  pub privacy_mode_forced: bool,
  #[prost(bool, tag = "8")]
  pub allow_sso: bool,
  #[prost(bool, tag = "9")]
  pub admin_only_usage_pricing: bool,
  #[prost(string, tag = "10")]
  pub subscription_status: String,
  #[prost(string, tag = "11")]
  pub bedrock_iam_role: String,
  #[prost(bool, tag = "12")]
  pub verified: bool,
  #[prost(bool, tag = "13")]
  pub is_enterprise: bool,
  #[prost(bool, tag = "14")]
  pub privacy_mode_migration_opted_out: bool,
  #[prost(string, tag = "15")]
  pub bedrock_external_id: String,
  #[prost(string, tag = "16")]
  pub membership_type: String,
}
/// aiserver.v1.GetTeamsResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTeamsResponse {
  #[prost(message, repeated, tag = "1")]
  pub teams: Vec<Team>,
}
/// aiserver.v1.GetTeamMembersRequest
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GetTeamMembersRequest {
  #[prost(int32, tag = "1")]
  pub team_id: i32,
}
/// aiserver.v1.TeamMember
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct TeamMember {
  #[prost(string, tag = "1")]
  pub name: String,
  #[prost(string, tag = "4")]
  pub email: String,
  #[prost(int32, tag = "2")]
  pub id: i32,
  #[prost(enumeration = "TeamRole", tag = "3")]
  pub role: i32,
}
/// aiserver.v1.GetTeamMembersResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetTeamMembersResponse {
  #[prost(message, repeated, tag = "1")]
  pub team_members: Vec<TeamMember>,
  #[prost(int32, tag = "2")]
  pub user_id: i32,
}
/// aiserver.v1.GetFilteredUsageEventsRequest
#[derive(::serde::Serialize, Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GetFilteredUsageEventsRequest {
  #[prost(int32, tag = "1")]
  pub team_id: i32,
  #[prost(int64, optional, tag = "2")]
  pub start_date: Option<i64>,
  #[prost(int64, optional, tag = "3")]
  pub end_date: Option<i64>,
  #[prost(int32, optional, tag = "4")]
  pub user_id: Option<i32>,
  #[prost(string, optional, tag = "5")]
  pub model_id: Option<String>,
  #[prost(int32, optional, tag = "6")]
  pub page: Option<i32>,
  #[prost(int32, optional, tag = "7")]
  pub page_size: Option<i32>,
}
/// aiserver.v1.GetFilteredUsageEventsResponse
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetFilteredUsageEventsResponse {
  #[serde(default)]
  #[prost(message, repeated, tag = "1")]
  pub usage_events: Vec<UsageEvent>,
  #[serde(default)]
  #[prost(int32, tag = "2")]
  pub total_usage_events_count: i32,
  #[serde(default)]
  #[prost(message, repeated, tag = "3")]
  pub usage_events_display: Vec<UsageEventDisplay>,
}
/// aiserver.v1.GetAggregatedUsageEventsRequest
#[derive(::serde::Serialize, Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GetAggregatedUsageEventsRequest {
  #[prost(int32, tag = "1")]
  pub team_id: i32,
  #[prost(int64, optional, tag = "2")]
  pub start_date: Option<i64>,
  #[prost(int64, optional, tag = "3")]
  pub end_date: Option<i64>,
  #[prost(int32, optional, tag = "4")]
  pub user_id: Option<i32>,
}
/// aiserver.v1.GetAggregatedUsageEventsResponse
#[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
pub struct GetAggregatedUsageEventsResponse {
  #[serde(default)]
  #[prost(message, repeated, tag = "1")]
  pub aggregations: Vec<get_aggregated_usage_events_response::ModelUsageAggregation>,
  #[serde(alias = "totalInputTokens", default)]
  #[prost(int64, tag = "2")]
  pub total_input_tokens: i64,
  #[serde(alias = "totalOutputTokens", default)]
  #[prost(int64, tag = "3")]
  pub total_output_tokens: i64,
  #[serde(alias = "totalCacheWriteTokens", default)]
  #[prost(int64, tag = "4")]
  pub total_cache_write_tokens: i64,
  #[serde(alias = "totalCacheReadTokens", default)]
  #[prost(int64, tag = "5")]
  pub total_cache_read_tokens: i64,
  #[serde(alias = "totalCostCents", default)]
  #[prost(double, tag = "6")]
  pub total_cost_cents: f64,
  #[serde(alias = "percentOfBurstUsed", default)]
  #[prost(double, tag = "7")]
  pub percent_of_burst_used: f64,
}
/// Nested message and enum types in `GetAggregatedUsageEventsResponse`.
pub mod get_aggregated_usage_events_response {
  /// aiserver.v1.GetAggregatedUsageEventsResponse.ModelUsageAggregation
  #[derive(::serde::Deserialize, Clone, PartialEq, ::prost::Message)]
  pub struct ModelUsageAggregation {
    #[serde(alias = "modelIntent", default)]
    #[prost(string, tag = "1")]
    pub model_intent: String,
    #[serde(alias = "inputTokens", default)]
    #[prost(int64, tag = "2")]
    pub input_tokens: i64,
    #[serde(alias = "outputTokens", default)]
    #[prost(int64, tag = "3")]
    pub output_tokens: i64,
    #[serde(alias = "cacheWriteTokens", default)]
    #[prost(int64, tag = "4")]
    pub cache_write_tokens: i64,
    #[serde(alias = "cacheReadTokens", default)]
    #[prost(int64, tag = "5")]
    pub cache_read_tokens: i64,
    #[serde(alias = "totalCents", default)]
    #[prost(double, tag = "6")]
    pub total_cents: f64,
  }
}
/// aiserver.v1.BugConfigResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BugConfigResponse {
  #[prost(message, optional, tag = "1")]
  pub linter_strategy_v1: Option<bug_config_response::LinterStrategyV1>,
  #[prost(message, optional, tag = "2")]
  pub bug_bot_v1: Option<bug_config_response::BugBotV1>,
  #[prost(message, optional, tag = "3")]
  pub linter_strategy_v2: Option<bug_config_response::LinterStrategyV2>,
}
/// Nested message and enum types in `BugConfigResponse`.
pub mod bug_config_response {
  /// aiserver.v1.BugConfigResponse.LinterStrategyV1
  #[derive(Clone, Copy, PartialEq, ::prost::Message)]
  pub struct LinterStrategyV1 {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, tag = "2")]
    pub try_trigger_on_save: bool,
    #[prost(double, tag = "3")]
    pub wait_between_triggers_ms: f64,
  }
  /// aiserver.v1.BugConfigResponse.LinterStrategyV2
  #[derive(Clone, Copy, PartialEq, ::prost::Message)]
  pub struct LinterStrategyV2 {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(double, tag = "2")]
    pub wait_between_triggers_ms: f64,
    #[prost(double, tag = "3")]
    pub debounce_triggers_ms: f64,
    #[prost(int32, tag = "4")]
    pub keep_lines_around_chunk: i32,
    #[prost(int32, tag = "5")]
    pub prevent_triggering_for_files_with_this_many_lines: i32,
    #[prost(bool, tag = "6")]
    pub prevent_triggering_when_lints: bool,
  }
  /// aiserver.v1.BugConfigResponse.BugBotV1
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct BugBotV1 {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(bool, tag = "2")]
    pub is_subsidized: bool,
    #[prost(int32, tag = "3")]
    pub background_call_frequency_ms: i32,
    #[prost(bool, tag = "4")]
    pub kill_switch: bool,
    #[prost(double, tag = "5")]
    pub show_intrusive_notification_only_if_last_time_was_more_than_ms_ago: f64,
    #[prost(int32, optional, tag = "6")]
    pub background_diff_absolute_max_tokens: Option<i32>,
    #[prost(int32, optional, tag = "7")]
    pub background_diff_min_min_token_threshold: Option<i32>,
    #[prost(int32, optional, tag = "8")]
    pub background_diff_min_max_token_threshold: Option<i32>,
    #[prost(double, optional, tag = "9")]
    pub background_diff_last_commit_less_than_this_many_ms_ago: Option<f64>,
    #[prost(int32, optional, tag = "15")]
    pub background_unified_context_lines: Option<i32>,
    #[prost(bool, optional, tag = "16")]
    pub background_diff_include_uncommitted: Option<bool>,
    #[prost(int32, optional, tag = "10")]
    pub default_diff_context_lines: Option<i32>,
    #[prost(int32, optional, tag = "11")]
    pub diff_absolute_max_tokens: Option<i32>,
    #[prost(int32, optional, tag = "12")]
    pub custom_instructions_max_char_length: Option<i32>,
    #[prost(int32, optional, tag = "13")]
    pub default_fallback_iterations: Option<i32>,
    #[prost(int32, optional, tag = "14")]
    pub threshold_for_expensive_run_modal_cents: Option<i32>,
    #[prost(string, optional, tag = "17")]
    pub cheap_model_name: Option<String>,
    #[prost(int32, optional, tag = "18")]
    pub cheap_absolute_max_tokens: Option<i32>,
    #[prost(int32, optional, tag = "19")]
    pub expensive_absolute_max_tokens: Option<i32>,
  }
}
/// aiserver.v1.HeapProfileConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct HeapProfileConfig {
  #[prost(int32, tag = "1")]
  pub sampling_interval: i32,
  #[prost(bool, optional, tag = "2")]
  pub include_objects_collected_by_major_gc: Option<bool>,
  #[prost(bool, optional, tag = "3")]
  pub include_objects_collected_by_minor_gc: Option<bool>,
}
/// aiserver.v1.CpuProfileConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct CpuProfileConfig {
  #[prost(int32, tag = "1")]
  pub interval: i32,
}
/// aiserver.v1.ProfileConfig
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ProfileConfig {
  #[prost(string, tag = "1")]
  pub id: String,
  #[prost(message, optional, tag = "2")]
  pub heap: Option<HeapProfileConfig>,
  #[prost(message, optional, tag = "3")]
  pub cpu: Option<CpuProfileConfig>,
}
/// aiserver.v1.ProfileScheduleConfig
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ProfileScheduleConfig {
  #[prost(string, tag = "1")]
  pub id: String,
  #[prost(string, tag = "2")]
  pub process: String,
  #[prost(string, tag = "3")]
  pub config_id: String,
  #[prost(message, optional, tag = "4")]
  pub schedule: Option<profile_schedule_config::ScheduleConfig>,
  #[prost(int32, tag = "5")]
  pub activity_timeout: i32,
}
/// Nested message and enum types in `ProfileScheduleConfig`.
pub mod profile_schedule_config {
  /// aiserver.v1.ProfileScheduleConfig.ScheduleConfig
  #[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
  pub struct ScheduleConfig {
    #[prost(int32, tag = "1")]
    pub interval: i32,
    #[prost(int32, tag = "2")]
    pub duration: i32,
  }
}
/// aiserver.v1.ProfilingConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProfilingConfig {
  #[prost(bool, tag = "1")]
  pub enabled: bool,
  #[prost(message, repeated, tag = "2")]
  pub configs: Vec<ProfileConfig>,
  #[prost(message, repeated, tag = "3")]
  pub schedules: Vec<ProfileScheduleConfig>,
}
/// aiserver.v1.InAppAd
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InAppAd {
  #[prost(string, tag = "1")]
  pub id: String,
  #[prost(message, optional, tag = "2")]
  pub header: Option<AdHeader>,
  #[prost(message, optional, tag = "3")]
  pub content: Option<AdContent>,
  #[prost(message, repeated, tag = "4")]
  pub buttons: Vec<AdButton>,
}
/// aiserver.v1.AdHeader
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AdHeader {
  #[prost(string, optional, tag = "1")]
  pub banner_url: Option<String>,
}
/// aiserver.v1.AdContent
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AdContent {
  #[prost(string, optional, tag = "1")]
  pub tag: Option<String>,
  #[prost(string, tag = "2")]
  pub title: String,
  #[prost(message, repeated, tag = "3")]
  pub sections: Vec<AdSection>,
  #[prost(bool, optional, tag = "4")]
  pub center_title: Option<bool>,
}
/// aiserver.v1.AdSection
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AdSection {
  #[prost(string, tag = "1")]
  pub icon_svg: String,
  #[prost(string, tag = "2")]
  pub title: String,
  #[prost(string, tag = "3")]
  pub description: String,
  #[prost(bool, optional, tag = "4")]
  pub is_beta: Option<bool>,
}
/// aiserver.v1.AdButton
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AdButton {
  #[prost(string, tag = "1")]
  pub text: String,
  #[prost(enumeration = "ButtonType", tag = "2")]
  pub button_type: i32,
  #[prost(oneof = "ad_button::Action", tags = "3, 4")]
  pub action: Option<ad_button::Action>,
}
/// Nested message and enum types in `AdButton`.
pub mod ad_button {
  #[derive(Clone, PartialEq, Eq, Hash, ::prost::Oneof)]
  pub enum Action {
    #[prost(string, tag = "3")]
    ExternalUrl(String),
    #[prost(string, tag = "4")]
    CommandId(String),
  }
}
/// aiserver.v1.IndexingConfig
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IndexingConfig {
  #[prost(int32, tag = "1")]
  pub max_concurrent_uploads: i32,
  #[prost(int32, tag = "2")]
  pub absolute_max_number_files: i32,
  #[prost(int32, tag = "3")]
  pub max_file_retries: i32,
  #[prost(int32, tag = "4")]
  pub sync_concurrency: i32,
  #[prost(int32, tag = "5")]
  pub auto_indexing_max_num_files: i32,
  #[prost(int32, tag = "6")]
  pub indexing_period_seconds: i32,
  #[prost(string, tag = "7")]
  pub repo42_auth_token: String,
  #[prost(bool, optional, tag = "8")]
  pub incremental: Option<bool>,
  #[prost(string, optional, tag = "9")]
  pub default_user_path_encryption_key: Option<String>,
  #[prost(string, optional, tag = "10")]
  pub default_team_path_encryption_key: Option<String>,
  #[prost(bool, optional, tag = "11")]
  pub multi_root_indexing_enabled: Option<bool>,
  #[prost(double, tag = "12")]
  pub copy_status_check_period_seconds: f64,
  #[prost(int32, tag = "13")]
  pub copy_timeout_seconds: i32,
  #[prost(int32, tag = "14")]
  pub max_batch_bytes: i32,
  #[prost(int32, tag = "15")]
  pub max_batch_num_requests: i32,
  #[prost(int32, tag = "16")]
  pub max_sync_merkle_batch_size: i32,
}
/// aiserver.v1.GitIndexingConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct GitIndexingConfig {
  #[prost(bool, optional, tag = "1")]
  pub enabled: Option<bool>,
  #[prost(int32, optional, tag = "2")]
  pub max_pr_count: Option<i32>,
  #[prost(int32, optional, tag = "3")]
  pub max_pr_size: Option<i32>,
  #[prost(int32, optional, tag = "4")]
  pub pr_embedding_context: Option<i32>,
  #[prost(int32, optional, tag = "5")]
  pub js_concurrency: Option<i32>,
  #[prost(int32, optional, tag = "6")]
  pub js_chunk_size: Option<i32>,
  #[prost(int32, optional, tag = "7")]
  pub cpu_concurrency: Option<i32>,
  #[prost(int32, optional, tag = "8")]
  pub indexing_period_seconds: Option<i32>,
  #[prost(int32, optional, tag = "9")]
  pub max_attempts_per_pr: Option<i32>,
  #[prost(bool, optional, tag = "10")]
  pub allow_index_copy_from_server: Option<bool>,
}
/// aiserver.v1.ClientTracingConfig
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct ClientTracingConfig {
  #[prost(double, tag = "1")]
  pub global_sample_rate: f64,
  #[prost(double, tag = "2")]
  pub traces_sample_rate: f64,
  #[prost(double, tag = "3")]
  pub logger_sample_rate: f64,
  #[prost(double, tag = "4")]
  pub minidump_sample_rate: f64,
  #[prost(double, tag = "5")]
  pub error_rate_limit: f64,
  #[prost(double, tag = "6")]
  pub performance_unit_rate_limit: f64,
  #[prost(double, tag = "7")]
  pub profiles_sample_rate: f64,
  #[prost(double, tag = "8")]
  pub json_stringify_sample_rate: f64,
}
/// aiserver.v1.ChatConfig
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ChatConfig {
  #[prost(bool, tag = "1")]
  pub disable_unification: bool,
  #[prost(int32, tag = "2")]
  pub full_context_token_limit: i32,
  #[prost(bool, tag = "3")]
  pub disable_yolo_mode: bool,
  #[prost(int32, tag = "4")]
  pub max_rule_length: i32,
  #[prost(int32, tag = "5")]
  pub max_mcp_tools: i32,
  #[prost(int32, tag = "6")]
  pub warn_mcp_tools: i32,
  #[prost(string, tag = "7")]
  pub summarization_message: String,
  #[prost(int32, tag = "8")]
  pub num_files_for_memory_generation: i32,
  #[prost(bool, tag = "9")]
  pub memory_default_enabled: bool,
  #[prost(int32, tag = "10")]
  pub num_summarizations_before_warning_shown: i32,
}
/// aiserver.v1.GetServerConfigRequest
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerConfigRequest {
  #[prost(bool, tag = "1")]
  pub telem_enabled: bool,
  #[prost(double, repeated, tag = "2")]
  pub bug_bot_dismissed_notification_last_10_times_unix_ms: Vec<f64>,
  #[prost(double, repeated, tag = "3")]
  pub bug_bot_viewed_notification_last_10_times_unix_ms: Vec<f64>,
  #[prost(message, optional, tag = "4")]
  pub os_stats: Option<VscodeOsStatistics>,
  #[prost(message, optional, tag = "5")]
  pub os_properties: Option<VscodeOsProperties>,
}
/// aiserver.v1.MetricsConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct MetricsConfig {
  #[prost(bool, tag = "2")]
  pub enabled_in_privacy_mode: bool,
  #[prost(bool, tag = "3")]
  pub enabled_in_non_privacy_mode: bool,
}
/// aiserver.v1.BackgroundComposerConfig
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct BackgroundComposerConfig {
  #[prost(bool, tag = "1")]
  pub enable_background_agent: bool,
  #[prost(bool, tag = "2")]
  pub show_background_agent_in_beta_settings: bool,
  #[prost(int32, optional, tag = "3")]
  pub window_in_window_preload_count: Option<i32>,
  #[prost(double, optional, tag = "4")]
  pub window_in_window_ping_interval_ms: Option<f64>,
  #[prost(bool, optional, tag = "5")]
  pub show_background_agent_disclaimer: Option<bool>,
  #[prost(bool, optional, tag = "6")]
  pub show_background_agent_slack_ad: Option<bool>,
  #[prost(bool, tag = "7")]
  pub show_background_agent_history_action: bool,
  #[prost(int32, optional, tag = "8")]
  pub max_window_in_windows: Option<i32>,
  #[prost(bool, tag = "9")]
  pub use_modal_experience: bool,
}
/// aiserver.v1.AutoContextConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct AutoContextConfig {
  #[prost(bool, tag = "1")]
  pub enabled: bool,
  #[prost(bool, tag = "2")]
  pub enabled_fallback: bool,
  #[prost(bool, tag = "3")]
  pub enabled_git_graph: bool,
  #[prost(bool, tag = "4")]
  pub enabled_sem_search: bool,
  #[prost(bool, tag = "5")]
  pub enabled_v2: bool,
}
/// aiserver.v1.MemoryMonitorConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct MemoryMonitorConfig {
  #[prost(bool, tag = "1")]
  pub enabled: bool,
  #[prost(bool, tag = "2")]
  pub show_status_entry: bool,
  #[prost(int32, optional, tag = "3")]
  pub base_threshold_mb: Option<i32>,
  #[prost(int32, optional, tag = "4")]
  pub critical_offset_mb: Option<i32>,
}
/// aiserver.v1.PerformanceEventsConfig
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct PerformanceEventsConfig {
  #[prost(bool, tag = "1")]
  pub enabled: bool,
  #[prost(int32, optional, tag = "2")]
  pub buffer_size: Option<i32>,
  #[prost(int32, optional, tag = "3")]
  pub flush_interval_ms: Option<i32>,
}
/// aiserver.v1.TraceConfig
#[derive(Clone, Copy, PartialEq, ::prost::Message)]
pub struct TraceConfig {
  #[prost(bool, tag = "1")]
  pub enabled: bool,
  #[prost(int32, optional, tag = "2")]
  pub buffer_size: Option<i32>,
  #[prost(int32, optional, tag = "3")]
  pub flush_interval_ms: Option<i32>,
  #[prost(double, optional, tag = "4")]
  pub sample_rate: Option<f64>,
}
/// aiserver.v1.ModelMigration
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct ModelMigration {
  #[prost(string, tag = "1")]
  pub id: String,
  #[prost(string, tag = "2")]
  pub model_setting: String,
  #[prost(string, tag = "3")]
  pub target_model: String,
  #[prost(string, optional, tag = "4")]
  pub previous_model: Option<String>,
  #[prost(bool, optional, tag = "5")]
  pub migrate_mode_models: Option<bool>,
  #[prost(bool, optional, tag = "6")]
  pub remove_previous_model: Option<bool>,
  #[prost(bool, optional, tag = "7")]
  pub max_mode: Option<bool>,
}
/// aiserver.v1.FolderSizeLimit
#[derive(Clone, Copy, PartialEq, Eq, Hash, ::prost::Message)]
pub struct FolderSizeLimit {
  #[prost(int32, tag = "1")]
  pub max_total_bytes: i32,
  #[prost(int32, tag = "2")]
  pub max_num_files: i32,
}
/// aiserver.v1.RunTerminalServerConfig
#[derive(Clone, PartialEq, Eq, Hash, ::prost::Message)]
pub struct RunTerminalServerConfig {
  #[prost(string, repeated, tag = "1")]
  pub composite_shell_commands: Vec<String>,
  #[prost(string, repeated, tag = "2")]
  pub safe_shell_commands: Vec<String>,
}
/// aiserver.v1.GetServerConfigResponse
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetServerConfigResponse {
  #[prost(message, optional, tag = "1")]
  pub bug_config_response: Option<BugConfigResponse>,
  #[prost(bool, tag = "2")]
  pub is_dev_do_not_use_for_secret_things_because_can_be_spoofed_by_users: bool,
  #[prost(message, optional, tag = "3")]
  pub indexing_config: Option<IndexingConfig>,
  #[prost(message, optional, tag = "4")]
  pub client_tracing_config: Option<ClientTracingConfig>,
  #[prost(message, optional, tag = "5")]
  pub chat_config: Option<ChatConfig>,
  #[prost(string, tag = "6")]
  pub config_version: String,
  #[prost(enumeration = "Http2Config", tag = "7")]
  pub http2_config: i32,
  #[prost(message, optional, tag = "8")]
  pub profiling_config: Option<ProfilingConfig>,
  #[prost(message, optional, tag = "9")]
  pub metrics_config: Option<MetricsConfig>,
  #[prost(message, optional, tag = "10")]
  pub background_composer_config: Option<BackgroundComposerConfig>,
  #[prost(message, optional, tag = "11")]
  pub auto_context_config: Option<AutoContextConfig>,
  #[prost(message, repeated, tag = "12")]
  pub model_migrations: Vec<ModelMigration>,
  #[prost(message, optional, tag = "13")]
  pub memory_monitor_config: Option<MemoryMonitorConfig>,
  #[prost(message, optional, tag = "14")]
  pub folder_size_limit: Option<FolderSizeLimit>,
  #[prost(message, optional, tag = "15")]
  pub git_indexing_config: Option<GitIndexingConfig>,
  #[prost(message, optional, tag = "16")]
  pub performance_events_config: Option<PerformanceEventsConfig>,
  #[prost(message, optional, tag = "17")]
  pub current_in_app_ad: Option<InAppAd>,
  #[prost(message, optional, tag = "18")]
  pub trace_config: Option<TraceConfig>,
  #[prost(message, optional, tag = "19")]
  pub run_terminal_server_config: Option<RunTerminalServerConfig>,
}
/// aiserver.v1.EmbeddingModel
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EmbeddingModel {
  Unspecified = 0,
  VoyageCode2 = 1,
  TextEmbeddingsLarge3 = 2,
  Qwen15bCustom = 3,
  MockChunkerError = 4,
  Qwen15b0618Custom = 5,
  Qwen15b0618Fp8MmCustom = 6,
}
/// aiserver.v1.FSUploadErrorType
#[derive(
  ::serde::Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
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
/// aiserver.v1.FSSyncErrorType
#[derive(
  ::serde::Serialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration,
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
      &super::FsSyncErrorType::try_from(*value).unwrap_or_default(),
      serializer,
    )
  }
}
/// aiserver.v1.DatabaseProvider
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DatabaseProvider {
  Unspecified = 0,
  Aurora = 1,
  Planetscale = 2,
}
/// aiserver.v1.ClientSideToolV2
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
}
/// aiserver.v1.RunTerminalCommandEndedReason
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum RunTerminalCommandEndedReason {
  Unspecified = 0,
  ExecutionCompleted = 1,
  ExecutionAborted = 2,
  ExecutionFailed = 3,
  ErrorOccurredCheckingReason = 4,
}
/// aiserver.v1.ChunkType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ChunkType {
  Unspecified = 0,
  Codebase = 1,
  LongFile = 2,
  Docs = 3,
}
/// aiserver.v1.SubagentType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SubagentType {
  Unspecified = 0,
  DeepSearch = 1,
  FixLints = 2,
  Task = 3,
}
/// aiserver.v1.UsageEventKind
#[derive(::serde::Serialize, ::serde::Deserialize, Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
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
    <super::UsageEventKind as ::serde::Deserialize>::deserialize(deserializer)
      .map(|val| val as i32)
  }
}
/// aiserver.v1.TeamRole
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TeamRole {
  Unspecified = 0,
  Owner = 1,
  Member = 2,
  FreeOwner = 3,
}
/// aiserver.v1.ButtonType
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ButtonType {
  Unspecified = 0,
  Leading = 1,
  Primary = 2,
  Secondary = 3,
}
/// aiserver.v1.Http2Config
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Http2Config {
  Unspecified = 0,
  ForceAllDisabled = 1,
  ForceAllEnabled = 2,
  ForceBidiDisabled = 3,
  ForceBidiEnabled = 4,
}
