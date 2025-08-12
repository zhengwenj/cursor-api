#![allow(unsafe_op_in_unsafe_fn)]

use ::core::{
    alloc::Layout,
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem::SizedTypeProperties as _,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};
use ::hashbrown::{Equivalent, HashSet};
use ::parking_lot::RwLock;

use super::manually_init::ManuallyInit;

/// 字符串内容的内部表示
///
/// # Memory Layout
/// ```text
/// +----------------+
/// | count: usize   |  引用计数
/// | string_len: usize | 字符串长度
/// +----------------+
/// | string data... |  UTF-8 字符串数据
/// +----------------+
/// ```
struct ArcStrInner {
    /// 原子引用计数
    count: AtomicUsize,
    /// 字符串的字节长度
    string_len: usize,
}

impl ArcStrInner {
    const MAX_LEN: usize = {
        let layout = Self::LAYOUT;
        isize::MAX as usize + 1 - layout.align() - layout.size()
    };

    /// 获取字符串数据的起始地址
    ///
    /// # Safety
    /// 调用者必须确保 self 是有效的指针
    #[inline(always)]
    const unsafe fn string_ptr(&self) -> *const u8 { (self as *const Self).add(1) as *const u8 }

    /// 获取字符串切片引用
    ///
    /// # Safety
    /// - self 必须是有效的指针
    /// - 字符串数据必须是有效的 UTF-8
    /// - string_len 必须正确反映实际字符串长度
    #[inline(always)]
    const unsafe fn as_str(&self) -> &str {
        let ptr = self.string_ptr();
        let slice = ::core::slice::from_raw_parts(ptr, self.string_len);
        ::core::str::from_utf8_unchecked(slice)
    }

    /// 计算存储指定长度字符串所需的内存布局
    fn layout_for_string(string_len: usize) -> Layout {
        if string_len > Self::MAX_LEN {
            __cold_path!();
            panic!("string is too long");
        }
        unsafe {
            Layout::new::<Self>()
                .extend(Layout::array::<u8>(string_len).unwrap_unchecked())
                .unwrap_unchecked()
                .0
                .pad_to_align()
        }
    }

    /// 在指定内存位置写入结构体和字符串数据
    ///
    /// # Safety
    /// - ptr 必须指向足够大的已分配内存
    /// - 内存必须正确对齐
    /// - string 必须是有效的 UTF-8 字符串
    unsafe fn write_with_string(ptr: NonNull<Self>, string: &str) {
        let inner = ptr.as_ptr();

        // 初始化结构体
        ::core::ptr::write(inner, Self {
            count: AtomicUsize::new(1),
            string_len: string.len(),
        });

        // 复制字符串数据到紧跟结构体后的内存
        let string_ptr = (*inner).string_ptr() as *mut u8;
        ::core::ptr::copy_nonoverlapping(string.as_ptr(), string_ptr, string.len());
    }
}

/// 引用计数的不可变字符串，支持全局字符串池复用
///
/// # Examples
/// ```
/// let s1 = ArcStr::new("hello");
/// let s2 = ArcStr::new("hello");
/// assert!(std::ptr::eq(s1.as_str(), s2.as_str())); // 复用相同内容
/// ```
#[repr(transparent)]
pub struct ArcStr {
    ptr: NonNull<ArcStrInner>,
    _marker: PhantomData<ArcStrInner>,
}

// Safety: ArcStr 使用原子引用计数，可以安全地在线程间传递
unsafe impl Send for ArcStr {}
unsafe impl Sync for ArcStr {}

impl Clone for ArcStr {
    #[inline]
    fn clone(&self) -> Self {
        // Safety: ptr 始终指向有效的 ArcStrInner
        let count = unsafe { self.ptr.as_ref().count.fetch_add(1, Ordering::Relaxed) };

        // 防止引用计数溢出
        if count > isize::MAX as usize {
            __cold_path!();
            std::process::abort();
        }

        Self {
            ptr: self.ptr,
            _marker: PhantomData,
        }
    }
}

/// 线程安全的内部指针包装，用于在 HashSet 中作为键
#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
struct ThreadSafePtr(NonNull<ArcStrInner>);

// Safety: ThreadSafePtr 只是指针的包装，本身是 POD 类型
unsafe impl Send for ThreadSafePtr {}
unsafe impl Sync for ThreadSafePtr {}

impl ::core::ops::Deref for ThreadSafePtr {
    type Target = NonNull<ArcStrInner>;

