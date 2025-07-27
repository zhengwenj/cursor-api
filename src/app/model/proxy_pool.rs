use crate::app::lazy::{PROXIES_FILE_PATH, SERVICE_TIMEOUT, TCP_KEEPALIVE};
use ahash::{HashMap, HashSet};
use arc_swap::{ArcSwap, ArcSwapAny};
use memmap2::{MmapMut, MmapOptions};
use reqwest::Client;
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
use std::{
    str::FromStr,
    sync::{Arc, OnceLock},
    time::Duration,
};
use tokio::fs::OpenOptions;
mod proxy_url;
use proxy_url::ProxyUrl;

// 代理值常量
const NON_PROXY: &str = "non";
const SYS_PROXY: &str = "sys";

// 代理相关错误消息常量
const PROXY_NOT_FOUND_IN_LIST: &str = "General proxy not found in proxy list";
const PROXY_CLIENT_NOT_FOUND_IN_POOL: &str = "Client for general proxy not found in client pool";

#[inline]
pub fn default_proxies() -> HashMap<String, SingleProxy> {
    HashMap::from_iter([(SYS_PROXY.to_string(), SingleProxy::Sys)])
}

/// 名称到代理配置的映射
static PROXIES: OnceLock<ArcSwap<HashMap<String, SingleProxy>>> = OnceLock::new();

/// 通用名称
static GENERAL_NAME: OnceLock<ArcSwap<String>> = OnceLock::new();

/// 代理配置到客户端实例的映射
///
/// 避免重复创建相同配置的客户端
static CLIENTS: OnceLock<ArcSwap<HashMap<SingleProxy, Client>>> = OnceLock::new();

/// 通用客户端
///
/// 用于未指定特定代理的请求
static GENERAL_CLIENT: OnceLock<ArcSwapAny<Client>> = OnceLock::new();

#[derive(Clone, Deserialize, Serialize, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct Proxies {
    /// 名称到代理配置的映射
    proxies: HashMap<String, SingleProxy>,
    /// 通用名称
    general: String,
}

impl Default for Proxies {
    #[inline]
    fn default() -> Self {
        Self {
            proxies: HashMap::from_iter([(SYS_PROXY.to_string(), SingleProxy::Sys)]),
            general: SYS_PROXY.to_string(),
        }
    }
}

impl Proxies {
    #[inline]
    pub fn init(mut self) {
        if self.proxies.is_empty() {
            self.proxies = HashMap::from_iter([(SYS_PROXY.to_string(), SingleProxy::Sys)]);
            if self.general.as_str() != SYS_PROXY {
                self.general = SYS_PROXY.to_string();
            }
        } else if !self.proxies.contains_key(&self.general) {
            self.general = __unwrap!(self.proxies.keys().next()).clone();
        }
        let proxies = self.proxies.values().collect::<HashSet<_>>();
        let mut clients =
            HashMap::with_capacity_and_hasher(proxies.len(), ::ahash::RandomState::new());
        for proxy in proxies {
            proxy.insert_to(&mut clients);
        }
        let _ = GENERAL_CLIENT.set(ArcSwapAny::from(
            clients
                .get(
                    self.proxies
                        .get(&self.general)
                        .expect(PROXY_NOT_FOUND_IN_LIST),
                )
                .expect(PROXY_CLIENT_NOT_FOUND_IN_POOL)
                .clone(),
        ));
        let _ = CLIENTS.set(ArcSwap::from_pointee(clients));
        let _ = PROXIES.set(ArcSwap::from_pointee(self.proxies));
        let _ = GENERAL_NAME.set(ArcSwap::from_pointee(self.general));
    }

    #[inline]
    pub fn update_global(self) {
        proxies().store(Arc::new(self.proxies));
        general_name().store(Arc::new(self.general));
    }

