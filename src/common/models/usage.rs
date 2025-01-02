use serde::Serialize;

#[derive(Serialize)]
pub enum GetUserInfo {
    #[serde(rename = "usage")]
    Usage(UserUsageInfo),
    #[serde(rename = "error")]
    Error(String),
}

#[derive(Serialize, Clone)]
pub struct UserUsageInfo {
    pub fast_requests: u32,
    pub max_fast_requests: u32,
}
