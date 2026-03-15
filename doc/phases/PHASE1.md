# Phase 1: 기본 인프라

> **목표**: Rust 프로젝트 초기화, 기본 CLI 구조, HTTP 클라이언트, 데이터 모델, 설정 관리 구현

---

## 체크리스트

### 1.1 프로젝트 초기화
- [x] `cargo init` 실행
- [x] `Cargo.toml` 의존성 추가
- [x] 기본 디렉토리 구조 생성
- [x] `cargo build` 통과

### 1.2 CLI 기본 구조 (clap)
- [x] `src/cli/mod.rs` 생성
- [x] `src/cli/root.rs` - 루트 커맨드 정의
- [x] `src/main.rs` - 진입점 연결
- [x] `--help`, `--version` 동작 확인

### 1.3 설정 관리
- [x] `src/config/mod.rs` 생성
- [x] `src/config/settings.rs` - 설정 구조체
- [x] 설정 파일 로드 (`~/.config/reddit-cli/config.toml`)
- [x] 기본 설정값 정의

### 1.4 HTTP 클라이언트
- [x] `src/api/mod.rs` 생성
- [x] `src/api/client.rs` - reqwest 래퍼
- [x] User-Agent 헤더 설정
- [ ] 기본 GET 요청 테스트 (Phase 2에서 구현)

### 1.5 데이터 모델
- [x] `src/models/mod.rs` 생성
- [x] `src/models/common.rs` - Thing, Listing 타입
- [x] `src/models/link.rs` - Link(게시물) 모델
- [x] `src/models/comment.rs` - Comment 모델
- [x] `src/models/subreddit.rs` - Subreddit 모델
- [x] `src/models/user.rs` - User 모델

### 1.6 에러 처리
- [x] `src/error.rs` - 커스텀 에러 타입
- [x] thiserror 활용
- [x] API 에러 매핑

### 1.7 로깅
- [x] tracing 설정
- [x] RUST_LOG 환경변수 지원

---

## 상세 구현 가이드

### 1.1 Cargo.toml

```toml
[package]
name = "reddit-cli"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your@email.com>"]
description = "A CLI client for Reddit API"
license = "MIT"

[dependencies]
# CLI
clap = { version = "4", features = ["derive", "env"] }

# Async runtime
tokio = { version = "1", features = ["full"] }

# HTTP client
reqwest = { version = "0.12", default-features = false, features = [
    "json",
    "rustls-tls",
] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Configuration
config = "0.14"
toml = "0.8"
dirs = "5"

# Error handling
thiserror = "1"
anyhow = "1"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# URL handling
url = "2"

[dev-dependencies]
tokio-test = "0.4"
```

### 1.2 디렉토리 구조 생성

```bash
mkdir -p src/{cli,api/endpoints,models,output,config,cache,utils}
```

### 1.3 src/main.rs

```rust
mod api;
mod cache;
mod cli;
mod config;
mod error;
mod models;
mod output;
mod utils;

use clap::Parser;
use cli::Cli;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cli = Cli::parse();
    cli.run().await
}
```

### 1.4 src/cli/mod.rs

```rust
mod root;

pub use root::Cli;
```

### 1.5 src/cli/root.rs

