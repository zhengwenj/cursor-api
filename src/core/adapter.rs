use super::aiserver::v1::{ComposerExternalLink, WebReference};

pub mod anthropic;
pub mod openai;

crate::define_typed_constants! {
    &'static str => {
        /// 图片功能禁用错误消息
        ERR_VISION_DISABLED = "图片功能已禁用",
        /// Base64 图片限制错误消息
        ERR_BASE64_ONLY = "仅支持 base64 编码的图片",
        /// Web 搜索模式
        WEB_SEARCH_MODE = "full_search",
        /// Ask 模式名称
        ASK_MODE_NAME = "Ask",
        /// Agent 模式名称
        AGENT_MODE_NAME = "Agent",
        /// 换行符
        NEWLINE = "\n",
    }
}

#[inline]
fn parse_web_references(text: &str) -> Vec<WebReference> {
    let mut web_refs = Vec::new();
    let lines = text.lines().skip(1); // 跳过 "WebReferences:" 行

    for line in lines {
        let line = line.trim();
        if line.is_empty() {
            break;
        }

        // 跳过序号和空格
        let mut chars = line.chars();
        for c in chars.by_ref() {
            if c == '.' {
                break;
            }
        }
        let remaining = chars.as_str().trim_start();

        // 解析 [title](url) 部分
        let mut chars = remaining.chars();
        if chars.next() != Some('[') {
            continue;
        }

        let mut title = String::with_capacity(64);
        let mut url = String::with_capacity(64);
        let mut chunk = String::with_capacity(64);
        let mut current = &mut title;
        let mut state = 0; // 0: title, 1: url, 2: chunk

        while let Some(c) = chars.next() {
            match (state, c) {
                (0, ']') => {
                    state = 1;
                    if chars.next() != Some('(') {
                        break;
                    }
                    current = &mut url;
                }
                (1, ')') => {
                    state = 2;
                    if chars.next() == Some('<') {
                        current = &mut chunk;
                    } else {
                        break;
                    }
                }
                (2, '>') => break,
                (_, c) => current.push(c),
            }
        }

        web_refs.push(WebReference { title, url, chunk });
    }

    web_refs
}

// 解析消息中的外部链接
#[inline]
fn extract_external_links(text: &str, base_uuid: &mut u16) -> Vec<ComposerExternalLink> {
    let mut external_links = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '@' {
            let mut url = String::new();
            while let Some(&next_char) = chars.peek() {
                if next_char.is_whitespace() {
                    break;
                }
                url.push(__unwrap!(chars.next()));
            }

            if !url.is_empty()
                && let Ok(parsed_url) = url::Url::parse(&url)
                && (parsed_url.scheme() == "http" || parsed_url.scheme() == "https")
            {
                external_links.push(ComposerExternalLink {
                    url,
                    uuid: base_uuid.to_string(),
                });
                *base_uuid = base_uuid.wrapping_add(1);
            }
        }
    }

    external_links
}

// 检测并分离 WebReferences
#[inline]
fn extract_web_references_info(text: &str) -> (String, Vec<WebReference>, bool) {
    if text.starts_with("WebReferences:") {
        if let Some((web_refs_text, content_text)) = text.split_once("\n\n") {
            let web_refs = parse_web_references(web_refs_text);
            let has_web_refs = !web_refs.is_empty();
            (content_text.to_string(), web_refs, has_web_refs)
        } else {
            (text.to_string(), vec![], false)
        }
    } else {
        (text.to_string(), vec![], false)
    }
}

trait ToOpt: Copy {
    fn to_opt(self) -> Option<Self>;
}

impl ToOpt for bool {
    #[inline(always)]
    fn to_opt(self) -> Option<Self> { if self { Some(true) } else { None } }
}

#[inline]
fn sanitize_tool_name(input: &str) -> String {
    let mut result = String::with_capacity(input.len());

    for c in input.chars() {
        match c {
            '.' => result.push('_'),
            c if c.is_whitespace() => result.push('_'),
            c if c.is_ascii_alphanumeric() || c == '_' || c == '-' => result.push(c),
            _ => {} // 忽略其他字符
        }
    }

    result.shrink_to_fit();

    result
}
