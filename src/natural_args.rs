// src/natural_args.rs
#![allow(unsafe_op_in_unsafe_fn)]

use crate::common::utils::StringBuilder;
use dotenvy::LoadResult;
use std::{env, process::exit};

// 常量定义
crate::define_typed_constants! {
    pub &str => {
        // Token 关键字
        TOKEN_IMPORT = "import",
        TOKEN_ENV = "env",
        TOKEN_FROM = "from",
        TOKEN_LISTEN = "listen",
        TOKEN_ON = "on",
        TOKEN_PORT = "port",
        TOKEN_HELP = "help",
        TOKEN_QUESTION = "?",
        TOKEN_OVERRIDE = "override",
        TOKEN_OVERRIDING = "overriding",
        TOKEN_AND = "and",
        TOKEN_EXISTING = "existing",

        // 解析器用到的字符串
        WORD_ME = "me",
        WORD_WHAT = "what",
        WORD_CAN = "can",
        WORD_YOU = "you",
        WORD_DO = "do",

        // 默认值
        DEFAULT_ENV_FILE = ".env",
        DEFAULT_LISTEN_HOST = "0.0.0.0",
        DEFAULT_LISTEN_PORT = "3000",

        // 环境变量名
        ENV_HOST = "HOST",
        ENV_PORT = "PORT",

        // 字符串分隔符
        COLON_SEPARATOR = ":",

        // 错误消息前缀
        ERROR_NOT_UNDERSTAND = "I don't understand '",
        ERROR_TRY_HELP = "Try '",
        ERROR_TRY_HELP_SUFFIX = " help' to see what I can do\n\n",

        // 信息消息前缀
        INFO_IMPORTING = "Importing environment from ",
        INFO_LOADED = " (loaded: ",
        INFO_SKIPPED = ", skipped: ",
        INFO_OVERRIDDEN = ", overridden: ",
        INFO_CLOSING = ")",
        INFO_STARTING = "Starting server on ",
    }
}

// Token::Number 直接持有原始输入的字符串切片 &'a str，实现零拷贝
#[derive(Debug, PartialEq, Clone, Copy)]
enum Token<'a> {
    Import,
    Env,
    From,
    Listen,
    On,
    Port,
    Help,
    Question,
    Override,
    Overriding,
    And,
    Existing,
    String(&'a str),
    Number(&'a str),
}

// 生命周期 'a 绑定到输入参数
#[derive(Debug)]
pub enum Action<'a> {
    ImportEnv {
        file: Option<&'a str>,
        override_existing: bool,
    },
    Listen {
        host: Option<&'a str>,
        port: Option<&'a str>, // 完美匹配 Token::Number(&'a str)
    },
    Help,
}

// Vec<Token<'a>> 配合 Token::Number(&'a str)
// 已经是零拷贝且模式匹配友好的方式，
// 比 Vec<(Token, &'a str)> 或两个 Vec 更直接，
// 且避免了 PortBuffer 或 to_string()，性能和清晰度更佳。
pub struct NaturalParser<'a> {
    tokens: Vec<Token<'a>>,
}

impl<'a> NaturalParser<'a> {
    // 优化版本：直接从参数数组构建，避免 join
    pub fn from_args(args: &'a [String]) -> Self {
        // 预分配容量，减少重分配
        let mut tokens = Vec::with_capacity(args.len() + args.len() / 2);

        for arg in args {
            // 允许参数本身包含空格，如 "import env"
            for word in arg.split_whitespace() {
                let token = match word {
                    TOKEN_IMPORT => Token::Import,
                    TOKEN_ENV => Token::Env,
                    TOKEN_FROM => Token::From,
                    TOKEN_LISTEN => Token::Listen,
                    TOKEN_ON => Token::On,
                    TOKEN_PORT => Token::Port,
                    TOKEN_HELP => Token::Help,
                    TOKEN_QUESTION => Token::Question,
                    TOKEN_OVERRIDE => Token::Override,
                    TOKEN_OVERRIDING => Token::Overriding,
                    TOKEN_AND => Token::And,
                    TOKEN_EXISTING => Token::Existing,
                    _ => {
                        // parse 仅用于验证，Token 存储原始 word: &'a str
                        if word.parse::<u16>().is_ok() {
                            Token::Number(word) // 存储 word
                        } else {
                            Token::String(word)
                        }
                    }
                };
                tokens.push(token);
            }
        }
        tokens.shrink_to_fit(); // 回收多余容量
        Self { tokens }
    }

    // 零分配解析实现，使用 &self
    pub unsafe fn parse(&self) -> Vec<Action<'a>> {
        let mut actions = Vec::with_capacity(self.tokens.len() / 3); // 预估 Action 数量
        let mut i = 0;

