#![allow(unsafe_op_in_unsafe_fn)]

//! 高性能十六进制编解码工具

// ASCII 十六进制解码表，只需要 128 个元素
pub const HEX_DECODE_TABLE: [u8; 128] = {
    let mut table = [0xFF; 128];
    let mut i = 0;
    while i < 10 {
        table[b'0' as usize + i] = i as u8;
        i += 1;
    }
    let mut i = 0;
    while i < 6 {
        table[b'a' as usize + i] = 10 + i as u8;
        table[b'A' as usize + i] = 10 + i as u8;
        i += 1;
    }
    table
};

/// 十六进制字符表
pub const HEX_CHARS: &[u8; 16] = b"0123456789abcdef";

/// 将单个字节编码为两个十六进制字符（小写）
#[inline(always)]
pub fn byte_to_hex(byte: u8, out: &mut [u8; 2]) {
    out[0] = unsafe { *HEX_CHARS.get_unchecked((byte >> 4) as usize) };
    out[1] = unsafe { *HEX_CHARS.get_unchecked((byte & 0x0f) as usize) };
}

/// 解码两个十六进制字符为一个字节（带边界检查）
#[inline(always)]
pub const fn hex_to_byte(hi: u8, lo: u8) -> Option<u8> {
    if hi >= 128 || lo >= 128 {
        return None;
    }

    let high = HEX_DECODE_TABLE[hi as usize];
    let low = HEX_DECODE_TABLE[lo as usize];

    if high == 0xFF || low == 0xFF {
        None
    } else {
        Some((high << 4) | low)
    }
}

// /// 解码两个十六进制字符为一个字节（无边界检查）
// ///
// /// # Safety
// /// 调用者必须保证 hi 和 lo 都是有效的十六进制字符
// #[inline(always)]
// pub unsafe fn hex_to_byte_unchecked(hi: u8, lo: u8) -> u8 {
//     debug_assert!(hi < 128 && lo < 128);
//     let high = *HEX_DECODE_TABLE.get_unchecked(hi as usize);
//     let low = *HEX_DECODE_TABLE.get_unchecked(lo as usize);
//     debug_assert!(high != 0xFF && low != 0xFF);
//     (high << 4) | low
// }

// /// 编码字节数组为十六进制字符串
// #[inline]
// pub fn encode_hex(src: &[u8], dst: &mut [u8]) {
//     debug_assert!(dst.len() >= src.len() * 2);
//     for (i, &byte) in src.iter().enumerate() {
//         byte_to_hex(byte, &mut dst[i * 2..i * 2 + 2]);
//     }
// }

// /// 解码十六进制字符串为字节数组
// #[inline]
// pub fn decode_hex(src: &[u8], dst: &mut [u8]) -> Option<()> {
//     if src.len() % 2 != 0 || dst.len() < src.len() / 2 {
//         return None;
//     }

//     for i in 0..src.len() / 2 {
//         dst[i] = hex_to_byte(src[i * 2], src[i * 2 + 1])?;
//     }

//     Some(())
// }
