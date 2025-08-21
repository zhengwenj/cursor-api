mod payment_id;
mod subscription_status;

use ::chrono::{DateTime, Utc};
use ::rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use ::serde::{Deserialize, Serialize};
pub use payment_id::PaymentId;
pub use subscription_status::SubscriptionStatus;

// #[derive(Serialize)]
// #[serde(untagged)]
// pub enum GetUserInfo {
//     Usage(Box<(UsageProfile, UserProfile, StripeProfile)>),
//     Error { error: String },
// }

// #[derive(Deserialize, Serialize, Clone, Archive, RkyvDeserialize, RkyvSerialize)]
// pub struct TokenProfile {
//     pub usage: UsageProfile,
//     pub user: UserProfile,
//     pub stripe: StripeProfile,
// }

#[derive(PartialEq, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
#[repr(u8)]
pub enum MembershipType {
    Free,
    FreeTrial,
    Pro,
    ProPlus,
    Ultra,
    Enterprise,
}

impl MembershipType {
    // 定义常量字符串
    const FREE: &'static str = "free";
    const PRO: &'static str = "pro";
    const PRO_PLUS: &'static str = "pro_plus";
    const ENTERPRISE: &'static str = "enterprise";
    const FREE_TRIAL: &'static str = "free_trial";
    const ULTRA: &'static str = "ultra";

    #[inline]
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            Self::FREE => Some(MembershipType::Free),
            Self::FREE_TRIAL => Some(MembershipType::FreeTrial),
            Self::PRO => Some(MembershipType::Pro),
            Self::PRO_PLUS => Some(MembershipType::ProPlus),
            Self::ULTRA => Some(MembershipType::Ultra),
            Self::ENTERPRISE => Some(MembershipType::Enterprise),
            _ => None,
        }
    }

    #[inline]
    pub fn as_str(&self) -> &'static str {
        match self {
            MembershipType::Free => Self::FREE,
            MembershipType::FreeTrial => Self::FREE_TRIAL,
            MembershipType::Pro => Self::PRO,
            MembershipType::ProPlus => Self::PRO_PLUS,
            MembershipType::Ultra => Self::ULTRA,
            MembershipType::Enterprise => Self::ENTERPRISE,
        }
    }
}

impl ::serde::Serialize for MembershipType {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> ::serde::Deserialize<'de> for MembershipType {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        let s = <String as ::serde::Deserialize>::deserialize(deserializer)?;
        Self::from_str(&s)
            .ok_or_else(|| ::serde::de::Error::custom(format_args!("unknown membership type: {s}")))
    }
}

#[derive(Deserialize, Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct StripeProfile {
    #[serde(alias = "membershipType")]
    pub membership_type: MembershipType,
    #[serde(alias = "paymentId", default, skip_serializing_if = "Option::is_none")]
    pub payment_id: Option<PaymentId>,
    #[serde(alias = "daysRemainingOnTrial")]
    pub days_remaining_on_trial: u32,
    #[serde(alias = "subscriptionStatus")]
    pub subscription_status: Option<SubscriptionStatus>,
    #[serde(alias = "verifiedStudent", default)]
    pub verified_student: bool,
    #[serde(alias = "trialEligible", default)]
    pub trial_eligible: bool,
    #[serde(alias = "isOnStudentPlan", default)]
    pub is_on_student_plan: bool,
    // #[serde(alias = "customerBalance")]
    // pub customer_balance: Option<f64>,
}

// #[derive(Deserialize, Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
// pub struct ModelUsage {
//     #[serde(alias = "numRequests")]
//     pub num_requests: u32,
//     #[serde(
//         alias = "numRequestsTotal",
//         default,
//         skip_serializing_if = "Option::is_none"
//     )]
//     pub total_requests: Option<u32>,
//     #[serde(alias = "numTokens")]
//     pub num_tokens: u32,
//     #[serde(alias = "maxRequestUsage", skip_serializing_if = "Option::is_none")]
//     pub max_requests: Option<u32>,
//     #[serde(alias = "maxTokenUsage", skip_serializing_if = "Option::is_none")]
//     pub max_tokens: Option<u32>,
// }

// #[derive(Deserialize, Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
// pub struct UsageProfile {
//     #[serde(alias = "gpt-4")]
//     pub premium: ModelUsage,
//     #[serde(alias = "gpt-3.5-turbo")]
//     pub standard: ModelUsage,
//     // #[serde(alias = "gpt-4-32k")]
//     // pub unknown: ModelUsage,
//     #[serde(alias = "startOfMonth")]
//     pub start_of_month: DateTime<Utc>,
// }

