use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};
use image::guess_format;
use rand::Rng as _;
use reqwest::Client;
use uuid::Uuid;

use crate::{
  app::{
    constant::EMPTY_STRING,
    lazy::get_default_instructions,
    model::{AppConfig, VisionAbility, proxy_pool::get_general_client},
  },
  common::utils::encode_message,
  core::{
    aiserver::v1::{
      AzureState, ClientSideToolV2, ComposerCapabilityRequest, ComposerExternalLink,
      ConversationMessage, ConversationMessageHeader, CursorPosition, CursorRange, EnvironmentInfo,
      ExplicitContext, ImageProto, ModelDetails, StreamUnifiedChatRequest,
      StreamUnifiedChatRequestWithTools, composer_capability_request, conversation_message,
      image_proto, mcp_params, stream_unified_chat_request,
    },
    constant::{ERR_UNSUPPORTED_GIF, ERR_UNSUPPORTED_IMAGE_FORMAT, LONG_CONTEXT_MODELS},
    model::{
      ExtModel, Role,
      anthropic::{
        ContentBlockParam, ImageSource, MediaType, MessageContent, MessageCreateParams,
        MessageParam, SystemContent, Tool,
      },
    },
  },
};

use super::{
  AGENT_MODE_NAME, ASK_MODE_NAME, ERR_BASE64_ONLY, ERR_VISION_DISABLED, NEWLINE, ToOpt as _,
  WEB_SEARCH_MODE, extract_external_links, extract_web_references_info, sanitize_tool_name,
};

async fn process_message_params(
  messages: Vec<MessageParam>,
  system: Option<SystemContent>,
  tools: Vec<Tool>,
  supported_tools: Vec<i32>,
  now_with_tz: chrono::DateTime<chrono_tz::Tz>,
  image_support: bool,
  is_agentic: bool,
) -> Result<
  (
    String,
    Vec<ConversationMessage>,
    Vec<ConversationMessageHeader>,
    Vec<ComposerExternalLink>,
  ),
  Box<dyn std::error::Error + Send + Sync>,
