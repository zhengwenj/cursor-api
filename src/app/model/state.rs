mod log;
mod page;
mod token;

use std::sync::atomic::{AtomicU64, Ordering};
use tokio::sync::{Mutex, RwLock};

use super::{RequestLog, proxy_pool::Proxies};
pub use log::LogManager;
pub use page::{PageContent, Pages};
pub use token::{TokenError, TokenManager};

pub struct AppState {
    pub token_manager: RwLock<TokenManager>,
    pub log_manager: Mutex<LogManager>,
    pub total_requests: AtomicU64,
    pub active_requests: AtomicU64,
    pub error_requests: AtomicU64,
}

impl AppState {
    pub async fn load() -> Result<Self, Box<dyn core::error::Error>> {
        // 并行加载日志、令牌和代理
        let (log_manager_result, token_manager_result, proxies_result) =
            tokio::join!(LogManager::load(), TokenManager::load(), Proxies::load());

        // 获取结果，处理错误
        let log_manager = log_manager_result?;
        let token_manager = token_manager_result?;

        // 处理代理
        let proxies = proxies_result.unwrap_or_default();
        proxies.init();

        // 计算初始统计信息
        let error_count = log_manager.error_count();
        let total_count = log_manager.total_count();

        Ok(Self {
            token_manager: RwLock::new(token_manager),
            log_manager: Mutex::new(log_manager),
            total_requests: AtomicU64::new(total_count),
            active_requests: AtomicU64::new(0),
            error_requests: AtomicU64::new(error_count),
        })
    }

    /// 增加总请求计数
    #[inline(always)]
    pub fn increment_total(&self) { self.total_requests.fetch_add(1, Ordering::Relaxed); }

    /// 增加活跃请求计数
    #[inline(always)]
    pub fn increment_active(&self) { self.active_requests.fetch_add(1, Ordering::Relaxed); }

    /// 减少活跃请求计数
    #[inline(always)]
    pub fn decrement_active(&self) { self.active_requests.fetch_sub(1, Ordering::Relaxed); }

    /// 增加错误请求计数
    #[inline(always)]
    pub fn increment_error(&self) { self.error_requests.fetch_add(1, Ordering::Relaxed); }

    /// 获取日志管理器锁
    #[inline]
    pub async fn log_manager_lock(&self) -> tokio::sync::MutexGuard<'_, LogManager> {
        self.log_manager.lock().await
    }

    /// 向请求日志添加新记录
    #[inline]
    pub async fn push_log(&self, log: RequestLog, token: super::ExtToken) {
        self.log_manager
            .lock()
            .await
            .push_log_with_token(log, token);
    }

    /// 获取下一个日志ID
    #[inline]
    pub async fn next_log_id(&self) -> u64 { self.log_manager.lock().await.next_log_id() }

    /// 查找指定ID的日志并修改
    #[inline]
    pub async fn update_log<F>(&self, id: u64, f: F)
    where
        F: FnOnce(&mut RequestLog),
    {
        self.log_manager.lock().await.update_log(id, f);
    }

    /// 获取TokenManager的读锁
    #[inline]
    pub async fn token_manager_read(&self) -> tokio::sync::RwLockReadGuard<'_, TokenManager> {
        self.token_manager.read().await
    }

    /// 获取TokenManager的写锁
    #[inline]
    pub async fn token_manager_write(&self) -> tokio::sync::RwLockWriteGuard<'_, TokenManager> {
        self.token_manager.write().await
    }

    pub async fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        // 并行保存日志、令牌和代理
        let (log_result, tokens_result, proxies_result) =
            tokio::join!(self.save_logs(), self.save_tokens(), Proxies::save());

        log_result?;
        tokens_result?;
        proxies_result?;
        Ok(())
    }

    async fn save_logs(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.log_manager.lock().await.save().await
    }

    async fn save_tokens(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.token_manager.read().await.save().await
    }

    /// 更新token manager中的client key
    pub async fn update_client_key(&self) { self.token_manager.write().await.update_client_key() }
}
