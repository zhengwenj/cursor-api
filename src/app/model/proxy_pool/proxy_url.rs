use reqwest::Proxy;
use rkyv::{Archive, Deserialize, Serialize};
use std::{fmt, str::FromStr};

/// 一个可以被Archive的字符串化URL
#[derive(Clone, Archive, Deserialize, Serialize)]
#[rkyv(compare(PartialEq))]
#[repr(transparent)]
pub struct ProxyUrl(String);

impl ProxyUrl {
    #[inline]
    pub fn to_proxy(&self) -> Proxy { Proxy::all(&*self.0).expect("创建代理对象失败") }
}

impl From<ProxyUrl> for Proxy {
    fn from(url: ProxyUrl) -> Self { url.to_proxy() }
}

impl fmt::Display for ProxyUrl {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
}

impl FromStr for ProxyUrl {
    type Err = reqwest::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Proxy::all(s)?;
        Ok(Self(s.to_string()))
    }
}

impl PartialEq for ProxyUrl {
    #[inline]
    fn eq(&self, other: &Self) -> bool { self.0 == other.0 }
}

impl Eq for ProxyUrl {}

impl core::hash::Hash for ProxyUrl {
    #[inline]
    fn hash<H: core::hash::Hasher>(&self, state: &mut H) { self.0.hash(state); }
}
