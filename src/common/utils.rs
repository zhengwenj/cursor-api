#![allow(clippy::too_many_arguments)]

// mod checksum;
// mod token;
pub mod base62;
mod base64;
pub mod duration_fmt;
pub mod hex;
pub mod string_builder;

pub use hex::{byte_to_hex, hex_to_byte};
pub use string_builder::StringBuilder;

use std::{borrow::Cow, str::FromStr as _};

use ::base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
pub use base64::*;
use prost::Message as _;
use reqwest::Client;

use super::model::userinfo::{
  GetTeamsResponse, ListActiveSessionsResponse, Session, StripeProfile, Team, UsageProfile,
  UserProfile,
};
use crate::{
  app::{
    lazy::{
      aggregated_usage_events_url, chat_models_url, filtered_usage_events_url,
      is_on_new_pricing_url, server_config_url, teams_url,
    },
    model::{ChainUsage, Checksum, DateTime, ExtToken, GcppHost, Hash, RawToken, Token},
  },
  common::model::userinfo::{MembershipType, SubscriptionStatus},
  core::{
    aiserver::v1::{
      AvailableModelsRequest, AvailableModelsResponse, GetAggregatedUsageEventsRequest,
      GetAggregatedUsageEventsResponse, GetFilteredUsageEventsRequest,
      GetFilteredUsageEventsResponse, GetServerConfigResponse,
    },
    config::key_config,
  },
};

pub fn parse_bool_from_env(key: &str, default: bool) -> bool {
  std::env::var(key)
    .ok()
    .map(|mut val| {
      let res = {
        val.make_ascii_lowercase();
        val.trim()
      };
      match res {
        "true" | "1" => true,
        "false" | "0" => false,
        _ => default,
      }
    })
    .unwrap_or(default)
}

