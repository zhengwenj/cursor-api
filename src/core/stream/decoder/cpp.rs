use ::bytes::{Buf as _, BytesMut};
use ::prost::Message as _;

use super::decompress_gzip;
use crate::core::{aiserver::v1::StreamCppResponse, error::StreamError};

#[derive(::serde::Serialize, PartialEq, Clone)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum StreamMessage {
    // 模型信息消息
    ModelInfo {
        is_fused_cursor_prediction_model: bool,
        is_multidiff_model: bool,
    },
    // 范围替换消息
    RangeReplace {
        start_line_number: i32,
        end_line_number_inclusive: i32,
        text: String,
    },
    // 光标预测消息
    CursorPrediction {
        relative_path: String,
        line_number_one_indexed: i32,
        expected_content: String,
        should_retrigger_cpp: bool,
    },
    // 文本消息
    Text {
        text: String,
    },
    // 编辑完成标志
    DoneEdit,
    // 流完成标志
    DoneStream,
    // 调试信息消息
    Debug {
        model_input: String,
        model_output: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        total_time: Option<String>,
        stream_time: String,
        ttft_time: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        server_timing: Option<String>,
    },
    // 错误
    Error {
        message: String,
    },
    // 流结束标志
    StreamEnd,
}

pub struct StreamDecoder {
    // 主要数据缓冲区 (32字节)
    buffer: BytesMut,
    // 结果相关 (24字节)
    first_result: Option<Vec<StreamMessage>>,
    // 计数器和时间 (8字节)
    empty_stream_count: usize,
    // 状态标志 (1字节 + 1字节)
    first_result_ready: bool,
    first_result_taken: bool,
}

impl StreamDecoder {
    pub fn new() -> Self {
        Self {
            buffer: BytesMut::new(),
            first_result: None,
            empty_stream_count: 0,
            first_result_ready: false,
            first_result_taken: false,
        }
    }

    #[inline]
    pub fn get_empty_stream_count(&self) -> usize { self.empty_stream_count }

    #[inline]
    pub fn reset_empty_stream_count(&mut self) {
        if self.empty_stream_count > 0 {
            // crate::debug!("重置连续空流计数，之前的计数为: {}", self.empty_stream_count);
            self.empty_stream_count = 0;
        }
    }

    #[inline]
    pub fn take_first_result(&mut self) -> Option<Vec<StreamMessage>> {
        if !self.buffer.is_empty() {
            return None;
        }
        if self.first_result.is_some() {
            self.first_result_taken = true;
        }
        self.first_result.take()
    }

    #[inline]
    pub fn is_first_result_ready(&self) -> bool { self.first_result_ready }

    // #[inline]
    // pub fn no_first_cache(mut self) -> Self {
    //     self.first_result_ready = true;
    //     self.first_result_taken = true;
    //     self
    // }

    pub fn decode(&mut self, data: &[u8]) -> Result<Vec<StreamMessage>, StreamError> {
        if !data.is_empty() {
            self.reset_empty_stream_count();
        }

        self.buffer.extend_from_slice(data);

        if self.buffer.len() < 5 {
            if self.buffer.is_empty() {
                self.empty_stream_count += 1;

                return Err(StreamError::EmptyStream);
            }
            crate::debug!("数据长度小于5字节，当前数据: {}", hex::encode(&self.buffer));
            return Err(StreamError::DataLengthLessThan5);
        }

        self.reset_empty_stream_count();

        let reserve = {
            let mut offset = 0;
            let mut count = 0;
            while offset + 5 <= self.buffer.len() {
                let msg_len: usize;

                // SAFETY: The loop condition `offset + 5 <= self.buffer.len()` guarantees
                // that indices `offset` through `offset + 4` are within bounds.
                unsafe {
                    msg_len = u32::from_be_bytes([
                        *self.buffer.get_unchecked(offset + 1),
                        *self.buffer.get_unchecked(offset + 2),
                        *self.buffer.get_unchecked(offset + 3),
                        *self.buffer.get_unchecked(offset + 4),
                    ]) as usize;
                }

                offset += 5;

                if msg_len == 0 {
                    continue;
                }

                if offset + msg_len > self.buffer.len() {
                    // 最后一次循环 offset -= 5;
                    break;
                }

                offset += msg_len;
                count += 1;
            }
            count
        };

        let mut messages = Vec::with_capacity(reserve);
        let mut offset = 0;

        while offset + 5 <= self.buffer.len() {
            let msg_type: u8;
            let msg_len: usize;

            // SAFETY: The loop condition `offset + 5 <= self.buffer.len()` guarantees
            // that indices `offset` through `offset + 4` are within bounds.
            unsafe {
                msg_type = *self.buffer.get_unchecked(offset);
                msg_len = u32::from_be_bytes([
                    *self.buffer.get_unchecked(offset + 1),
                    *self.buffer.get_unchecked(offset + 2),
                    *self.buffer.get_unchecked(offset + 3),
                    *self.buffer.get_unchecked(offset + 4),
                ]) as usize;
            }

            offset += 5;

            if msg_len == 0 {
                continue;
            }

            let expected_size = offset + msg_len;
            if expected_size > self.buffer.len() {
                offset -= 5;
                break;
            }
            let msg_data = unsafe { self.buffer.get_unchecked(offset..expected_size) };

            if let Some(msg) = Self::process_message(msg_type, msg_data)? {
                messages.push(msg);
            }

            offset += msg_len;
        }

        self.buffer.advance(offset);

        if !self.first_result_taken && !messages.is_empty() {
            if self.first_result.is_none() {
                self.first_result = Some(::core::mem::take(&mut messages));
            } else if !self.first_result_ready
                && let Some(first_result) = &mut self.first_result
            {
                first_result.append(&mut messages);
            }
        }
        if !self.first_result_ready {
            self.first_result_ready =
                self.first_result.is_some() && self.buffer.is_empty() && !self.first_result_taken;
        }
        Ok(messages)
    }