        while i < self.tokens.len() {
            // 直接在原始切片上进行模式匹配，完全避免中间分配
            // 使用模式绑定 (file, host, port_str)，安全高效！
            match self.tokens.get_unchecked(i..) {
                // import env from file and override existing
                [
                    Token::Import,
                    Token::Env,
                    Token::From,
                    Token::String(file),
                    Token::And,
                    Token::Override,
                    Token::Existing,
                    ..,
                ] => {
                    actions.push(Action::ImportEnv {
                        file: Some(file),
                        override_existing: true,
                    });
                    i += 7;
                }
                // import env and override existing
                [
                    Token::Import,
                    Token::Env,
                    Token::And,
                    Token::Override,
                    Token::Existing,
                    ..,
                ] => {
                    actions.push(Action::ImportEnv {
                        file: None,
                        override_existing: true,
                    });
                    i += 5;
                }
                // import env overriding existing
                [
                    Token::Import,
                    Token::Env,
                    Token::Overriding,
                    Token::Existing,
                    ..,
                ] => {
                    actions.push(Action::ImportEnv {
                        file: None,
                        override_existing: true,
                    });
                    i += 4;
                }
                // import env from file overriding existing
                [
                    Token::Import,
                    Token::Env,
                    Token::From,
                    Token::String(file),
                    Token::Overriding,
                    Token::Existing,
                    ..,
                ] => {
                    actions.push(Action::ImportEnv {
                        file: Some(file),
                        override_existing: true,
                    });
                    i += 6;
                }
                // import env from file
                [
                    Token::Import,
                    Token::Env,
                    Token::From,
                    Token::String(file),
                    ..,
                ] => {
                    actions.push(Action::ImportEnv {
                        file: Some(file),
                        override_existing: false,
                    });
                    i += 4;
                }
                // import env
                [Token::Import, Token::Env, ..] => {
                    actions.push(Action::ImportEnv {
                        file: None,
                        override_existing: false,
                    });
                    i += 2;
                }
                // 匹配 Token::Number(port_str)
                // listen on <host> port <number>
                [
                    Token::Listen,
                    Token::On,
                    Token::String(host),
                    Token::Port,
                    Token::Number(port_str),
                    ..,
                ] => {
                    // 直接使用 port_str: &'a str，零拷贝
                    actions.push(Action::Listen {
                        host: Some(host),
                        port: Some(port_str),
                    });
                    i += 5;
                }
                // listen on port <number>
                [
                    Token::Listen,
                    Token::On,
                    Token::Port,
                    Token::Number(port_str),
                    ..,
                ] => {
                    actions.push(Action::Listen {
                        host: None,
                        port: Some(port_str),
                    });
                    i += 4;
                }
                // listen on <address> (host:port, just host, or just port)
                // 合并处理 String 和 Number, 覆盖 "listen on 8080" 和 "listen on localhost" 和 "listen on localhost:8080"
                [Token::Listen, Token::On, Token::String(addr), ..]
                | [Token::Listen, Token::On, Token::Number(addr), ..] => {
                    // addr 可能是 host:port, host, 或仅 port (被识别为Number)
                    if let Some((host, port)) = addr.split_once(COLON_SEPARATOR) {
                        // host:port 格式
                        actions.push(Action::Listen {
                            host: Some(host), // host 和 port 都是 addr 的切片，生命周期为 'a
                            port: Some(port),
                        });
                    // 检查原始 Token 类型，精确判断 "listen on 8080"
                    } else if matches!(self.tokens[i + 2], Token::Number(_)) {
                        actions.push(Action::Listen {
                            host: None,
                            port: Some(addr),
                        });
                    } else {
                        // 只是 host (例如 "listen on localhost")
                        actions.push(Action::Listen {
                            host: Some(addr),
                            port: None,
                        });
                    }
                    i += 3;
                }

                // help me (使用 match guard)
                [Token::Help, Token::String(s), ..] if *s == WORD_ME => {
                    actions.push(Action::Help);
                    i += 2;
                }
                // help / ?
                [Token::Help, ..] | [Token::Question, ..] => {
                    actions.push(Action::Help);
                    i += 1;
                }
                // what can you do (使用 match guard)
                [
                    Token::String(w1),
                    Token::String(w2),
                    Token::String(w3),
                    Token::String(w4),
                    ..,
                ] if *w1 == WORD_WHAT && *w2 == WORD_CAN && *w3 == WORD_YOU && *w4 == WORD_DO => {
                    actions.push(Action::Help);
                    i += 4;
                }
                // 兜底，无法识别，跳过一个 token
                // 可以标记为 unlikely，但现代分支预测通常足够好
                _ => i += 1,
            }
        }
        actions.shrink_to_fit();
        actions
    }
}

