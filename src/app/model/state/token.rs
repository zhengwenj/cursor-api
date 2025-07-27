use ahash::HashMap;
use memmap2::{Mmap, MmapMut};
use std::{borrow::Cow, collections::VecDeque, error::Error};
use tokio::fs::OpenOptions;

use crate::app::{
    constant::{UNNAMED, UNNAMED_PATTERN},
    lazy::TOKENS_FILE_PATH,
    model::{Alias, TokenInfo, TokenInfoHelper, TokenKey},
};

/// 简单错误类型，用于基本操作
#[derive(Debug)]
pub enum TokenError {
    AliasExists,
    InvalidId,
}

impl std::fmt::Display for TokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            TokenError::AliasExists => "别名已存在",
            TokenError::InvalidId => "无效的Token ID",
        })
    }
}

impl Error for TokenError {}

/// Token管理器，管理所有私有Token
pub struct TokenManager {
    /// 主存储，通过ID访问
    tokens: Vec<Option<TokenInfo>>,
    /// Token到ID的映射
    id_map: HashMap<TokenKey, usize>,
    /// 别名到ID的映射
    alias_map: HashMap<Alias, usize>,
    /// ID到别名的映射
    id_to_alias: Vec<Option<Alias>>,
    /// 可重用的ID队列，按FIFO顺序重用
    free_ids: VecDeque<usize>,
}

impl TokenManager {
    /// 创建一个新的Token管理器
    #[inline]
    pub fn new(capacity: usize) -> Self {
        Self {
            tokens: Vec::with_capacity(capacity),
            id_map: HashMap::with_capacity_and_hasher(capacity, ::ahash::RandomState::new()),
            alias_map: HashMap::with_capacity_and_hasher(capacity, ::ahash::RandomState::new()),
            id_to_alias: Vec::with_capacity(capacity),
            free_ids: VecDeque::with_capacity(capacity / 10),
        }
    }

    /// 添加一个Token到管理器中
    #[inline(never)] // 复杂逻辑，避免内联
    pub fn add<'a, S: Into<Cow<'a, str>>>(
        &mut self,
        token_info: TokenInfo,
        alias: S,
    ) -> Result<usize, TokenError> {
        // 检查别名是否已经存在
        let mut alias: Cow<'_, str> = alias.into();
        if alias == UNNAMED || alias.starts_with(UNNAMED_PATTERN) {
            let id = self.free_ids.front().copied().unwrap_or(self.tokens.len());

            alias = Cow::Owned(generate_unnamed_alias(id));
        }

        if self.alias_map.contains_key(alias.as_ref()) {
            return Err(TokenError::AliasExists);
        }

        // 分配ID - 优先使用队列中最早释放的ID
        let id = if let Some(reused_id) = self.free_ids.pop_front() {
            reused_id
        } else {
            let new_id = self.tokens.len();
            self.tokens.push(None);
            self.id_to_alias.push(None);
            new_id
        };

        // 存储Token信息
        self.id_map
            .insert(token_info.bundle.primary_token.key(), id);
        unsafe { *self.tokens.get_unchecked_mut(id) = Some(token_info) };

        let alias = Alias::new(alias);
        self.alias_map.insert(alias.clone(), id);
        unsafe { *self.id_to_alias.get_unchecked_mut(id) = Some(alias) };

        Ok(id)
    }

    /// 通过ID获取Token
    #[inline] // 频繁调用的简单方法
    pub fn get_by_id(&self, id: usize) -> Option<&TokenInfo> {
        self.tokens.get(id).and_then(|t| t.as_ref())
    }

    /// 通过别名获取Token
    #[inline] // 频繁调用
    pub fn get_by_alias(&self, alias: &str) -> Option<&TokenInfo> {
        self.alias_map.get(alias).and_then(|&id| self.get_by_id(id))
    }

    // /// 通过ID或别名获取Token
    // #[inline] // 常用方法
    // pub fn get(&self, id_or_alias: &str) -> Option<&TokenInfo> {
    //     // 尝试将输入解析为ID
    //     if let Ok(id) = id_or_alias.parse::<usize>() {
    //         if let Some(token) = self.get_by_id(id) {
    //             return Some(token);
    //         }
    //     }

    //     // 否则尝试作为别名查找
    //     self.get_by_alias(id_or_alias)
    // }

    /// 删除一个Token
    #[inline(never)] // 复杂的清理逻辑
    pub fn remove(&mut self, id: usize) -> Option<TokenInfo> {
        if id >= self.tokens.len() {
            return None;
        }

        // 交换出TokenInfo
        let token_info = unsafe { self.tokens.get_unchecked_mut(id).take()? };

        // 如果有别名，从别名映射中删除
        if let Some(alias) = unsafe { self.id_to_alias.get_unchecked_mut(id).take() } {
            self.alias_map.remove(&alias);
        }

        // 添加ID到可重用队列的末尾
        self.free_ids.push_back(id);

        Some(token_info)
    }

