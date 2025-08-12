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
      AzureState, ComposerExternalLink, ConversationMessage, ConversationMessageHeader,
      CursorPosition, CursorRange, EnvironmentInfo, ExplicitContext, ImageProto, ModelDetails,
      StreamUnifiedChatRequest, StreamUnifiedChatRequestWithTools, conversation_message,
      image_proto, stream_unified_chat_request,
    },
    constant::{ERR_UNSUPPORTED_GIF, ERR_UNSUPPORTED_IMAGE_FORMAT, LONG_CONTEXT_MODELS},
    model::{ExtModel, Role, openai},
  },
};

use super::{
  ASK_MODE_NAME, ERR_BASE64_ONLY, ERR_VISION_DISABLED, NEWLINE, ToOpt as _, WEB_SEARCH_MODE,
  extract_external_links, extract_web_references_info,
};

crate::define_typed_constants! {
    &'static str => {
        /// 无效 Base64 格式错误消息
        ERR_INVALID_BASE64_FORMAT = "无效的 base64 图片格式",
        /// 支持的图片格式
        FORMAT_PNG = "png",
        FORMAT_JPEG = "jpeg",
        FORMAT_JPG = "jpg",
        FORMAT_WEBP = "webp",
        FORMAT_GIF = "gif",
        /// Data URL 前缀
        DATA_IMAGE_PREFIX = "data:image/",
        /// Base64 分隔符
        BASE64_SEPARATOR = ";base64,",
        /// 双换行符用于分隔指令
        DOUBLE_NEWLINE = "\n\n",
    }
}

async fn process_chat_inputs(
  inputs: Vec<openai::Message>,
  now_with_tz: chrono::DateTime<chrono_tz::Tz>,
  image_support: bool,
) -> Result<
  (
    String,
    Vec<ConversationMessage>,
    Vec<ConversationMessageHeader>,
    Vec<ComposerExternalLink>,
  ),
  Box<dyn std::error::Error + Send + Sync>,
