use parking_lot::Mutex;
use std::{collections::HashSet, sync::LazyLock};

#[derive(Default)]
struct StringPool {
    pool: HashSet<&'static str>,
}

impl StringPool {
    // 驻留字符串
    fn intern(&mut self, s: &str) -> &'static str {
        if let Some(&interned) = self.pool.get(s) {
            interned
        } else {
            // 如果字符串不存在，使用 Box::leak 将其泄漏，并添加到 pool 中
            let leaked: &'static str = Box::leak(Box::from(s));
            self.pool.insert(leaked);
            leaked
        }
    }
}

// 全局 StringPool 实例
static STRING_POOL: LazyLock<Mutex<StringPool>> =
    LazyLock::new(|| Mutex::new(StringPool::default()));

pub fn intern_string<S: AsRef<str>>(s: S) -> &'static str {
    STRING_POOL.lock().intern(s.as_ref())
}

// #[derive(Clone, Copy, PartialEq, Eq, Hash)]
// pub struct InternedString(&'static str);

// impl InternedString {
//     #[inline(always)]
//     pub fn as_str(&self) -> &'static str {
//         self.0
//     }
// }

// impl Deref for InternedString {
//     type Target = str;

//     #[inline(always)]
//     fn deref(&self) -> &'static Self::Target {
//         self.0
//     }
// }

// impl Borrow<str> for InternedString {
//     #[inline(always)]
//     fn borrow(&self) -> &'static str {
//         self.0
//     }
// }

// impl core::fmt::Debug for InternedString {
//     #[inline(always)]
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         self.0.fmt(f)
//     }
// }

// impl core::fmt::Display for InternedString {
//     #[inline(always)]
//     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
//         self.0.fmt(f)
//     }
// }

// impl serde::Serialize for InternedString {
//     #[inline]
//     fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//     where
//         S: serde::Serializer,
//     {
//         serializer.serialize_str(self.0)
//     }
// }