> {
  // 收集 system 指令
  let instructions = system.map(|content| match content {
    SystemContent::String(text) => text,
    SystemContent::Array(contents) => contents
      .into_iter()
      .map(|c| c.text)
      .collect::<Vec<String>>()
      .join(NEWLINE),
  });

  // 使用默认指令或收集到的指令
  let instructions = if let Some(instructions) = instructions {
    instructions
  } else {
    get_default_instructions(now_with_tz)
  };

  let mut inputs = messages;

  // 处理空对话情况
  if inputs.is_empty() {
    let bubble_id = Uuid::new_v4().to_string();
    return Ok((
      instructions,
      vec![ConversationMessage {
        r#type: conversation_message::MessageType::Human as i32,
        bubble_id: bubble_id.clone(),
        unified_mode: Some(stream_unified_chat_request::UnifiedMode::Chat as i32),
        is_simple_looping_message: Some(false),
        ..Default::default()
      }],
      vec![ConversationMessageHeader {
        bubble_id,
        server_bubble_id: None,
        r#type: conversation_message::MessageType::Human as i32,
      }],
      vec![],
    ));
  }

  // 如果第一条是 assistant，插入空的 user 消息
  if inputs
    .first()
    .is_some_and(|input| input.role == Role::Assistant)
  {
    inputs.insert(0, MessageParam {
      role: Role::User,
      content: MessageContent::String(EMPTY_STRING.into()),
    });
  }

  // 确保最后一条是 user
  if inputs
    .last()
    .is_some_and(|input| input.role == Role::Assistant)
  {
    inputs.push(MessageParam {
      role: Role::User,
      content: MessageContent::String(EMPTY_STRING.into()),
    });
  }

  // 转换为 proto messages
  let mut messages = Vec::new();
  let mut messages_headers = Vec::new();
  let mut base_uuid = rand::rng().random_range(256u16..384);

  for input in inputs {
    let (text, images, all_thinking_blocks) = match input.content {
      MessageContent::String(text) => (text, vec![], vec![]),
      MessageContent::Array(contents) if input.role == Role::User => {
        let mut text_parts = Vec::new();
        let mut images = Vec::new();

        for content in contents {
          match content {
            ContentBlockParam::Text { text } => text_parts.push(text),
            ContentBlockParam::Image { source } => {
              if image_support {
                let res = {
                  let vision_ability = AppConfig::get_vision_ability();

                  match vision_ability {
                    VisionAbility::None => Err(ERR_VISION_DISABLED.into()),
                    VisionAbility::Base64 => match source {
                      ImageSource::Base64 { media_type, data } =>
                        process_base64_image(media_type, &data),
                      ImageSource::Url { .. } => Err(ERR_BASE64_ONLY.into()),
                    },
                    VisionAbility::All => match source {
                      ImageSource::Base64 { media_type, data } =>
                        process_base64_image(media_type, &data),
                      ImageSource::Url { url } =>
                        tokio::spawn(
                          async move { process_http_image(&url, get_general_client()).await },
                        )
                        .await?,
                    },
                  }
                };
                match res {
                  Ok((image_data, dimension)) => {
                    images.push(ImageProto {
                      data: image_data,
                      dimension,
                      uuid: {
                        let s = base_uuid.to_string();
                        base_uuid = base_uuid.wrapping_add(1);
                        s
                      },
                      // task_specific_description: None,
                    });
                  }
                  Err(e) => return Err(e),
                }
              }
            }
            _ => {}
          }
        }

        (text_parts.join(NEWLINE), images, vec![])
      }
      MessageContent::Array(contents) if input.role == Role::Assistant => {
        let mut text_parts = Vec::new();
        let mut all_thinking_blocks = Vec::new();
        let last = contents.len() - 1;

        for (index, content) in contents.into_iter().enumerate() {
          match content {
            ContentBlockParam::Text { text } => {
              text_parts.push(text);
            }
            ContentBlockParam::Thinking {
              thinking,
              signature,
            } => {
              all_thinking_blocks.push(conversation_message::Thinking {
                text: thinking,
                signature,
                redacted_thinking: String::new(),
                is_last_thinking_chunk: index == last,
              });
            }
            ContentBlockParam::RedactedThinking { data } => {
              all_thinking_blocks.push(conversation_message::Thinking {
                text: String::new(),
                signature: String::new(),
                redacted_thinking: data,
                is_last_thinking_chunk: index == last,
              });
            }
            _ => {}
          }
        }

        (text_parts.join(NEWLINE), vec![], all_thinking_blocks)
      }
      _ => __unreachable!(),
    };

    // 处理消息内容和相关字段
    let (final_text, web_references, use_web, external_links) = match input.role {
      Role::Assistant => {
        let (text, web_refs, has_web) = extract_web_references_info(&text);
        (text, web_refs, has_web.to_opt(), vec![])
      }
      Role::User => {
        let external_links = extract_external_links(&text, &mut base_uuid);
        (text, vec![], None, external_links)
      }
      _ => __unreachable!(),
    };

    let r#type = if input.role == Role::User {
      conversation_message::MessageType::Human as i32
    } else {
      conversation_message::MessageType::Ai as i32
    };
    let bubble_id = Uuid::new_v4().to_string();
    let server_bubble_id = if input.role == Role::User {
      None
    } else {
      Some(Uuid::new_v4().to_string())
    };

    messages.push(ConversationMessage {
      text: final_text,
      r#type,
      images,
      bubble_id: bubble_id.clone(),
      server_bubble_id: server_bubble_id.clone(),
      tool_results: vec![],
      is_capability_iteration: Some(is_agentic),
      is_agentic,
      web_references,
      thinking: match all_thinking_blocks.len() {
        0 => None,
        1 => Some(all_thinking_blocks[0].clone()),
        _ => Some(conversation_message::Thinking {
          text: all_thinking_blocks
            .iter()
            .map(|t| t.text.as_str())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>()
            .join(EMPTY_STRING),
          signature: String::new(),
          redacted_thinking: String::new(),
          is_last_thinking_chunk: true,
        }),
      },
      all_thinking_blocks,
      unified_mode: Some(stream_unified_chat_request::UnifiedMode::Chat as i32),
      supported_tools: vec![],
      external_links,
      use_web,
      is_simple_looping_message: Some(false),
      ..Default::default()
    });
    messages_headers.push(ConversationMessageHeader {
      bubble_id,
      server_bubble_id,
      r#type,
    });
  }

  // 获取最后一条用户消息的URLs
  let external_links = messages
    .last_mut()
    .map(|msg| {
      msg.capabilities = tools
        .into_iter()
        .map(|t| ComposerCapabilityRequest {
          r#type: composer_capability_request::ComposerCapabilityType::ToolCall as i32,
          data: Some(composer_capability_request::Data::ToolCall(
            composer_capability_request::ToolCallCapability {
              custom_instructions: t.description,
              tool_schemas: vec![composer_capability_request::ToolSchema {
                r#type: composer_capability_request::ToolType::Iterate as i32,
                name: t.name,
                properties: unsafe {
                  ::core::intrinsics::transmute_unchecked(t.input_schema.properties)
                },
                required: t.input_schema.required,
              }],
              ..Default::default()
            },
          )),
        })
        .collect();
      msg.supported_tools = supported_tools;
      msg.external_links.clone()
    })
    .unwrap_or_default();

  Ok((instructions, messages, messages_headers, external_links))
}