    #[inline(always)]
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl Hash for ThreadSafePtr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Safety: 通过 ThreadSafePtr 保证指针有效性
        unsafe {
            let inner = self.0.as_ref();
            state.write_str(inner.as_str());
        }
    }
}

impl PartialEq for ThreadSafePtr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.0 == other.0 {
            return true;
        }

        // Safety: ThreadSafePtr 保证指针有效
        unsafe {
            let self_inner = self.0.as_ref();
            let other_inner = other.0.as_ref();
            self_inner.string_len == other_inner.string_len
                && self_inner.as_str() == other_inner.as_str()
        }
    }
}

impl Eq for ThreadSafePtr {}

// 实现 str 与 ThreadSafePtr 的比较，用于 HashSet::get 操作
impl PartialEq<ThreadSafePtr> for str {
    #[inline]
    fn eq(&self, other: &ThreadSafePtr) -> bool {
        // Safety: ThreadSafePtr 保证指针有效
        unsafe {
            let inner = other.0.as_ref();
            inner.string_len == self.len() && inner.as_str() == self
        }
    }
}

impl Equivalent<ThreadSafePtr> for str {
    #[inline]
    fn equivalent(&self, key: &ThreadSafePtr) -> bool { self.eq(key) }
}

/// 全局字符串池，用于复用相同内容的字符串
///
/// 使用 ahash 提供更好的哈希性能
static ARC_STR_POOL: ManuallyInit<RwLock<HashSet<ThreadSafePtr, ::ahash::RandomState>>> =
    ManuallyInit::new();

#[inline(always)]
pub(super) unsafe fn __init() {
    ARC_STR_POOL.init(RwLock::new(HashSet::with_capacity_and_hasher(
        128,
        ::ahash::RandomState::new(),
    )))
}

impl ArcStr {
    /// 创建或复用字符串实例
    ///
    /// 如果池中已存在相同内容的字符串，则增加其引用计数并返回；
    /// 否则创建新实例并加入池中。
    pub fn new<S: AsRef<str>>(s: S) -> Self {
        let string = s.as_ref();

        // 快速路径：尝试从池中查找
        {
            let pool = ARC_STR_POOL.read();
            if let Some(ptr_ref) = pool.get(string) {
                let ptr = ptr_ref.0;
                // Safety: 池中的指针始终有效
                unsafe {
                    let count = ptr.as_ref().count.fetch_add(1, Ordering::Relaxed);
                    if count > isize::MAX as usize {
                        __cold_path!();
                        std::process::abort();
                    }
                }
                return Self {
                    ptr,
                    _marker: PhantomData,
                };
            }
        }

        // 慢速路径：创建新实例
        let mut pool = ARC_STR_POOL.write();

        // 双重检查，防止竞态条件
        if let Some(ptr_ref) = pool.get(string) {
            let ptr = ptr_ref.0;
            // Safety: 池中的指针始终有效
            unsafe {
                let count = ptr.as_ref().count.fetch_add(1, Ordering::Relaxed);
                if count > isize::MAX as usize {
                    __cold_path!();
                    std::process::abort();
                }
            }
            return Self {
                ptr,
                _marker: PhantomData,
            };
        }

        // 分配并初始化新实例
        let layout = ArcStrInner::layout_for_string(string.len());
        let ptr = unsafe {
            let alloc = ::std::alloc::alloc(layout) as *mut ArcStrInner;
            if alloc.is_null() {
                __cold_path!();
                ::std::alloc::handle_alloc_error(layout);
            }
            let ptr = NonNull::new_unchecked(alloc);
            ArcStrInner::write_with_string(ptr, string);
            ptr
        };

        pool.insert(ThreadSafePtr(ptr));

        Self {
            ptr,
            _marker: PhantomData,
        }
    }

    /// 获取字符串切片
    #[inline(always)]
    pub fn as_str(&self) -> &str {
        // Safety: ptr 始终指向有效的 ArcStrInner
        unsafe { self.ptr.as_ref().as_str() }
    }

    /// 获取字符串长度（字节数）
    #[inline(always)]
    pub fn len(&self) -> usize {
        // Safety: ptr 始终指向有效的 ArcStrInner
        unsafe { self.ptr.as_ref().string_len }
    }

    /// 检查字符串是否为空
    #[inline(always)]
    pub fn is_empty(&self) -> bool { self.len() == 0 }

