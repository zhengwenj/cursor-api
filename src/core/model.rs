pub mod anthropic;
pub mod openai;
mod resolver;

pub(crate) use resolver::{ExtModel, init_resolver};
use serde::{Serialize, ser::SerializeStruct as _};

use super::constant::Models;

#[derive(
    ::serde::Serialize,
    ::serde::Deserialize,
    ::rkyv::Archive,
    ::rkyv::Serialize,
    ::rkyv::Deserialize,
    Clone,
    Copy,
    PartialEq,
)]
#[repr(u8)]
pub enum Role {
    #[serde(rename = "system", alias = "developer")]
    System = 0u8,
    #[serde(rename = "user", alias = "human")]
    User,
    #[serde(rename = "assistant", alias = "ai")]
    Assistant,
}

// 模型定义
#[derive(Clone, Copy)]
pub struct Model {
    pub server_id: &'static str,
    pub client_id: &'static str,
    pub id: &'static str,
    pub owned_by: &'static str,
    pub is_thinking: bool,
    pub is_image: bool,
    pub is_max: bool,
    pub is_non_max: bool,
}

impl Serialize for Model {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        // 系统常量
        const MODEL_OBJECT: &str = "model";
        const CREATED: &i64 = &1706659200;
        const CREATED_AT: &str = "2024-01-31T00:00:00Z";

        let mut state = serializer.serialize_struct(MODEL_OBJECT, 11)?;

        state.serialize_field("id", &self.id)?;
        state.serialize_field("display_name", &self.client_id)?;
        state.serialize_field("created", CREATED)?;
        state.serialize_field("created_at", CREATED_AT)?;
        state.serialize_field("object", MODEL_OBJECT)?;
        state.serialize_field("type", MODEL_OBJECT)?;
        state.serialize_field("owned_by", &self.owned_by)?;
        state.serialize_field("supports_thinking", &self.is_thinking)?;
        state.serialize_field("supports_images", &self.is_image)?;
        state.serialize_field("supports_max_mode", &self.is_max)?;
        state.serialize_field("supports_non_max_mode", &self.is_non_max)?;

        state.end()
    }
}

impl PartialEq for Model {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.is_thinking == other.is_thinking
            && self.is_image == other.is_image
            && self.is_max == other.is_max
            && self.is_non_max == other.is_non_max
    }
}

pub struct ModelsResponse;

impl Serialize for ModelsResponse {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ModelsResponse", 2)?;

        state.serialize_field("object", "list")?;
        state.serialize_field("data", &Models::to_arc())?;

        state.end()
    }
}

#[repr(transparent)]
pub struct RawModelsResponse(pub(super) ::std::sync::Arc<crate::core::aiserver::v1::AvailableModelsResponse>);

impl Serialize for RawModelsResponse {
    #[inline]
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("RawModelsResponse", 3)?;

        state.serialize_field("raw", &self.0)?;
        state.serialize_field("dur", &Models::last_update_elapsed())?;
        state.serialize_field("now", &crate::app::model::DateTime::now())?;

        state.end()
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct MessageId(u128);

impl MessageId {
    pub const fn new(v: u128) -> Self { Self(v) }

    #[allow(clippy::wrong_self_convention)]
    #[inline(always)]
    pub fn to_str<'buf>(&self, buf: &'buf mut [u8; 22]) -> &'buf mut str {
        crate::common::utils::base62::encode_bytes(self.0, buf);
        unsafe { ::core::str::from_utf8_unchecked_mut(buf) }
    }
}

impl ::core::fmt::Display for MessageId {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.write_str(self.to_str(&mut [0; 22]))
    }
}
