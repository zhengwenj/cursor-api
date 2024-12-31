mod checksum;
pub use checksum::*;

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
