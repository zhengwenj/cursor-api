use axum::{
    http::{HeaderMap, header::CONTENT_TYPE},
    response::{IntoResponse as _, Response},
};

use crate::app::{
    constant::HEADER_VALUE_TEXT_PLAIN_UTF8,
    model::{Checksum, Hash, TimestampHeader},
};

pub async fn handle_gen_uuid() -> Response {
    let uuid = uuid::Uuid::new_v4().to_string();

    let headers = HeaderMap::from_iter([(CONTENT_TYPE, HEADER_VALUE_TEXT_PLAIN_UTF8)]);

    (headers, uuid).into_response()
}

pub async fn handle_gen_hash() -> Response {
    let hash = Hash::random().to_string();

    let headers = HeaderMap::from_iter([(CONTENT_TYPE, HEADER_VALUE_TEXT_PLAIN_UTF8)]);

    (headers, hash).into_response()
}

pub async fn handle_gen_checksum() -> Response {
    let checksum = Checksum::random().to_string();

    let headers = HeaderMap::from_iter([(CONTENT_TYPE, HEADER_VALUE_TEXT_PLAIN_UTF8)]);

    (headers, checksum).into_response()
}

pub async fn handle_get_timestamp_header() -> Response {
    let timestamp_header = TimestampHeader::get_global().to_string();

    let headers = HeaderMap::from_iter([(CONTENT_TYPE, HEADER_VALUE_TEXT_PLAIN_UTF8)]);

    (headers, timestamp_header).into_response()
}
