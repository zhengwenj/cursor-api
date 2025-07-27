use std::{borrow::Cow, convert::Infallible, sync::Arc};

use axum::{
    Json,
    body::Body,
    response::{IntoResponse as _, Response},
};
use bytes::Bytes;
use futures::StreamExt as _;
use http::{
    Extensions, HeaderMap, StatusCode,
    header::{
        ACCESS_CONTROL_ALLOW_CREDENTIALS, ACCESS_CONTROL_ALLOW_HEADERS, CACHE_CONTROL, CONNECTION,
        CONTENT_LENGTH, CONTENT_TYPE, COOKIE, TRANSFER_ENCODING, VARY,
    },
};
use tokio::sync::Mutex;

use crate::{
    app::{
        constant::{
            CHUNKED, CLIENT_KEY, ERR_STREAM_RESPONSE, ERROR, EVENT_STREAM, JSON, KEEP_ALIVE,
            NO_CACHE_REVALIDATE,
        },
        lazy::{cpp_config_url, cpp_models_url},
        model::{CppService, ExtToken},
    },
    common::{
        client::{AiServiceRequest, build_client_request},
        model::{GenericError, error::ChatError},
        utils::{encode_message, new_uuid_v4},
    },
    core::{
        aiserver::v1::{
            AvailableCppModelsResponse, CppConfigRequest, CppConfigResponse, FsSyncFileRequest,
            FsSyncFileResponse, FsUploadFileRequest, FsUploadFileResponse, StreamCppRequest,
        },
        error::StreamError,
        stream::decoder::{
            cpp::{StreamDecoder, StreamMessage},
            direct,
            types::DecodedMessage,
        },
    },
};

pub async fn handle_cpp_config(
    mut headers: HeaderMap,
    mut extensions: Extensions,
    Json(request): Json<CppConfigRequest>,
) -> Result<Json<CppConfigResponse>, Response> {
    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");

    let req = build_client_request(AiServiceRequest {
        ext_token,
        fs_client_key: headers.remove(CLIENT_KEY),
        url: cpp_config_url(is_pri),
        is_stream: false,
        trace_id: Some(new_uuid_v4()),
        is_pri,
        cookie: headers.remove(COOKIE),
    });

    let body = __unwrap!(encode_message(&request, false));

    match async { req.body(body).send().await?.bytes().await }.await {
        Ok(bytes) => match direct::decode::<CppConfigResponse>(&bytes) {
            Ok(DecodedMessage::Protobuf(data)) => Ok(Json(data)),
            Ok(DecodedMessage::Text(s)) => Err(__unwrap!(
                Response::builder()
                    .header(CONTENT_TYPE, JSON)
                    .header(CONTENT_LENGTH, s.len())
                    .body(Body::from(s))
            )),
            Err(e) => Err((
                StatusCode::BAD_GATEWAY,
                Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
            )
                .into_response()),
        },
        Err(mut e) => {
            e = e.without_url();

            Err((
                if e.is_timeout() {
                    StatusCode::GATEWAY_TIMEOUT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                },
                Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
            )
                .into_response())
        }
    }
}

pub async fn handle_cpp_models(
    mut headers: HeaderMap,
    mut extensions: Extensions,
) -> Result<Json<AvailableCppModelsResponse>, Response> {
    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");

    let req = build_client_request(AiServiceRequest {
        ext_token,
        fs_client_key: headers.remove(CLIENT_KEY),
        url: cpp_models_url(is_pri),
        is_stream: false,
        trace_id: Some(new_uuid_v4()),
        is_pri,
        cookie: headers.remove(COOKIE),
    });

    match async { req.send().await?.bytes().await }.await {
        Ok(bytes) => match direct::decode::<AvailableCppModelsResponse>(&bytes) {
            Ok(DecodedMessage::Protobuf(data)) => Ok(Json(data)),
            Ok(DecodedMessage::Text(s)) => Err(__unwrap!(
                Response::builder()
                    .header(CONTENT_TYPE, JSON)
                    .header(CONTENT_LENGTH, s.len())
                    .body(Body::from(s))
            )),
            Err(e) => Err((
                StatusCode::BAD_GATEWAY,
                Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
            )
                .into_response()),
        },
        Err(mut e) => {
            e = e.without_url();

            Err((
                if e.is_timeout() {
                    StatusCode::GATEWAY_TIMEOUT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                },
                Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
            )
                .into_response())
        }
    }
}

const TO_REMOVE_HEADERS: [http::HeaderName; 5] = [
    CONTENT_TYPE,
    CONTENT_LENGTH,
    VARY,
    ACCESS_CONTROL_ALLOW_CREDENTIALS,
    ACCESS_CONTROL_ALLOW_HEADERS,
];

