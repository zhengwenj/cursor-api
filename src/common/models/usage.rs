use serde::{Deserialize, Serialize};

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
    pub mtype: String,
    pub trial_days: u32,
}

#[derive(Deserialize)]
pub struct StripeProfile {
    #[serde(rename = "membershipType")]
    pub membership_type: String,
    #[serde(rename = "daysRemainingOnTrial")]
    pub days_remaining_on_trial: i32,
}
