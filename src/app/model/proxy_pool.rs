use memmap2::{MmapMut, MmapOptions};
use parking_lot::RwLock;
use reqwest::{Client, Proxy};
use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::OpenOptions;
use std::str::FromStr;
use std::sync::LazyLock;

mod proxy_url;
use super::super::lazy::PROXIES_FILE_PATH;
use proxy_url::UrlWrapper;

// 恢复原来的常量定义
pub const NO_PROXY: &str = "no";
pub const EMPTY_PROXY: &str = "";
pub const SYSTEM_PROXY: &str = "system";
pub const DEFAULT_PROXY: &str = "default";

// 新的代理值常量
pub const NON_PROXY: &str = "non";
pub const SYS_PROXY: &str = "sys";

// 静态映射，将原来的值映射到新的值
pub static PROXY_MAP: LazyLock<HashMap<&'static str, &'static str>> = LazyLock::new(|| {
    let mut map = HashMap::new();
    map.insert(NO_PROXY, NON_PROXY);
    map.insert(EMPTY_PROXY, NON_PROXY); // 空字符串也映射到NON_PROXY
    map.insert(SYSTEM_PROXY, SYS_PROXY);
    map.insert(DEFAULT_PROXY, SYS_PROXY); // DEFAULT_PROXY映射到SYS_PROXY
    map
});

// 直接初始化PROXY_POOL为一个带有系统代理的基本实例
pub static PROXY_POOL: LazyLock<RwLock<ProxyPool>> = LazyLock::new(|| {
    let mut clients = HashMap::new();

    // 添加系统代理
    let system_client = Client::new();
    clients.insert(SYS_PROXY.to_string(), system_client.clone());

    RwLock::new(ProxyPool {
        clients,
        general: Some(system_client),
    })
});

#[derive(Clone, Deserialize, Serialize, Archive, RkyvDeserialize, RkyvSerialize)]
pub struct Proxies {
    // name to proxy
    proxies: HashMap<String, SingleProxy>,
    general: String,
}

impl Default for Proxies {
    fn default() -> Self {
        Self::new()
    }
}

impl Proxies {
    pub fn new() -> Self {
        Self {
            proxies: HashMap::from([(SYS_PROXY.to_string(), SingleProxy::Sys)]),
            general: SYS_PROXY.to_string(),
        }
    }

    pub fn get_proxies(&self) -> &HashMap<String, SingleProxy> {
        &self.proxies
    }

    pub fn add_proxy(&mut self, name: String, proxy: SingleProxy) {
        self.proxies.insert(name, proxy);
    }

    pub fn remove_proxy(&mut self, name: &str) {
        self.proxies.remove(name);
    }

    pub fn set_general(&mut self, name: &str) {
        if self.proxies.contains_key(name) {
            self.general = name.to_string();
        }
    }

    pub fn get_general(&self) -> &str {
        &self.general
    }

    // 更新全局代理池
    pub fn update_global_pool(&self) -> Result<(), Box<dyn std::error::Error>> {
        let mut pool = PROXY_POOL.write();

        // 清除现有的客户端
        pool.clients.clear();

        let proxies = self.get_proxies();
        if proxies.is_empty() {
            // 添加系统代理
            let system_client = Client::new();
            pool.clients
                .insert(SYS_PROXY.to_string(), system_client.clone());
            pool.general = Some(system_client);
            return Ok(());
        }

        // 初始化客户端并设置第一个代理为通用客户端
        let mut first_name = None;
        for (name, proxy) in proxies {
            if first_name.is_none() {
                first_name = Some(name.clone());
            }

            // 初始化客户端
            pool.append(name, &proxy);
        }

        // 设置通用客户端
        if let Some(name) = first_name {
            pool.general = pool.clients.get(&name).cloned();
        } else {
            // 添加系统代理
            let system_client = Client::new();
            pool.clients
                .insert(SYS_PROXY.to_string(), system_client.clone());
            pool.general = Some(system_client);
        }

        Ok(())
    }

