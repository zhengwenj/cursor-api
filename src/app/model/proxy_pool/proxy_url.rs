use rkyv::{Archive, Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

/// 一个可以被Archive的URL包装器
#[derive(Clone, Archive, Deserialize, Serialize)]
#[archive(compare(PartialEq))]
pub struct UrlWrapper(String);

impl UrlWrapper {
    pub fn new(url: &url::Url) -> Self {
        Self(url.to_string())
    }

    pub fn into_url(self) -> Result<url::Url, url::ParseError> {
        url::Url::parse(&self.0)
    }

    pub fn as_url(&self) -> Result<url::Url, url::ParseError> {
        url::Url::parse(&self.0)
    }
}

impl From<url::Url> for UrlWrapper {
    fn from(url: url::Url) -> Self {
        Self(url.to_string())
    }
}

impl TryFrom<UrlWrapper> for url::Url {
    type Error = url::ParseError;

    fn try_from(wrapper: UrlWrapper) -> Result<Self, Self::Error> {
        wrapper.into_url()
    }
}

impl fmt::Display for UrlWrapper {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for UrlWrapper {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 验证字符串是有效的URL
        url::Url::parse(s)?;
        Ok(Self(s.to_string()))
    }
}

impl PartialEq for UrlWrapper {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Eq for UrlWrapper {}