    /// 为Token设置别名
    #[inline(never)] // 复杂的验证和更新逻辑
    pub fn set_alias<'a, S: Into<Cow<'a, str>>>(
        &mut self,
        id: usize,
        alias: S,
    ) -> Result<(), TokenError> {
        // 检查ID是否有效
        if self.tokens.get(id).is_none_or(|v| v.is_none()) {
            return Err(TokenError::InvalidId);
        }

        // 检查别名是否已经存在
        let mut alias: Cow<'_, str> = alias.into();
        if alias == UNNAMED || alias.starts_with(UNNAMED_PATTERN) {
            alias = Cow::Owned(generate_unnamed_alias(id));
        }
        if self.alias_map.contains_key(alias.as_ref()) {
            return Err(TokenError::AliasExists);
        }

        // 移除旧别名
        if let Some(old_alias) = unsafe { self.id_to_alias.get_unchecked_mut(id).take() } {
            self.alias_map.remove(&old_alias);
        }

        // 设置新别名
        let alias = Alias::new(alias);
        self.alias_map.insert(alias.clone(), id);
        unsafe { *self.id_to_alias.get_unchecked_mut(id) = Some(alias) };

        Ok(())
    }

    pub fn tokens(&self) -> &Vec<Option<TokenInfo>> { &self.tokens }

    pub fn tokens_mut(&mut self) -> &mut Vec<Option<TokenInfo>> { &mut self.tokens }

    pub fn id_map(&self) -> &HashMap<TokenKey, usize> { &self.id_map }

    pub fn alias_map(&self) -> &HashMap<Alias, usize> { &self.alias_map }

    pub fn id_to_alias(&self) -> &Vec<Option<Alias>> { &self.id_to_alias }

    /// 列出所有Token
    #[inline(never)] // 涉及遍历和分配
    pub fn list(&self) -> Vec<(usize, Alias, TokenInfo)> {
        self.tokens
            .iter()
            .enumerate()
            .filter_map(|(id, token_opt)| {
                token_opt.as_ref().map(|token| {
                    let alias = unsafe {
                        self.id_to_alias
                            .get_unchecked(id)
                            .as_ref()
                            .unwrap_unchecked()
                    };
                    (id, alias.clone(), token.clone())
                })
            })
            .collect()
    }

    #[inline(always)]
    pub fn update_client_key(&mut self) {
        for token_info in self.tokens.iter_mut().flatten() {
            token_info.bundle.client_key = super::super::Hash::random();
            token_info.bundle.session_id = uuid::Uuid::new_v4();
        }
    }

    /// 持久化Token管理器
    #[inline(never)]
    pub async fn save(&self) -> Result<(), Box<dyn Error>> {
        let helpers: Vec<TokenInfoHelper> = self
            .tokens
            .iter()
            .enumerate()
            .filter_map(|(id, token_opt)| {
                token_opt.as_ref().map(|token_info| {
                    let alias = unsafe {
                        self.id_to_alias
                            .get_unchecked(id)
                            .as_ref()
                            .map(|a| a.to_string())
                            .unwrap_unchecked()
                    };

                    TokenInfoHelper::new(token_info, alias)
                })
            })
            .collect();

        let bytes = ::rkyv::to_bytes::<::rkyv::rancor::Error>(&helpers)?;
        if bytes.len() > usize::MAX >> 1 {
            return Err("Token数据过大".into());
        }

        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&*TOKENS_FILE_PATH)
            .await?;
        file.set_len(bytes.len() as u64).await?;

        let mut mmap = unsafe { MmapMut::map_mut(&file)? };
        mmap.copy_from_slice(&bytes);
        mmap.flush()?;

        Ok(())
    }

    /// 从持久化存储加载Token管理器
    #[inline(never)]
    pub async fn load() -> Result<Self, Box<dyn Error>> {
        let file = match OpenOptions::new().read(true).open(&*TOKENS_FILE_PATH).await {
            Ok(file) => file,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok(Self::new(0));
            }
            Err(e) => return Err(Box::new(e)),
        };

        if file.metadata().await?.len() > usize::MAX as u64 {
            return Err("Token文件过大".into());
        }

        let mmap = unsafe { Mmap::map(&file)? };
        let helpers = unsafe {
            ::rkyv::from_bytes_unchecked::<Vec<TokenInfoHelper>, ::rkyv::rancor::Error>(&mmap)
        }?;
        let mut manager = Self::new(helpers.len());

        for helper in helpers {
            let (token_info, alias) = helper.extract();
            let _ = manager.add(token_info, alias)?;
        }

        Ok(manager)
    }
}

#[inline]
fn generate_unnamed_alias(id: usize) -> String {
    // 预分配容量：pattern + 6位数字
    // 6位足够覆盖999,999个token，满足绝大多数使用场景
    // 即使超过6位，String会自动扩容，只是多一次realloc
    const CAPACITY: usize = UNNAMED_PATTERN.len() + 6;
    let mut s = String::with_capacity(CAPACITY);
    s.push_str(UNNAMED_PATTERN);

    // 手动实现数字转字符串，避免format!的开销
    if id == 0 {
        s.push('0');
    } else {
        let start = s.len();
        let mut n = id;
        while n > 0 {
            s.push((b'0' + (n % 10) as u8) as char);
            n /= 10;
        }
        // 反转数字部分
        unsafe { s[start..].as_bytes_mut().reverse() };
    }

    s
}
