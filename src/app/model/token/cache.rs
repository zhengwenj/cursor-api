#![allow(unsafe_op_in_unsafe_fn)]

use ::core::{
    alloc::Layout,
    hash::Hasher,
    marker::PhantomData,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};
use ::hashbrown::HashMap;
use ::parking_lot::RwLock;

use super::{Randomness, RawToken, UserId};
use crate::{
    common::utils::{from_base64, to_base64},
    leak::manually_init::ManuallyInit,
};

/// Token 的唯一标识键
///
/// 由用户ID和随机数组成，用于在全局缓存中查找对应的 Token
#[derive(
    PartialEq, Eq, Hash, Clone, Copy, ::rkyv::Archive, ::rkyv::Serialize, ::rkyv::Deserialize,
)]
#[rkyv(derive(PartialEq, Eq, Hash))]
pub struct TokenKey {
    /// 用户唯一标识
    pub user_id: UserId,
    /// 随机数部分，用于保证 Token 的唯一性
    pub randomness: Randomness,
}

impl TokenKey {
    /// 将 TokenKey 序列化为 base64 字符串
    ///
    /// 格式：24字节（16字节 user_id + 8字节 randomness）编码为 32 字符的 base64
    #[allow(clippy::inherent_to_string)]
    #[inline]
    pub fn to_string(self) -> String {
        let mut bytes = [0u8; 24];
        unsafe {
            ::core::ptr::copy_nonoverlapping(
                self.user_id.to_bytes().as_ptr(),
                bytes.as_mut_ptr(),
                16,
            );
            ::core::ptr::copy_nonoverlapping(
                self.randomness.to_bytes().as_ptr(),
                bytes.as_mut_ptr().add(16),
                8,
            );
        }
        to_base64(&bytes)
    }

    /// 将 TokenKey 序列化为可读字符串
    ///
    /// 格式：`<user_id>-<randomness>`
    #[inline]
    pub fn to_string2(self) -> String {
        let mut string = String::with_capacity(60);
        string.push_str(&self.user_id.as_u128().to_string());
        string.push('-');
        string.push_str(&self.randomness.as_u64().to_string());
        string
    }

    /// 从字符串解析 TokenKey
    ///
    /// 支持两种格式：
    /// 1. 32字符的 base64 编码
    /// 2. `<user_id>-<randomness>` 格式
    pub fn from_string(s: &str) -> Option<Self> {
        let bytes = s.as_bytes();

        if bytes.len() > 60 {
            return None;
        }

        // base64 格式
        if bytes.len() == 32 {
            let decoded = from_base64(s)?;
            let user_id = UserId::from_bytes(__unwrap!(decoded.get_unchecked(..16).try_into()));
            let randomness =
                Randomness::from_bytes(__unwrap!(decoded.get_unchecked(16..24).try_into()));
            return Some(Self {
                user_id,
                randomness,
            });
        }

        // 分隔符格式
        let mut sep_pos = None;

        for (i, b) in bytes.iter().enumerate() {
            if !b.is_ascii_digit() {
                if sep_pos.is_none() {
                    sep_pos = Some(i);
                } else {
                    __cold_path!();
                    return None;
                }
            }
        }

        let sep_pos = sep_pos?;

        let first_part =
            unsafe { ::core::str::from_utf8_unchecked(bytes.get_unchecked(..sep_pos)) };
        let second_part =
            unsafe { ::core::str::from_utf8_unchecked(bytes.get_unchecked(sep_pos + 1..)) };

        let user_id_val = first_part.parse::<u128>().ok()?;
        let randomness_val = second_part.parse::<u64>().ok()?;

        Some(Self {
            user_id: UserId::from_u128(user_id_val),
            randomness: Randomness::from_u64(randomness_val),
        })
    }
}

