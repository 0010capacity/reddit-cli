# Phase 2: 읽기 API

> **목표**: OAuth 없이 사용 가능한 모든 읽기 API 구현
> **참고**: 2023년부터 Reddit은 모든 API 요청에 OAuth 인증을 요구합니다. Phase 3에서 OAuth 구현 후 API가 작동합니다.

**전제조건**: Phase 1 완료

---

## 체크리스트

### 2.1 리스트업 API
- [x] `src/api/endpoints/mod.rs` 생성
- [x] `src/api/endpoints/listing.rs` - hot/new/top/rising/controversial
- [x] CLI 커맨드와 연결
- [x] 페이지네이션 지원 (--after, --before, --limit)

### 2.2 서브레딧 API
- [x] `src/api/endpoints/subreddit.rs` - 서브레딧 정보
- [x] 서브레딧 규칙 조회
- [ ] 사이드바 조회 (미구현)
- [x] 서브레딧 목록 (popular, new, search)

### 2.3 게시물 API
- [x] `src/api/endpoints/link.rs` - 게시물 조회
- [x] 게시물 + 댓글 트리 조회
- [x] 게시물 정보 (by id)
- [x] 중복 게시물 조회

### 2.4 사용자 API
- [x] `src/api/endpoints/user.rs` - 사용자 정보
- [x] 사용자 게시물/댓글/개요
- [x] 사용자 트로피

### 2.5 검색 API
- [x] `src/api/endpoints/search.rs` - 검색
- [x] 전체 검색
- [x] 서브레딧 내 검색

### 2.6 출력 포맷팅
- [x] `src/output/mod.rs` 생성
- [x] `src/output/table.rs` - 테이블 출력
- [x] `src/output/json.rs` - JSON 출력
- [ ] `src/output/markdown.rs` - 마크다운 렌더링 (미구현)

---

## 상세 구현 가이드

### 2.1 src/api/endpoints/mod.rs

```rust
pub mod listing;
pub mod link;
pub mod search;
pub mod subreddit;
pub mod user;

pub use listing::ListingEndpoint;
pub use link::LinkEndpoint;
pub use search::SearchEndpoint;
pub use subreddit::SubredditEndpoint;
pub use user::UserEndpoint;
```

### 2.2 src/api/endpoints/listing.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::{Link, ListingResponse, SortMethod, TimePeriod};

pub struct ListingEndpoint<'a> {
    client: &'a Client,
}

impl<'a> ListingEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get hot posts
    pub async fn hot(
        &self,
        subreddit: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/hot", sr),
            None => "/hot".to_string(),
        };

        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client.get_with_query(&path, &query).await
    }

    /// Get new posts
    pub async fn new(
        &self,
        subreddit: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/new", sr),
            None => "/new".to_string(),
        };

        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client.get_with_query(&path, &query).await
    }

    /// Get top posts
    pub async fn top(
        &self,
        subreddit: Option<&str>,
        time: TimePeriod,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/top", sr),
            None => "/top".to_string(),
        };

        let mut query: Vec<(&str, &str)> = vec![("t", time.to_string().as_str())];
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client.get_with_query(&path, &query).await
    }

    /// Get rising posts
    pub async fn rising(
        &self,
        subreddit: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/rising", sr),
            None => "/rising".to_string(),
        };

        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client.get_with_query(&path, &query).await
    }

    /// Get controversial posts
    pub async fn controversial(
        &self,
        subreddit: Option<&str>,
        time: TimePeriod,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/controversial", sr),
            None => "/controversial".to_string(),
        };

        let mut query: Vec<(&str, &str)> = vec![("t", time.to_string().as_str())];
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client.get_with_query(&path, &query).await
    }
}
```

### 2.3 src/api/endpoints/subreddit.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::{Link, ListingResponse, Subreddit, SubredditRules};
use serde::Deserialize;

pub struct SubredditEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct SubredditResponse {
    pub data: Subreddit,
}

#[derive(Debug, Deserialize)]
pub struct SubredditRulesResponse {
    pub rules: Vec<SubredditRules>,
}

impl<'a> SubredditEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get subreddit info
    pub async fn about(&self, name: &str) -> Result<SubredditResponse> {
        self.client.get(&format!("/r/{}/about", name)).await
    }

    /// Get subreddit rules
    pub async fn rules(&self, name: &str) -> Result<SubredditRules> {
        self.client.get(&format!("/r/{}/about/rules", name)).await
    }

    /// Get subreddit sidebar
    pub async fn sidebar(&self, name: &str) -> Result<String> {
        // Response is HTML, we need to extract
        let url = format!("{}/r/{}/about/sidebar.json",
            self.client.www_url(), name);
        // TODO: Implement sidebar extraction
        Ok(String::new())
    }

    /// Get popular subreddits
    pub async fn popular(&self, limit: Option<u32>) -> Result<ListingResponse<Subreddit>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        self.client.get_with_query("/subreddits/popular", &query).await
    }

    /// Get new subreddits
    pub async fn new_subreddits(&self, limit: Option<u32>) -> Result<ListingResponse<Subreddit>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        self.client.get_with_query("/subreddits/new", &query).await
    }

    /// Search subreddits
    pub async fn search(&self, query: &str, limit: Option<u32>) -> Result<ListingResponse<Subreddit>> {
        let mut params: Vec<(&str, &str)> = vec![("q", query)];
        if let Some(l) = limit {
            params.push(("limit", &l.to_string()));
        }
        self.client.get_with_query("/subreddits/search", &params).await
    }
}
```