/// 处理 base64 编码的图片
fn process_base64_image(
  media_type: MediaType,
  data: &str,
) -> Result<(Vec<u8>, Option<image_proto::Dimension>), Box<dyn std::error::Error + Send + Sync>> {
  // 检查图片格式是否支持
  match media_type {
    MediaType::ImagePng | MediaType::ImageJpeg | MediaType::ImageWebp => {
      // 这些格式都支持
    }
    MediaType::ImageGif => {
      // GIF 需要额外检查是否为动态图
    }
  }

  let image_data = BASE64.decode(data)?;

  // 检查是否为动态 GIF
  if matches!(media_type, MediaType::ImageGif)
    && let Ok(frames) = gif::DecodeOptions::new().read_info(std::io::Cursor::new(&image_data))
    && frames.into_iter().nth(1).is_some()
  {
    return Err(ERR_UNSUPPORTED_GIF.into());
  }

  // 获取图片尺寸
  let dimensions = if let Ok(img) = image::load_from_memory(&image_data) {
    Some(image_proto::Dimension {
      width: img.width() as i32,
      height: img.height() as i32,
    })
  } else {
    None
  };

  Ok((image_data, dimensions))
}

// 处理 HTTP 图片 URL
async fn process_http_image(
  url: &str,
  client: Client,
) -> Result<(Vec<u8>, Option<image_proto::Dimension>), Box<dyn std::error::Error + Send + Sync>> {
  let response = client.get(url).send().await?;
  let image_data = response.bytes().await?.to_vec();
  let format = guess_format(&image_data)?;

  // 检查图片格式
  match format {
    image::ImageFormat::Png | image::ImageFormat::Jpeg | image::ImageFormat::WebP => {
      // 这些格式都支持
    }
    image::ImageFormat::Gif => {
      if let Ok(frames) = gif::DecodeOptions::new().read_info(std::io::Cursor::new(&image_data))
        && frames.into_iter().nth(1).is_some()
      {
        return Err(ERR_UNSUPPORTED_GIF.into());
      }
    }
    _ => return Err(ERR_UNSUPPORTED_IMAGE_FORMAT.into()),
  }

  // 获取图片尺寸
  let dimensions = if let Ok(img) = image::load_from_memory_with_format(&image_data, format) {
    Some(image_proto::Dimension {
      width: img.width() as i32,
      height: img.height() as i32,
    })
  } else {
    None
  };

  Ok((image_data, dimensions))
}