/// Token 的内部表示
///
/// # Memory Layout
/// ```text
/// +----------------------+
/// | raw: RawToken        | 原始 token 数据
/// | count: AtomicUsize   | 引用计数
/// | string_len: usize    | 字符串长度
/// +----------------------+
/// | string data...       | UTF-8 字符串表示
/// +----------------------+
/// ```
#[repr(C)]
struct TokenInner {
    /// 原始 token 数据
    raw: RawToken,
    /// 原子引用计数
    count: AtomicUsize,
    /// 字符串表示的长度
    string_len: usize,
}

impl TokenInner {
    /// 获取字符串数据的起始地址
    #[inline(always)]
    const unsafe fn string_ptr(&self) -> *const u8 { (self as *const Self).add(1) as *const u8 }

    /// 获取字符串切片
    #[inline(always)]
    const unsafe fn as_str(&self) -> &str {
        let ptr = self.string_ptr();
        let slice = ::core::slice::from_raw_parts(ptr, self.string_len);
        ::core::str::from_utf8_unchecked(slice)
    }

    /// 计算存储指定长度字符串所需的内存布局
    fn layout_for_string(string_len: usize) -> Layout {
        Layout::new::<Self>()
            .extend(__unwrap!(Layout::array::<u8>(string_len)))
            .unwrap()
            .0
            .pad_to_align()
    }

    /// 在指定内存位置写入结构体和字符串数据
    unsafe fn write_with_string(ptr: NonNull<Self>, raw: RawToken, string: &str) {
        let inner = ptr.as_ptr();

        // 写入结构体字段
        (*inner).raw = raw;
        (*inner).count = AtomicUsize::new(1);
        (*inner).string_len = string.len();

        // 复制字符串数据
        let string_ptr = (*inner).string_ptr() as *mut u8;
        ::core::ptr::copy_nonoverlapping(string.as_ptr(), string_ptr, string.len());
    }
}

/// 引用计数的 Token，支持全局缓存复用
///
/// Token 是不可变的，线程安全的，并且会自动进行缓存管理。
/// 相同的 TokenKey 会复用同一个底层实例。
#[repr(transparent)]
pub struct Token {
    ptr: NonNull<TokenInner>,
    _marker: PhantomData<TokenInner>,
}

// Safety: Token 使用原子引用计数，可以安全地在线程间传递
unsafe impl Send for Token {}
unsafe impl Sync for Token {}

impl Clone for Token {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            let count = self.ptr.as_ref().count.fetch_add(1, Ordering::Relaxed);
            if count > isize::MAX as usize {
                __cold_path!();
                std::process::abort();
            }
        }

        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

/// 线程安全的内部指针包装
#[derive(Clone, Copy)]
#[repr(transparent)]
struct ThreadSafePtr(NonNull<TokenInner>);

unsafe impl Send for ThreadSafePtr {}
unsafe impl Sync for ThreadSafePtr {}