pub async fn handle_upload_file(
    mut headers: HeaderMap,
    mut extensions: Extensions,
    Json(request): Json<FsUploadFileRequest>,
) -> Result<Response, Response> {
    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");
    let gcpp_host = ext_token.get_gcpp_host();

    let req = build_client_request(AiServiceRequest {
        ext_token,
        fs_client_key: headers.remove(CLIENT_KEY),
        url: gcpp_host.get_url(CppService::FSUploadFile, is_pri),
        is_stream: false,
        trace_id: Some(new_uuid_v4()),
        is_pri,
        cookie: headers.remove(COOKIE),
    });

    let body = __unwrap!(encode_message(&request, false));

    let mut e = match async { req.body(body).send().await }.await {
        Ok(res) => {
            let (mut parts, body) = http::Response::from(res).into_parts();
            for key in TO_REMOVE_HEADERS {
                let _ = parts.headers.remove(key);
            }
            match async {
                http_body_util::BodyExt::collect(body)
                    .await
                    .map(|buf| buf.to_bytes())
            }
            .await
            {
                Ok(bytes) => {
                    return match direct::decode::<FsUploadFileResponse>(&bytes) {
                        Ok(DecodedMessage::Protobuf(data)) => Ok(Response::from_parts(
                            parts,
                            Body::from(__unwrap!(serde_json::to_vec(&data))),
                        )),
                        Ok(DecodedMessage::Text(s)) => Err(__unwrap!(
                            Response::builder()
                                .header(CONTENT_TYPE, JSON)
                                .header(CONTENT_LENGTH, s.len())
                                .body(Body::from(s))
                        )),
                        Err(e) => Err((
                            StatusCode::BAD_GATEWAY,
                            Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
                        )
                            .into_response()),
                    };
                }
                Err(e) => e,
            }
        }
        Err(e) => e,
    };
    e = e.without_url();
    let status_code = if e.is_timeout() {
        StatusCode::GATEWAY_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };
    Err((
        status_code,
        Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
    )
        .into_response())
}

pub async fn handle_sync_file(
    mut headers: HeaderMap,
    mut extensions: Extensions,
    Json(request): Json<FsSyncFileRequest>,
) -> Result<Response, Response> {
    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");
    let gcpp_host = ext_token.get_gcpp_host();

    let req = build_client_request(AiServiceRequest {
        ext_token,
        fs_client_key: headers.remove(CLIENT_KEY),
        url: gcpp_host.get_url(CppService::FSSyncFile, is_pri),
        is_stream: false,
        trace_id: Some(new_uuid_v4()),
        is_pri,
        cookie: headers.remove(COOKIE),
    });

    let body = __unwrap!(encode_message(&request, false));

    let mut e = match async { req.body(body).send().await }.await {
        Ok(res) => {
            let (mut parts, body) = http::Response::from(res).into_parts();
            for key in TO_REMOVE_HEADERS {
                let _ = parts.headers.remove(key);
            }
            match async {
                http_body_util::BodyExt::collect(body)
                    .await
                    .map(|buf| buf.to_bytes())
            }
            .await
            {
                Ok(bytes) => {
                    return match direct::decode::<FsSyncFileResponse>(&bytes) {
                        Ok(DecodedMessage::Protobuf(data)) => Ok(Response::from_parts(
                            parts,
                            Body::from(__unwrap!(serde_json::to_vec(&data))),
                        )),
                        Ok(DecodedMessage::Text(s)) => Err(__unwrap!(
                            Response::builder()
                                .header(CONTENT_TYPE, JSON)
                                .header(CONTENT_LENGTH, s.len())
                                .body(Body::from(s))
                        )),
                        Err(e) => Err((
                            StatusCode::BAD_GATEWAY,
                            Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
                        )
                            .into_response()),
                    };
                }
                Err(e) => e,
            }
        }
        Err(e) => e,
    };
    e = e.without_url();
    let status_code = if e.is_timeout() {
        StatusCode::GATEWAY_TIMEOUT
    } else {
        StatusCode::INTERNAL_SERVER_ERROR
    };
    Err((
        status_code,
        Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
    )
        .into_response())
}