// 标记为 cold path
#[cold]
#[inline(never)]
fn handle_help_and_exit(program_name: &str) -> ! {
    print_help(program_name);
    exit(0);
}

#[inline(always)]
fn load_env_file(filename: &str, override_existing: bool) -> LoadResult {
    match if override_existing {
        dotenvy::from_filename_override(filename)
    } else {
        dotenvy::from_filename(filename)
    } {
        Ok(result) => {
            // 字符串构建是热路径，保持高效
            let mut msg = StringBuilder::with_capacity(7)
                .append(INFO_IMPORTING)
                .append(filename)
                .append(INFO_LOADED)
                .append(result.loaded.to_string());

            if result.skipped > 0 {
                msg.append_mut(INFO_SKIPPED)
                    .append_mut(result.skipped.to_string());
            }

            if result.overridden > 0 {
                msg.append_mut(INFO_OVERRIDDEN)
                    .append_mut(result.overridden.to_string());
            }

            msg.append_mut(INFO_CLOSING);
            __println!(msg.build());
            result
        }
        // 错误路径，调用 cold 函数
        Err(e) => {
            __cold_path!();
            eprintln!("Failed to load {filename}: {e}");
            LoadResult {
                loaded: 0,
                skipped: 0,
                overridden: 0,
            }
        }
    }
}

// 安全的入口函数
pub fn process_args(program_name: &str) {
    let args: Vec<String> = env::args_os()
        .skip(1)
        .map(|s| match s.into_string() {
            Ok(s) => s,
            Err(s) => s.to_string_lossy().into_owned(),
        })
        .collect();
    unsafe { __process_args_impl(program_name, &args) }
}

#[inline(always)]
unsafe fn __process_args_impl(program_name: &str, args: &[String]) {
    if args.is_empty() {
        // 静默加载默认配置
        load_env_file(DEFAULT_ENV_FILE, false);
        return;
    }

    let parser = NaturalParser::from_args(args);
    let actions = parser.parse();

    if actions.is_empty() {
        __cold_path!();
        let command = args.join(" ");
        __eprint!(
            StringBuilder::with_capacity(6)
                .append(ERROR_NOT_UNDERSTAND)
                .append(&command)
                .append("'\n")
                .append(ERROR_TRY_HELP)
                .append(program_name)
                .append(ERROR_TRY_HELP_SUFFIX)
                .build()
        );
        return;
    }

    // 主循环，热路径
    for action in actions {
        match action {
            Action::ImportEnv {
                file,
                override_existing,
            } => {
                let env_file = file.unwrap_or(DEFAULT_ENV_FILE);
                load_env_file(env_file, override_existing);
            }
            Action::Listen { host, port } => {
                let h = host.unwrap_or(DEFAULT_LISTEN_HOST);
                let p = port.unwrap_or(DEFAULT_LISTEN_PORT); // 这里的 port 是 &'a str 或 &'static str

                __println!(
                    StringBuilder::with_capacity(4)
                        .append(INFO_STARTING)
                        .append(h)
                        .append(":")
                        .append(p)
                        .build()
                );

                env::set_var(ENV_HOST, h);
                env::set_var(ENV_PORT, p);
            }
            // Help 路径，调用 cold 函数
            Action::Help => handle_help_and_exit(program_name),
        }
    }
}

#[cold]
#[inline(never)]
fn print_help(program: &str) {
    println!(
        "
Hi! I'm {program}, and here's what I understand:

📦 Environment stuff:
   {program} import env                                      Load from default .env file
   {program} import env from config.env                      Load environment from a specific file
   {program} import env and override existing                Override existing vars from .env
   {program} import env overriding existing                  Alternative syntax for override
   {program} import env from prod.env and override existing  Override from specific file
   {program} import env from prod.env overriding existing    Alternative syntax

🌐 Server stuff:  
   {program} listen on 127.0.0.1 port 8080                   Listen on specific IP and port
   {program} listen on localhost port 3000                   Listen on localhost with port
   {program} listen on port 8080                             Listen on all interfaces (0.0.0.0)
   {program} listen on 192.168.1.1:8080                      IP:port format
   {program} listen on 8080                                  Just the port (defaults to 0.0.0.0)
   {program} listen on localhost                             Just the host (defaults to port 3000)

❓ Getting help:  
   {program} help                                            Show this message
   {program} help me                                         Same thing, but more polite
   {program} ?                                               Quick help
   {program} what can you do                                 Natural language help

Examples:
   {program} import env from .env.prod and override existing listen on 10.0.0.1 port 8080
   {program} listen on localhost:5000 import env overriding existing
"
    );
}