    pub async fn save_proxies(&self) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = rkyv::to_bytes::<_, 256>(self)?;

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&*PROXIES_FILE_PATH)?;

        if bytes.len() > usize::MAX / 2 {
            return Err("代理数据过大".into());
        }

        file.set_len(bytes.len() as u64)?;
        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        mmap.copy_from_slice(&bytes);
        mmap.flush()?;

        Ok(())
    }

    pub async fn load_proxies() -> Result<Self, Box<dyn std::error::Error>> {
        let file = match OpenOptions::new().read(true).open(&*PROXIES_FILE_PATH) {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::new());
            }
            Err(e) => return Err(Box::new(e)),
        };

        if file.metadata()?.len() > usize::MAX as u64 {
            return Err("代理文件过大".into());
        }

        let mmap = unsafe { MmapOptions::new().map(&file)? };
        let archived = unsafe { rkyv::archived_root::<Self>(&mmap) };
        Ok(archived.deserialize(&mut rkyv::Infallible)?)
    }

    // 更新全局代理池并保存配置
    pub async fn update_and_save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 更新全局代理池
        self.update_global_pool()?;

        // 保存配置到文件
        self.save_proxies().await
    }
}

#[derive(Clone, Archive, RkyvDeserialize, RkyvSerialize)]
#[archive(compare(PartialEq))]
pub enum SingleProxy {
    Non,
    Sys,
    Url(UrlWrapper),
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

        impl<'de> serde::de::Visitor<'de> for SingleProxyVisitor {
            type Value = SingleProxy;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("a string representing 'non', 'sys', or a valid URL")
            }

            fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                // 检查是否是保留的代理名称，如果是则进行映射
                if let Some(&mapped) = PROXY_MAP.get(value) {
                    match mapped {
                        NON_PROXY => return Ok(Self::Value::Non),
                        SYS_PROXY => return Ok(Self::Value::Sys),
                        _ => {}
                    }
                }

                // 直接匹配新的代理值
                match value {
                    NON_PROXY => Ok(Self::Value::Non),
                    SYS_PROXY => Ok(Self::Value::Sys),
                    url_str => url::Url::parse(url_str)
                        .map(|url| Self::Value::Url(UrlWrapper::from(url)))
                        .map_err(|e| E::custom(format!("Invalid URL: {}", e))),
                }
            }
        }

        deserializer.deserialize_str(SingleProxyVisitor)
    }
}

impl ToString for SingleProxy {
    fn to_string(&self) -> String {
        match self {
            Self::Non => NON_PROXY.to_string(),
            Self::Sys => SYS_PROXY.to_string(),
            Self::Url(url) => url.to_string(),
        }
    }
}

impl FromStr for SingleProxy {
    type Err = url::ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // 检查是否是保留的代理名称，如果是则进行映射
        if let Some(&mapped) = PROXY_MAP.get(s) {
            match mapped {
                NON_PROXY => return Ok(Self::Non),
                SYS_PROXY => return Ok(Self::Sys),
                _ => {}
            }
        }

        // 直接匹配新的代理值
        match s {
            NON_PROXY => Ok(Self::Non),
            SYS_PROXY => Ok(Self::Sys),
            url_str => url::Url::parse(url_str).map(|url| Self::Url(UrlWrapper::from(url))),
        }
    }
}

pub struct ProxyPool {
    // name to client
    clients: HashMap<String, Client>,
    general: Option<Client>,
}

impl ProxyPool {
    // 添加客户端
    fn append(&mut self, name: &str, proxy: &SingleProxy) {
        if self.clients.contains_key(name) {
            return;
        }

        // 根据SingleProxy类型创建客户端
        let client = match proxy {
            SingleProxy::Non => Client::builder()
                .no_proxy()
                .build()
                .expect("创建无代理客户端失败"),
            SingleProxy::Sys => Client::new(),
            SingleProxy::Url(url) => {
                if let Ok(proxy_obj) = Proxy::all(&url.to_string()) {
                    Client::builder()
                        .proxy(proxy_obj)
                        .build()
                        .expect("创建代理客户端失败")
                } else {
                    return;
                }
            }
        };

        self.clients.insert(name.to_string(), client);
    }

    // 获取客户端
    pub fn get_client(url: &str) -> Client {
        let pool = PROXY_POOL.read();

        // 检查是否需要映射
        let mapped_url = PROXY_MAP.get(url).copied().unwrap_or(url);

        pool.clients
            .get(mapped_url.trim())
            .cloned()
            .unwrap_or_else(Self::get_general_client)
    }

    pub fn get_general_client() -> Client {
        let pool = PROXY_POOL.read();
        pool.general.clone().expect("获取通用客户端不应该失败")
    }

    pub fn get_client_or_general(url: Option<&str>) -> Client {
        match url {
            Some(url) => Self::get_client(url),
            None => Self::get_general_client(),
        }
    }
}