```rust
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "reddit")]
#[command(about = "A CLI client for Reddit API", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format (table, json, plain)
    #[arg(short, long, global = true, default_value = "table")]
    format: String,

    /// Number of items to fetch
    #[arg(short = 'n', long, global = true, default_value = "25")]
    limit: u32,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// View hot posts
    Hot {
        /// Subreddit name (without r/)
        #[arg(short, long)]
        subreddit: Option<String>,
    },
    /// View new posts
    New {
        #[arg(short, long)]
        subreddit: Option<String>,
    },
    /// View top posts
    Top {
        #[arg(short, long)]
        subreddit: Option<String>,
        /// Time period: hour, day, week, month, year, all
        #[arg(short = 't', long, default_value = "day")]
        time: String,
    },
    /// View rising posts
    Rising {
        #[arg(short, long)]
        subreddit: Option<String>,
    },
    /// View controversial posts
    Controversial {
        #[arg(short, long)]
        subreddit: Option<String>,
        #[arg(short = 't', long, default_value = "day")]
        time: String,
    },
    /// Subreddit commands
    #[command(subcommand)]
    Subreddit(SubredditCommands),
    /// User commands
    #[command(subcommand)]
    User(UserCommands),
    /// Search posts
    Search {
        /// Search query
        query: String,
        /// Restrict to subreddit
        #[arg(short, long)]
        subreddit: Option<String>,
        /// Sort by: relevance, hot, top, new, comments
        #[arg(short, long, default_value = "relevance")]
        sort: String,
    },
    /// View a post
    Post {
        /// Post ID (base36)
        id: String,
    },
    /// Authentication commands
    #[command(subcommand)]
    Auth(AuthCommands),
}

#[derive(Subcommand)]
pub enum SubredditCommands {
    /// Show subreddit info
    Show { name: String },
    /// View subreddit posts (alias for hot)
    Hot { name: String },
    /// View subreddit new posts
    New { name: String },
    /// View subreddit top posts
    Top {
        name: String,
        #[arg(short = 't', long, default_value = "day")]
        time: String,
    },
    /// View subreddit rules
    Rules { name: String },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Show user info
    Show { username: String },
    /// View user's posts
    Posts { username: String },
    /// View user's comments
    Comments { username: String },
    /// View user's overview
    Overview { username: String },
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Login to Reddit (OAuth)
    Login,
    /// Logout
    Logout,
    /// Show auth status
    Status,
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        // Load config
        let _config = config::Settings::load()?;

        // TODO: Initialize API client
        // let client = api::Client::new(&config)?;

        match &self.command {
            Commands::Hot { subreddit } => {
                println!("Hot posts from {:?}", subreddit);
                // TODO: Implement
            }
            Commands::New { subreddit } => {
                println!("New posts from {:?}", subreddit);
            }
            Commands::Top { subreddit, time } => {
                println!("Top posts from {:?} (time: {})", subreddit, time);
            }
            Commands::Rising { subreddit } => {
                println!("Rising posts from {:?}", subreddit);
            }
            Commands::Controversial { subreddit, time } => {
                println!("Controversial posts from {:?} (time: {})", subreddit, time);
            }
            Commands::Subreddit(cmd) => {
                match cmd {
                    SubredditCommands::Show { name } => {
                        println!("Subreddit info: {}", name);
                    }
                    SubredditCommands::Hot { name } => {
                        println!("Hot posts from r/{}", name);
                    }
                    SubredditCommands::New { name } => {
                        println!("New posts from r/{}", name);
                    }
                    SubredditCommands::Top { name, time } => {
                        println!("Top posts from r/{} (time: {})", name, time);
                    }
                    SubredditCommands::Rules { name } => {
                        println!("Rules for r/{}", name);
                    }
                }
            }
            Commands::User(cmd) => {
                match cmd {
                    UserCommands::Show { username } => {
                        println!("User info: u/{}", username);
                    }
                    UserCommands::Posts { username } => {
                        println!("Posts by u/{}", username);
                    }
                    UserCommands::Comments { username } => {
                        println!("Comments by u/{}", username);
                    }
                    UserCommands::Overview { username } => {
                        println!("Overview of u/{}", username);
                    }
                }
            }
            Commands::Search { query, subreddit, sort } => {
                println!("Search: {} (subreddit: {:?}, sort: {})", query, subreddit, sort);
            }
            Commands::Post { id } => {
                println!("View post: {}", id);
            }
            Commands::Auth(cmd) => {
                match cmd {
                    AuthCommands::Login => {
                        println!("Logging in...");
                    }
                    AuthCommands::Logout => {
                        println!("Logging out...");
                    }
                    AuthCommands::Status => {
                        println!("Checking auth status...");
                    }
                }
            }
        }

        Ok(())
    }
}
```