    /// 获取当前引用计数
    ///
    /// 主要用于调试和测试
    #[inline(always)]
    pub fn ref_count(&self) -> usize {
        // Safety: ptr 始终指向有效的 ArcStrInner
        unsafe { self.ptr.as_ref().count.load(Ordering::Relaxed) }
    }
}

impl Drop for ArcStr {
    fn drop(&mut self) {
        // Safety: ptr 始终指向有效的 ArcStrInner
        unsafe {
            let inner = self.ptr.as_ref();

            // 递减引用计数，如果不是最后一个引用则直接返回
            if inner.count.fetch_sub(1, Ordering::Release) != 1 {
                return;
            }

            // 确保其他线程的所有操作都已完成
            ::core::sync::atomic::fence(Ordering::Acquire);

            // 从池中移除并释放内存
            let mut pool = ARC_STR_POOL.write();
            pool.remove(&ThreadSafePtr(self.ptr));

            let layout = ArcStrInner::layout_for_string(inner.string_len);
            ::std::alloc::dealloc(self.ptr.cast().as_ptr(), layout);
        }
    }
}

// ===== Trait 实现 =====

impl PartialEq for ArcStr {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // 快速路径：指针相同
        if self.ptr == other.ptr {
            return true;
        }

        // Safety: 两个指针都有效
        unsafe {
            let self_inner = self.ptr.as_ref();
            let other_inner = other.ptr.as_ref();
            self_inner.string_len == other_inner.string_len
                && self_inner.as_str() == other_inner.as_str()
        }
    }
}

impl Eq for ArcStr {}

impl Hash for ArcStr {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) { state.write_str(self.as_str()); }
}

impl ::core::fmt::Display for ArcStr {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl ::core::fmt::Debug for ArcStr {
    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        f.debug_struct("ArcStr")
            .field("ptr", &self.ptr)
            .field("content", &self.as_str())
            .field("len", &self.len())
            .field("ref_count", &self.ref_count())
            .finish()
    }
}

impl AsRef<str> for ArcStr {
    #[inline]
    fn as_ref(&self) -> &str { self.as_str() }
}

impl ::core::ops::Deref for ArcStr {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target { self.as_str() }
}

// ===== 与其他字符串类型的相等性比较 =====

impl PartialEq<str> for ArcStr {
    #[inline]
    fn eq(&self, other: &str) -> bool { self.as_str() == other }
}

impl PartialEq<&str> for ArcStr {
    #[inline]
    fn eq(&self, other: &&str) -> bool { self.as_str() == *other }
}

impl PartialEq<ArcStr> for str {
    #[inline]
    fn eq(&self, other: &ArcStr) -> bool { self == other.as_str() }
}

impl PartialEq<ArcStr> for &str {
    #[inline]
    fn eq(&self, other: &ArcStr) -> bool { *self == other.as_str() }
}

impl PartialEq<String> for ArcStr {
    #[inline]
    fn eq(&self, other: &String) -> bool { self.as_str() == other.as_str() }
}

impl PartialEq<ArcStr> for String {
    #[inline]
    fn eq(&self, other: &ArcStr) -> bool { self.as_str() == other.as_str() }
}

// ===== From 转换实现 =====

impl<'a> From<&'a str> for ArcStr {
    #[inline]
    fn from(s: &'a str) -> Self { Self::new(s) }
}

impl From<String> for ArcStr {
    #[inline]
    fn from(s: String) -> Self { Self::new(s.as_str()) }
}

impl<'a> From<std::borrow::Cow<'a, str>> for ArcStr {
    #[inline]
    fn from(cow: std::borrow::Cow<'a, str>) -> Self { Self::new(cow.as_ref()) }
}

// ===== 测试辅助函数 =====

#[cfg(test)]
/// 获取字符串池的统计信息
pub fn pool_stats() -> (usize, usize) {
    let pool = ARC_STR_POOL.read();
    (pool.len(), pool.capacity())
}

#[cfg(test)]
/// 清空字符串池（仅用于测试）
pub fn clear_pool_for_test() {
    // 等待可能的并发操作完成
    std::thread::sleep(std::time::Duration::from_millis(10));
    ARC_STR_POOL.write().clear();
}

#[cfg(test)]
mod tests {
    use std::{thread, time::Duration};

    use super::*;

    /// 运行隔离的测试，确保池状态不会相互影响
    fn run_isolated_test<F: FnOnce()>(f: F) {
        clear_pool_for_test();
        f();
        clear_pool_for_test();
    }