pub async fn encode_message_params(
  params: MessageCreateParams,
  now_with_tz: chrono::DateTime<chrono_tz::Tz>,
  model: ExtModel,
  msg_id: Uuid,
  disable_vision: bool,
  enable_slow_pool: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
  let is_chat = params.tools.is_empty();
  let is_agentic = !is_chat;
  let supported_tools = if is_agentic {
    vec![ClientSideToolV2::Mcp as i32]
  } else {
    vec![]
  };

  let (instructions, messages, messages_headers, external_links) = process_message_params(
    params.messages,
    params.system,
    params.tools.clone(),
    supported_tools.clone(),
    now_with_tz,
    !disable_vision && model.is_image,
    is_agentic,
  )
  .await?;

  let explicit_context = if !instructions.trim().is_empty() {
    Some(ExplicitContext {
      context: instructions,
      repo_context: None,
      rules: vec![],
      mode_specific_context: None,
    })
  } else {
    None
  };

  let long_context = AppConfig::get_long_context() || LONG_CONTEXT_MODELS.contains(&model.id);

  let message = StreamUnifiedChatRequestWithTools {
        request: Some(crate::core::aiserver::v1::stream_unified_chat_request_with_tools::Request::StreamUnifiedChatRequest(Box::new(StreamUnifiedChatRequest {
            conversation: messages,
            full_conversation_headers_only: messages_headers,
            allow_long_file_scan: Some(false),
            explicit_context,
            can_handle_filenames_after_language_ids: Some(false),
            model_details: Some(ModelDetails {
                model_name: Some(model.id.to_string()),
                azure_state: Some(AzureState::default()),
                enable_slow_pool: enable_slow_pool.to_opt(),
                max_mode: Some(model.max),
                ..Default::default()
            }),
            use_web: if model.web {
                Some(WEB_SEARCH_MODE.to_string())
            } else {
                None
            },
            external_links,
            should_cache: Some(false),
            current_file: Some(crate::core::aiserver::v1::CurrentFileInfo {
                contents_start_at_line: 1,
                cursor_position: Some(CursorPosition { line: 0, column: 0 }),
                total_number_of_lines: 1,
                selection: Some(CursorRange {
                    start_position: Some(CursorPosition { line: 0, column: 0 }),
                    end_position: Some(CursorPosition { line: 0, column: 0 }),
                }),
                ..Default::default()
            }),
            use_reference_composer_diff_prompt: Some(false),
            use_new_compression_scheme: Some(false),
            is_chat,
            conversation_id: msg_id.to_string(),
            environment_info: Some(EnvironmentInfo::default()),
            is_agentic,
            supported_tools: supported_tools.clone(),
            mcp_tools: params.tools.into_iter().map(|t| mcp_params::Tool {
                server_name: sanitize_tool_name(&t.name),
                name: t.name,
                description: t.description.unwrap_or_default(),
                parameters: __unwrap!(serde_json::to_string(&t.input_schema))
            }).collect(),
            use_full_inputs_context: long_context.to_opt(),
            is_resume: Some(false),
            allow_model_fallbacks: Some(false),
            number_of_times_shown_fallback_model_warning: Some(0),
            unified_mode: Some(stream_unified_chat_request::UnifiedMode::Chat as i32),
            tools_requiring_accepted_return: supported_tools,
            should_disable_tools: Some(is_chat),
            thinking_level: Some(if model.is_thinking {
                stream_unified_chat_request::ThinkingLevel::High
            } else {
                stream_unified_chat_request::ThinkingLevel::Unspecified
            } as i32),
            uses_rules: Some(false),
            mode_uses_auto_apply: Some(false),
            unified_mode_name: Some(if is_chat { ASK_MODE_NAME } else { AGENT_MODE_NAME }.to_string()),
            ..Default::default()
        })))
    };

  encode_message(&message, true)
}