### 2.4 src/api/endpoints/user.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::{Comment, Link, ListingResponse, User};
use serde::Deserialize;

pub struct UserEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub data: User,
}

#[derive(Debug, Deserialize)]
pub struct TrophyResponse {
    pub data: Trophies,
}

#[derive(Debug, Deserialize)]
pub struct Trophies {
    pub trophies: Vec<Trophy>,
}

#[derive(Debug, Deserialize)]
pub struct Trophy {
    pub name: String,
    pub description: Option<String>,
    pub icon_40: Option<String>,
}

impl<'a> UserEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get user info
    pub async fn about(&self, username: &str) -> Result<UserResponse> {
        self.client.get(&format!("/user/{}/about", username)).await
    }

    /// Get user's posts
    pub async fn submitted(
        &self,
        username: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }
        self.client.get_with_query(&format!("/user/{}/submitted", username), &query).await
    }

    /// Get user's comments
    pub async fn comments(
        &self,
        username: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Comment>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }
        self.client.get_with_query(&format!("/user/{}/comments", username), &query).await
    }

    /// Get user's overview (posts + comments)
    pub async fn overview(
        &self,
        username: &str,
        limit: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };
        self.client.get_with_query(&format!("/user/{}/overview", username), &query).await
    }

    /// Get user's trophies
    pub async fn trophies(&self, username: &str) -> Result<TrophyResponse> {
        self.client.get(&format!("/user/{}/trophies", username)).await
    }

    /// Get user's gilded content
    pub async fn gilded(
        &self,
        username: &str,
        limit: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };
        self.client.get_with_query(&format!("/user/{}/gilded", username), &query).await
    }
}
```

### 2.5 src/api/endpoints/link.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::{Comment, Link, ListingResponse};
use serde::Deserialize;

pub struct LinkEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct LinkInfoResponse {
    pub data: LinkInfo,
}

#[derive(Debug, Deserialize)]
pub struct LinkInfo {
    pub children: Vec<LinkThing>,
}

#[derive(Debug, Deserialize)]
pub struct LinkThing {
    pub kind: String,
    pub data: Link,
}

/// Post with comments response
/// Returns an array: [post_listing, comments_listing]
pub type PostWithComments = Vec<serde_json::Value>;

impl<'a> LinkEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get post with comments
    pub async fn comments(&self, article_id: &str, limit: Option<u32>) -> Result<PostWithComments> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };
        self.client.get_with_query(&format!("/comments/{}", article_id), &query).await
    }

    /// Get post info by ID
    pub async fn info(&self, id: &str) -> Result<LinkInfoResponse> {
        let query = vec![("id", id)];
        self.client.get_with_query("/api/info", &query).await
    }

    /// Get duplicate posts (crossposts)
    pub async fn duplicates(&self, article_id: &str) -> Result<serde_json::Value> {
        self.client.get(&format!("/duplicates/{}", article_id)).await
    }
}
```

### 2.6 src/api/endpoints/search.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::{Link, ListingResponse};

pub struct SearchEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: String,
    pub subreddit: Option<String>,
    pub sort: Option<String>,
    pub time: Option<String>,
    pub limit: Option<u32>,
    pub restrict_sr: bool,
}

impl<'a> SearchEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Search posts
    pub async fn search(&self, params: &SearchParams) -> Result<ListingResponse<Link>> {
        let path = match &params.subreddit {
            Some(sr) => format!("/r/{}/search", sr),
            None => "/search".to_string(),
        };

        let mut query: Vec<(&str, &str)> = vec![("q", &params.query)];

        if let Some(ref sort) = params.sort {
            query.push(("sort", sort));
        }
        if let Some(ref time) = params.time {
            query.push(("t", time));
        }
        if let Some(l) = params.limit {
            query.push(("limit", &l.to_string()));
        }
        if params.restrict_sr {
            query.push(("restrict_sr", "true"));
        }

        self.client.get_with_query(&path, &query).await
    }
}
```

### 2.7 src/output/mod.rs

```rust
mod json;
mod table;

pub use json::JsonOutput;
pub use table::TableOutput;

