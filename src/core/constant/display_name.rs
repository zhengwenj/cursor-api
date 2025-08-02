use ::ahash::HashMap;
use ::parking_lot::Mutex;

use crate::{app::constant::EMPTY_STRING, leak::manually_init::ManuallyInit};

static DISPLAY_NAME_CACHE: ManuallyInit<Mutex<HashMap<&'static str, &'static str>>> =
    ManuallyInit::new();

pub fn init_display_name_cache() {
    unsafe { DISPLAY_NAME_CACHE.init(Mutex::new(HashMap::default())) }
}

/// 计算 AI 模型标识符的显示名称。
///
/// 规则：
/// 1. 单个数字通过 '-' 连接时 (前后都不是数字)，'-' 变为 '.' (例如 "3-5" -> "3.5")。
/// 2. 日期格式中的 '-' (如 YYYY-MM-DD, MM-DD) 保持不变。
/// 3. 其他所有的 '-' 都被替换为空格 ' '。
/// 4. 由原始 '-' 分隔的各部分（处理后）首字母大写 (Title Case)。
/// 5. 特殊规则：如果原始标识符以 "gpt" 开头，则输出的对应部分为 "GPT"。
///
/// # Arguments
///
/// * `identifier` - AI 模型的原始标识符字符串。
///
/// # Returns
///
/// * `String` - 计算得到的显示名称。
///
/// # Examples
///
/// ```
/// assert_eq!(calculate_display_name_v4("claude-3-5-sonnet"), "Claude 3.5 Sonnet");
/// assert_eq!(calculate_display_name_v4("gpt-4-turbo-2024-04-09"), "GPT 4 Turbo 2024-04-09"); // 日期 '-' 不变
/// assert_eq!(calculate_display_name_v4("gemini-1.5-flash-500k"), "Gemini 1.5 Flash 500k");
/// assert_eq!(calculate_display_name_v4("deepseek-v3"), "Deepseek V3");
/// assert_eq!(calculate_display_name_v4("gpt-4o"), "GPT 4o");
/// assert_eq!(calculate_display_name_v4("gpt-3.5-turbo"), "GPT 3.5 Turbo"); // 输入有 .
/// assert_eq!(calculate_display_name_v4("gemini-2.5-pro-exp-03-25"), "Gemini 2.5 Pro Exp 03-25"); // 日期 '-' 不变
/// assert_eq!(calculate_display_name_v4("version-10-beta"), "Version 10 Beta"); // 10 不是单数字
/// assert_eq!(calculate_display_name_v4("model-1-test-9-case"), "Model 1 Test 9 Case"); // d-d 转义被空格隔开
/// assert_eq!(calculate_display_name_v4("deepseek-r1-0528"), "Deepseek R1 0528"); // 新增测试
/// ```
pub fn calculate_display_name_v4(identifier: &'static str) -> &'static str {
    if let Some(cached) = DISPLAY_NAME_CACHE.lock().get(identifier) {
        return cached;
    }

    let result = calculate_display_name_internal(identifier);

    DISPLAY_NAME_CACHE.lock().insert(identifier, result);

    result
}

#[inline(always)]
fn calculate_display_name_internal(identifier: &'static str) -> &'static str {
    const GPT: &str = "GPT";

    if identifier.is_empty() {
        return EMPTY_STRING;
    }

    let mut result = String::with_capacity(identifier.len());
    let mut capitalize_next = true;
    let mut prev_char: Option<char> = None;
    let mut prev_prev_char: Option<char> = None;

    let mut char_iter = identifier.chars().peekable();
    if let Some(rest) = identifier.strip_prefix("gpt") {
        if rest.is_empty() {
            return GPT;
        } else if unsafe { *rest.as_bytes().get_unchecked(0) } == b'-' {
            result.push_str(GPT);
            for _ in 0..4 {
                char_iter.next();
            }
            result.push(' ');
            capitalize_next = true;
            prev_char = Some('-');
            prev_prev_char = Some('t');
        }
    }

    while let Some(current_char) = char_iter.next() {
        match current_char {
            '-' => {
                let prev_is_digit = prev_char.is_some_and(|p| p.is_ascii_digit());
                let next_is_digit = char_iter.peek().is_some_and(|&n| n.is_ascii_digit());

                if prev_is_digit && next_is_digit {
                    // 检查前面是否有多个数字
                    let prev_is_single_digit =
                        !prev_prev_char.is_some_and(|pp| pp.is_ascii_digit());

                    // 检查后面是否只有一个数字
                    let mut next_is_single_digit = true;
                    let mut lookahead = char_iter.clone();
                    lookahead.next(); // Skip the next digit we already know about
                    if let Some(&next_next_char) = lookahead.peek()
                        && next_next_char.is_ascii_digit()
                    {
                        next_is_single_digit = false;
                    }

                    // 只有当前后都是单个数字时，才转换为 '.'
                    if prev_is_single_digit && next_is_single_digit {
                        result.push('.');
                        capitalize_next = false;
                    } else {
                        // 否则，如果是日期格式（两位数-两位数），保留 '-'
                        // 其他情况转换为空格
                        if is_date_pattern(prev_prev_char, prev_char, &mut char_iter.clone()) {
                            result.push('-');
                            capitalize_next = true;
                        } else {
                            result.push(' ');
                            capitalize_next = true;
                        }
                    }
                } else {
                    result.push(' ');
                    capitalize_next = true;
                }
                prev_prev_char = prev_char;
                prev_char = Some('-');
            }
            c => {
                if capitalize_next {
                    result.extend(c.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
                prev_prev_char = prev_char;
                prev_char = Some(c);
            }
        }
    }

    crate::leak::intern_static(result)
}

// 检查是否是日期模式（如 MM-DD 或类似格式）
fn is_date_pattern(
    prev_prev: Option<char>,
    prev: Option<char>,
    char_iter: &mut std::iter::Peekable<std::str::Chars>,
) -> bool {
    // 检查是否是两位数字-两位数字的格式
    if let (Some(pp), Some(p)) = (prev_prev, prev)
        && pp.is_ascii_digit()
        && p.is_ascii_digit()
    {
        // 查看接下来的两个字符
        let mut lookahead = char_iter.clone();
        if let Some(&next1) = lookahead.peek()
            && next1.is_ascii_digit()
        {
            lookahead.next();
            if let Some(&next2) = lookahead.peek() {
                // 如果是 DD-DD 格式，认为是日期
                return next2.is_ascii_digit();
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_name_v4_final() {
        // Anthropic
        assert_eq!(calculate_display_name_v4("claude-3-opus"), "Claude 3 Opus");
        assert_eq!(
            calculate_display_name_v4("claude-3.5-sonnet"),
            "Claude 3.5 Sonnet"
        ); // Input has dot
        assert_eq!(
            calculate_display_name_v4("claude-3-5-sonnet"),
            "Claude 3.5 Sonnet"
        ); // d-d conversion
        assert_eq!(
            calculate_display_name_v4("claude-3-haiku-200k"),
            "Claude 3 Haiku 200k"
        );

        // OpenAI (GPT & Date Preservation)
        assert_eq!(calculate_display_name_v4("gpt-4"), "GPT 4");
        assert_eq!(calculate_display_name_v4("gpt-4o"), "GPT 4o");
        assert_eq!(calculate_display_name_v4("gpt-3.5-turbo"), "GPT 3.5 Turbo"); // Input has dot
        assert_eq!(
            calculate_display_name_v4("gpt-4-turbo-2024-04-09"),
            "GPT 4 Turbo 2024-04-09"
        ); // Date preserved!
        assert_eq!(calculate_display_name_v4("gpt-4o-mini"), "GPT 4o Mini");
        assert_eq!(calculate_display_name_v4("gpt"), "GPT");
        assert_eq!(calculate_display_name_v4("gpt-"), "GPT "); // Trailing hyphen becomes space

        // Google (Date Preservation)
        assert_eq!(
            calculate_display_name_v4("gemini-1.5-flash-500k"),
            "Gemini 1.5 Flash 500k"
        ); // Input has dot
        assert_eq!(
            calculate_display_name_v4("gemini-2.5-pro-exp-03-25"),
            "Gemini 2.5 Pro Exp 03-25"
        ); // Date preserved!

        // Deepseek
        assert_eq!(calculate_display_name_v4("deepseek-v3"), "Deepseek V3");
        assert_eq!(
            calculate_display_name_v4("deepseek-r1-0528"),
            "Deepseek R1 0528"
        ); // 新增测试

        // Other & Edge Cases
        assert_eq!(calculate_display_name_v4("o1-mini"), "O1 Mini");
        assert_eq!(
            calculate_display_name_v4("model-1-test-9-case"),
            "Model 1 Test 9 Case"
        ); // d-d handled
        assert_eq!(
            calculate_display_name_v4("version-10-beta"),
            "Version 10 Beta"
        ); // 10 is not single digit
        assert_eq!(
            calculate_display_name_v4("alpha-1-5-omega"),
            "Alpha 1.5 Omega"
        ); // d-d handled
        assert_eq!(calculate_display_name_v4("my-gpt-model"), "My Gpt Model"); // gpt not at start
        assert_eq!(calculate_display_name_v4(""), ""); // Empty
        assert_eq!(calculate_display_name_v4("-start-"), " Start "); // Leading/trailing hyphens
        assert_eq!(calculate_display_name_v4("a-b-c"), "A B C");
        assert_eq!(calculate_display_name_v4("3-5"), "3.5"); // Only d-d
        assert_eq!(calculate_display_name_v4("2024-release"), "2024 Release"); // Number-text
        assert_eq!(calculate_display_name_v4("data-01-01"), "Data 01-01"); // Date like MM-DD
    }
}
