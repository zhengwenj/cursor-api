//! 高性能 Base64 编解码实现
//!
//! 本模块提供了一个优化的 Base64 编解码器，使用自定义字符集：
//! - 字符集：`-AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz1032547698_`
//! - 特点：URL 安全，无需填充字符
//!
//! # 性能优化
//!
//! - 使用查找表加速解码
//! - 使用 unsafe 避免边界检查
//! - 针对不同数据大小使用不同的内联策略
//! - 预分配内存避免动态扩容
//!
//! # 示例
//!
//! ```
//! let data = b"Hello, World!";
//! let encoded = to_base64(data);
//! let decoded = from_base64(&encoded).unwrap();
//! assert_eq!(data, &decoded[..]);
//! ```

/// Base64 字符集
///
/// 使用自定义的 64 个字符，按特定顺序排列。
/// 该字符集设计为 URL 安全，避免了标准 Base64 中的 `+` 和 `/` 字符。
const BASE64_CHARS: &[u8; 64] = b"-AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz1032547698_";

/// Base64 解码查找表
///
/// 将 ASCII 字符映射到其在 BASE64_CHARS 中的索引。
/// 无效字符映射为 0xFF。
const BASE64_DECODE_TABLE: [u8; 128] = {
    let mut table = [0xFF_u8; 128];
    let mut i = 0;
    while i < BASE64_CHARS.len() {
        table[BASE64_CHARS[i] as usize] = i as u8;
        i += 1;
    }
    table
};

/// 将字节数据编码为 Base64 字符串
///
/// # 参数
///
/// * `bytes` - 要编码的字节数据
///
/// # 返回值
///
/// 返回编码后的 Base64 字符串。空输入返回空字符串。
///
/// # 性能
///
/// - 时间复杂度：O(n)
/// - 空间复杂度：O(n)，其中 n 是输入长度
///
/// # 示例
///
/// ```
/// let encoded = to_base64(b"Hello");
/// assert_eq!(encoded, "SGVsbG8");
/// ```
#[inline]
pub fn to_base64(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return String::new();
    }

    let len = bytes.len();
    let output_len = len.div_ceil(3) * 4;
    let mut output = Vec::with_capacity(output_len);

    unsafe {
        let ptr = bytes.as_ptr();
        let mut i = 0;

        // 主循环：一次处理3个字节，生成4个Base64字符
        while i + 2 < len {
            let b1 = *ptr.add(i);
            let b2 = *ptr.add(i + 1);
            let b3 = *ptr.add(i + 2);

            // 将3个字节（24位）组合成一个u32
            let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);

            // 提取4个6位值并转换为Base64字符
            output.push(*BASE64_CHARS.get_unchecked((n >> 18) as usize));
            output.push(*BASE64_CHARS.get_unchecked(((n >> 12) & 0x3F) as usize));
            output.push(*BASE64_CHARS.get_unchecked(((n >> 6) & 0x3F) as usize));
            output.push(*BASE64_CHARS.get_unchecked((n & 0x3F) as usize));

            i += 3;
        }

        let remaining = len - i;
        if remaining > 2 {
            ::core::hint::unreachable_unchecked();
        }
        if remaining > 0 {
            let b1 = *ptr.add(i);
            let b2 = if remaining > 1 { *ptr.add(i + 1) } else { 0 };

            let n = ((b1 as u32) << 16) | ((b2 as u32) << 8);

            output.push(*BASE64_CHARS.get_unchecked((n >> 18) as usize));
            output.push(*BASE64_CHARS.get_unchecked(((n >> 12) & 0x3F) as usize));

            if remaining > 1 {
                output.push(*BASE64_CHARS.get_unchecked(((n >> 6) & 0x3F) as usize));
            }
        }

        // 安全：BASE64_CHARS 中的所有字符都是有效的 ASCII
        String::from_utf8_unchecked(output)
    }
}

/// 将 Base64 字符串解码为字节数据
///
/// # 参数
///
/// * `input` - 要解码的 Base64 字符串
///
/// # 返回值
///
/// - `Some(Vec<u8>)` - 解码成功时返回字节数据
/// - `None` - 输入包含无效字符或长度不合法时返回
///
/// # 错误情况
///
/// - 输入长度为 4n+1（Base64 编码不会产生这种长度）
/// - 输入包含非 ASCII 字符
/// - 输入包含不在字符集中的字符
///
/// # 性能
///
/// - 时间复杂度：O(n)
/// - 空间复杂度：O(n)，其中 n 是输入长度
///
/// # 示例
///
/// ```
/// let decoded = from_base64("SGVsbG8").unwrap();
/// assert_eq!(decoded, b"Hello");
///
/// // 无效输入返回 None
/// assert!(from_base64("Invalid@Base64").is_none());
/// ```
#[inline]
pub fn from_base64(input: &str) -> Option<Vec<u8>> {
    let input = input.as_bytes();
    let len = input.len();

    if len == 0 {
        return Some(Vec::new());
    }

    // Base64 编码的长度必须是 4 的倍数或余 2、3
    if len & 3 == 1 {
        return None;
    }

    // 验证所有字符都是有效的 Base64 字符
    if input
        .iter()
        .any(|&b| b >= 128 || BASE64_DECODE_TABLE[b as usize] == 0xFF)
    {
        return None;
    }

    let output_len = (len * 3) >> 2;
    let mut output = Vec::with_capacity(output_len);

    unsafe {
        let ptr = input.as_ptr();
        let mut i = 0;

        // 主循环：一次处理4个Base64字符，生成3个字节
        while i + 3 < len {
            let c1 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i) as usize);
            let c2 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i + 1) as usize);
            let c3 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i + 2) as usize);
            let c4 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i + 3) as usize);

            // 将4个6位值组合成24位
            let n = ((c1 as u32) << 18) | ((c2 as u32) << 12) | ((c3 as u32) << 6) | (c4 as u32);

            // 提取3个字节
            output.push((n >> 16) as u8);
            output.push((n >> 8) as u8);
            output.push(n as u8);

            i += 4;
        }

        // 处理剩余的2或3个字符
        let remainder = len - i;
        if remainder == 1 || remainder > 3 {
            ::core::hint::unreachable_unchecked();
        }
        if remainder >= 2 {
            let c1 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i) as usize);
            let c2 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i + 1) as usize);

            // 第一个字节：c1的6位 + c2的高2位
            output.push((c1 << 2) | (c2 >> 4));

            if remainder == 3 {
                let c3 = *BASE64_DECODE_TABLE.get_unchecked(*ptr.add(i + 2) as usize);
                // 第二个字节：c2的低4位 + c3的高4位
                output.push((c2 << 4) | (c3 >> 2));
            }
        }
    }

    Some(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(to_base64(b""), "");
        assert_eq!(from_base64("").unwrap(), b"");
    }

    #[test]
    fn test_basic() {
        let test_cases = [
            (b"f" as &[u8], "Zg"),
            (b"fo", "Zm8"),
            (b"foo", "Zm8v"),
            (b"foob", "Zm8vYg"),
            (b"fooba", "Zm8vYmE"),
            (b"foobar", "Zm8vYmFy"),
        ];

        for (input, expected) in test_cases {
            let encoded = to_base64(input);
            assert_eq!(encoded, expected);
            assert_eq!(from_base64(&encoded).unwrap(), input);
        }
    }

    #[test]
    fn test_invalid_input() {
        assert!(from_base64("!@#$").is_none());
        assert!(from_base64("ABC").is_none()); // 长度 % 4 == 1
        assert!(from_base64("测试").is_none()); // 非 ASCII
    }
}