### 1.6 src/config/mod.rs

```rust
mod settings;

pub use settings::Settings;
```

### 1.7 src/config/settings.rs

```rust
use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone)]
pub struct Settings {
    #[serde(default)]
    pub api: ApiSettings,
    #[serde(default)]
    pub output: OutputSettings,
    #[serde(default)]
    pub auth: AuthSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiSettings {
    #[serde(default = "default_api_base_url")]
    pub base_url: String,
    #[serde(default = "default_www_url")]
    pub www_url: String,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            base_url: default_api_base_url(),
            www_url: default_www_url(),
            user_agent: default_user_agent(),
        }
    }
}

fn default_api_base_url() -> String {
    "https://oauth.reddit.com".to_string()
}

fn default_www_url() -> String {
    "https://www.reddit.com".to_string()
}

fn default_user_agent() -> String {
    format!(
        "cli:reddit-cli:{} (by /u/anonymous)",
        env!("CARGO_PKG_VERSION")
    )
}

#[derive(Debug, Deserialize, Clone)]
pub struct OutputSettings {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_pager")]
    pub pager: bool,
    #[serde(default = "default_color")]
    pub color: bool,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            format: default_format(),
            pager: default_pager(),
            color: default_color(),
            page_size: default_page_size(),
        }
    }
}

fn default_format() -> String {
    "table".to_string()
}

fn default_pager() -> bool {
    true
}

fn default_color() -> bool {
    true
}

fn default_page_size() -> u32 {
    25
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AuthSettings {
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
    #[serde(default = "default_redirect_uri")]
    pub redirect_uri: String,
}

fn default_redirect_uri() -> String {
    "http://127.0.0.1:65010".to_string()
}

impl Settings {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let settings: Settings = toml::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Self::default())
        }
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("reddit-cli");
        Ok(config_dir.join("config.toml"))
    }

    pub fn token_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("reddit-cli");
        Ok(config_dir.join("token.json"))
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            api: ApiSettings::default(),
            output: OutputSettings::default(),
            auth: AuthSettings::default(),
        }
    }
}
```

### 1.8 src/error.rs

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedditError {
    #[error("API error: {0}")]
    Api(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Rate limited. Retry after {0} seconds")]
    RateLimited(u64),

    #[error("Not authenticated. Run `reddit auth login` first")]
    NotAuthenticated,

    #[error("Thing not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, RedditError>;
```

### 1.9 src/api/mod.rs

```rust
mod client;
pub mod endpoints;
pub mod ratelimit;

pub use client::Client;
```

### 1.10 src/api/client.rs

```rust
use crate::config::Settings;
use crate::error::{RedditError, Result};
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
    www_url: String,
}

impl Client {
    pub fn new(settings: &Settings) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&settings.api.user_agent)
                .map_err(|e| RedditError::Config(format!("Invalid user agent: {}", e)))?,
        );

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http,
            base_url: settings.api.base_url.clone(),
            www_url: settings.api.www_url.clone(),
        })
    }

    /// Make a GET request to the www (non-OAuth) API
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}.json", self.www_url, path);

        tracing::debug!("GET {}", url);

        let response = self
            .http
            .get(&url)
            .query(&[("raw_json", "1")])
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        // Check rate limit headers
        if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
            tracing::debug!("Rate limit remaining: {:?}", remaining);
        }

        response.json().await.map_err(Into::into)
    }

    /// Make a GET request with query parameters
    pub async fn get_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}.json", self.www_url, path);

        tracing::debug!("GET {} with query {:?}", url, query);

        let mut params: Vec<(&str, &str)> = query.to_vec();
        params.push(("raw_json", "1"));

        let response = self.http.get(&url).query(&params).send().await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }
}
```

### 1.11 src/models/mod.rs

```rust
pub mod common;
pub mod comment;
pub mod link;
pub mod subreddit;
pub mod user;

pub use common::*;
pub use comment::Comment;
pub use link::Link;
pub use subreddit::Subreddit;
pub use user::User;
```

### 1.12 src/models/common.rs

```rust
use serde::Deserialize;

