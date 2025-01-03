mod checksum;
pub use checksum::*;
mod tokens;
pub use tokens::*;
use prost::Message as _;

use crate::{app::constant::CURSOR_API2_GET_USER_INFO, chat::aiserver::v1::GetUserInfoResponse};

use super::models::usage::{StripeProfile, UserUsageInfo};

pub fn parse_bool_from_env(key: &str, default: bool) -> bool {
    std::env::var(key)
        .ok()
        .map(|v| match v.to_lowercase().as_str() {
            "true" | "1" => true,
            "false" | "0" => false,
            _ => default,
        })
        .unwrap_or(default)
}

pub fn parse_string_from_env(key: &str, default: &str) -> String {
    std::env::var(key).unwrap_or_else(|_| default.to_string())
}

pub fn i32_to_u32(value: i32) -> u32 {
    if value < 0 {
        0
    } else {
        value as u32
    }
}

pub async fn get_user_usage(auth_token: &str, checksum: &str) -> Option<UserUsageInfo> {
    // 构建请求客户端
    let client = super::client::build_client(auth_token, checksum, CURSOR_API2_GET_USER_INFO);
    let response = client
        .body(Vec::new())
        .send()
        .await
        .ok()?
        .bytes()
        .await
        .ok()?;
    let user_info = GetUserInfoResponse::decode(response.as_ref()).ok()?;

    let (mtype, trial_days) = get_stripe_profile(auth_token).await?;

    user_info.usage.map(|user_usage| UserUsageInfo {
        fast_requests: i32_to_u32(user_usage.gpt4_requests),
        max_fast_requests: i32_to_u32(user_usage.gpt4_max_requests),
        mtype,
        trial_days,
    })
}

pub async fn get_stripe_profile(auth_token: &str) -> Option<(String, u32)> {
    let client = super::client::build_profile_client(auth_token);
    let response = client.send().await.ok()?.json::<StripeProfile>().await.ok()?;
    Some((response.membership_type, i32_to_u32(response.days_remaining_on_trial)))
}

pub fn validate_token_and_checksum(auth_token: &str) -> Option<(String, String, Option<String>)> {
    // 提取 token、checksum 和可能的 alias
    let (token, checksum, alias) = {
        // 先尝试提取 alias
        let (token_part, alias) = if let Some(pos) = auth_token.find("::") {
            let (alias, rest) = auth_token.split_at(pos);
            (&rest[2..], Some(alias))
        } else if let Some(pos) = auth_token.find("%3A%3A") {
            let (alias, rest) = auth_token.split_at(pos);
            (&rest[6..], Some(alias))
        } else {
            (auth_token, None)
        };

        // 提取 token 和 checksum
        if let Some(comma_pos) = token_part.find(',') {
            let (token, checksum) = token_part.split_at(comma_pos);
            (token, &checksum[1..], alias)
        } else {
            return None; // 缺少必要的 checksum
        }
    };

    // 验证 token 和 checksum 有效性
    if validate_token(token) && validate_checksum(checksum) {
        Some((token.to_string(), checksum.to_string(), alias.map(String::from)))
    } else {
        None
    }
}
