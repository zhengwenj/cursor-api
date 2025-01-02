use crate::{
    app::{
        constant::EMPTY_STRING,
        model::TokenInfo,
        lazy::{TOKEN_FILE, TOKEN_LIST_FILE},
    },
    common::utils::{generate_checksum, generate_hash},
};

// 规范化文件内容并写入
fn normalize_and_write(content: &str, file_path: &str) -> String {
    let normalized = content.replace("\r\n", "\n");
    if normalized != content {
        if let Err(e) = std::fs::write(file_path, &normalized) {
            eprintln!("警告: 无法更新规范化的文件: {}", e);
        }
    }
    normalized
}

// 解析token和别名
fn parse_token_alias(token_part: &str, line: &str) -> Option<(String, Option<String>)> {
    match token_part.split("::").collect::<Vec<_>>() {
        parts if parts.len() == 1 => Some((parts[0].to_string(), None)),
        parts if parts.len() == 2 => Some((parts[1].to_string(), Some(parts[0].to_string()))),
        _ => {
            eprintln!("警告: 忽略无效的行: {}", line);
            None
        }
    }
}

// Token 加载函数
pub fn load_tokens() -> Vec<TokenInfo> {
    let token_file = TOKEN_FILE.as_str();
    let token_list_file = TOKEN_LIST_FILE.as_str();

    // 确保文件存在
    for file in [&token_file, &token_list_file] {
        if !std::path::Path::new(file).exists() {
            if let Err(e) = std::fs::write(file, EMPTY_STRING) {
                eprintln!("警告: 无法创建文件 '{}': {}", file, e);
            }
        }
    }

    // 读取和规范化 token 文件
    let token_entries = match std::fs::read_to_string(&token_file) {
        Ok(content) => {
            let normalized = normalize_and_write(&content, &token_file);
            normalized
                .lines()
                .filter_map(|line| {
                    let line = line.trim();
                    if line.is_empty() || line.starts_with('#') {
                        return None;
                    }
                    parse_token_alias(line, line)
                })
                .collect::<Vec<_>>()
        }
        Err(e) => {
            eprintln!("警告: 无法读取token文件 '{}': {}", token_file, e);
            Vec::new()
        }
    };

    // 读取和规范化 token-list 文件
    let mut token_map: std::collections::HashMap<String, (String, Option<String>)> =
        match std::fs::read_to_string(&token_list_file) {
            Ok(content) => {
                let normalized = normalize_and_write(&content, &token_list_file);
                normalized
                    .lines()
                    .filter_map(|line| {
                        let line = line.trim();
                        if line.is_empty() || line.starts_with('#') {
                            return None;
                        }

                        let parts: Vec<&str> = line.split(',').collect();
                        match parts[..] {
                            [token_part, checksum] => {
                                let (token, alias) = parse_token_alias(token_part, line)?;
                                Some((token, (checksum.to_string(), alias)))
                            }
                            _ => {
                                eprintln!("警告: 忽略无效的token-list行: {}", line);
                                None
                            }
                        }
                    })
                    .collect()
            }
            Err(e) => {
                eprintln!("警告: 无法读取token-list文件: {}", e);
                std::collections::HashMap::new()
            }
        };

    // 更新或添加新token
    for (token, alias) in token_entries {
        if let Some((_, existing_alias)) = token_map.get(&token) {
            // 只在alias不同时更新已存在的token
            if alias != *existing_alias {
                if let Some((checksum, _)) = token_map.get(&token) {
                    token_map.insert(token.clone(), (checksum.clone(), alias));
                }
            }
        } else {
            // 为新token生成checksum
            let checksum = generate_checksum(&generate_hash(), Some(&generate_hash()));
            token_map.insert(token, (checksum, alias));
        }
    }

    // 更新 token-list 文件
    let token_list_content = token_map
        .iter()
        .map(|(token, (checksum, alias))| {
            if let Some(alias) = alias {
                format!("{}::{},{}", alias, token, checksum)
            } else {
                format!("{},{}", token, checksum)
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    if let Err(e) = std::fs::write(&token_list_file, token_list_content) {
        eprintln!("警告: 无法更新token-list文件: {}", e);
    }

    // 转换为 TokenInfo vector
    token_map
        .into_iter()
        .map(|(token, (checksum, alias))| TokenInfo {
            token,
            checksum,
            alias,
            usage: None,
        })
        .collect()
}