    // 更新全局代理池
    fn update_global_pool() -> Result<(), Box<dyn std::error::Error>> {
        let proxies = proxies().load();
        let mut general_name = general_name().load_full();
        let mut clients = (*clients().load_full()).clone();

        // 确保self.proxies至少有系统代理，且general有效
        if proxies.is_empty() {
            self::proxies().store(Arc::new(HashMap::from_iter([(
                SYS_PROXY.to_string(),
                SingleProxy::Sys,
            )])));
            if general_name.as_str() != SYS_PROXY {
                general_name = Arc::new(SYS_PROXY.to_string());
            }
        } else if !proxies.contains_key(&*general_name) {
            // general指向的代理不存在，更新为某个存在的代理
            general_name = Arc::new(__unwrap!(proxies.keys().next()).clone());
        }

        // 1. 收集当前配置中的所有唯一代理
        let current_proxies: HashSet<&SingleProxy> = proxies.values().collect();

        // 2. 首先移除不再使用的客户端
        let to_remove: Vec<SingleProxy> = clients
            .keys()
            .filter(|proxy| !current_proxies.contains(proxy))
            .cloned()
            .collect();

        for proxy in to_remove {
            clients.remove(&proxy);
        }

        // 3. 然后为新的代理配置创建客户端
        for proxy in current_proxies {
            if !clients.contains_key(proxy) {
                // 创建新的客户端
                proxy.insert_to(&mut clients);
            }
        }

        self::clients().store(Arc::new(clients));

        // 4. 设置通用名称
        self::general_name().store(general_name);

        // 5. 设置通用客户端
        set_general();

        Ok(())
    }

    pub async fn save() -> Result<(), Box<dyn std::error::Error>> {
        let bytes = ::rkyv::to_bytes::<::rkyv::rancor::Error>(&Self {
            proxies: (*proxies().load_full()).clone(),
            general: (*general_name().load_full()).clone(),
        })?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&*PROXIES_FILE_PATH)
            .await?;

        if bytes.len() > usize::MAX >> 1 {
            return Err("代理数据过大".into());
        }

        file.set_len(bytes.len() as u64).await?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        mmap.copy_from_slice(&bytes);
        mmap.flush()?;

        Ok(())
    }

    pub async fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let file = match OpenOptions::new()
            .read(true)
            .open(&*PROXIES_FILE_PATH)
            .await
        {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::default());
            }
            Err(e) => return Err(Box::new(e)),
        };

        if file.metadata().await?.len() > usize::MAX as u64 {
            return Err("代理文件过大".into());
        }

        let mmap = unsafe { MmapOptions::new().map(&file)? };
        unsafe {
            ::rkyv::from_bytes_unchecked::<Self, ::rkyv::rancor::Error>(&mmap).map_err(Into::into)
        }
    }

    // 更新全局代理池并保存配置
    #[inline]
    pub async fn update_and_save() -> Result<(), Box<dyn std::error::Error>> {
        // 更新全局代理池
        Self::update_global_pool()?;

        // 保存配置到文件
        Self::save().await
    }
}

#[derive(Clone, Archive, RkyvDeserialize, RkyvSerialize, PartialEq, Eq, Hash)]
#[rkyv(compare(PartialEq))]
pub enum SingleProxy {
    Non,
    Sys,
    Url(ProxyUrl),
}

