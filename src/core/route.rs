mod logs;
pub use logs::{handle_get_logs, handle_get_logs_tokens, handle_logs};
mod health;
pub use health::{handle_health, handle_root};
mod token;
pub use token::{handle_build_key, handle_gen_token, handle_get_config_version};
mod tokens;
pub use tokens::{
    handle_add_tokens, handle_delete_tokens, handle_get_tokens, handle_refresh_tokens,
    handle_set_tokens, handle_set_tokens_alias, handle_set_tokens_proxy, handle_set_tokens_status,
    handle_set_tokens_timezone, handle_update_tokens_config_version, handle_update_tokens_profile,
};
mod checksum;
pub use checksum::{
    handle_gen_checksum, handle_gen_hash, handle_gen_uuid, handle_get_timestamp_header,
};
// mod profile;
// pub use profile::{handle_token_upgrade, handle_user_info};
mod proxies;
pub use proxies::{
    handle_add_proxy, handle_delete_proxies, handle_get_proxies, handle_set_general_proxy,
    handle_set_proxies,
};
mod page;
pub use page::{
    handle_about, handle_api_page, handle_build_key_page, handle_config_page, handle_env_example,
    handle_proxies_page, handle_readme, handle_static, handle_tokens_page,
};

pub async fn handle_options() -> axum::http::StatusCode { axum::http::StatusCode::OK }