pub enum OutputFormat {
    Table,
    Json,
    Plain,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            "plain" => Ok(OutputFormat::Plain),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

pub trait Output {
    fn format_links(&self, links: &[crate::models::Link]) -> String;
    fn format_subreddit(&self, subreddit: &crate::models::Subreddit) -> String;
    fn format_user(&self, user: &crate::models::User) -> String;
}

pub fn get_output(format: OutputFormat) -> Box<dyn Output> {
    match format {
        OutputFormat::Table => Box::new(TableOutput),
        OutputFormat::Json => Box::new(JsonOutput),
        OutputFormat::Plain => Box::new(PlainOutput),
    }
}

struct PlainOutput;

impl Output for PlainOutput {
    fn format_links(&self, links: &[crate::models::Link]) -> String {
        links
            .iter()
            .map(|l| format!("{} - {} by u/{}", l.id, l.title, l.author))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_subreddit(&self, sr: &crate::models::Subreddit) -> String {
        format!("r/{} - {} subscribers", sr.display_name, sr.subscribers)
    }

    fn format_user(&self, user: &crate::models::User) -> String {
        format!("u/{} - {} karma", user.name, user.total_karma.unwrap_or(0))
    }
}
```

### 2.8 src/output/table.rs

```rust
use crate::models::{Link, Subreddit, User};

pub struct TableOutput;

impl super::Output for TableOutput {
    fn format_links(&self, links: &[Link]) -> String {
        let mut output = String::new();

        for link in links {
            let score = format!("{:+}", link.score);
            let comments = link.num_comments;
            let subreddit = &link.subreddit;
            let title = if link.title.len() > 80 {
                format!("{}...", &link.title[..77])
            } else {
                link.title.clone()
            };

            let nsfw = if link.over_18 { "[NSFW] " } else { "" };
            let sticky = if link.stickied { "[STICKY] " } else { "" };

            output.push_str(&format!(
                "{}{} {:>6} {:>4}c r/{:<15} {}\n",
                sticky, nsfw, score, comments, subreddit, title
            ));
        }

        output
    }

    fn format_subreddit(&self, sr: &Subreddit) -> String {
        let mut output = String::new();

        output.push_str(&format!("r/{}\n", sr.display_name));
        output.push_str(&format!("{}\n", sr.title));
        output.push_str(&format!("{} subscribers\n", sr.subscribers));
        if let Some(active) = sr.active_user_count {
            output.push_str(&format!("{} online\n", active));
        }
        output.push_str(&format!("\n{}\n", sr.public_description));

        output
    }

    fn format_user(&self, user: &User) -> String {
        let mut output = String::new();

        output.push_str(&format!("u/{}\n", user.name));
        output.push_str(&format!("Link karma: {}\n", user.link_karma));
        output.push_str(&format!("Comment karma: {}\n", user.comment_karma));
        if let Some(total) = user.total_karma {
            output.push_str(&format!("Total karma: {}\n", total));
        }
        output.push_str(&format!("Created: {}\n", user.created_utc.format("%Y-%m-%d")));

        output
    }
}
```

### 2.9 src/output/json.rs

```rust
use crate::models::{Link, Subreddit, User};

pub struct JsonOutput;

impl super::Output for JsonOutput {
    fn format_links(&self, links: &[Link]) -> String {
        serde_json::to_string_pretty(links).unwrap_or_default()
    }

    fn format_subreddit(&self, sr: &Subreddit) -> String {
        serde_json::to_string_pretty(sr).unwrap_or_default()
    }

    fn format_user(&self, user: &User) -> String {
        serde_json::to_string_pretty(user).unwrap_or_default()
    }
}
```

---

## CLI 커맨드 연결 예시

### src/cli/root.rs (업데이트)

```rust
impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        let settings = config::Settings::load()?;
        let client = api::Client::new(&settings)?;
        let output = output::get_output(self.format.parse()?);

        match &self.command {
            Commands::Hot { subreddit } => {
                let listing = api::endpoints::ListingEndpoint::new(&client)
                    .hot(subreddit.as_deref(), Some(self.limit), None)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                println!("{}", output.format_links(&links));
            }
            // ... other commands
        }

        Ok(())
    }
}
```

---

## 완료 기준

1. `cargo run -- hot` - 실제 Reddit 데이터 출력
2. `cargo run -- new --subreddit rust` - 특정 서브레딧 새 글
3. `cargo run -- top --time week --limit 10` - 주간 탑 10개
4. `cargo run -- subreddit show rust` - 서브레딧 정보
5. `cargo run -- user show spez` - 사용자 정보
6. `cargo run -- search "rust cli" --subreddit rust` - 검색
7. `cargo run -- post view 15bfi0` - 게시물 + 댓글
8. `--format json` 옵션 동작

---

## 다음 단계

Phase 2 완료 후 → [PHASE3.md](PHASE3.md): OAuth 인증 구현