/// Reddit thing type prefix
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThingType {
    Comment,   // t1_
    Account,   // t2_
    Link,      // t3_
    Message,   // t4_
    Subreddit, // t5_
    Award,     // t6_
}

impl std::fmt::Display for ThingType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThingType::Comment => write!(f, "t1"),
            ThingType::Account => write!(f, "t2"),
            ThingType::Link => write!(f, "t3"),
            ThingType::Message => write!(f, "t4"),
            ThingType::Subreddit => write!(f, "t5"),
            ThingType::Award => write!(f, "t6"),
        }
    }
}

/// Generic Reddit thing wrapper
#[derive(Debug, Deserialize)]
pub struct Thing<T> {
    pub kind: String,
    pub data: T,
}

/// Listing response (pagination)
#[derive(Debug, Deserialize)]
pub struct Listing<T> {
    #[serde(rename = "before")]
    pub before: Option<String>,
    #[serde(rename = "after")]
    pub after: Option<String>,
    #[serde(rename = "children")]
    pub children: Vec<Thing<T>>,
}

/// Generic listing response wrapper
#[derive(Debug, Deserialize)]
pub struct ListingResponse<T> {
    #[serde(rename = "data")]
    pub data: Listing<T>,
}

/// Time period for top/controversial
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TimePeriod {
    Hour,
    Day,
    Week,
    Month,
    Year,
    All,
}

impl std::fmt::Display for TimePeriod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimePeriod::Hour => write!(f, "hour"),
            TimePeriod::Day => write!(f, "day"),
            TimePeriod::Week => write!(f, "week"),
            TimePeriod::Month => write!(f, "month"),
            TimePeriod::Year => write!(f, "year"),
            TimePeriod::All => write!(f, "all"),
        }
    }
}

impl std::str::FromStr for TimePeriod {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "hour" => Ok(TimePeriod::Hour),
            "day" => Ok(TimePeriod::Day),
            "week" => Ok(TimePeriod::Week),
            "month" => Ok(TimePeriod::Month),
            "year" => Ok(TimePeriod::Year),
            "all" => Ok(TimePeriod::All),
            _ => Err(format!("Invalid time period: {}", s)),
        }
    }
}

/// Sort method for listings
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SortMethod {
    Hot,
    New,
    Top,
    Rising,
    Controversial,
    Best,
}

impl std::fmt::Display for SortMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SortMethod::Hot => write!(f, "hot"),
            SortMethod::New => write!(f, "new"),
            SortMethod::Top => write!(f, "top"),
            SortMethod::Rising => write!(f, "rising"),
            SortMethod::Controversial => write!(f, "controversial"),
            SortMethod::Best => write!(f, "best"),
        }
    }
}
```

### 1.13 src/models/link.rs

```rust
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Reddit link (post)
#[derive(Debug, Deserialize)]
pub struct Link {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub fullname: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "subreddit")]
    pub subreddit: String,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: String,
    #[serde(rename = "selftext")]
    pub selftext: String,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "permalink")]
    pub permalink: String,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "score")]
    pub score: i64,
    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: f64,
    #[serde(rename = "num_comments")]
    pub num_comments: u32,
    #[serde(rename = "over_18")]
    pub over_18: bool,
    #[serde(rename = "spoiler")]
    pub spoiler: bool,
    #[serde(rename = "stickied")]
    pub stickied: bool,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "is_self")]
    pub is_self: bool,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<String>,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<String>,
    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,
    #[serde(rename = "preview")]
    pub preview: Option<Preview>,
}

#[derive(Debug, Deserialize)]
pub struct Preview {
    #[serde(rename = "images")]
    pub images: Vec<PreviewImage>,
}

#[derive(Debug, Deserialize)]
pub struct PreviewImage {
    #[serde(rename = "source")]
    pub source: ImageSource,
}