    #[inline]
    fn process_message(
        msg_type: u8,
        msg_data: &[u8],
    ) -> Result<Option<StreamMessage>, StreamError> {
        match msg_type {
            0 => Self::handle_text_message(msg_data),
            1 => Self::handle_gzip_message(msg_data, Self::handle_text_message),
            2 => Self::handle_json_message(msg_data),
            3 => Self::handle_gzip_message(msg_data, Self::handle_json_message),
            t => {
                eprintln!("收到未知消息类型: {t}，请尝试联系开发者以获取支持");
                crate::debug!("消息类型: {t}，消息内容: {}", hex::encode(msg_data));
                Ok(None)
            }
        }
    }

    #[inline]
    fn handle_text_message(msg_data: &[u8]) -> Result<Option<StreamMessage>, StreamError> {
        Ok(match StreamCppResponse::decode(msg_data) {
            Ok(response) =>
                if let Some(model_info) = response.model_info {
                    Some(StreamMessage::ModelInfo {
                        is_fused_cursor_prediction_model: model_info
                            .is_fused_cursor_prediction_model,
                        is_multidiff_model: model_info.is_multidiff_model,
                    })
                } else if let Some(range) = response.range_to_replace {
                    Some(StreamMessage::RangeReplace {
                        start_line_number: range.start_line_number,
                        end_line_number_inclusive: range.end_line_number_inclusive,
                        text: response.text,
                    })
                } else if let Some(cursor_target) = response.cursor_prediction_target {
                    Some(StreamMessage::CursorPrediction {
                        relative_path: cursor_target.relative_path,
                        line_number_one_indexed: cursor_target.line_number_one_indexed,
                        expected_content: cursor_target.expected_content,
                        should_retrigger_cpp: cursor_target.should_retrigger_cpp,
                    })
                } else if response.done_edit.unwrap_or(false) {
                    Some(StreamMessage::DoneEdit)
                } else if response.done_stream.unwrap_or(false) {
                    Some(StreamMessage::DoneStream)
                } else if response.debug_stream_time.is_some() && response.debug_ttft_time.is_some()
                {
                    Some(StreamMessage::Debug {
                        model_input: response.debug_model_input.unwrap_or_default(),
                        model_output: response.debug_model_output.unwrap_or_default(),
                        total_time: response.debug_total_time,
                        stream_time: response.debug_stream_time.unwrap_or_default(),
                        ttft_time: response.debug_ttft_time.unwrap_or_default(),
                        server_timing: response.debug_server_timing,
                    })
                } else if !response.text.is_empty() {
                    Some(StreamMessage::Text {
                        text: response.text,
                    })
                } else {
                    None
                },
            Err(_) => None,
        })
    }

    #[inline]
    fn handle_gzip_message(
        msg_data: &[u8],
        f: fn(&[u8]) -> Result<Option<StreamMessage>, StreamError>,
    ) -> Result<Option<StreamMessage>, StreamError> {
        if let Some(msg_data) = decompress_gzip(msg_data) {
            f(&msg_data)
        } else {
            Ok(None)
        }
    }

    #[inline]
    fn handle_json_message(msg_data: &[u8]) -> Result<Option<StreamMessage>, StreamError> {
        if msg_data.len() == 2 {
            return Ok(Some(StreamMessage::StreamEnd));
        }
        if let Some(text) = super::utils::string_from_utf8(msg_data) {
            // if let Ok(error) = serde_json::from_str::<ChatError>(&text) {
            //     return Err(StreamError::ChatError(error));
            // }
            return Ok(Some(StreamMessage::Error { message: text }));
        }
        Ok(None)
    }
}