impl ::core::ops::Deref for ThreadSafePtr {
    type Target = NonNull<TokenInner>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

/// 全局 Token 缓存池
static TOKEN_MAP: ManuallyInit<RwLock<HashMap<TokenKey, ThreadSafePtr, ::ahash::RandomState>>> =
    ManuallyInit::new();

#[inline(always)]
pub unsafe fn __init() {
    TOKEN_MAP.init(RwLock::new(HashMap::with_capacity_and_hasher(
        64,
        ::ahash::RandomState::new(),
    )))
}

impl Token {
    /// 创建或复用 Token 实例
    ///
    /// 如果缓存中已存在相同的 TokenKey 且 RawToken 相同，则复用；
    /// 否则创建新实例（可能会覆盖旧的）。
    pub fn new(raw: RawToken, string: Option<String>) -> Self {
        let key = raw.key();

        // 快速路径：尝试从缓存中查找
        {
            let cache = TOKEN_MAP.read();
            if let Some(&ThreadSafePtr(ptr)) = cache.get(&key) {
                unsafe {
                    let inner = ptr.as_ref();
                    if inner.raw == raw {
                        let count = inner.count.fetch_add(1, Ordering::Relaxed);
                        if count > isize::MAX as usize {
                            __cold_path!();
                            std::process::abort();
                        }
                        return Self {
                            ptr,
                            _marker: PhantomData,
                        };
                    }
                }
            }
        }

        // 慢速路径：需要创建新实例
        let mut cache = TOKEN_MAP.write();

        // 双重检查，防止竞态条件
        if let Some(&ThreadSafePtr(ptr)) = cache.get(&key) {
            unsafe {
                let inner = ptr.as_ref();
                if inner.raw == raw {
                    let count = inner.count.fetch_add(1, Ordering::Relaxed);
                    if count > isize::MAX as usize {
                        __cold_path!();
                        std::process::abort();
                    }
                    return Self {
                        ptr,
                        _marker: PhantomData,
                    };
                }
            }
        }

        // 准备字符串表示
        let string = string.unwrap_or_else(|| raw.to_string());
        let layout = TokenInner::layout_for_string(string.len());

        // 分配并初始化新实例
        let ptr = unsafe {
            let alloc = ::std::alloc::alloc(layout) as *mut TokenInner;
            if alloc.is_null() {
                __cold_path!();
                ::std::alloc::handle_alloc_error(layout);
            }
            let ptr = NonNull::new_unchecked(alloc);
            TokenInner::write_with_string(ptr, raw, &string);
            ptr
        };

        cache.insert(key, ThreadSafePtr(ptr));

        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// 获取原始 token 数据
    #[inline(always)]
    pub const fn raw(&self) -> &RawToken { unsafe { &self.ptr.as_ref().raw } }

    /// 获取字符串表示
    #[inline(always)]
    pub const fn as_str(&self) -> &str { unsafe { self.ptr.as_ref().as_str() } }

    /// 获取 token 的键
    #[inline(always)]
    pub const fn key(&self) -> TokenKey { self.raw().key() }

    /// 检查是否为网页 token
    #[inline(always)]
    pub const fn is_web(&self) -> bool { self.raw().is_web() }

    /// 检查是否为会话 token
    #[inline(always)]
    pub const fn is_session(&self) -> bool { self.raw().is_session() }
}

impl Drop for Token {
    fn drop(&mut self) {
        unsafe {
            let inner = self.ptr.as_ref();

            // 递减引用计数，如果不是最后一个引用则直接返回
            if inner.count.fetch_sub(1, Ordering::Release) != 1 {
                return;
            }

            // 确保其他线程的所有操作都已完成
            ::core::sync::atomic::fence(Ordering::Acquire);

            // 从缓存中移除并释放内存
            let key = inner.raw.key();
            TOKEN_MAP.write().remove(&key);

            let layout = TokenInner::layout_for_string(inner.string_len);
            ::std::alloc::dealloc(self.ptr.cast().as_ptr(), layout);
        }
    }
}

// ===== Trait 实现 =====

impl PartialEq for Token {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        unsafe { self.ptr.as_ref().raw == other.ptr.as_ref().raw }
    }
}

impl Eq for Token {}

impl ::core::hash::Hash for Token {
    #[inline(always)]
    fn hash<H: Hasher>(&self, state: &mut H) { self.key().hash(state); }
}

impl ::core::fmt::Display for Token {
    #[inline(always)]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(self.as_str())
    }
}

// ===== Serde 实现 =====

mod serde_impls {
    use super::*;
    use ::serde::{Deserialize, Deserializer, Serialize, Serializer};

    impl Serialize for Token {
        #[inline]
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            self.as_str().serialize(serializer)
        }
    }

    impl<'de> Deserialize<'de> for Token {
        #[inline]
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            let raw_token = s.parse().map_err(::serde::de::Error::custom)?;
            Ok(Token::new(raw_token, Some(s)))
        }
    }
}