#[derive(Debug, Deserialize)]
pub struct ImageSource {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "width")]
    pub width: u32,
    #[serde(rename = "height")]
    pub height: u32,
}
```

### 1.14 src/models/comment.rs

```rust
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Reddit comment
#[derive(Debug, Deserialize)]
pub struct Comment {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub fullname: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "body_html")]
    pub body_html: Option<String>,
    #[serde(rename = "subreddit")]
    pub subreddit: String,
    #[serde(rename = "link_id")]
    pub link_id: String,
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "permalink")]
    pub permalink: String,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "score")]
    pub score: i64,
    #[serde(rename = "controversiality")]
    pub controversiality: u8,
    #[serde(rename = "stickied")]
    pub stickied: bool,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "edited")]
    pub edited: Edited,
    #[serde(rename = "replies")]
    pub replies: Option<CommentReplies>,
}

/// Edited can be either bool or timestamp
#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Edited {
    Bool(bool),
    Timestamp(f64),
}

impl Edited {
    pub fn is_edited(&self) -> bool {
        match self {
            Edited::Bool(b) => *b,
            Edited::Timestamp(_) => true,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct CommentReplies {
    #[serde(rename = "data")]
    pub data: CommentListing,
}

#[derive(Debug, Deserialize)]
pub struct CommentListing {
    #[serde(rename = "children")]
    pub children: Vec<crate::models::common::Thing<Comment>>,
}
```

### 1.15 src/models/subreddit.rs

```rust
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Subreddit information
#[derive(Debug, Deserialize)]
pub struct Subreddit {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub fullname: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "public_description")]
    pub public_description: String,
    #[serde(rename = "subscribers")]
    pub subscribers: u64,
    #[serde(rename = "active_user_count")]
    pub active_user_count: Option<u64>,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "over18")]
    pub over18: bool,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "icon_img")]
    pub icon_img: Option<String>,
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    #[serde(rename = "header_img")]
    pub header_img: Option<String>,
    #[serde(rename = "user_is_subscriber")]
    pub user_is_subscriber: Option<bool>,
    #[serde(rename = "user_is_moderator")]
    pub user_is_moderator: Option<bool>,
}

/// Subreddit rule
#[derive(Debug, Deserialize)]
pub struct SubredditRule {
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "priority")]
    pub priority: u32,
}

/// Subreddit rules response
#[derive(Debug, Deserialize)]
pub struct SubredditRules {
    #[serde(rename = "rules")]
    pub rules: Vec<SubredditRule>,
}
```

### 1.16 src/models/user.rs

```rust
use chrono::{DateTime, Utc};
use serde::Deserialize;

/// Reddit user
#[derive(Debug, Deserialize)]
pub struct User {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "link_karma")]
    pub link_karma: i64,
    #[serde(rename = "comment_karma")]
    pub comment_karma: i64,
    #[serde(rename = "total_karma")]
    pub total_karma: Option<i64>,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "has_verified_email")]
    pub has_verified_email: Option<bool>,
    #[serde(rename = "is_gold")]
    pub is_gold: Option<bool>,
    #[serde(rename = "is_mod")]
    pub is_mod: Option<bool>,
    #[serde(rename = "is_employee")]
    pub is_employee: Option<bool>,
    #[serde(rename = "icon_img")]
    pub icon_img: Option<String>,
    #[serde(rename = "snoovatar_img")]
    pub snoovatar_img: Option<String>,
    #[serde(rename = "subreddit")]
    pub subreddit: Option<UserSubreddit>,
}

#[derive(Debug, Deserialize)]
pub struct UserSubreddit {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "public_description")]
    pub public_description: Option<String>,
}
```

---

## 완료 기준

1. `cargo build` 성공
2. `cargo run -- --help` 동작
3. `cargo run -- hot` 기본 출력
4. `cargo run -- subreddit show rust` 기본 출력
5. 설정 파일 로드 동작
6. 모든 모델 타입 정의 완료

---

## 다음 단계

Phase 1 완료 후 → [PHASE2.md](PHASE2.md): 읽기 API 구현