#[derive(Deserialize, Serialize, Clone, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct UserProfile {
    pub email: String,
    // pub email_verified: bool,
    pub name: String,
    // #[serde(alias = "sub")]
    // pub id: UserId,
    pub updated_at: DateTime<Utc>,
    // Image link, rendered in /logs? and /tokens?
    pub picture: Option<String>,
    #[serde(skip_deserializing)]
    pub is_on_new_pricing: bool,
}

#[derive(Deserialize, Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
#[serde(rename_all(serialize = "snake_case"))]
#[repr(u8)]
pub enum SessionType {
    #[serde(alias = "SESSION_TYPE_UNSPECIFIED")]
    Unspecified,
    #[serde(alias = "SESSION_TYPE_WEB")]
    Web,
    #[serde(alias = "SESSION_TYPE_CLIENT")]
    Client,
    #[serde(alias = "SESSION_TYPE_BUGBOT")]
    Bugbot,
    #[serde(alias = "SESSION_TYPE_BACKGROUND_AGENT")]
    BackgroundAgent,
}

#[derive(Deserialize, Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct Session {
    pub session_id: crate::app::model::Hash,
    pub r#type: SessionType,
    pub created_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
}

#[derive(Deserialize)]
pub struct ListActiveSessionsResponse {
    pub sessions: Vec<Session>,
}

/// aiserver.v1.Team
#[derive(::serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Team {
    // pub name: String,
    // pub id: i32,
    // pub role: TeamRole,
    // pub seats: i32,
    #[serde(default)]
    pub has_billing: bool,
    // #[serde(default)]
    // pub request_quota_per_seat: i32,
    // #[serde(default)]
    // pub privacy_mode_forced: bool,
    // #[serde(default)]
    // pub allow_sso: bool,
    // #[serde(default)]
    // pub admin_only_usage_pricing: bool,
    pub subscription_status: Option<SubscriptionStatus>,
    // #[serde(default)]
    // pub bedrock_iam_role: String,
    // #[serde(default)]
    // pub verified: bool,
    // #[serde(default)]
    // pub is_enterprise: bool,
}
#[derive(::serde::Deserialize)]
pub struct GetTeamsResponse {
    #[serde(default)]
    pub teams: Vec<Team>,
}
// #[derive(::serde::Serialize, Clone, Copy, Archive, RkyvDeserialize, RkyvSerialize)]
// #[serde(rename_all(serialize = "snake_case"))]
// #[repr(u8)]
// pub enum TeamRole {
//     Unspecified = 0,
//     Owner = 1,
//     Member = 2,
//     FreeOwner = 3,
// }
// impl TeamRole {
//     const STR_UNSPECIFIED: &'static str = "TEAM_ROLE_UNSPECIFIED";
//     const STR_OWNER: &'static str = "TEAM_ROLE_OWNER";
//     const STR_MEMBER: &'static str = "TEAM_ROLE_MEMBER";
//     const STR_FREE_OWNER: &'static str = "TEAM_ROLE_FREE_OWNER";
//     // pub fn as_str_name(&self) -> &'static str {
//     //     match self {
//     //         Self::Unspecified => Self::STR_UNSPECIFIED,
//     //         Self::Owner => Self::STR_OWNER,
//     //         Self::Member => Self::STR_MEMBER,
//     //         Self::FreeOwner => Self::STR_FREE_OWNER,
//     //     }
//     // }
//     pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
//         match value {
//             Self::STR_UNSPECIFIED => Some(Self::Unspecified),
//             Self::STR_OWNER => Some(Self::Owner),
//             Self::STR_MEMBER => Some(Self::Member),
//             Self::STR_FREE_OWNER => Some(Self::FreeOwner),
//             _ => None,
//         }
//     }
// }
// impl<'de> ::serde::Deserialize<'de> for TeamRole {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: ::serde::Deserializer<'de>,
//     {
//         struct TeamRoleVisitor;

//         impl<'de> ::serde::de::Visitor<'de> for TeamRoleVisitor {
//             type Value = TeamRole;

//             fn expecting(&self, formatter: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
//                 formatter.write_str("a valid TeamRole string")
//             }

//             fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
//             where
//                 E: ::serde::de::Error,
//             {
//                 TeamRole::from_str_name(value)
//                     .ok_or_else(|| E::custom(format_args!("unknown team role value: {value}")))
//             }
//         }

//         deserializer.deserialize_str(TeamRoleVisitor)
//     }
// }
