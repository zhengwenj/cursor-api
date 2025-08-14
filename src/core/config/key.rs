/// 动态配置的 API KEY
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct KeyConfig {
  /// 认证令牌（必需）
  #[prost(message, optional, tag = "1")]
  pub token_info: Option<key_config::TokenInfo>,
  /// 密码SHA256哈希值
  #[prost(bytes = "vec", optional, tag = "2")]
  pub secret: Option<Vec<u8>>,
  /// 是否禁用图片处理能力
  #[prost(bool, optional, tag = "3")]
  pub disable_vision: Option<bool>,
  /// 是否启用慢速池
  #[prost(bool, optional, tag = "4")]
  pub enable_slow_pool: Option<bool>,
  /// 包含网络引用
  #[prost(bool, optional, tag = "5")]
  pub include_web_references: Option<bool>,
  /// 使用量检查模型规则
  #[prost(message, optional, tag = "6")]
  pub usage_check_models: Option<key_config::UsageCheckModel>,
}
/// Nested message and enum types in `KeyConfig`.
pub mod key_config {
  /// 认证令牌信息
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct TokenInfo {
    /// 令牌
    #[prost(message, optional, tag = "1")]
    pub token: Option<token_info::Token>,
    /// 校验和(\[u8; 64\])
    #[prost(bytes = "vec", tag = "2")]
    pub checksum: Vec<u8>,
    /// 客户端标识
    #[prost(bytes = "vec", tag = "3")]
    pub client_key: Vec<u8>,
    /// 配置版本
    #[prost(bytes = "vec", optional, tag = "4")]
    pub config_version: Option<Vec<u8>>,
    /// 会话ID
    #[prost(bytes = "vec", tag = "5")]
    pub session_id: Vec<u8>,
    /// 代理名称
    #[prost(string, optional, tag = "11")]
    pub proxy_name: Option<String>,
    /// 时区
    #[prost(string, optional, tag = "12")]
    pub timezone: Option<String>,
    /// 代码补全
    #[prost(int32, optional, tag = "13")]
    pub gcpp_host: Option<i32>,
  }
  /// Nested message and enum types in `TokenInfo`.
  pub mod token_info {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Token {
      #[prost(string, tag = "1")]
      pub provider: String,
      /// 用户ID(\[u8; 16\])
      #[prost(bytes = "vec", tag = "2")]
      pub sub_id: Vec<u8>,
      /// 随机字符串(\[u8; 8\])
      #[prost(bytes = "vec", tag = "3")]
      pub randomness: Vec<u8>,
      /// 生成时间（Unix 时间戳）
      #[prost(int64, tag = "4")]
      pub start: i64,
      /// 过期时间（Unix 时间戳）
      #[prost(int64, tag = "5")]
      pub end: i64,
      /// 签名(\[u8; 32\])
      #[prost(bytes = "vec", tag = "6")]
      pub signature: Vec<u8>,
      /// 是否为会话令牌
      #[prost(bool, tag = "7")]
      pub is_session: bool,
    }
  }
  /// 使用量检查模型规则
  #[derive(Clone, PartialEq, ::prost::Message)]
  pub struct UsageCheckModel {
    /// 检查类型
    #[prost(enumeration = "usage_check_model::Type", tag = "1")]
    pub r#type: i32,
    /// 模型 ID 列表，当 type 为 TYPE_CUSTOM 时生效
    #[prost(string, repeated, tag = "2")]
    pub model_ids: Vec<String>,
  }
  /// Nested message and enum types in `UsageCheckModel`.
  pub mod usage_check_model {
    /// 检查类型
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum Type {
      /// 未指定
      Default = 0,
      /// 禁用
      Disabled = 1,
      /// 全部
      All = 2,
      /// 自定义列表
      Custom = 3,
    }
  }
}
