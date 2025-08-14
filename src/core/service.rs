pub mod cpp;

use ::std::{
    borrow::Cow,
    convert::Infallible,
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU8, AtomicU32, AtomicUsize, Ordering},
    },
};

use ::axum::{
    Json,
    body::Body,
    extract::{Query, State},
    response::Response,
};
use ::bytes::Bytes;
use ::futures::StreamExt as _;
use ::http::{
    Extensions, StatusCode,
    header::{CACHE_CONTROL, CONNECTION, CONTENT_LENGTH, CONTENT_TYPE, TRANSFER_ENCODING},
};
use ::tokio::sync::Mutex;

use crate::{
    app::{
        constant::{
            CHATCMPL_PREFIX, CHUNKED, EMPTY_STRING, ERR_RESPONSE_RECEIVED, ERR_STREAM_RESPONSE,
            EVENT_STREAM, INVALID_STREAM, JSON, KEEP_ALIVE, MSG01_PREFIX, NO_CACHE_REVALIDATE,
            OBJECT_CHAT_COMPLETION, OBJECT_CHAT_COMPLETION_CHUNK, UNKNOWN, UPSTREAM_FAILURE,
            get_thinking_tag_close, get_thinking_tag_open,
        },
        lazy::{AUTH_TOKEN, KEY_PREFIX, REAL_USAGE, chat_url},
        model::{
            Alias, AppConfig, AppState, Chain, ChainUsage, DateTime, ErrorInfo, ExtToken,
            LogStatus, LogTokenInfo, Prompt, RequestLog, TimingInfo, TokenKey, UsageCheck,
        },
    },
    common::{
        client::{AiServiceRequest, build_client_request},
        model::{ApiStatus, GenericError, error::ChatError, tri::TriState},
        utils::{
            TrimNewlines as _, format_time_ms, get_available_models, get_token_profile,
            get_token_usage, string_builder, tokeninfo_to_token,
        },
    },
    core::{
        config::{KeyConfig, parse_dynamic_token},
        constant::Models,
        error::StreamError,
        model::{
            ExtModel, MessageId, ModelsResponse, RawModelsResponse, Role,
            anthropic::{self, AnthropicError},
            openai::{self, OpenAiError},
        },
        stream::{
            decoder::{StreamDecoder, StreamMessage},
            droppable::DroppableStream,
        },
    },
};

pub async fn handle_raw_models() -> Result<Json<RawModelsResponse>, (StatusCode, Json<GenericError>)>
{
    if let Some(available_models) = Models::to_raw_arc() {
        Ok(Json(RawModelsResponse(available_models)))
    } else {
        Err((
            StatusCode::NOT_FOUND,
            Json(GenericError {
                status: ApiStatus::Error,
                code: Some(404),
                error: Some(Cow::Borrowed("Models data not available")),
                message: Some(Cow::Borrowed(
                    "Please request /v1/models first to initialize models data",
                )),
            }),
        ))
    }
}

pub async fn handle_models(
    State(state): State<Arc<AppState>>,
    headers: http::HeaderMap,
    Query(request): Query<super::aiserver::v1::AvailableModelsRequest>,
) -> Result<Json<ModelsResponse>, (StatusCode, Json<GenericError>)> {
    // 如果没有认证头，返回默认可用模型
    let auth_token = match super::middleware::auth(&headers) {
        None => return Ok(Json(ModelsResponse)),
        Some(h) => h,
    };

    // 获取token信息
    let (ext_token, is_pri) = {
        // 管理员Token
        if let Some(part) = auth_token.strip_prefix(&**AUTH_TOKEN) {
            let token_manager = state.token_manager.read().await;

            let token_info = if part.is_empty() {
                // 没有后缀，使用默认轮询模式
                let token_infos: Vec<_> = token_manager
                    .tokens()
                    .iter()
                    .flatten()
                    .filter(|t| t.is_enabled())
                    .collect();

                if token_infos.is_empty() {
                    return Err((
                        StatusCode::SERVICE_UNAVAILABLE,
                        Json(ChatError::NoTokens.to_generic()),
                    ));
                }

                static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);

                let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::SeqCst) % token_infos.len();
                token_infos[index]
            } else if let Some(alias) = part.strip_prefix('-') {
                // 使用带别名的模式
                if !token_manager.alias_map().contains_key(alias) {
                    return Err((
                        StatusCode::NOT_FOUND,
                        Json(ChatError::Unauthorized.to_generic()),
                    ));
                }

                if let Some(token_info) = token_manager.get_by_alias(alias) {
                    token_info
                } else {
                    return Err((
                        StatusCode::UNAUTHORIZED,
                        Json(ChatError::Unauthorized.to_generic()),
                    ));
                }
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                ));
            };

            (token_info.bundle.clone_without_user(), true)
        }
        // 共享Token
        else if AppConfig::is_share() && AppConfig::share_token_eq(auth_token) {
            let token_manager = state.token_manager.read().await;
            let token_infos: Vec<_> = token_manager
                .tokens()
                .iter()
                .flatten()
                .filter(|t| t.is_enabled())
                .collect();

            if token_infos.is_empty() {
                return Err((
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ChatError::NoTokens.to_generic()),
                ));
            }

            static CURRENT_KEY_INDEX: AtomicUsize = AtomicUsize::new(0);

            let index = CURRENT_KEY_INDEX.fetch_add(1, Ordering::SeqCst) % token_infos.len();
            let token_info = token_infos[index];
            (token_info.bundle.clone_without_user(), true)
        }
        // 普通用户Token
        else if let Some(key) = TokenKey::from_string(auth_token) {
            let log_manager = state.log_manager_lock().await;
            if let Some(bundle) = log_manager.tokens().get(&key) {
                (bundle.clone_without_user(), false)
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                ));
            }
        }
        // 动态密钥
        else if AppConfig::get_dynamic_key() && auth_token.starts_with(&**KEY_PREFIX) {
            if let Some(ext_token) = parse_dynamic_token(auth_token)
                .and_then(|key_config| key_config.token_info)
                .and_then(tokeninfo_to_token)
            {
                (ext_token, false)
            } else {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    Json(ChatError::Unauthorized.to_generic()),
                ));
            }
        } else {
            return Err((
                StatusCode::UNAUTHORIZED,
                Json(ChatError::Unauthorized.to_generic()),
            ));
        }
    };

    // 获取可用模型列表
    let models = get_available_models(ext_token, is_pri, request)
        .await
        .ok_or((
            UPSTREAM_FAILURE,
            Json(GenericError {
                status: ApiStatus::Error,
                code: Some(UPSTREAM_FAILURE.as_u16()),
                error: Some(Cow::Borrowed("Failed to fetch available models")),
                message: Some(Cow::Borrowed("Unable to get available models")),
            }),
        ))?;

    // 更新模型列表
    Models::update(models).map_err(|e| {
        (
            UPSTREAM_FAILURE,
            Json(GenericError {
                status: ApiStatus::Error,
                code: Some(UPSTREAM_FAILURE.as_u16()),
                error: Some(Cow::Borrowed("Failed to update models")),
                message: Some(Cow::Borrowed(e)),
            }),
        )
    })?;

    Ok(Json(ModelsResponse))
}

