use ::core::{fmt, ops::Deref};
use ::std::sync::atomic::{AtomicU64, Ordering};

// Base64 URL_SAFE_NO_PAD 编码表
const B64_ENCODE: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

// 全局缓存的时间戳头
pub static TIMESTAMP_HEADER: AtomicU64 = AtomicU64::new(0);

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct TimestampHeader(u64);

impl TimestampHeader {
    #[inline(always)]
    fn obfuscate_bytes(bytes: &mut [u8; 6]) {
        let mut prev = 165u8;

        bytes[0] = (bytes[0] ^ prev).wrapping_add(0);
        prev = bytes[0];

        bytes[1] = (bytes[1] ^ prev).wrapping_add(1);
        prev = bytes[1];

        bytes[2] = (bytes[2] ^ prev).wrapping_add(2);
        prev = bytes[2];

        bytes[3] = (bytes[3] ^ prev).wrapping_add(3);
        prev = bytes[3];

        bytes[4] = (bytes[4] ^ prev).wrapping_add(4);
        prev = bytes[4];

        bytes[5] = (bytes[5] ^ prev).wrapping_add(5);
    }

    #[inline(always)]
    const fn encode_base64(input: &[u8; 6]) -> [u8; 8] {
        let mut output = [0u8; 8];

        unsafe {
            Self::encode_chunk(
                &*(input.as_ptr() as *const _),
                &mut *(output.as_mut_ptr() as *mut _),
            );

            Self::encode_chunk(
                &*(input.as_ptr().add(3) as *const _),
                &mut *(output.as_mut_ptr().add(4) as *mut _),
            );
        }

        output
    }

    #[inline(always)]
    const fn encode_chunk(input: &[u8; 3], output: &mut [u8; 4]) {
        let b0 = input[0];
        let b1 = input[1];
        let b2 = input[2];

        output[0] = B64_ENCODE[(b0 >> 2) as usize];
        output[1] = B64_ENCODE[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize];
        output[2] = B64_ENCODE[(((b1 & 0x0F) << 2) | (b2 >> 6)) as usize];
        output[3] = B64_ENCODE[(b2 & 0x3F) as usize];
    }

    // 从千秒创建
    #[inline]
    pub fn new(kilo_seconds: u64) -> Self {
        let mut timestamp_bytes = [
            ((kilo_seconds >> 8) & 0xFF) as u8,
            (kilo_seconds & 0xFF) as u8,
            ((kilo_seconds >> 24) & 0xFF) as u8,
            ((kilo_seconds >> 16) & 0xFF) as u8,
            ((kilo_seconds >> 8) & 0xFF) as u8,
            (kilo_seconds & 0xFF) as u8,
        ];

        Self::obfuscate_bytes(&mut timestamp_bytes);
        Self(u64::from_ne_bytes(Self::encode_base64(&timestamp_bytes)))
    }

    #[inline(always)]
    pub fn as_str(&self) -> &str {
        unsafe {
            ::core::str::from_utf8_unchecked(::core::slice::from_raw_parts(
                &self.0 as *const u64 as *const u8,
                8,
            ))
        }
    }

    // 从全局原子变量获取
    #[inline]
    pub fn get_global() -> Self { Self(TIMESTAMP_HEADER.load(Ordering::Relaxed)) }

    // 使用指定千秒更新全局原子变量
    #[inline]
    pub fn update_global_with(kilo_seconds: u64) {
        TIMESTAMP_HEADER.store(Self::new(kilo_seconds).0, Ordering::Relaxed);
    }
}

impl fmt::Display for TimestampHeader {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}

impl Deref for TimestampHeader {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_str() }
}

impl AsRef<str> for TimestampHeader {
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}

// 编译时断言
const _: () = assert!(::core::mem::size_of::<TimestampHeader>() == 8);
const _: () = assert!(::core::mem::align_of::<TimestampHeader>() == 8);