> {
  // 分别收集 system 指令和 user/assistant 对话
  let (system_messages, chat_messages): (Vec<_>, Vec<_>) = inputs
    .into_iter()
    .partition(|input| input.role == Role::System);

  // 收集 system 指令
  let instructions = system_messages
    .into_iter()
    .map(|input| match input.content {
      openai::MessageContent::String(text) => text,
      openai::MessageContent::Array(contents) => contents
        .into_iter()
        .filter_map(openai::MessageContentObject::into_text)
        .collect::<Vec<String>>()
        .join(NEWLINE),
    })
    .collect::<Vec<String>>()
    .join(DOUBLE_NEWLINE);

  // 使用默认指令或收集到的指令
  let instructions = if instructions.is_empty() {
    get_default_instructions(now_with_tz)
  } else {
    instructions
  };

  // 过滤出 user 和 assistant 对话
  let mut chat_inputs = chat_messages;

  // 处理空对话情况
  if chat_inputs.is_empty() {
    let bubble_id = Uuid::new_v4().to_string();
    return Ok((
      instructions,
      vec![ConversationMessage {
        r#type: conversation_message::MessageType::Human as i32,
        bubble_id: bubble_id.clone(),
        is_capability_iteration: Some(false),
        unified_mode: Some(stream_unified_chat_request::UnifiedMode::Chat as i32),
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
  if chat_inputs
    .first()
    .is_some_and(|input| input.role == Role::Assistant)
  {
    chat_inputs.insert(0, openai::Message {
      role: Role::User,
      content: openai::MessageContent::String(EMPTY_STRING.into()),
    });
  }

  // 确保最后一条是 user
  if chat_inputs
    .last()
    .is_some_and(|input| input.role == Role::Assistant)
  {
    chat_inputs.push(openai::Message {
      role: Role::User,
      content: openai::MessageContent::String(EMPTY_STRING.into()),
    });
  }

  // 转换为 proto messages
  let mut messages = Vec::new();
  let mut messages_headers = Vec::new();
  let mut base_uuid = rand::rng().random_range(256u16..384);

  for input in chat_inputs {
    let (text, images) = match input.content {
      openai::MessageContent::String(text) => (text, vec![]),
      openai::MessageContent::Array(contents) if input.role == Role::User => {
        let mut text_parts = Vec::new();
        let mut images = Vec::new();

        for content in contents {
          match content {
            openai::MessageContentObject::Text { text } => text_parts.push(text),
            openai::MessageContentObject::ImageUrl { image_url } => {
              if image_support {
                let url = image_url.url;
                let res = {
                  let vision_ability = AppConfig::get_vision_ability();

                  match vision_ability {
                    VisionAbility::None => Err(ERR_VISION_DISABLED.into()),
                    VisionAbility::Base64 => {
                      if let Some(url) = url.strip_prefix(DATA_IMAGE_PREFIX) {
                        process_base64_image(url)
                      } else {
                        Err(ERR_BASE64_ONLY.into())
                      }
                    }
                    VisionAbility::All =>
                      if let Some(url) = url.strip_prefix(DATA_IMAGE_PREFIX) {
                        process_base64_image(url)
                      } else {
                        tokio::spawn(
                          async move { process_http_image(&url, get_general_client()).await },
                        )
                        .await?
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
          }
        }

        (text_parts.join(NEWLINE), images)
      }
      openai::MessageContent::Array(contents) => {
        let mut text_parts = Vec::new();

        for content in contents {
          if let Some(text) = content.into_text() {
            text_parts.push(text);
          }
        }

        (text_parts.join(NEWLINE), vec![])
      }
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
      is_capability_iteration: None,
      is_agentic: false,
      // existed_subsequent_terminal_command: false,
      // existed_previous_terminal_command: false,
      web_references,
      // git_context: None,
      // cached_conversation_summary: None,
      // attached_human_changes: false,
      thinking: None,
      unified_mode: Some(stream_unified_chat_request::UnifiedMode::Chat as i32),
      external_links,
      use_web,
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
    .last()
    .map(|msg| msg.external_links.clone())
    .unwrap_or_default();

  Ok((instructions, messages, messages_headers, external_links))
}

/// 处理 base64 编码的图片
fn process_base64_image(
  url: &str,
) -> Result<(Vec<u8>, Option<image_proto::Dimension>), Box<dyn std::error::Error + Send + Sync>> {
  let (format, data) = match url.split_once(BASE64_SEPARATOR) {
    Some(v) => v,
    None => return Err(ERR_INVALID_BASE64_FORMAT.into()),
  };

  // 检查图片格式
  if format != FORMAT_PNG
    && format != FORMAT_JPEG
    && format != FORMAT_JPG
    && format != FORMAT_WEBP
    && format != FORMAT_GIF
  {
    return Err(ERR_UNSUPPORTED_IMAGE_FORMAT.into());
  }

  let image_data = BASE64.decode(data)?;

  // 检查是否为动态 GIF
  if format == FORMAT_GIF
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

pub async fn encode_chat_message(
  inputs: Vec<openai::Message>,
  now_with_tz: chrono::DateTime<chrono_tz::Tz>,
  model: ExtModel,
  msg_id: Uuid,
  disable_vision: bool,
  enable_slow_pool: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
  let (instructions, messages, messages_headers, external_links) =
    process_chat_inputs(inputs, now_with_tz, !disable_vision && model.is_image).await?;

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

  let chat = StreamUnifiedChatRequestWithTools {
        request: Some(crate::core::aiserver::v1::stream_unified_chat_request_with_tools::Request::StreamUnifiedChatRequest(Box::new(StreamUnifiedChatRequest {
            conversation: messages,
            full_conversation_headers_only: messages_headers,
            // allow_long_file_scan: Some(false),
            explicit_context,
            // can_handle_filenames_after_language_ids: Some(false),
            model_details: Some(ModelDetails {
                model_name: Some(model.id.to_string()),
                azure_state: Some(AzureState::default()),
                enable_slow_pool: enable_slow_pool.to_opt(),
                max_mode: Some(model.max),
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
            }),
            use_reference_composer_diff_prompt: Some(false),
            use_new_compression_scheme: Some(false),
            is_chat: false,
            conversation_id: msg_id.to_string(),
            environment_info: Some(EnvironmentInfo::default()),
            is_agentic: false,
            supported_tools: vec![],
            // use_unified_chat_prompt: false,
            mcp_tools: vec![],
            use_full_inputs_context: long_context.to_opt(),
            is_resume: Some(false),
            allow_model_fallbacks: Some(false),
            number_of_times_shown_fallback_model_warning: Some(0),
            // is_headless: false,
            unified_mode: Some(stream_unified_chat_request::UnifiedMode::Chat as i32),
            tools_requiring_accepted_return: vec![],
            should_disable_tools: Some(true),
            thinking_level: Some(if model.is_thinking {
                stream_unified_chat_request::ThinkingLevel::High
            } else {
                stream_unified_chat_request::ThinkingLevel::Unspecified
            } as i32),
            // should_use_chat_prompt: None,
            uses_rules: Some(false),
            mode_uses_auto_apply: Some(false),
            unified_mode_name: Some(ASK_MODE_NAME.to_string()),
        })))
    };

  encode_message(&chat, true)
}