pub fn parse_string_from_env(key: &str, default: &'static str) -> Cow<'static, str> {
  match std::env::var(key) {
    Ok(mut value) => {
      let trimmed = value.trim();

      if trimmed.is_empty() {
        // 如果 trim 后为空，使用默认值（不分配）
        Cow::Borrowed(default)
      } else if trimmed.len() == value.len() {
        // 不需要 trim，直接使用
        Cow::Owned(value)
      } else {
        // 需要 trim - 就地修改
        let trimmed_len = trimmed.len();
        let start_offset = trimmed.as_ptr() as usize - value.as_ptr() as usize;

        unsafe {
          let vec = value.as_mut_vec();
          if start_offset > 0 {
            vec.copy_within(start_offset..start_offset + trimmed_len, 0);
          }
          vec.set_len(trimmed_len);
        }

        Cow::Owned(value)
      }
    }
    Err(_) => Cow::Borrowed(default),
  }
}

pub fn parse_usize_from_env(key: &str, default: usize) -> usize {
  std::env::var(key)
    .ok()
    .and_then(|v| v.trim().parse().ok())
    .unwrap_or(default)
}

pub fn now_secs() -> u64 {
  std::time::SystemTime::now()
    .duration_since(std::time::UNIX_EPOCH)
    .expect("system time before Unix epoch")
    .as_secs()
}

const LEN: usize = 2;

pub trait TrimNewlines {
  fn trim_leading_newlines(self) -> Self;
}

impl TrimNewlines for &str {
  #[inline(always)]
  fn trim_leading_newlines(self) -> Self {
    let bytes = self.as_bytes();
    if bytes.len() >= LEN && bytes[0] == b'\n' && bytes[1] == b'\n' {
      return unsafe { self.get_unchecked(LEN..) };
    }
    self
  }
}

impl TrimNewlines for String {
  #[inline(always)]
  fn trim_leading_newlines(mut self) -> Self {
    let bytes = self.as_bytes();
    if bytes.len() >= LEN && bytes[0] == b'\n' && bytes[1] == b'\n' {
      unsafe {
        let vec = self.as_mut_vec();
        vec.drain(..LEN);
      }
    }
    self
  }
}

/// 获取完整的token配置文件
/// 协调多个数据源，可选择性获取用户信息
#[inline(never)]
pub async fn get_token_profile(
  client: Client,
  token: &Token,
  maybe_token: Option<&Token>,
  is_pri: bool,
  include_user: bool,
  include_sessions: bool,
) -> (
  Option<UserProfile>,
  Option<StripeProfile>,
  Option<Vec<Session>>,
) {
  let maybe_token = maybe_token.unwrap_or(token);

  let mut buf = [0; 31];
  let user_id = maybe_token.raw().subject.id.to_str(&mut buf) as &str;

  if include_user {
    if include_sessions {
      // 并发获取所有数据，user为必需
      let (mut stripe, _, mut user, teams, is_on_new_pricing, sessions) = tokio::join!(
        get_stripe_profile(&client, token.as_str(), is_pri),
        get_usage_profile(&client, user_id, maybe_token.as_str(), is_pri),
        get_user_profile(&client, user_id, maybe_token.as_str(), is_pri),
        get_teams(&client, user_id, maybe_token.as_str(), is_pri),
        get_is_on_new_pricing(&client, user_id, maybe_token.as_str(), is_pri),
        get_sessions(&client, user_id, maybe_token.as_str(), is_pri)
      );

      if let Some(stripe) = stripe.as_mut()
        && teams.is_some_and(|teams| {
          teams.into_iter().any(|team| {
            team.has_billing
              && team.subscription_status.is_some_and(|subscription_status| {
                matches!(subscription_status, SubscriptionStatus::Active)
              })
          })
        })
      {
        stripe.membership_type = MembershipType::Enterprise;
      }

      if let Some(user) = user.as_mut() {
        user.is_on_new_pricing = is_on_new_pricing.unwrap_or(true);
      }

      // 所有数据都必需成功
      (user, stripe, sessions)
    } else {
      // 并发获取所有数据，user为必需
      let (mut stripe, _, mut user, teams, is_on_new_pricing) = tokio::join!(
        get_stripe_profile(&client, token.as_str(), is_pri),
        get_usage_profile(&client, user_id, maybe_token.as_str(), is_pri),
        get_user_profile(&client, user_id, maybe_token.as_str(), is_pri),
        get_teams(&client, user_id, maybe_token.as_str(), is_pri),
        get_is_on_new_pricing(&client, user_id, maybe_token.as_str(), is_pri)
      );

      if let Some(stripe) = stripe.as_mut()
        && teams.is_some_and(|teams| {
          teams.into_iter().any(|team| {
            team.has_billing
              && team.subscription_status.is_some_and(|subscription_status| {
                matches!(subscription_status, SubscriptionStatus::Active)
              })
          })
        })
      {
        stripe.membership_type = MembershipType::Enterprise;
      }

      if let Some(user) = user.as_mut() {
        user.is_on_new_pricing = is_on_new_pricing.unwrap_or(true);
      }

      // 所有数据都必需成功
      (user, stripe, None)
    }
  } else {
    // 仅获取stripe数据
    let (mut stripe, _, teams) = tokio::join!(
      get_stripe_profile(&client, token.as_str(), is_pri),
      get_usage_profile(&client, user_id, maybe_token.as_str(), is_pri),
      get_teams(&client, user_id, maybe_token.as_str(), is_pri),
    );

    if let Some(stripe) = stripe.as_mut()
      && teams.is_some_and(|teams| {
        teams.into_iter().any(|team| {
          team.has_billing
            && team.subscription_status.is_some_and(|subscription_status| {
              matches!(subscription_status, SubscriptionStatus::Active)
            })
        })
      })
    {
      stripe.membership_type = MembershipType::Enterprise;
    }

    (None, stripe, None)
  }
}

/// 获取用户使用情况配置文件
pub async fn get_usage_profile(client: &Client, user_id: &str, auth_token: &str, is_pri: bool) {
  if !*crate::app::lazy::log::DEBUG {
    return;
  }

  let request = super::client::build_usage_request(client, user_id, auth_token, is_pri);
  let response = match request.send().await {
    Ok(r) => r,
    Err(_) => {
      crate::debug!("<get_usage_profile> send error");
      return;
    }
  };
  crate::debug!("<get_usage_profile> got {}", response.status());
  let usage = response.json::<UsageProfile>().await.ok();
  crate::debug!(
    "<get_usage_profile> got {}",
    __unwrap!(serde_json::to_string_pretty(&usage))
  );
}

/// 获取Stripe付费配置文件
pub async fn get_stripe_profile(
  client: &Client,
  auth_token: &str,
  is_pri: bool,
) -> Option<StripeProfile> {
  let request = super::client::build_profile_request(client, auth_token, is_pri);

  let response = request.send().await.ok()?;
  crate::debug!("<get_stripe_profile> {}", response.status());
  response.json::<StripeProfile>().await.ok()
}

/// 获取用户基础配置文件
pub async fn get_user_profile(
  client: &Client,
  user_id: &str,
  auth_token: &str,
  is_pri: bool,
) -> Option<UserProfile> {
  let request = super::client::build_userinfo_request(client, user_id, auth_token, is_pri);

  // let response = request.send().await.ok()?;
  // crate::debug!("get_user_profile \n{response:?}");
  // let bytes = response.bytes().await.ok()?;
  // crate::debug!("bytes \n{:?}", unsafe { std::str::from_utf8_unchecked(&bytes[..]) });
  // serde_json::from_slice::<UserProfile>(&bytes).ok()
  let response = request.send().await.ok()?;
  crate::debug!("<get_user_profile> {}", response.status());
  response.json::<UserProfile>().await.ok()
}

pub async fn get_available_models(
  ext_token: ExtToken,
  is_pri: bool,
  request: AvailableModelsRequest,
) -> Option<AvailableModelsResponse> {
  let response = {
    let client = super::client::build_client_request(super::client::AiServiceRequest {
      ext_token,
      fs_client_key: None,
      url: chat_models_url(is_pri),
      is_stream: false,
      trace_id: Some(new_uuid_v4()),
      is_pri,
      cookie: None,
    });
    client
      .body(__unwrap!(encode_message(&request, false)))
      .send()
      .await
      .ok()?
      .bytes()
      .await
      .ok()?
  };
  let available_models = AvailableModelsResponse::decode(response.as_ref()).ok()?;
  Some(available_models)
}

pub async fn get_token_usage(
  ext_token: ExtToken,
  is_pri: bool,
  time: DateTime,
  model_id: &'static str,
) -> Option<ChainUsage> {
  let maybe_token = ext_token
    .secondary_token
    .as_ref()
    .unwrap_or_else(|| &ext_token.primary_token);

  let mut buf = [0; 31];
  let user_id = maybe_token.raw().subject.id.to_str(&mut buf) as &str;
  let mut token_usage = None;

  for _ in 0..5 {
    tokio::time::sleep(::core::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
    let res = get_filtered_usage_events(
      &ext_token.get_client(),
      user_id,
      maybe_token.as_str(),
      is_pri,
      FilteredUsageArgs {
        start: Some(time),
        end: None,
        model_id: Some(model_id),
        size: Some(10),
      },
    )
    .await?;

    if let Some(usage) = res.usage_events_display.get(0)?.token_usage {
        tokio::time::sleep(::core::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
      token_usage = Some(usage);
      break;
    };
  }

  token_usage.map(|token_usage| ChainUsage {
    input: token_usage.input_tokens,
    output: token_usage.output_tokens,
    cache_write: token_usage.cache_write_tokens,
    cache_read: token_usage.cache_read_tokens,
    cents: token_usage.total_cents,
  })
}

// pub fn validate_token_and_checksum(auth_token: &str) -> Option<(String, Checksum)> {
//     // 尝试使用自定义分隔符查找
//     let mut delimiter_pos = auth_token.rfind(*TOKEN_DELIMITER);

//     // 如果自定义分隔符未找到，并且 USE_COMMA_DELIMITER 为 true，则尝试使用逗号
//     if delimiter_pos.is_none() && *USE_COMMA_DELIMITER {
//         delimiter_pos = auth_token.rfind(COMMA);
//     }

//     // 如果最终都没有找到分隔符，则返回 None
//     let comma_pos = delimiter_pos?;

//     // 使用找到的分隔符位置分割字符串
//     let (token_part, checksum) = auth_token.split_at(comma_pos);
//     let checksum = &checksum[1..]; // 跳过逗号

//     // 解析 token - 为了向前兼容,忽略最后一个:或%3A前的内容
//     let colon_pos = token_part.rfind(':');
//     let encoded_colon_pos = token_part.rfind("%3A");

//     let token = match (colon_pos, encoded_colon_pos) {
//         (None, None) => token_part, // 最简单的构成: token,checksum
//         (Some(pos1), None) => &token_part[(pos1 + 1)..],
//         (None, Some(pos2)) => &token_part[(pos2 + 3)..],
//         (Some(pos1), Some(pos2)) => {
//             let pos = pos1.max(pos2);
//             let start = if pos == pos2 { pos + 3 } else { pos + 1 };
//             &token_part[start..]
//         }
//     };

//     // 验证 token 和 checksum 有效性
//     if let Ok(chekcsum) = Checksum::from_str(checksum) {
//         if validate_token(token) {
//             Some((token.to_string(), chekcsum))
//         } else {
//             None
//         }
//     } else {
//         None
//     }
// }

// pub fn extract_token(auth_token: &str) -> Option<&str> {
//     // 尝试使用自定义分隔符查找
//     let mut delimiter_pos = auth_token.rfind(*TOKEN_DELIMITER);

//     // 如果自定义分隔符未找到，并且 USE_COMMA_DELIMITER 为 true，则尝试使用逗号
//     if delimiter_pos.is_none() && *USE_COMMA_DELIMITER {
//         delimiter_pos = auth_token.rfind(COMMA);
//     }

//     // 根据是否找到分隔符来确定 token_part
//     let token_part = match delimiter_pos {
//         Some(pos) => &auth_token[..pos],
//         None => auth_token,
//     };

//     // 向前兼容
//     let colon_pos = token_part.rfind(':');
//     let encoded_colon_pos = token_part.rfind("%3A");

//     let token = match (colon_pos, encoded_colon_pos) {
//         (None, None) => token_part,
//         (Some(pos1), None) => &token_part[(pos1 + 1)..],
//         (None, Some(pos2)) => &token_part[(pos2 + 3)..],
//         (Some(pos1), Some(pos2)) => {
//             let pos = pos1.max(pos2);
//             let start = if pos == pos2 { pos + 3 } else { pos + 1 };
//             &token_part[start..]
//         }
//     };

//     // 验证 token 有效性
//     if validate_token(token) {
//         Some(token)
//     } else {
//         None
//     }
// }

#[inline(always)]
pub fn format_time_ms(seconds: f64) -> f64 { (seconds * 1000.0).round() / 1000.0 }

/// 将 JWT token 转换为 TokenInfo
#[inline]
pub fn token_to_tokeninfo(
  token: RawToken,
  checksum: Checksum,
  client_key: Hash,
  config_version: Option<uuid::Uuid>,
  session_id: uuid::Uuid,
  proxy_name: Option<String>,
  timezone: Option<String>,
  gcpp_host: Option<i32>,
) -> key_config::TokenInfo {
  key_config::TokenInfo {
    token: Some(key_config::token_info::Token::from_raw(token)),
    checksum: checksum.into_bytes().to_vec(),
    client_key: client_key.into_bytes().to_vec(),
    config_version: config_version.map(|v| v.into_bytes().to_vec()),
    session_id: session_id.into_bytes().to_vec(),
    proxy_name,
    timezone,
    gcpp_host,
  }
}

/// 将 TokenInfo 转换为 JWT token
#[inline]
pub fn tokeninfo_to_token(info: key_config::TokenInfo) -> Option<ExtToken> {
  let checksum = Checksum::from_bytes(info.checksum.try_into().ok()?);
  let client_key = Hash::from_bytes(info.client_key.try_into().ok()?);
  let config_version = info
    .config_version
    .and_then(|v| uuid::Uuid::from_slice(&v).ok());
  let session_id = uuid::Uuid::from_slice(&info.session_id).ok()?;
  let timezone = info.timezone.and_then(|s| chrono_tz::Tz::from_str(&s).ok());
  let gcpp_host = info.gcpp_host.and_then(GcppHost::from_i32);
  Some(ExtToken {
    primary_token: Token::new(info.token?.into_raw()?, None),
    secondary_token: None,
    checksum,
    client_key,
    config_version,
    session_id,
    proxy: info.proxy_name,
    timezone,
    gcpp_host,
    user: None,
  })
}

/// 压缩数据为gzip格式
#[inline]
fn compress_gzip(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
  use flate2::{Compression, write::GzEncoder};
  use std::io::Write as _;

  const LEVEL: Compression = Compression::new(6);

  let mut encoder = GzEncoder::new(Vec::new(), LEVEL);
  encoder.write_all(data)?;
  encoder.finish()
}

#[allow(clippy::uninit_vec)]
#[inline(always)]
pub fn encode_message(
  message: &impl prost::Message,
  maybe_stream: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
  const COMPRESSION_THRESHOLD: usize = 1024; // 1KB
  const LENGTH_OVERFLOW_MSG: &str = "Message length exceeds ~4 GiB";

  let estimated_size = message.encoded_len();

  if !maybe_stream {
    let mut encoded = Vec::with_capacity(estimated_size);
    __unwrap!(message.encode(&mut encoded));
    return Ok(encoded);
  }

  // 预留头部空间
  let mut buf = Vec::with_capacity(5 + estimated_size);

  unsafe {
    // 跳过头部5字节
    buf.set_len(5);

    // 编码消息
    __unwrap!(message.encode(&mut buf));
    let message_len = buf.len() - 5;

    // 判断是否需要压缩
    let (compression_flag, final_len) = if message_len >= COMPRESSION_THRESHOLD {
      // 需要压缩
      let compressed = compress_gzip(buf.get_unchecked(5..))?;
      let compressed_len = compressed.len();

      // 只在压缩后更小时才使用压缩版本
      if compressed_len < message_len {
        // 直接覆盖原数据
        let dst = buf.as_mut_ptr().add(5);
        ::core::ptr::copy_nonoverlapping(compressed.as_ptr(), dst, compressed_len);
        // 截断到正确长度
        buf.set_len(5 + compressed_len);
        (0x01, compressed_len)
      } else {
        // 压缩后反而更大，保持原样
        (0x00, message_len)
      }
    } else {
      // 不需要压缩
      (0x00, message_len)
    };

    // 统一写入头部
    let len = u32::try_from(final_len).map_err(|_| LENGTH_OVERFLOW_MSG)?;
    let ptr = buf.as_mut_ptr();
    *ptr = compression_flag;
    *(ptr.add(1) as *mut [u8; 4]) = len.to_be_bytes();
  }

  Ok(buf)
}

/// 生成 PKCE code_verifier 和对应的 code_challenge (S256 method).
/// 返回一个包含 (verifier, challenge) 的元组。
#[inline]
fn generate_pkce_pair() -> ([u8; 43], [u8; 43]) {
  use rand::TryRngCore as _;
  use sha2::Digest as _;

  // 1. 生成 code_verifier 的原始随机字节 (32 bytes is recommended)
  let mut verifier_bytes = [0u8; 32];

  // 使用 OsRng 填充字节。如果失败（极其罕见），则直接 panic
  rand::rngs::OsRng
    .try_fill_bytes(&mut verifier_bytes)
    .expect("获取系统安全随机数失败，这是一个严重错误！");

  // 2. 将随机字节编码为 URL 安全 Base64 字符串，这就是 code_verifier
  let mut code_verifier = [0; 43];
  __unwrap_panic!(URL_SAFE_NO_PAD.encode_slice(verifier_bytes, &mut code_verifier));

  // 3. 计算 code_verifier 字符串的 SHA-256 哈希值
  let hash_result = sha2::Sha256::digest(code_verifier);

  // 4. 将哈希结果编码为 URL 安全 Base64 字符串，这就是 code_challenge
  let mut code_challenge = [0; 43];
  __unwrap_panic!(URL_SAFE_NO_PAD.encode_slice(hash_result, &mut code_challenge));

  // 5. 同时返回 verifier 和 challenge
  (code_verifier, code_challenge)
}

const POLL_MAX_ATTEMPTS: u8 = 5;
const POLL_INTERVAL_MS: u64 = 1000;

pub async fn get_new_token(ext_token: &mut ExtToken, is_pri: bool) -> bool {
  let is_session = ext_token.primary_token.is_session();

  match if is_session {
    refresh_token(ext_token, is_pri).await
  } else {
    upgrade_token(ext_token, is_pri).await
  } {
    Some((new_token, s)) => {
      let tmp = Token::new(new_token, Some(s));
      if !is_session && ext_token.secondary_token.is_none() {
        let old_token = ::core::mem::replace(&mut ext_token.primary_token, tmp);
        ext_token.secondary_token = Some(old_token);
      } else {
        ext_token.primary_token = tmp;
      }
      true
    }
    None => false,
  }
}

async fn upgrade_token(ext_token: &ExtToken, is_pri: bool) -> Option<(RawToken, String)> {
  #[derive(::serde::Deserialize)]
  #[serde(rename_all = "camelCase")]
  struct PollResponse {
    pub access_token: String,
    // pub refresh_token: String,
    // pub challenge: String,
    // pub auth_id: String,
    // pub uuid: String,
  }

  let (verifier, challenge) = generate_pkce_pair();
  let verifier = unsafe { ::core::str::from_utf8_unchecked(&verifier) };
  let challenge = unsafe { ::core::str::from_utf8_unchecked(&challenge) };
  let mut buf = [0; 36];
  let uuid = uuid::Uuid::new_v4().hyphenated().encode_lower(&mut buf) as &str;

  let token = ext_token
    .secondary_token
    .as_ref()
    .unwrap_or(&ext_token.primary_token);
  let mut buf = [0; 31];
  let user_id = token.raw().subject.id.to_str(&mut buf) as &str;
  let auth_token = token.as_str();

  // 发起刷新请求
  let upgrade_response = super::client::build_token_upgrade_request(
    &ext_token.get_client(),
    uuid,
    challenge,
    user_id,
    auth_token,
    is_pri,
  )
  .send()
  .await
  .ok()?;

  if !upgrade_response.status().is_success() {
    return None;
  }

  // 轮询获取token
  for _ in 0..POLL_MAX_ATTEMPTS {
    let poll_response =
      super::client::build_token_poll_request(&ext_token.get_client(), uuid, verifier, is_pri)
        .send()
        .await
        .ok()?;

    match poll_response.status() {
      reqwest::StatusCode::OK => {
        let token = poll_response
          .json::<PollResponse>()
          .await
          .ok()?
          .access_token;
        return parse_token(token);
      }
      reqwest::StatusCode::NOT_FOUND => {
        tokio::time::sleep(::core::time::Duration::from_millis(POLL_INTERVAL_MS)).await;
      }
      _ => return None,
    }
  }

  None
}

async fn refresh_token(ext_token: &ExtToken, is_pri: bool) -> Option<(RawToken, String)> {
  const CLIENT_ID: &str = "KbZUR41cY7W6zRSdpSUJ7I7mLYBKOCmB";

  struct RefreshTokenRequest<'a> {
    refresh_token: &'a str,
  }

  impl ::serde::Serialize for RefreshTokenRequest<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
      S: ::serde::Serializer,
    {
      use ::serde::ser::SerializeStruct as _;
      let mut state = serializer.serialize_struct("RefreshTokenRequest", 3)?;
      state.serialize_field("grant_type", "refresh_token")?;
      state.serialize_field("client_id", CLIENT_ID)?;
      state.serialize_field("refresh_token", self.refresh_token)?;
      state.end()
    }
  }

  #[derive(::serde::Deserialize)]
  struct RefreshTokenResponse {
    access_token: String,
    // id_token: String,
    // #[serde(rename = "shouldLogout")]
    // should_logout: bool,
  }

  let refresh_request = RefreshTokenRequest {
    refresh_token: ext_token.primary_token.as_str(),
  };

  let body = serde_json::to_vec(&refresh_request).ok()?;

  let response = super::client::build_token_refresh_request(&ext_token.get_client(), is_pri, body)
    .send()
    .await
    .ok()?;

  let token = response
    .json::<RefreshTokenResponse>()
    .await
    .ok()?
    .access_token;

  parse_token(token)
}

// 提取token解析逻辑
#[inline]
fn parse_token(token_string: String) -> Option<(RawToken, String)> {
  let raw_token = token_string.parse().ok()?;
  Some((raw_token, token_string))
}

pub async fn get_server_config(ext_token: ExtToken, is_pri: bool) -> Option<uuid::Uuid> {
  let response = {
    let client = super::client::build_client_request(super::client::AiServiceRequest {
      ext_token,
      fs_client_key: None,
      url: server_config_url(is_pri),
      is_stream: false,
      trace_id: Some(new_uuid_v4()),
      is_pri,
      cookie: None,
    });
    client.send().await.ok()?.bytes().await.ok()?
  };
  let server_config = GetServerConfigResponse::decode(response.as_ref()).ok()?;
  uuid::Uuid::try_parse(&server_config.config_version).ok()
}

// pub async fn get_geo_cpp_backend_url(
//     client: Client,
//     auth_token: &str,
//     checksum: Checksum,
//     client_key: Hash,
//     timezone: &'static str,
//     session_id: Option<uuid::Uuid>,
//     is_pri: bool,
// ) -> Option<String> {
//     let response = {
//         let client = super::client::build_client_request(super::client::AiServiceRequest {
//             client,
//             auth_token,
//             checksum,
//             client_key,
//             fs_client_key: None,
//             url: crate::app::lazy::cpp_config_url(is_pri),
//             is_stream: false,
//             config_version: None,
//             timezone,
//             trace_id: Some(new_uuid_v4()),
//             session_id,
//             is_pri,
//         });
//         let request = crate::core::aiserver::v1::CppConfigRequest::default();
//         client
//             .body(__unwrap!(encode_message(&request, false)))
//             .send()
//             .await
//             .ok()?
//             .bytes()
//             .await
//             .ok()?
//     };
//     crate::core::aiserver::v1::CppConfigResponse::decode(response.as_ref())
//         .ok()
//         .map(|res| res.geo_cpp_backend_url)
// }

const EMPTY_JSON: bytes::Bytes = bytes::Bytes::from_static(b"{}");

pub async fn get_teams(
  client: &Client,
  user_id: &str,
  auth_token: &str,
  is_pri: bool,
) -> Option<Vec<Team>> {
  let request = super::client::build_proto_web_request(
    client, user_id, auth_token, teams_url, is_pri, EMPTY_JSON,
  );

  request
    .send()
    .await
    .ok()?
    .json::<GetTeamsResponse>()
    .await
    .ok()
    .map(|r| r.teams)
}

pub async fn get_is_on_new_pricing(
  client: &Client,
  user_id: &str,
  auth_token: &str,
  is_pri: bool,
) -> Option<bool> {
  let request = super::client::build_proto_web_request(
    client,
    user_id,
    auth_token,
    is_on_new_pricing_url,
    is_pri,
    EMPTY_JSON,
  );

  #[derive(serde::Deserialize)]
  struct PricingConfig {
    #[serde(rename = "isOnNewPricing")]
    is_on_new_pricing: bool,
  }

  request
    .send()
    .await
    .ok()?
    .json::<PricingConfig>()
    .await
    .ok()
    .map(|r| r.is_on_new_pricing)
}

pub async fn get_sessions(
  client: &Client,
  user_id: &str,
  auth_token: &str,
  is_pri: bool,
) -> Option<Vec<Session>> {
  let request = super::client::build_sessions_request(client, user_id, auth_token, is_pri);

  request
    .send()
    .await
    .ok()?
    .json::<ListActiveSessionsResponse>()
    .await
    .ok()
    .map(|r| r.sessions)
}

#[allow(unused)]
pub async fn get_aggregated_usage_events(
  client: &Client,
  user_id: &str,
  auth_token: &str,
  is_pri: bool,
) -> Option<GetAggregatedUsageEventsResponse> {
  let request = super::client::build_proto_web_request(
    client,
    user_id,
    auth_token,
    aggregated_usage_events_url,
    is_pri,
    bytes::Bytes::from(__unwrap!(serde_json::to_vec(&{
      const DELTA: chrono::TimeDelta = chrono::TimeDelta::new(2629743, 765840000).unwrap();
      let now = DateTime::utc_now();
      let start_date = now - DELTA;
      GetAggregatedUsageEventsRequest {
        team_id: -1,
        start_date: Some(start_date.timestamp_millis()),
        end_date: Some(now.timestamp_millis()),
        user_id: None,
      }
    }))),
  );

  request
    .send()
    .await
    .ok()?
    .json::<GetAggregatedUsageEventsResponse>()
    .await
    .ok()
}

pub struct FilteredUsageArgs {
  pub start: Option<DateTime>,
  pub end: Option<DateTime>,
  pub model_id: Option<&'static str>,
  pub size: Option<i32>,
}

impl From<FilteredUsageArgs> for GetFilteredUsageEventsRequest {
  #[inline]
  fn from(args: FilteredUsageArgs) -> Self {
    const TZ: chrono::FixedOffset = chrono::FixedOffset::west_opt(16 * 3600).unwrap();
    const TIME: chrono::NaiveTime = chrono::NaiveTime::from_hms_opt(0, 0, 0).unwrap();
    const START: chrono::TimeDelta = chrono::TimeDelta::days(-7);
    const END: chrono::TimeDelta = chrono::TimeDelta::new(86399, 999000000).unwrap();

    let (start_date, end_date) = if let (Some(a), Some(b)) = (args.start, args.end) {
      (a.timestamp_millis(), b.timestamp_millis())
    } else {
      let now = chrono::DateTime::<chrono::FixedOffset>::from_naive_utc_and_offset(
        DateTime::naive_now(),
        TZ,
      )
      .date_naive()
      .and_time(TIME);
      match (args.start, args.end) {
        (None, None) => (
          (now + START)
            .and_local_timezone(TZ)
            .unwrap()
            .timestamp_millis(),
          (now + END)
            .and_local_timezone(TZ)
            .unwrap()
            .timestamp_millis(),
        ),
        (None, Some(b)) => (
          (now + START)
            .and_local_timezone(TZ)
            .unwrap()
            .timestamp_millis(),
          b.timestamp_millis(),
        ),
        (Some(a), None) => (
          a.timestamp_millis(),
          (now + END)
            .and_local_timezone(TZ)
            .unwrap()
            .timestamp_millis(),
        ),
        (Some(_), Some(_)) => unsafe { ::core::hint::unreachable_unchecked() },
      }
    };
    Self {
      team_id: 0,
      start_date: Some(start_date),
      end_date: Some(end_date),
      user_id: None,
      model_id: args.model_id.map(ToString::to_string),
      page: Some(1),
      page_size: Some(args.size.unwrap_or(100)),
    }
  }
}

pub async fn get_filtered_usage_events(
  client: &Client,
  user_id: &str,
  auth_token: &str,
  is_pri: bool,
  args: FilteredUsageArgs,
) -> Option<GetFilteredUsageEventsResponse> {
  let request = super::client::build_proto_web_request(
    client,
    user_id,
    auth_token,
    filtered_usage_events_url,
    is_pri,
    bytes::Bytes::from(__unwrap!(serde_json::to_vec(&{
      let req: GetFilteredUsageEventsRequest = args.into();
      req
    }))),
  );

  let res = request.send().await.ok()?;
  crate::debug!("<get_filtered_usage_events> {}", res.status());
  let res = res.bytes().await.ok()?;
  crate::debug!("<get_filtered_usage_events> {}", unsafe {
    ::core::str::from_utf8_unchecked(&res[..])
  });
  serde_json::from_slice(&res[..]).ok()
  // .json::<GetFilteredUsageEventsResponse>()
  // .await
  // .ok()
}

#[inline]
pub fn new_uuid_v4() -> [u8; 36] {
  let mut buf = [0; 36];
  uuid::Uuid::new_v4().hyphenated().encode_lower(&mut buf);
  buf
}