impl SingleProxy {
    #[inline]
    fn insert_to(&self, clients: &mut HashMap<SingleProxy, Client>) {
        // 创建新的客户端
        match self {
            SingleProxy::Non => {
                clients.insert(
                    SingleProxy::Non,
                    Client::builder()
                        .https_only(true)
                        .tcp_keepalive(Duration::from_secs(*TCP_KEEPALIVE))
                        .connect_timeout(Duration::from_secs(*SERVICE_TIMEOUT))
                        .no_proxy()
                        .build()
                        .expect("创建无代理客户端失败"),
                );
            }
            SingleProxy::Sys => {
                clients.insert(
                    SingleProxy::Sys,
                    Client::builder()
                        .https_only(true)
                        .tcp_keepalive(Duration::from_secs(*TCP_KEEPALIVE))
                        .connect_timeout(Duration::from_secs(*SERVICE_TIMEOUT))
                        .build()
                        .expect("创建默认客户端失败"),
                );
            }
            SingleProxy::Url(url) => {
                clients.insert(
                    (*self).clone(),
                    Client::builder()
                        .https_only(true)
                        .tcp_keepalive(Duration::from_secs(*TCP_KEEPALIVE))
                        .connect_timeout(Duration::from_secs(*SERVICE_TIMEOUT))
                        .proxy(url.to_proxy())
                        .build()
                        .expect("创建代理客户端失败"),
                );
            }
        }
    }
}

impl Serialize for SingleProxy {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Non => serializer.serialize_str(NON_PROXY),
            Self::Sys => serializer.serialize_str(SYS_PROXY),
            Self::Url(url) => serializer.serialize_str(&url.to_string()),
        }
    }
}

impl<'de> Deserialize<'de> for SingleProxy {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct SingleProxyVisitor;

        impl serde::de::Visitor<'_> for SingleProxyVisitor {
            type Value = SingleProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing 'non', 'sys', or a valid URL")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    NON_PROXY => Ok(Self::Value::Non),
                    SYS_PROXY => Ok(Self::Value::Sys),
                    url_str => Ok(Self::Value::Url(
                        ProxyUrl::from_str(url_str)
                            .map_err(|e| E::custom(format_args!("Invalid URL: {e}")))?,
                    )),
                }
            }
        }

        deserializer.deserialize_str(SingleProxyVisitor)
    }
}

impl std::fmt::Display for SingleProxy {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Non => write!(f, "{NON_PROXY}"),
            Self::Sys => write!(f, "{SYS_PROXY}"),
            Self::Url(url) => write!(f, "{url}"),
        }
    }
}

impl FromStr for SingleProxy {
    type Err = reqwest::Error;

    #[inline]
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            NON_PROXY => Ok(Self::Non),
            SYS_PROXY => Ok(Self::Sys),
            url_str => Ok(Self::Url(ProxyUrl::from_str(url_str)?)),
        }
    }
}

// 获取客户端
#[inline]
pub fn get_client(url: &str) -> Client {
    // 先通过名称查找代理配置
    if let Some(proxy) = proxies().load().get(url) {
        // 然后通过代理配置查找客户端
        if let Some(client) = clients().load().get(proxy) {
            return client.clone();
        }
    }

    // 返回通用客户端或默认客户端
    get_general_client()
}

// 获取通用客户端
#[inline]
pub fn get_general_client() -> Client { general_client().load_full() }

// 获取客户端或通用客户端
#[inline]
pub fn get_client_or_general(url: Option<&str>) -> Client {
    match url {
        Some(url) => get_client(url),
        None => get_general_client(),
    }
}

/// 设置通用客户端
#[inline]
fn set_general() {
    general_client().store(
        clients()
            .load()
            .get(
                proxies()
                    .load()
                    .get(&*general_name().load_full())
                    .expect(PROXY_NOT_FOUND_IN_LIST),
            )
            .expect(PROXY_CLIENT_NOT_FOUND_IN_POOL)
            .clone(),
    );
}

#[inline]
pub fn proxies() -> &'static ArcSwap<HashMap<String, SingleProxy>> {
    PROXIES.get().expect("proxies does not init")
}

#[inline]
pub fn general_name() -> &'static ArcSwap<String> {
    GENERAL_NAME.get().expect("general_name does not init")
}

#[inline]
fn clients() -> &'static ArcSwap<HashMap<SingleProxy, Client>> {
    CLIENTS.get().expect("clients does not init")
}

#[inline]
fn general_client() -> &'static ArcSwapAny<Client> {
    GENERAL_CLIENT.get().expect("general_client does not init")
}
