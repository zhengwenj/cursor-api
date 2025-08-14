use std::sync::LazyLock;

use axum::{
    body::Body,
    http::{HeaderMap, header::CONTENT_TYPE},
    response::{IntoResponse as _, Response},
};

use crate::app::{
    constant::HEADER_VALUE_TEXT_PLAIN_UTF8,
    model::{Checksum, Hash, TimestampHeader},
};

static HEADERS_TEXT_PLAIN: LazyLock<HeaderMap> =
    LazyLock::new(|| HeaderMap::from_iter([(CONTENT_TYPE, HEADER_VALUE_TEXT_PLAIN_UTF8)]));

pub async fn handle_gen_uuid() -> Response {
    let mut buf = [0u8; 36];
    ::uuid::Uuid::new_v4().hyphenated().encode_lower(&mut buf);
    let body = Body::from(buf.to_vec());

    (HEADERS_TEXT_PLAIN.clone(), body).into_response()
}

pub async fn handle_gen_hash() -> Response {
    let mut buf = [0u8; 64];
    Hash::random().to_str(&mut buf);
    let body = Body::from(buf.to_vec());

    (HEADERS_TEXT_PLAIN.clone(), body).into_response()
}

pub async fn handle_gen_checksum() -> Response {
    let mut buf = [0u8; 137];
    Checksum::random().to_str(&mut buf);
    let body = Body::from(buf.to_vec());

    (HEADERS_TEXT_PLAIN.clone(), body).into_response()
}

pub async fn handle_get_timestamp_header() -> Response {
    static TIMESTAMP_HEADER: ::bytes::Bytes =
        ::bytes::Bytes::from_static(TimestampHeader::as_str().as_bytes());
    (HEADERS_TEXT_PLAIN.clone(), TIMESTAMP_HEADER.clone()).into_response()
}
