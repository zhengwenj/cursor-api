use serde::{Deserialize, Serialize};

use crate::{app::constant::COMMA, core::constant::Models};

#[derive(Deserialize)]
pub struct BuildKeyRequest {
    pub token: super::RawToken,
    pub checksum: super::Checksum,
    pub client_key: super::Hash,
    pub config_version: Option<uuid::Uuid>,
    pub session_id: uuid::Uuid,
    pub secret: Option<String>,
    pub proxy_name: Option<String>,
    pub timezone: Option<String>,
    pub gcpp_host: Option<super::GcppHost>,
    pub disable_vision: Option<bool>,
    pub enable_slow_pool: Option<bool>,
    pub include_web_references: Option<bool>,
    pub usage_check_models: Option<UsageCheckModelConfig>,
}

pub struct UsageCheckModelConfig {
    pub model_type: UsageCheckModelType,
    pub model_ids: Vec<&'static str>,
}

impl<'de> Deserialize<'de> for UsageCheckModelConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Helper {
            r#type: UsageCheckModelType,
            #[serde(default)]
            model_ids: String,
        }

        let helper = Helper::deserialize(deserializer)?;

        let model_ids = if helper.model_ids.is_empty() {
            Vec::new()
        } else {
            helper
                .model_ids
                .split(COMMA)
                .filter_map(|model| {
                    let model = model.trim();
                    Models::find_id(model).map(|m| m.id)
                })
                .collect()
        };

        Ok(UsageCheckModelConfig {
            model_type: helper.r#type,
            model_ids,
        })
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UsageCheckModelType {
    Default,
    Disabled,
    All,
    Custom,
}

#[derive(Serialize)]
#[serde(rename_all = "lowercase")]
pub enum BuildKeyResponse {
    Keys([String; 3]),
    Error(&'static str),
}

#[derive(Deserialize)]
pub struct GetConfigVersionRequest {
    pub token: super::RawToken,
    pub checksum: super::Checksum,
    pub client_key: super::Hash,
    pub session_id: uuid::Uuid,
    pub proxy_name: Option<String>,
    pub timezone: Option<String>,
    pub gcpp_host: Option<super::GcppHost>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetConfigVersionResponse {
    ConfigVersion(uuid::Uuid),
    Error(&'static str),
}
