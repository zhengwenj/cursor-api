use prost::Message as _;

use crate::{
    AppConfig,
    app::{
        lazy::KEY_PREFIX,
        model::{Randomness, RawToken, Subject, TokenDuration, UserId},
    },
    common::utils::from_base64,
};

// include!(concat!(env!("OUT_DIR"), "/key.rs"));
include!("config/key.rs");

impl KeyConfig {
    pub fn new_with_global() -> Self {
        Self {
            token_info: None,
            secret: None,
            disable_vision: Some(AppConfig::get_vision_ability().is_none()),
            enable_slow_pool: Some(AppConfig::get_slow_pool()),
            usage_check_models: None,
            include_web_references: Some(AppConfig::get_web_refs()),
        }
    }

    pub fn copy_without_auth_token(&self, config: &mut Self) {
        if self.disable_vision.is_some() {
            config.disable_vision = self.disable_vision;
        }
        if self.enable_slow_pool.is_some() {
            config.enable_slow_pool = self.enable_slow_pool;
        }
        if self.usage_check_models.is_some() {
            config.usage_check_models = self.usage_check_models.clone();
        }
        if self.include_web_references.is_some() {
            config.include_web_references = self.include_web_references;
        }
    }
}

impl key_config::token_info::Token {
    #[inline]
    pub fn from_raw(raw: RawToken) -> Self {
        Self {
            provider: raw.subject.provider.to_string(),
            signature: raw.signature.to_vec(),
            sub_id: raw.subject.id.to_bytes().to_vec(),
            randomness: raw.randomness.to_bytes().to_vec(),
            start: raw.duration.start,
            end: raw.duration.end,
            is_session: raw.is_session,
        }
    }

    #[inline]
    pub fn into_raw(self) -> Option<RawToken> {
        Some(RawToken {
            subject: Subject {
                provider: self.provider.parse().ok()?,
                id: UserId::from_bytes(self.sub_id.try_into().ok()?),
            },
            randomness: Randomness::from_bytes(self.randomness.try_into().ok()?),
            signature: self.signature.try_into().ok()?,
            duration: TokenDuration {
                start: self.start,
                end: self.end,
            },
            is_session: self.is_session,
        })
    }
}

pub fn parse_dynamic_token(auth_token: &str) -> Option<KeyConfig> {
    auth_token
        .strip_prefix(&**KEY_PREFIX)
        .and_then(from_base64)
        .and_then(|decoded_bytes| KeyConfig::decode(&decoded_bytes[..]).ok())
}