pub async fn handle_stream_cpp(
    mut headers: HeaderMap,
    mut extensions: Extensions,
    Json(request): Json<StreamCppRequest>,
) -> Result<Response, (StatusCode, Json<GenericError>)> {
    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");
    let gcpp_host = ext_token.get_gcpp_host();

    let req = build_client_request(AiServiceRequest {
        ext_token,
        fs_client_key: headers.remove(CLIENT_KEY),
        url: gcpp_host.get_url(CppService::StreamCpp, is_pri),
        is_stream: true,
        trace_id: Some(new_uuid_v4()),
        is_pri,
        cookie: headers.remove(COOKIE),
    });

    let body = encode_message(&request, true).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
        )
    })?;

    let res = match async { req.body(body).send().await }.await {
        Ok(r) => r,
        Err(mut e) => {
            e = e.without_url();

            return Err((
                if e.is_timeout() {
                    StatusCode::GATEWAY_TIMEOUT
                } else {
                    StatusCode::INTERNAL_SERVER_ERROR
                },
                Json(ChatError::RequestFailed(Cow::Owned(e.to_string())).to_generic()),
            ));
        }
    };

    // 处理消息并生成响应数据的辅助函数
    fn process_messages<I>(messages: impl IntoIterator<Item = I::Item, IntoIter = I>) -> Vec<u8>
    where
        I: Iterator<Item = StreamMessage>,
    {
        let mut response_data = Vec::with_capacity(64);

        for message in messages {
            let event = match message {
                StreamMessage::ModelInfo { .. } => "model_info",
                StreamMessage::RangeReplace { .. } => "range_replace",
                StreamMessage::CursorPrediction { .. } => "cursor_prediction",
                StreamMessage::Text { .. } => "text",
                StreamMessage::DoneEdit => "done_edit",
                StreamMessage::DoneStream => "done_stream",
                StreamMessage::Debug { .. } => "debug",
                StreamMessage::Error { .. } => ERROR,
                StreamMessage::StreamEnd => "stream_end",
            };
            response_data.extend_from_slice(b"event: ");
            response_data.extend_from_slice(event.as_bytes());
            response_data.extend_from_slice(b"\ndata: ");
            response_data.extend_from_slice(&__unwrap!(serde_json::to_vec(&message)));
            response_data.extend_from_slice(b"\n\n");
        }

        response_data
    }

    // 首先处理stream直到获得第一个结果
    let mut stream = res.bytes_stream();
    let decoder = Arc::new(Mutex::new(StreamDecoder::new()));
    {
        let mut decoder = decoder.lock().await;
        while !decoder.is_first_result_ready() {
            match stream.next().await {
                Some(Ok(chunk)) => {
                    if let Err(StreamError::Upstream(error)) = decoder.decode(&chunk) {
                        let canonical = error.canonical();
                        return Err((canonical.status_code(), Json(canonical.into_generic())));
                    }
                }
                Some(Err(e)) => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            ChatError::RequestFailed(Cow::Owned(format!(
                                "Failed to read response chunk: {e}"
                            )))
                            .to_generic(),
                        ),
                    ));
                }
                None => {
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(
                            ChatError::RequestFailed(Cow::Borrowed(ERR_STREAM_RESPONSE))
                                .to_generic(),
                        ),
                    ));
                }
            }
        }
    }

    let decoder_clone = decoder.clone();

    // 处理后续的stream
    let stream = stream.then(move |chunk| {
        let decoder = decoder_clone.clone();
        async move {
            let chunk = match chunk {
                Ok(c) => c,
                Err(_) => {
                    // crate::debug_println!("Find chunk error: {e:?}");
                    return Ok::<_, Infallible>(Bytes::new());
                }
            };

            // 使用decoder处理chunk
            let messages = match decoder.lock().await.decode(&chunk) {
                Ok(msgs) => msgs,
                Err(e) => {
                    match e {
                        // 处理普通空流错误
                        StreamError::EmptyStream => {
                            eprintln!(
                                "[警告] Stream error: empty stream (连续计数: {})",
                                decoder.lock().await.get_empty_stream_count()
                            );
                            return Ok(Bytes::new());
                        }
                        // 罕见
                        StreamError::Upstream(e) => {
                            __cold_path!();
                            let message =
                                __unwrap!(serde_json::to_string(&e.canonical().into_generic()));
                            let messages = [StreamMessage::Error { message }];
                            return Ok(Bytes::from(process_messages(messages)));
                        }
                        // 处理其他错误
                        _ => {
                            __cold_path!();
                            eprintln!("[警告] Stream error: {e}");
                            return Ok(Bytes::new());
                        }
                    }
                }
            };

            let mut first_response = None;

            if let Some(first_msg) = decoder.lock().await.take_first_result() {
                first_response = Some(process_messages(first_msg));
            }

            let current_response = process_messages(messages);
            let response_data = if let Some(mut first_response) = first_response {
                first_response.extend_from_slice(&current_response);
                first_response
            } else {
                current_response
            };

            Ok(Bytes::from(response_data))
        }
    });

    Ok(__unwrap!(
        Response::builder()
            .header(CACHE_CONTROL, NO_CACHE_REVALIDATE)
            .header(CONNECTION, KEEP_ALIVE)
            .header(CONTENT_TYPE, EVENT_STREAM)
            .header(TRANSFER_ENCODING, CHUNKED)
            .body(Body::from_stream(stream))
    ))
}