// 聊天处理函数的签名
pub async fn handle_chat_completions(
    State(state): State<Arc<AppState>>,
    mut extensions: Extensions,
    Json(request): Json<openai::ChatRequest>,
) -> Result<Response<Body>, (StatusCode, Json<OpenAiError>)> {
    // 验证模型是否支持并获取模型信息
    let model = if let Some(model) = ExtModel::from_str(&request.model) {
        model
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ChatError::ModelNotSupported(request.model).to_openai()),
        ));
    };

    // 验证请求
    if request.messages.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ChatError::EmptyMessages.to_openai()),
        ));
    }

    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");

    let current_config = extensions
        .remove::<KeyConfig>()
        .expect("middleware doesn't have `KeyConfig`");

    let current_id: u64;
    let mut usage_check = None;

    let request_time = DateTime::now();

    // 更新请求日志
    state.increment_total();
    state.increment_active();
    if state.log_manager_lock().await.is_enabled() {
        // let mut need_profile_check = false;

        // {
        //     let log_manager = state.log_manager_lock().await;
        //     for log in log_manager.logs().iter().rev() {
        //         if log_manager
        //             .get_token(&log.token_info.key)
        //             .expect(ERR_LOG_TOKEN_NOT_FOUND)
        //             .primary_token
        //             == ext_token.primary_token
        //             && let (Some(stripe), Some(usage)) =
        //                 (&log.token_info.stripe, &log.token_info.usage)
        //         {
        //             if stripe.membership_type == MembershipType::Free {
        //                 need_profile_check = if FREE_MODELS.contains(&model.id) {
        //                     usage
        //                         .standard
        //                         .max_requests
        //                         .is_some_and(|max| usage.standard.num_requests >= max)
        //                 } else {
        //                     usage
        //                         .premium
        //                         .max_requests
        //                         .is_some_and(|max| usage.premium.num_requests >= max)
        //                 };
        //             }
        //             break;
        //         }
        //     }
        // }

        // // 处理检查结果
        // if need_profile_check {
        //     state.decrement_active();
        //     state.increment_error();
        //     return Err((
        //         StatusCode::UNAUTHORIZED,
        //         Json(ChatError::Unauthorized.to_openai()),
        //     ));
        // }

        let next_id = state.next_log_id().await;
        current_id = next_id;

        state
            .push_log(
                RequestLog {
                    id: next_id,
                    timestamp: request_time,
                    model: model.id,
                    token_info: LogTokenInfo {
                        key: ext_token.primary_token.key(),
                        stripe: None,
                    },
                    chain: None,
                    timing: TimingInfo { total: 0.0 },
                    stream: request.stream,
                    status: LogStatus::Pending,
                    error: ErrorInfo::None,
                },
                ext_token.clone_without_user(),
            )
            .await;

        // 如果需要获取用户使用情况,创建后台任务获取profile
        if model.is_usage_check(
            current_config
                .usage_check_models
                .as_ref()
                .map(UsageCheck::from_proto),
        ) {
            let token = ext_token.primary_token.clone();
            let state = state.clone();
            let log_id = next_id;
            let client = ext_token.get_client();

            usage_check = Some(async move {
                if let Some(include_user) = {
                    state
                        .log_manager_lock()
                        .await
                        .find_log_with_token(log_id)
                        .map(|(_, bundle)| bundle.user.is_none())
                } {
                    let (user, stripe, _) =
                        get_token_profile(client, &token, None, is_pri, include_user, false).await;
                    // 更新日志中的profile
                    if include_user {
                        if let Some((log, bundle)) = state
                            .log_manager_lock()
                            .await
                            .find_log_with_token_mut(log_id)
                        {
                            bundle.user = user.clone();
                            log.token_info.stripe = stripe;
                        };
                    } else {
                        state.log_manager_lock().await.update_log(log_id, |log| {
                            log.token_info.stripe = stripe;
                        });
                    }

                    let mut alias_updater = None;

                    // 更新token manager中的profile
                    if let Some(id) = {
                        state
                            .token_manager_read()
                            .await
                            .id_map()
                            .get(&token.key())
                            .copied()
                    } && let alias_is_unnamed = unsafe {
                        state
                            .token_manager_read()
                            .await
                            .id_to_alias()
                            .get_unchecked(id)
                            .as_ref()
                            .map(Alias::is_unnamed)
                            .unwrap_or(false)
                    } && let Some(Some(token_info)) =
                        state.token_manager_write().await.tokens_mut().get_mut(id)
                    {
                        if include_user {
                            if alias_is_unnamed && let Some(ref user) = user {
                                alias_updater = Some((id, user.email.clone()));
                            }
                            token_info.bundle.user = user;
                        }
                        token_info.stripe = stripe;
                    };

                    if let Some((id, alias)) = alias_updater {
                        let _ = state.token_manager_write().await.set_alias(id, alias);
                    }
                };
            });
        }
    } else {
        current_id = 0;
    }

    // 将消息转换为hex格式
    let msg_id = uuid::Uuid::new_v4();
    let hex_data = match super::adapter::openai::encode_chat_message(
        request.messages,
        ext_token.now(),
        model,
        msg_id,
        current_config.disable_vision(),
        current_config.enable_slow_pool(),
    )
    .await
    {
        Ok(data) => data,
        Err(e) => {
            let e = e.to_string();
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Failure;
                    log.error = ErrorInfo::Error(crate::leak::intern_static(e.as_str()));
                })
                .await;
            state.decrement_active();
            state.increment_error();
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ChatError::ProcessingFailed(Cow::Owned(e)).to_openai()),
            ));
        }
    };
    let msg_id = MessageId::new(msg_id.as_u128());

    // 构建请求客户端
    let req = build_client_request(AiServiceRequest {
        ext_token: ext_token.clone_without_user(),
        fs_client_key: None,
        url: chat_url(is_pri),
        is_stream: true,
        trace_id: Some({
            let mut buf = [0; 36];
            uuid::Uuid::new_v4().as_hyphenated().encode_lower(&mut buf);
            buf
        }),
        is_pri,
        cookie: None,
    });
    // 发送请求
    let response = req.body(hex_data).send().await;

    // 处理请求结果
    let response = match response {
        Ok(resp) => {
            // 更新请求日志为成功
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Success;
                })
                .await;
            resp
        }
        Err(mut e) => {
            e = e.without_url();

            // 根据错误类型返回不同的状态码
            let status_code = if e.is_timeout() {
                StatusCode::GATEWAY_TIMEOUT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let e = e.to_string();

            // 更新请求日志为失败
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Failure;
                    log.error = ErrorInfo::Error(crate::leak::intern_static(e.as_str()));
                })
                .await;
            state.decrement_active();
            state.increment_error();

            return Err((
                status_code,
                Json(ChatError::RequestFailed(Cow::Owned(e)).to_openai()),
            ));
        }
    };

    // 释放活动请求计数
    state.decrement_active();

    let convert_web_ref = current_config.include_web_references();

    if request.stream {
        let response_id = Arc::new({
            let mut buf = [0; 22];
            let mut s = String::with_capacity(31);
            s.push_str(CHATCMPL_PREFIX);
            s.push_str(msg_id.to_str(&mut buf));
            s
        });
        let is_start = Arc::new(AtomicBool::new(true));
        let meet_thinking = Arc::new(AtomicBool::new(false));
        let start_time = std::time::Instant::now();
        let decoder = Arc::new(Mutex::new(StreamDecoder::new()));
        let need_usage = Arc::new(Mutex::new(NeedUsage::new(
            request.stream_options.is_some_and(|opt| opt.include_usage),
            ext_token,
            is_pri,
        )));
        let is_end = Arc::new(AtomicBool::new(false));

        // 定义消息处理器的上下文结构体
        struct MessageProcessContext<'a> {
            response_id: &'a str,
            model: &'static str,
            is_start: &'a AtomicBool,
            meet_thinking: &'a AtomicBool,
            start_time: std::time::Instant,
            state: Arc<AppState>,
            current_id: u64,
            need_usage: &'a Mutex<NeedUsage>,
            created: i64,
            is_end: &'a AtomicBool,
            start: DateTime,
        }

        pub struct NeedUsage {
            // 直接存储 ExtToken，利用指针来判断是否已取走
            token: *mut ExtToken,
            is_need: bool,
            is_pri: bool,
        }

        impl NeedUsage {
            #[inline(always)]
            pub fn new(is_need: bool, token: ExtToken, is_pri: bool) -> Self {
                Self {
                    token: Box::into_raw(Box::new(token)),
                    is_need,
                    is_pri,
                }
            }

            #[inline(always)]
            pub fn is_need(&self) -> bool { self.is_need && !self.token.is_null() }

            #[inline(always)]
            pub fn take(&mut self) -> Option<(bool, Box<ExtToken>, bool)> {
                if self.token.is_null() {
                    None
                } else {
                    let token = unsafe { Box::from_raw(self.token) };
                    let result = (self.is_need, token, self.is_pri);

                    self.token = std::ptr::null_mut();
                    self.is_need = false;

                    Some(result)
                }
            }
        }

        impl Drop for NeedUsage {
            fn drop(&mut self) {
                if !self.token.is_null() {
                    unsafe {
                        drop(Box::from_raw(self.token));
                    }
                }
            }
        }

        unsafe impl Send for NeedUsage where ExtToken: Send {}
        unsafe impl Sync for NeedUsage where ExtToken: Sync {}

        // 处理消息并生成响应数据的辅助函数
        async fn process_messages<I>(
            messages: impl IntoIterator<Item = I::Item, IntoIter = I>,
            ctx: &MessageProcessContext<'_>,
        ) -> Vec<u8>
        where
            I: Iterator<Item = StreamMessage>,
        {
            #[inline]
            pub fn extend_from_slice(vector: &mut Vec<u8>, value: &openai::ChatResponse) {
                vector.extend_from_slice(b"data: ");
                vector.extend_from_slice(&__unwrap!(serde_json::to_vec(value)));
                vector.extend_from_slice(b"\n\n");
            }

            let mut response_data = Vec::with_capacity(128);

            for message in messages {
                match message {
                    StreamMessage::Content(text) => {
                        let is_first = ctx.is_start.load(Ordering::Acquire);
                        let meet_thinking = ctx.meet_thinking.load(Ordering::Acquire);

                        if meet_thinking {
                            ctx.meet_thinking.store(false, Ordering::Release);
                            let response = openai::ChatResponse {
                                id: ctx.response_id,
                                object: OBJECT_CHAT_COMPLETION_CHUNK,
                                created: ctx.created,
                                model: None,
                                choices: Some(openai::Choice {
                                    index: 0,
                                    message: None,
                                    delta: Some(openai::Delta {
                                        role: None,
                                        content: Some(Cow::Borrowed(get_thinking_tag_close())),
                                    }),
                                    finish_reason: false,
                                }),
                                usage: if ctx.need_usage.lock().await.is_need() {
                                    TriState::Null
                                } else {
                                    TriState::Undefined
                                },
                            };
                            extend_from_slice(&mut response_data, &response);
                        }

                        let response = openai::ChatResponse {
                            id: ctx.response_id,
                            object: OBJECT_CHAT_COMPLETION_CHUNK,
                            created: ctx.created,
                            model: if is_first { Some(ctx.model) } else { None },
                            choices: Some(openai::Choice {
                                index: 0,
                                message: None,
                                delta: Some(openai::Delta {
                                    role: if is_first {
                                        Some(Role::Assistant)
                                    } else {
                                        None
                                    },
                                    content: Some(Cow::Owned(if is_first {
                                        ctx.is_start.store(false, Ordering::Release);
                                        text.trim_leading_newlines()
                                    } else {
                                        text
                                    })),
                                }),
                                finish_reason: false,
                            }),
                            usage: if ctx.need_usage.lock().await.is_need() {
                                TriState::Null
                            } else {
                                TriState::Undefined
                            },
                        };

                        extend_from_slice(&mut response_data, &response);
                    }
                    StreamMessage::Thinking(thinking) => {
                        let is_first = ctx.is_start.load(Ordering::Acquire);
                        let meet_thinking = ctx.meet_thinking.load(Ordering::Acquire);

                        if !meet_thinking {
                            ctx.meet_thinking.store(true, Ordering::Release);
                            let response = openai::ChatResponse {
                                id: ctx.response_id,
                                object: OBJECT_CHAT_COMPLETION_CHUNK,
                                created: ctx.created,
                                model: if is_first { Some(ctx.model) } else { None },
                                choices: Some(openai::Choice {
                                    index: 0,
                                    message: None,
                                    delta: Some(openai::Delta {
                                        role: if is_first {
                                            Some(Role::Assistant)
                                        } else {
                                            None
                                        },
                                        content: Some(Cow::Borrowed(get_thinking_tag_open())),
                                    }),
                                    finish_reason: false,
                                }),
                                usage: if ctx.need_usage.lock().await.is_need() {
                                    TriState::Null
                                } else {
                                    TriState::Undefined
                                },
                            };
                            extend_from_slice(&mut response_data, &response);
                        }

                        let response = openai::ChatResponse {
                            id: ctx.response_id,
                            object: OBJECT_CHAT_COMPLETION_CHUNK,
                            created: ctx.created,
                            model: None,
                            choices: Some(openai::Choice {
                                index: 0,
                                message: None,
                                delta: Some(openai::Delta {
                                    role: None,
                                    content: Some(Cow::Owned(if is_first {
                                        ctx.is_start.store(false, Ordering::Release);
                                        thinking.text.trim_leading_newlines()
                                    } else {
                                        thinking.text
                                    })),
                                }),
                                finish_reason: false,
                            }),
                            usage: if ctx.need_usage.lock().await.is_need() {
                                TriState::Null
                            } else {
                                TriState::Undefined
                            },
                        };
                        extend_from_slice(&mut response_data, &response);
                    }
                    StreamMessage::StreamEnd => {
                        // 计算总时间和首次片段时间
                        let total_time = ctx.start_time.elapsed().as_secs_f64();

                        ctx.state
                            .update_log(ctx.current_id, |log| {
                                log.timing.total = format_time_ms(total_time);
                            })
                            .await;

                        let response = openai::ChatResponse {
                            id: ctx.response_id,
                            object: OBJECT_CHAT_COMPLETION_CHUNK,
                            created: ctx.created,
                            model: None,
                            choices: Some(openai::Choice {
                                index: 0,
                                message: None,
                                delta: Some(openai::Delta {
                                    role: None,
                                    content: None,
                                }),
                                finish_reason: true,
                            }),
                            usage: if ctx.need_usage.lock().await.is_need() {
                                TriState::Null
                            } else {
                                TriState::Undefined
                            },
                        };
                        extend_from_slice(&mut response_data, &response);
                        if let Some((is_need, ext_token, is_pri)) =
                            ctx.need_usage.lock().await.take()
                        {
                            let usage = if *REAL_USAGE {
                                let usage = tokio::spawn(get_token_usage(
                                    *ext_token, is_pri, ctx.start, ctx.model,
                                ))
                                .await
                                .unwrap_or_default();
                                if let Some(usage) = usage {
                                    ctx.state
                                        .update_log(ctx.current_id, |log| {
                                            if let Some(chain) = &mut log.chain {
                                                chain.usage = Some(usage);
                                            } else {
                                                log.chain = Some(Chain {
                                                    prompt: Prompt::None,
                                                    delays: None,
                                                    usage: Some(usage),
                                                    think: None,
                                                })
                                            }
                                        })
                                        .await;
                                }
                                usage.map(ChainUsage::to_openai)
                            } else {
                                None
                            };

                            if is_need {
                                let response = openai::ChatResponse {
                                    id: ctx.response_id,
                                    object: OBJECT_CHAT_COMPLETION_CHUNK,
                                    created: ctx.created,
                                    model: None,
                                    choices: None,
                                    usage: TriState::Value(usage.unwrap_or_default()),
                                };
                                extend_from_slice(&mut response_data, &response);
                            }
                        };

                        ctx.is_end.store(true, Ordering::Release);
                    }
                    StreamMessage::Debug(debug_prompt) => {
                        ctx.state
                            .update_log(ctx.current_id, |log| {
                                if log.chain.is_some() {
                                    __cold_path!();
                                    crate::debug!("UB!1 {debug_prompt:?}");
                                    // chain.prompt.push_str(&debug_prompt);
                                } else {
                                    log.chain = Some(Chain {
                                        prompt: Prompt::new(debug_prompt),
                                        delays: None,
                                        usage: None,
                                        think: None,
                                    });
                                }
                            })
                            .await;
                    }
                    _ => {} // 忽略其他消息类型
                }
            }

            response_data
        }

        // 首先处理stream直到获得第一个结果
        let (mut stream, drop_handle) = DroppableStream::new(response.bytes_stream());
        {
            let mut decoder = decoder.lock().await;
            while !decoder.is_first_result_ready() {
                match stream.next().await {
                    Some(Ok(chunk)) => {
                        if let Err(StreamError::Upstream(error)) =
                            decoder.decode(&chunk, convert_web_ref)
                        {
                            let canonical = error.canonical();
                            // 更新请求日志为失败
                            state
                                .update_log(current_id, |log| {
                                    log.status = LogStatus::Failure;
                                    log.error =
                                        ErrorInfo::Error(if let Some(title) = canonical.title() {
                                            crate::leak::intern_static(title)
                                        } else {
                                            UNKNOWN
                                        });
                                    if let Some(detail) = canonical.detail() {
                                        log.error.add_detail(detail)
                                    }
                                    log.timing.total =
                                        format_time_ms(start_time.elapsed().as_secs_f64());
                                })
                                .await;
                            state.increment_error();
                            return Err((canonical.status_code(), Json(canonical.into_openai())));
                        }
                    }
                    Some(Err(e)) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ChatError::RequestFailed(Cow::Owned(format!(
                                    "Failed to read response chunk: {e}"
                                )))
                                .to_openai(),
                            ),
                        ));
                    }
                    None => {
                        // 更新请求日志为失败
                        state
                            .update_log(current_id, |log| {
                                log.status = LogStatus::Failure;
                                log.error = ErrorInfo::Error(ERR_STREAM_RESPONSE);
                            })
                            .await;
                        state.increment_error();
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ChatError::RequestFailed(Cow::Borrowed(ERR_STREAM_RESPONSE))
                                    .to_openai(),
                            ),
                        ));
                    }
                }
            }
        }

        let created = Arc::new(std::sync::OnceLock::new());

        let decoder_clone = decoder.clone();
        let state_clone = state.clone();

        // 处理后续的stream
        let stream = stream
      .then(move |chunk| {
        let decoder = decoder_clone.clone();
        let response_id = response_id.clone();
        let is_start = is_start.clone();
        let meet_thinking = meet_thinking.clone();
        let state = state_clone.clone();
        let need_usage = need_usage.clone();
        let created = created.clone();
        let is_end = is_end.clone();
        let drop_handle = drop_handle.clone();

        async move {
          let chunk = match chunk {
            Ok(c) => c,
            Err(_) => {
              // crate::debug_println!("Find chunk error: {e:?}");
              return Ok::<_, Infallible>(Bytes::new());
            }
          };

          let ctx = MessageProcessContext {
            response_id: &response_id,
            model: model.id,
            is_start: &is_start,
            meet_thinking: &meet_thinking,
            start_time,
            state: state.clone(),
            current_id,
            need_usage: &need_usage,
            created: *created.get_or_init(|| DateTime::utc_now().timestamp()),
            is_end: &is_end,
            start: request_time,
          };

          // 使用decoder处理chunk
          let messages = match decoder.lock().await.decode(&chunk, convert_web_ref) {
            Ok(msgs) => msgs,
            Err(e) => {
              match e {
                // 处理普通空流错误
                StreamError::EmptyStream => {
                  let empty_stream_count = decoder.lock().await.get_empty_stream_count();
                  if empty_stream_count > 1 {
                    eprintln!("[警告] Stream error: empty stream (连续计数: {empty_stream_count})");
                  }
                  return Ok(Bytes::new());
                }
                // 罕见
                StreamError::Upstream(e) => {
                  __cold_path!();
                  let message = __unwrap!(serde_json::to_string(&e.canonical().into_openai()));
                  let messages = [StreamMessage::Content(message), StreamMessage::StreamEnd];
                  return Ok(Bytes::from(process_messages(messages, &ctx).await));
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
            first_response = Some(process_messages(first_msg, &ctx).await);
          }

          let current_response = process_messages(messages, &ctx).await;
          let response_data = if let Some(mut first_response) = first_response {
            first_response.extend_from_slice(&current_response);
            first_response
          } else {
            current_response
          };

          if is_end.load(Ordering::Acquire) {
            drop_handle.drop_stream();
          }

          Ok(Bytes::from(response_data))
        }
      })
      .chain(futures::stream::once(async move {
        // 更新delays
        let mut decoder_guard = decoder.lock().await;
        let content_delays = decoder_guard.take_content_delays();
        let thinking_content = decoder_guard.take_thinking_content();

        state
          .update_log(current_id, move |log| {
            if let Some(chain) = &mut log.chain {
              chain.delays = content_delays;
            } else {
              log.chain = Some(Chain {
                prompt: Prompt::None,
                delays: content_delays,
                usage: None,
                think: thinking_content,
              });
            }
          })
          .await;

        if let Some(usage_check) = usage_check {
          tokio::spawn(usage_check);
        }

        Ok(Bytes::from_static(b"data: [DONE]\n\n"))
      }));

        Ok(__unwrap!(
            Response::builder()
                .header(CACHE_CONTROL, NO_CACHE_REVALIDATE)
                .header(CONNECTION, KEEP_ALIVE)
                .header(CONTENT_TYPE, EVENT_STREAM)
                .header(TRANSFER_ENCODING, CHUNKED)
                .body(Body::from_stream(stream))
        ))
    } else {
        // 非流式响应
        let start_time = std::time::Instant::now();
        let mut decoder = StreamDecoder::new().no_first_cache();
        let mut thinking_text = String::with_capacity(128);
        let mut full_text = String::with_capacity(128);
        let mut stream = response.bytes_stream();
        let mut prompt = Prompt::None;

        // 逐个处理chunks
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ChatError::RequestFailed(Cow::Owned(format!(
                            "Failed to read response chunk: {e}"
                        )))
                        .to_openai(),
                    ),
                )
            })?;

            // 立即处理当前chunk
            match decoder.decode(&chunk, convert_web_ref) {
                Ok(messages) =>
                    for message in messages {
                        match message {
                            StreamMessage::Content(text) => {
                                full_text.push_str(&text);
                            }
                            StreamMessage::Thinking(thinking) => {
                                thinking_text.push_str(&thinking.text);
                            }
                            StreamMessage::Debug(debug_prompt) =>
                                if prompt.is_none() {
                                    prompt = Prompt::new(debug_prompt);
                                } else {
                                    __cold_path!();
                                    crate::debug!("UB!2 {debug_prompt:?}");
                                },
                            _ => {}
                        }
                    },
                Err(StreamError::Upstream(error)) => {
                    let canonical = error.canonical();
                    state
                        .update_log(current_id, |log| {
                            log.status = LogStatus::Failure;
                            log.error = ErrorInfo::Error(if let Some(title) = canonical.title() {
                                crate::leak::intern_static(title)
                            } else {
                                UNKNOWN
                            });
                            if let Some(detail) = canonical.detail() {
                                log.error.add_detail(detail)
                            }
                        })
                        .await;
                    state.increment_error();
                    return Err((canonical.status_code(), Json(canonical.into_openai())));
                }
                Err(StreamError::EmptyStream) => {
                    let empty_stream_count = decoder.get_empty_stream_count();
                    if empty_stream_count > 1 {
                        eprintln!(
                            "[警告] Stream error: empty stream (连续计数: {})",
                            decoder.get_empty_stream_count()
                        );
                    }
                }
                Err(StreamError::DataLengthLessThan5) => {
                    state
                        .update_log(current_id, |log| {
                            log.status = LogStatus::Failure;
                            log.error = ErrorInfo::Error(INVALID_STREAM);
                        })
                        .await;
                    state.increment_error();
                    let error_detail = openai::ErrorDetail {
                        code: Some(Cow::Borrowed(INVALID_STREAM)),
                        message: Cow::Borrowed(EMPTY_STRING),
                    };
                    return Err((UPSTREAM_FAILURE, Json(error_detail.into_openai())));
                }
            }
        }

        full_text = if !thinking_text.is_empty() {
            thinking_text = thinking_text.trim_leading_newlines();
            string_builder::StringBuilder::with_capacity(4)
                .append(get_thinking_tag_open())
                .append(&thinking_text)
                .append(get_thinking_tag_close())
                .append(&full_text)
                .build()
        } else {
            full_text.trim_leading_newlines()
        };

        // 检查响应是否为空
        if full_text.is_empty() {
            // 更新请求日志为失败
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Failure;
                    log.error = ErrorInfo::Error(ERR_RESPONSE_RECEIVED);
                })
                .await;
            state.increment_error();
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ChatError::RequestFailed(Cow::Borrowed(ERR_RESPONSE_RECEIVED)).to_openai()),
            ));
        }

        let (chain_usage, openai_usage) = if *REAL_USAGE {
            let usage = get_token_usage(ext_token, is_pri, request_time, model.id).await;
            let openai = usage.map(ChainUsage::to_openai);
            (usage, openai)
        } else {
            (None, None)
        };

        let response_data = openai::ChatResponse {
            id: &{
                let mut buf = [0; 22];
                let mut s = String::with_capacity(31);
                s.push_str(CHATCMPL_PREFIX);
                s.push_str(msg_id.to_str(&mut buf));
                s
            },
            object: OBJECT_CHAT_COMPLETION,
            created: DateTime::utc_now().timestamp(),
            model: Some(model.id),
            choices: Some(openai::Choice {
                index: 0,
                message: Some(openai::Message {
                    role: Role::Assistant,
                    content: openai::MessageContent::String(full_text),
                }),
                delta: None,
                finish_reason: true,
            }),
            usage: TriState::Value(openai_usage.unwrap_or_default()),
        };

        // 更新请求日志时间信息和状态
        let total_time = format_time_ms(start_time.elapsed().as_secs_f64());
        let content_delays = decoder.take_content_delays();
        let thinking_content = decoder.take_thinking_content();

        state
            .update_log(current_id, |log| {
                log.timing.total = total_time;
                log.status = LogStatus::Success;
                log.chain = Some(Chain {
                    prompt,
                    delays: content_delays,
                    usage: chain_usage,
                    think: thinking_content,
                });
            })
            .await;

        if let Some(usage_check) = usage_check {
            tokio::spawn(usage_check);
        }

        let data = __unwrap!(serde_json::to_vec(&response_data));
        Ok(__unwrap!(
            Response::builder()
                .header(CACHE_CONTROL, NO_CACHE_REVALIDATE)
                .header(CONNECTION, KEEP_ALIVE)
                .header(CONTENT_TYPE, JSON)
                .header(CONTENT_LENGTH, data.len())
                .body(Body::from(data))
        ))
    }
}