    #[test]
    fn test_basic_functionality() {
        run_isolated_test(|| {
            let s1 = ArcStr::new("hello");
            let s2 = ArcStr::new("hello");
            let s3 = ArcStr::new("world");

            assert_eq!(s1, s2);
            assert_ne!(s1, s3);
            assert_eq!(s1.ptr, s2.ptr);
            assert_ne!(s1.ptr, s3.ptr);
            assert_eq!(s1.as_str(), "hello");
            assert_eq!(s1.len(), 5);
            assert!(!s1.is_empty());

            let (count, _) = pool_stats();
            assert_eq!(count, 2);
        });
    }

    #[test]
    fn test_reference_counting() {
        run_isolated_test(|| {
            let s1 = ArcStr::new("test");
            assert_eq!(s1.ref_count(), 1);
            assert_eq!(pool_stats().0, 1);

            let s2 = s1.clone();
            assert_eq!(s1.ref_count(), 2);
            assert_eq!(s2.ref_count(), 2);
            assert_eq!(s1.ptr, s2.ptr);
            assert_eq!(pool_stats().0, 1);

            drop(s2);
            assert_eq!(s1.ref_count(), 1);
            assert_eq!(pool_stats().0, 1);

            drop(s1);
            thread::sleep(Duration::from_millis(1));
            assert_eq!(pool_stats().0, 0);
        });
    }

    #[test]
    fn test_pool_reuse() {
        run_isolated_test(|| {
            let s1 = ArcStr::new("reuse_test");
            let s2 = ArcStr::new("reuse_test");

            assert_eq!(s1.ptr, s2.ptr);
            assert_eq!(s1.ref_count(), 2);
            assert_eq!(pool_stats().0, 1);
        });
    }

    #[test]
    fn test_automatic_cleanup() {
        run_isolated_test(|| {
            assert_eq!(pool_stats().0, 0);

            {
                let s1 = ArcStr::new("cleanup_test");
                assert_eq!(pool_stats().0, 1);

                let _s2 = ArcStr::new("cleanup_test");
                assert_eq!(pool_stats().0, 1);
                assert_eq!(s1.ref_count(), 2);
            }

            thread::sleep(Duration::from_millis(5));
            let (count, _) = pool_stats();
            assert_eq!(count, 0);
        });
    }

    #[test]
    fn test_from_implementations() {
        run_isolated_test(|| {
            use std::borrow::Cow;

            let s1 = ArcStr::from("from_str");
            let s2 = ArcStr::from(String::from("from_string"));
            let s3 = ArcStr::from(Cow::Borrowed("from_cow"));
            let s4 = ArcStr::from(Cow::Owned::<str>(String::from("from_cow_owned")));

            assert_eq!(s1.as_str(), "from_str");
            assert_eq!(s2.as_str(), "from_string");
            assert_eq!(s3.as_str(), "from_cow");
            assert_eq!(s4.as_str(), "from_cow_owned");
            assert_eq!(pool_stats().0, 4);
        });
    }

    #[test]
    fn test_equality_operations() {
        run_isolated_test(|| {
            let arc_str = ArcStr::new("test");
            let arc_str2 = ArcStr::new("test");
            let arc_str3 = ArcStr::new("test3");

            assert_eq!(arc_str, arc_str2);
            assert_ne!(arc_str, arc_str3);
            assert_eq!(arc_str, "test");
            assert_eq!("test", arc_str);
            assert_eq!(arc_str, String::from("test"));
            assert_eq!(String::from("test"), arc_str);
        });
    }

    #[test]
    fn test_empty_string() {
        run_isolated_test(|| {
            let empty = ArcStr::new("");
            assert!(empty.is_empty());
            assert_eq!(empty.len(), 0);
            assert_eq!(empty.as_str(), "");
        });
    }

    #[test]
    fn test_thread_safety() {
        run_isolated_test(|| {
            use std::sync::Arc;

            let s = Arc::new(ArcStr::new("shared"));
            let handles: Vec<_> = (0..10)
                .map(|_| {
                    let s_clone = Arc::clone(&s);
                    thread::spawn(move || {
                        let local = ArcStr::new("shared");
                        assert_eq!(*s_clone, local);
                        assert_eq!(s_clone.ptr, local.ptr);
                    })
                })
                .collect();

            for handle in handles {
                handle.join().unwrap();
            }
        });
    }
}
