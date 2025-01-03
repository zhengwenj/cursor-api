use crate::{
    chat::constant::ERR_NODATA,
    common::{
        models::usage::GetUserInfo,
        utils::{generate_checksum_with_default, get_user_usage},
    },
};
use axum::Json;

use super::token::TokenRequest;

pub async fn get_user_info(Json(request): Json<TokenRequest>) -> Json<GetUserInfo> {
    let auth_token = match request.token {
        Some(token) => token,
        None => return Json(GetUserInfo::Error(ERR_NODATA.to_string())),
    };

    // 解析 token 和 checksum
    let (token_part, checksum) = if let Some(pos) = auth_token.find("::") {
        let (_, rest) = auth_token.split_at(pos + 2);
        if let Some(comma_pos) = rest.find(',') {
            let (token, checksum) = rest.split_at(comma_pos);
            (token, checksum[1..].to_string())
        } else {
            (rest, generate_checksum_with_default())
        }
    } else if let Some(pos) = auth_token.find("%3A%3A") {
        let (_, rest) = auth_token.split_at(pos + 6);
        if let Some(comma_pos) = rest.find(',') {
            let (token, checksum) = rest.split_at(comma_pos);
            (token, checksum[1..].to_string())
        } else {
            (rest, generate_checksum_with_default())
        }
    } else {
        if let Some(comma_pos) = auth_token.find(',') {
            let (token, checksum) = auth_token.split_at(comma_pos);
            (token, checksum[1..].to_string())
        } else {
            (&auth_token[..], generate_checksum_with_default())
        }
    };

    match get_user_usage(&token_part, &checksum).await {
        Some(usage) => Json(GetUserInfo::Usage(usage)),
        None => Json(GetUserInfo::Error(ERR_NODATA.to_string())),
    }
}