pub async fn handle_messages(
    State(state): State<Arc<AppState>>,
    mut extensions: Extensions,
    Json(mut request): Json<anthropic::MessageCreateParams>,
) -> Result<Response<Body>, (StatusCode, Json<AnthropicError>)> {
    // 验证模型是否支持并获取模型信息
    let model = &mut request.model;
    if matches!(
        request.thinking,
        Some(anthropic::ThinkingConfig::Enabled { .. })
    ) {
        model.push_str("-thinking");
    }
    let model = if let Some(model) = ExtModel::from_str(model.as_str()) {
        model
    } else {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ChatError::ModelNotSupported(request.model).to_anthropic()),
        ));
    };
    let params = request;

    // 验证请求
    if params.messages.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ChatError::EmptyMessages.to_anthropic()),
        ));
    }

    let (ext_token, is_pri) = extensions
        .remove::<(ExtToken, bool)>()
        .expect("middleware doesn't have `(ExtToken, bool)`");

    let current_config = extensions
        .remove::<KeyConfig>()
        .expect("middleware doesn't have `KeyConfig`");

    let current_id: u64;
    let mut usage_check = None;

    let request_time = DateTime::now();

    // 更新请求日志
    state.increment_total();
    state.increment_active();
    if state.log_manager_lock().await.is_enabled() {
        // let mut need_profile_check = false;

        // {
        //     let log_manager = state.log_manager_lock().await;
        //     for log in log_manager.logs().iter().rev() {
        //         if log_manager
        //             .get_token(&log.token_info.key)
        //             .expect(ERR_LOG_TOKEN_NOT_FOUND)
        //             .primary_token
        //             == ext_token.primary_token
        //             && let (Some(stripe), Some(usage)) =
        //                 (&log.token_info.stripe, &log.token_info.usage)
        //         {
        //             if stripe.membership_type == MembershipType::Free {
        //                 need_profile_check = if FREE_MODELS.contains(&model.id) {
        //                     usage
        //                         .standard
        //                         .max_requests
        //                         .is_some_and(|max| usage.standard.num_requests >= max)
        //                 } else {
        //                     usage
        //                         .premium
        //                         .max_requests
        //                         .is_some_and(|max| usage.premium.num_requests >= max)
        //                 };
        //             }
        //             break;
        //         }
        //     }
        // }

        // // 处理检查结果
        // if need_profile_check {
        //     state.decrement_active();
        //     state.increment_error();
        //     return Err((
        //         StatusCode::UNAUTHORIZED,
        //         Json(ChatError::Unauthorized.to_generic()),
        //     ));
        // }

        let next_id = state.next_log_id().await;
        current_id = next_id;

        state
            .push_log(
                RequestLog {
                    id: next_id,
                    timestamp: request_time,
                    model: model.id,
                    token_info: LogTokenInfo {
                        key: ext_token.primary_token.key(),
                        stripe: None,
                    },
                    chain: None,
                    timing: TimingInfo { total: 0.0 },
                    stream: params.stream,
                    status: LogStatus::Pending,
                    error: ErrorInfo::None,
                },
                ext_token.clone_without_user(),
            )
            .await;

        // 如果需要获取用户使用情况,创建后台任务获取profile
        if model.is_usage_check(
            current_config
                .usage_check_models
                .as_ref()
                .map(UsageCheck::from_proto),
        ) {
            let token = ext_token.primary_token.clone();
            let state = state.clone();
            let log_id = next_id;
            let client = ext_token.get_client();

            usage_check = Some(async move {
                if let Some(include_user) = {
                    state
                        .log_manager_lock()
                        .await
                        .find_log_with_token(log_id)
                        .map(|(_, bundle)| bundle.user.is_none())
                } {
                    let (user, stripe, _) =
                        get_token_profile(client, &token, None, is_pri, include_user, false).await;
                    // 更新日志中的profile
                    if include_user {
                        if let Some((log, bundle)) = state
                            .log_manager_lock()
                            .await
                            .find_log_with_token_mut(log_id)
                        {
                            bundle.user = user.clone();
                            log.token_info.stripe = stripe;
                        };
                    } else {
                        state.log_manager_lock().await.update_log(log_id, |log| {
                            log.token_info.stripe = stripe;
                        });
                    }

                    let mut alias_updater = None;

                    // 更新token manager中的profile
                    if let Some(id) = {
                        state
                            .token_manager_read()
                            .await
                            .id_map()
                            .get(&token.key())
                            .copied()
                    } && let alias_is_unnamed = unsafe {
                        state
                            .token_manager_read()
                            .await
                            .id_to_alias()
                            .get_unchecked(id)
                            .as_ref()
                            .map(Alias::is_unnamed)
                            .unwrap_or(false)
                    } && let Some(Some(token_info)) =
                        state.token_manager_write().await.tokens_mut().get_mut(id)
                    {
                        if include_user {
                            if alias_is_unnamed && let Some(ref user) = user {
                                alias_updater = Some((id, user.email.clone()));
                            }
                            token_info.bundle.user = user;
                        }
                        token_info.stripe = stripe;
                    };

                    if let Some((id, alias)) = alias_updater {
                        let _ = state.token_manager_write().await.set_alias(id, alias);
                    }
                };
            });
        }
    } else {
        current_id = 0;
    }

    // 将消息转换为hex格式
    let stream = params.stream;
    let msg_id = uuid::Uuid::new_v4();
    let hex_data = match super::adapter::anthropic::encode_message_params(
        params,
        ext_token.now(),
        model,
        msg_id,
        current_config.disable_vision(),
        current_config.enable_slow_pool(),
    )
    .await
    {
        Ok(data) => data,
        Err(e) => {
            let e = e.to_string();
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Failure;
                    log.error = ErrorInfo::Error(crate::leak::intern_static(e.as_str()));
                })
                .await;
            state.decrement_active();
            state.increment_error();
            return Err((
                StatusCode::BAD_REQUEST,
                Json(ChatError::ProcessingFailed(Cow::Owned(e)).to_anthropic()),
            ));
        }
    };
    let msg_id = MessageId::new(msg_id.as_u128());

    // 构建请求客户端
    let req = build_client_request(AiServiceRequest {
        ext_token: ext_token.clone_without_user(),
        fs_client_key: None,
        url: chat_url(is_pri),
        is_stream: true,
        trace_id: Some({
            let mut buf = [0; 36];
            uuid::Uuid::new_v4().as_hyphenated().encode_lower(&mut buf);
            buf
        }),
        is_pri,
        cookie: None,
    });
    // 发送请求
    let response = req.body(hex_data).send().await;

    // 处理请求结果
    let response = match response {
        Ok(resp) => {
            // 更新请求日志为成功
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Success;
                })
                .await;
            resp
        }
        Err(mut e) => {
            e = e.without_url();

            // 根据错误类型返回不同的状态码
            let status_code = if e.is_timeout() {
                StatusCode::GATEWAY_TIMEOUT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            };
            let e = e.to_string();

            // 更新请求日志为失败
            state
                .update_log(current_id, |log| {
                    log.status = LogStatus::Failure;
                    log.error = ErrorInfo::Error(crate::leak::intern_static(e.as_str()));
                })
                .await;
            state.decrement_active();
            state.increment_error();

            return Err((
                status_code,
                Json(ChatError::RequestFailed(Cow::Owned(e)).to_anthropic()),
            ));
        }
    };

    // 释放活动请求计数
    state.decrement_active();

    let convert_web_ref = current_config.include_web_references();

    if stream {
        let msg_id = Arc::new({
            let mut buf = [0; 22];
            let mut s = String::with_capacity(28);
            s.push_str(MSG01_PREFIX);
            s.push_str(msg_id.to_str(&mut buf));
            s
        });
        let index = Arc::new(AtomicU32::new(0));
        let start_time = std::time::Instant::now();
        let decoder = Arc::new(Mutex::new(StreamDecoder::new()));
        let stream_state = Arc::new(AtomicU8::new(0));
        let last_content_type = Arc::new(AtomicU8::new(0)); // 新增：记录上次内容类型
        let need_usage = Arc::new(Mutex::new(NeedUsage::new(ext_token, is_pri)));

        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq)]
        enum StreamState {
            /// 初始状态，什么都未开始
            NotStarted = 0,
            /// message_start 已完成，等待 content_block_start
            MessageStarted = 1,
            /// content_block_start 已完成，正在接收 content_block_delta
            ContentBlockActive = 2,
            /// content_block_stop 已完成，等待下一个 content_block_start 或 message_delta
            BetweenBlocks = 3,
            // /// message_delta 已完成，等待 message_stop
            // MessageEnding = 4,
            /// message_stop 已完成，流结束
            Completed = 5,
        }

        #[repr(u8)]
        #[derive(Clone, Copy, PartialEq)]
        enum LastContentType {
            None = 0,
            Text = 1,
            Thinking = 2,
        }

        // 定义消息处理器的上下文结构体
        struct MessageProcessContext<'a> {
            msg_id: &'a str,
            model: &'static str,
            index: &'a AtomicU32,
            start_time: std::time::Instant,
            app_state: Arc<AppState>,
            stream_state: &'a AtomicU8,
            last_content_type: &'a AtomicU8,
            current_id: u64,
            need_usage: &'a Mutex<NeedUsage>,
            start: DateTime,
        }

        pub struct NeedUsage {
            // 直接存储 ExtToken，利用指针来判断是否已取走
            token: *mut ExtToken,
            is_pri: bool,
        }

        impl NeedUsage {
            #[inline(always)]
            pub fn new(token: ExtToken, is_pri: bool) -> Self {
                Self {
                    token: Box::into_raw(Box::new(token)),
                    is_pri,
                }
            }

            #[inline(always)]
            pub fn take(&mut self) -> Option<(Box<ExtToken>, bool)> {
                if self.token.is_null() {
                    None
                } else {
                    let token = unsafe { Box::from_raw(self.token) };
                    let result = (token, self.is_pri);

                    self.token = std::ptr::null_mut();

                    Some(result)
                }
            }
        }

        impl Drop for NeedUsage {
            fn drop(&mut self) {
                if !self.token.is_null() {
                    unsafe {
                        drop(Box::from_raw(self.token));
                    }
                }
            }
        }

        unsafe impl Send for NeedUsage where ExtToken: Send {}
        unsafe impl Sync for NeedUsage where ExtToken: Sync {}

        #[inline]
        pub fn extend_from_slice(vector: &mut Vec<u8>, value: &anthropic::RawMessageStreamEvent) {
            vector.extend_from_slice(b"event: ");
            vector.extend_from_slice(value.type_name().as_bytes());
            vector.extend_from_slice(b"\ndata: ");
            vector.extend_from_slice(&__unwrap!(serde_json::to_vec(value)));
            vector.extend_from_slice(b"\n\n");
        }

        // 处理消息并生成响应数据的辅助函数
        async fn process_messages(
            messages: Vec<StreamMessage>,
            ctx: &MessageProcessContext<'_>,
        ) -> Vec<u8> {
            let mut response_data = Vec::with_capacity(256);

            for message in messages {
                match message {
                    StreamMessage::Content(text) => {
                        // 检查是否需要开始消息
                        let current_state = ctx.stream_state.load(Ordering::Acquire);
                        if current_state == StreamState::NotStarted as u8 {
                            let event = anthropic::RawMessageStreamEvent::MessageStart {
                                message: anthropic::Message {
                                    content: vec![],
                                    usage: anthropic::Usage::default(),
                                    id: ctx.msg_id,
                                    model: ctx.model,
                                },
                            };
                            extend_from_slice(&mut response_data, &event);
                            ctx.stream_state
                                .store(StreamState::MessageStarted as u8, Ordering::Release);
                        }

                        // 检查是否需要切换或开始内容块
                        let last_type = ctx.last_content_type.load(Ordering::Acquire);
                        let current_state = ctx.stream_state.load(Ordering::Acquire);

                        if last_type != LastContentType::Text as u8 {
                            // 如果上次不是文本类型，需要结束上个块(如果有的话)
                            if last_type != LastContentType::None as u8 {
                                let event = anthropic::RawMessageStreamEvent::ContentBlockStop {
                                    index: ctx.index.load(Ordering::Acquire),
                                };
                                extend_from_slice(&mut response_data, &event);
                                ctx.index.fetch_add(1, Ordering::AcqRel);
                                ctx.stream_state
                                    .store(StreamState::BetweenBlocks as u8, Ordering::Release);
                            }

                            // 开始新的文本块
                            let event = anthropic::RawMessageStreamEvent::ContentBlockStart {
                                index: ctx.index.load(Ordering::Acquire),
                                content_block: anthropic::ContentBlock::Text {
                                    text: String::new(),
                                },
                            };
                            extend_from_slice(&mut response_data, &event);

                            // 如果是刚开始，发送ping事件
                            if current_state == StreamState::MessageStarted as u8 {
                                let event = anthropic::RawMessageStreamEvent::Ping;
                                extend_from_slice(&mut response_data, &event);
                            }

                            ctx.last_content_type
                                .store(LastContentType::Text as u8, Ordering::Release);
                            ctx.stream_state
                                .store(StreamState::ContentBlockActive as u8, Ordering::Release);
                        }

                        let event = anthropic::RawMessageStreamEvent::ContentBlockDelta {
                            index: ctx.index.load(Ordering::Acquire),
                            delta: anthropic::RawContentBlockDelta::TextDelta { text },
                        };
                        extend_from_slice(&mut response_data, &event);
                    }
                    StreamMessage::Thinking(thinking) => {
                        // 检查是否需要开始消息
                        let current_state = ctx.stream_state.load(Ordering::Acquire);
                        if current_state == StreamState::NotStarted as u8 {
                            let event = anthropic::RawMessageStreamEvent::MessageStart {
                                message: anthropic::Message {
                                    content: vec![],
                                    usage: anthropic::Usage::default(),
                                    id: ctx.msg_id,
                                    model: ctx.model,
                                },
                            };
                            extend_from_slice(&mut response_data, &event);
                            ctx.stream_state
                                .store(StreamState::MessageStarted as u8, Ordering::Release);
                        }

                        // 检查是否需要切换或开始内容块
                        let last_type = ctx.last_content_type.load(Ordering::Acquire);
                        let current_state = ctx.stream_state.load(Ordering::Acquire);

                        if last_type != LastContentType::Thinking as u8 {
                            // 如果上次不是思考类型，需要结束上个块(如果有的话)
                            if last_type != LastContentType::None as u8 {
                                let event = anthropic::RawMessageStreamEvent::ContentBlockStop {
                                    index: ctx.index.load(Ordering::Acquire),
                                };
                                extend_from_slice(&mut response_data, &event);
                                ctx.index.fetch_add(1, Ordering::AcqRel);
                                ctx.stream_state
                                    .store(StreamState::BetweenBlocks as u8, Ordering::Release);
                            }

                            // 开始新的思考块
                            let event = anthropic::RawMessageStreamEvent::ContentBlockStart {
                                index: ctx.index.load(Ordering::Acquire),
                                content_block: anthropic::ContentBlock::Thinking {
                                    thinking: String::new(),
                                    signature: String::new(),
                                },
                            };
                            extend_from_slice(&mut response_data, &event);

                            // 如果是刚开始，发送ping事件
                            if current_state == StreamState::MessageStarted as u8 {
                                let event = anthropic::RawMessageStreamEvent::Ping;
                                extend_from_slice(&mut response_data, &event);
                            }

                            ctx.last_content_type
                                .store(LastContentType::Thinking as u8, Ordering::Release);
                            ctx.stream_state
                                .store(StreamState::ContentBlockActive as u8, Ordering::Release);
                        }

                        if !thinking.text.is_empty() {
                            let event = anthropic::RawMessageStreamEvent::ContentBlockDelta {
                                index: ctx.index.load(Ordering::Acquire),
                                delta: anthropic::RawContentBlockDelta::ThinkingDelta {
                                    thinking: thinking.text,
                                },
                            };
                            extend_from_slice(&mut response_data, &event);
                        }
                        if !thinking.signature.is_empty() {
                            let event = anthropic::RawMessageStreamEvent::ContentBlockDelta {
                                index: ctx.index.load(Ordering::Acquire),
                                delta: anthropic::RawContentBlockDelta::SignatureDelta {
                                    signature: thinking.signature,
                                },
                            };
                            extend_from_slice(&mut response_data, &event);
                        }
                    }
                    StreamMessage::StreamEnd => {
                        // 计算总时间和首次片段时间
                        let total_time = ctx.start_time.elapsed().as_secs_f64();

                        ctx.app_state
                            .update_log(ctx.current_id, |log| {
                                log.timing.total = format_time_ms(total_time);
                            })
                            .await;

                        // 结束当前内容块(如果有的话)
                        let last_type = ctx.last_content_type.load(Ordering::Acquire);
                        if last_type != LastContentType::None as u8 {
                            let event = anthropic::RawMessageStreamEvent::ContentBlockStop {
                                index: ctx.index.load(Ordering::Acquire),
                            };
                            extend_from_slice(&mut response_data, &event);
                            ctx.stream_state
                                .store(StreamState::BetweenBlocks as u8, Ordering::Release);
                        }

                        // 处理使用量统计
                        if let Some((ext_token, is_pri)) = ctx.need_usage.lock().await.take() {
                            let usage = if *REAL_USAGE {
                                let usage = tokio::spawn(get_token_usage(
                                    *ext_token, is_pri, ctx.start, ctx.model,
                                ))
                                .await
                                .unwrap_or_default();
                                if let Some(usage) = usage {
                                    ctx.app_state
                                        .update_log(ctx.current_id, |log| {
                                            if let Some(chain) = &mut log.chain {
                                                chain.usage = Some(usage);
                                            } else {
                                                log.chain = Some(Chain {
                                                    prompt: Prompt::None,
                                                    delays: None,
                                                    usage: Some(usage),
                                                    think: None,
                                                })
                                            }
                                        })
                                        .await;
                                }
                                usage.map(ChainUsage::to_anthropic_delta)
                            } else {
                                None
                            };

                            let event = anthropic::RawMessageStreamEvent::MessageDelta {
                                delta: anthropic::MessageDelta,
                                usage: usage.unwrap_or_default(),
                            };
                            extend_from_slice(&mut response_data, &event);
                        };

                        ctx.stream_state
                            .store(StreamState::Completed as u8, Ordering::Release);
                    }
                    StreamMessage::Debug(debug_prompt) => {
                        ctx.app_state
                            .update_log(ctx.current_id, |log| {
                                if log.chain.is_some() {
                                    __cold_path!();
                                    crate::debug!("UB!1 {debug_prompt:?}");
                                    // chain.prompt.push_str(&debug_prompt);
                                } else {
                                    log.chain = Some(Chain {
                                        prompt: Prompt::new(debug_prompt),
                                        delays: None,
                                        usage: None,
                                        think: None,
                                    });
                                }
                            })
                            .await;
                    }
                    _ => {} // 忽略其他消息类型
                }
            }

            response_data
        }

        // 首先处理stream直到获得第一个结果
        let (mut stream, drop_handle) = DroppableStream::new(response.bytes_stream());
        {
            let mut decoder = decoder.lock().await;
            while !decoder.is_first_result_ready() {
                match stream.next().await {
                    Some(Ok(chunk)) => {
                        if let Err(StreamError::Upstream(error)) =
                            decoder.decode(&chunk, convert_web_ref)
                        {
                            let canonical = error.canonical();
                            // 更新请求日志为失败
                            state
                                .update_log(current_id, |log| {
                                    log.status = LogStatus::Failure;
                                    log.error =
                                        ErrorInfo::Error(if let Some(title) = canonical.title() {
                                            crate::leak::intern_static(title)
                                        } else {
                                            UNKNOWN
                                        });
                                    if let Some(detail) = canonical.detail() {
                                        log.error.add_detail(detail)
                                    }
                                    log.timing.total =
                                        format_time_ms(start_time.elapsed().as_secs_f64());
                                })
                                .await;
                            state.increment_error();
                            return Err((
                                canonical.status_code(),
                                Json(canonical.into_anthropic()),
                            ));
                        }
                    }
                    Some(Err(e)) => {
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ChatError::RequestFailed(Cow::Owned(format!(
                                    "Failed to read response chunk: {e}"
                                )))
                                .to_anthropic(),
                            ),
                        ));
                    }
                    None => {
                        // 更新请求日志为失败
                        state
                            .update_log(current_id, |log| {
                                log.status = LogStatus::Failure;
                                log.error = ErrorInfo::Error(ERR_STREAM_RESPONSE);
                            })
                            .await;
                        state.increment_error();
                        return Err((
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(
                                ChatError::RequestFailed(Cow::Borrowed(ERR_STREAM_RESPONSE))
                                    .to_anthropic(),
                            ),
                        ));
                    }
                }
            }
        }

        let decoder_clone = decoder.clone();
        let state_clone = state.clone();

        // 处理后续的stream
        let stream = stream
      .then(move |chunk| {
        let decoder = decoder_clone.clone();
        let msg_id = msg_id.clone();
        let index = index.clone();
        let app_state = state_clone.clone();
        let stream_state = stream_state.clone();
        let last_content_type = last_content_type.clone();
        let need_usage = need_usage.clone();
        let drop_handle = drop_handle.clone();

        async move {
          let chunk = match chunk {
            Ok(c) => c,
            Err(_) => {
              // crate::debug_println!("Find chunk error: {e:?}");
              return Ok::<_, Infallible>(Bytes::new());
            }
          };

          let ctx = MessageProcessContext {
            msg_id: &msg_id,
            model: model.id,
            index: &index,
            start_time,
            app_state: app_state.clone(),
            stream_state: &stream_state,
            last_content_type: &last_content_type,
            current_id,
            need_usage: &need_usage,
            start: request_time,
          };

          // 使用decoder处理chunk
          let messages = match decoder.lock().await.decode(&chunk, convert_web_ref) {
            Ok(msgs) => msgs,
            Err(e) => {
              match e {
                // 处理普通空流错误
                StreamError::EmptyStream => {
                  let empty_stream_count = decoder.lock().await.get_empty_stream_count();
                  if empty_stream_count > 1 {
                    eprintln!("[警告] Stream error: empty stream (连续计数: {empty_stream_count})");
                  }
                  return Ok(Bytes::new());
                }
                // 罕见
                StreamError::Upstream(e) => {
                  __cold_path!();
                  let canonical = e.canonical();
                  let mut buf = Vec::with_capacity(128);
                  extend_from_slice(&mut buf, &anthropic::RawMessageStreamEvent::Error {
                    error: {
                      unsafe { ::core::intrinsics::transmute_unchecked(canonical.into_anthropic()) }
                    },
                  });
                  return Ok(Bytes::from(buf));
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
            first_response = Some(process_messages(first_msg, &ctx).await);
          }

          let current_response = process_messages(messages, &ctx).await;
          let response_data = if let Some(mut first_response) = first_response {
            first_response.extend_from_slice(&current_response);
            first_response
          } else {
            current_response
          };

          // 检查是否已完成
          if ctx.stream_state.load(Ordering::Acquire) == StreamState::Completed as u8 {
            drop_handle.drop_stream();
          }

          Ok(Bytes::from(response_data))
        }
      })
      .chain(futures::stream::once(async move {
        // 更新delays
        let mut decoder_guard = decoder.lock().await;
        let content_delays = decoder_guard.take_content_delays();
        let thinking_content = decoder_guard.take_thinking_content();

        state
          .update_log(current_id, move |log| {
            if let Some(chain) = &mut log.chain {
              chain.delays = content_delays;
            } else {
              log.chain = Some(Chain {
                prompt: Prompt::None,
                delays: content_delays,
                usage: None,
                think: thinking_content,
              });
            }
          })
          .await;

        if let Some(usage_check) = usage_check {
          tokio::spawn(usage_check);
        }

        Ok(Bytes::from_static(
          b"event: message_stop\ndata: {\"type\":\"message_stop\"}",
        ))
      }));

        Ok(__unwrap!(
            Response::builder()
                .header(CACHE_CONTROL, NO_CACHE_REVALIDATE)
                .header(CONNECTION, KEEP_ALIVE)
                .header(CONTENT_TYPE, EVENT_STREAM)
                .header(TRANSFER_ENCODING, CHUNKED)
                .body(Body::from_stream(stream))
        ))
    } else {
        // 非流式响应
        let start_time = std::time::Instant::now();
        let mut decoder = StreamDecoder::new().no_first_cache();
        let mut content = Vec::with_capacity(16);
        let mut stream = response.bytes_stream();
        let mut prompt = Prompt::None;

        // 逐个处理chunks
        while let Some(chunk) = stream.next().await {
            let chunk = chunk.map_err(|e| {
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(
                        ChatError::RequestFailed(Cow::Owned(format!(
                            "Failed to read response chunk: {e}"
                        )))
                        .to_anthropic(),
                    ),
                )
            })?;

            // 立即处理当前chunk
            match decoder.decode(&chunk, convert_web_ref) {
                Ok(messages) =>
                    for message in messages {
                        match message {
                            StreamMessage::Content(atext) => {
                                if let Some(anthropic::ContentBlock::Text { text }) =
                                    content.last_mut()
                                {
                                    text.reserve_exact(atext.len() * 2);
                                    text.push_str(&atext);
                                } else {
                                    let mut text = atext;
                                    text.reserve_exact(text.len());
                                    content.push(anthropic::ContentBlock::Text { text });
                                }
                            }
                            StreamMessage::Thinking(athinking) => {
                                if !athinking.signature.is_empty() {
                                    if let Some(anthropic::ContentBlock::Thinking {
                                        signature,
                                        ..
                                    }) = content.last_mut()
                                    {
                                        *signature = athinking.signature;
                                    } else {
                                        crate::debug!("UB!3 {athinking:?}");
                                        let mut signature = athinking.signature;
                                        signature.reserve_exact(signature.len());
                                        content.push(anthropic::ContentBlock::Thinking {
                                            thinking: String::new(),
                                            signature,
                                        });
                                    }
                                }

                                if !athinking.text.is_empty() {
                                    if let Some(anthropic::ContentBlock::Thinking {
                                        thinking,
                                        ..
                                    }) = content.last_mut()
                                    {
                                        thinking.reserve_exact(athinking.text.len() * 2);
                                        thinking.push_str(&athinking.text);
                                    } else {
                                        let mut thinking = athinking.text;
                                        thinking.reserve_exact(thinking.len());
                                        content.push(anthropic::ContentBlock::Thinking {
                                            thinking,
                                            signature: String::new(),
                                        });
                                    }
                                }

                                if !athinking.redacted_thinking.is_empty() {
                                    content.push(anthropic::ContentBlock::RedactedThinking {
                                        data: athinking.redacted_thinking,
                                    });
                                }
                            }
                            StreamMessage::Debug(debug_prompt) =>
                                if prompt.is_none() {
                                    prompt = Prompt::new(debug_prompt);
                                } else {
                                    __cold_path!();
                                    crate::debug!("UB!2 {debug_prompt:?}");
                                },
                            _ => {}
                        }
                    },
                Err(StreamError::Upstream(error)) => {
                    let canonical = error.canonical();
                    state
                        .update_log(current_id, |log| {
                            log.status = LogStatus::Failure;
                            log.error = ErrorInfo::Error(if let Some(title) = canonical.title() {
                                crate::leak::intern_static(title)
                            } else {
                                UNKNOWN
                            });
                            if let Some(detail) = canonical.detail() {
                                log.error.add_detail(detail)
                            }
                        })
                        .await;
                    state.increment_error();
                    return Err((canonical.status_code(), Json(canonical.into_anthropic())));
                }
                Err(StreamError::EmptyStream) => {
                    let empty_stream_count = decoder.get_empty_stream_count();
                    if empty_stream_count > 1 {
                        eprintln!(
                            "[警告] Stream error: empty stream (连续计数: {})",
                            decoder.get_empty_stream_count()
                        );
                    }
                }
                Err(StreamError::DataLengthLessThan5) => {
                    state
                        .update_log(current_id, |log| {
                            log.status = LogStatus::Failure;
                            log.error = ErrorInfo::Error(INVALID_STREAM);
                        })
                        .await;
                    state.increment_error();
                    let error_detail = anthropic::ErrorDetail {
                        r#type: INVALID_STREAM,
                        message: Cow::Borrowed(EMPTY_STRING),
                    };
                    return Err((UPSTREAM_FAILURE, Json(error_detail.into_anthropic())));
                }
            }
        }

        let (chain_usage, anthropic_usage) = if *REAL_USAGE {
            let usage = get_token_usage(ext_token, is_pri, request_time, model.id).await;
            let anthropic = usage.map(ChainUsage::to_anthropic);
            (usage, anthropic)
        } else {
            (None, None)
        };

        let response_data = anthropic::Message {
            content,
            usage: anthropic_usage.unwrap_or_default(),
            id: &{
                let mut buf = [0; 22];
                let mut s = String::with_capacity(28);
                s.push_str(MSG01_PREFIX);
                s.push_str(msg_id.to_str(&mut buf));
                s
            },
            model: model.id,
        };

        // 更新请求日志时间信息和状态
        let total_time = format_time_ms(start_time.elapsed().as_secs_f64());
        let content_delays = decoder.take_content_delays();
        let thinking_content = decoder.take_thinking_content();

        state
            .update_log(current_id, |log| {
                log.timing.total = total_time;
                log.status = LogStatus::Success;
                log.chain = Some(Chain {
                    prompt,
                    delays: content_delays,
                    usage: chain_usage,
                    think: thinking_content,
                });
            })
            .await;

        if let Some(usage_check) = usage_check {
            tokio::spawn(usage_check);
        }

        let data = __unwrap!(serde_json::to_vec(&response_data));
        Ok(__unwrap!(
            Response::builder()
                .header(CACHE_CONTROL, NO_CACHE_REVALIDATE)
                .header(CONNECTION, KEEP_ALIVE)
                .header(CONTENT_TYPE, JSON)
                .header(CONTENT_LENGTH, data.len())
                .body(Body::from(data))
        ))
    }
}
