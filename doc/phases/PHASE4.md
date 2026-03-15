# Phase 4: 쓰기 API

> **목표**: OAuth 인증이 필요한 쓰기 작업 구현 (투표, 저장, 구독, 게시물/댓글 작성)

**전제조건**: Phase 3 완료

---

## 체크리스트

### 4.1 투표 API
- [x] `src/api/endpoints/vote.rs` - 투표 API
- [x] `reddit upvote <id>` 커맨드
- [x] `reddit downvote <id>` 커맨드
- [x] `reddit unvote <id>` 커맨드

### 4.2 저장/숨기기 API
- [x] `src/api/endpoints/save.rs` - 저장/숨기기 API
- [x] `reddit save <id>` 커맨드
- [x] `reddit unsave <id>` 커맨드
- [x] `reddit hide <id>` 커맨드
- [x] `reddit unhide <id>` 커맨드

### 4.3 구독 API
- [x] `src/api/endpoints/subscribe.rs` - 구독 API
- [x] `reddit subscribe <subreddit>` 커맨드
- [x] `reddit unsubscribe <subreddit>` 커맨드

### 4.4 게시물 작성 API
- [x] `src/api/endpoints/submit.rs` - 게시물 작성 API
- [x] `reddit submit link` 커맨드
- [x] `reddit submit text` 커맨드
- [x] `reddit edit <id>` 커맨드
- [x] `reddit delete <id>` 커맨드

### 4.5 댓글 API
- [x] `src/api/endpoints/comment.rs` - 댓글 API
- [x] `reddit comment <parent>` 커맨드
- [x] `reddit comment edit <id>` 커맨드
- [x] `reddit comment delete <id>` 커맨드

### 4.6 팔로우 API
- [x] `src/api/endpoints/follow.rs` - 팔로우 API
- [x] `reddit follow <post_id>` 커맨드
- [x] `reddit unfollow <post_id>` 커맨드

---

## 상세 구현 가이드

### 4.1 src/api/endpoints/vote.rs

```rust
use crate::api::Client;
use crate::error::{RedditError, Result};
use serde::Deserialize;

pub struct VoteEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct VoteResponse {
    pub success: bool,
}

impl<'a> VoteEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Upvote a post or comment
    /// id: fullname (t3_xxx for posts, t1_xxx for comments)
    pub async fn upvote(&self, id: &str) -> Result<()> {
        self.vote(id, 1).await
    }

    /// Downvote a post or comment
    pub async fn downvote(&self, id: &str) -> Result<()> {
        self.vote(id, -1).await
    }

    /// Remove vote (unvote)
    pub async fn unvote(&self, id: &str) -> Result<()> {
        self.vote(id, 0).await
    }

    async fn vote(&self, id: &str, dir: i8) -> Result<()> {
        let form = vec![
            ("id", id),
            ("dir", &dir.to_string()),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/vote", &form)
            .await?;

        Ok(())
    }
}
```

### 4.2 src/api/endpoints/save.rs

```rust
use crate::api::Client;
use crate::error::Result;

pub struct SaveEndpoint<'a> {
    client: &'a Client,
}

impl<'a> SaveEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Save a post or comment
    pub async fn save(&self, id: &str, category: Option<&str>) -> Result<()> {
        let mut form = vec![("id", id)];
        let category_val;
        if let Some(cat) = category {
            category_val = cat.to_string();
            form.push(("category", &category_val));
        }

        let _: serde_json::Value = self.client
            .post_authenticated("/api/save", &form)
            .await?;

        Ok(())
    }

    /// Unsave a post or comment
    pub async fn unsave(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unsave", &form)
            .await?;

        Ok(())
    }

    /// Hide a post
    pub async fn hide(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/hide", &form)
            .await?;

        Ok(())
    }

    /// Unhide a post
    pub async fn unhide(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unhide", &form)
            .await?;

        Ok(())
    }

    /// Get saved categories
    pub async fn categories(&self) -> Result<Vec<String>> {
        let result: serde_json::Value = self.client
            .get_authenticated("/api/saved_categories")
            .await?;

        // Parse categories from response
        Ok(vec![])
    }
}
```

### 4.3 src/api/endpoints/subscribe.rs

```rust
use crate::api::Client;
use crate::error::Result;

pub struct SubscribeEndpoint<'a> {
    client: &'a Client,
}

impl<'a> SubscribeEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Subscribe to a subreddit
    pub async fn subscribe(&self, subreddit: &str) -> Result<()> {
        let fullname = format!("t5_{}", subreddit); // or use proper fullname
        let form = vec![
            ("action", "sub"),
            ("sr_name", subreddit),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }

    /// Unsubscribe from a subreddit
    pub async fn unsubscribe(&self, subreddit: &str) -> Result<()> {
        let form = vec![
            ("action", "unsub"),
            ("sr_name", subreddit),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }

    /// Subscribe to multiple subreddits at once
    pub async fn subscribe_multiple(&self, subreddits: &[&str]) -> Result<()> {
        let sr_names = subreddits.join(",");
        let form = vec![
            ("action", "sub"),
            ("sr_name", &sr_names),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }
}
```

### 4.4 src/api/endpoints/submit.rs

```rust
use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

pub struct SubmitEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct SubmitResponse {
    pub json: SubmitJson,
}

#[derive(Debug, Deserialize)]
pub struct SubmitJson {
    pub data: SubmitData,
    pub errors: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitData {
    pub url: String,
    pub id: String,
    pub name: String,
}

pub enum SubmitKind {
    Link,
    SelfPost,
    Image,
    Video,
    VideoGif,
}

impl std::fmt::Display for SubmitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitKind::Link => write!(f, "link"),
            SubmitKind::SelfPost => write!(f, "self"),
            SubmitKind::Image => write!(f, "image"),
            SubmitKind::Video => write!(f, "video"),
            SubmitKind::VideoGif => write!(f, "videogif"),
        }
    }
}

pub struct SubmitOptions {
    pub subreddit: String,
    pub title: String,
    pub kind: SubmitKind,
    pub url: Option<String>,
    pub text: Option<String>,
    pub flair_id: Option<String>,
    pub flair_text: Option<String>,
    pub nsfw: bool,
    pub spoiler: bool,
    pub send_replies: bool,
}

impl<'a> SubmitEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Submit a post
    pub async fn submit(&self, options: &SubmitOptions) -> Result<SubmitResponse> {
        let mut form = vec![
            ("api_type", "json"),
            ("sr", &options.subreddit),
            ("title", &options.title),
            ("kind", &options.kind.to_string()),
            ("resubmit", "true"),
            ("sendreplies", &options.send_replies.to_string()),
        ];

        if let Some(ref url) = options.url {
            form.push(("url", url));
        }
        if let Some(ref text) = options.text {
            form.push(("text", text));
        }
        if let Some(ref flair_id) = options.flair_id {
            form.push(("flair_id", flair_id));
        }
        if let Some(ref flair_text) = options.flair_text {
            form.push(("flair_text", flair_text));
        }
        if options.nsfw {
            form.push(("nsfw", "true"));
        }
        if options.spoiler {
            form.push(("spoiler", "true"));
        }

        self.client
            .post_authenticated("/api/submit", &form)
            .await
    }

    /// Edit a post or comment
    pub async fn edit(&self, id: &str, text: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("thing_id", id),
            ("text", text),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/editusertext", &form)
            .await?;

        Ok(())
    }

    /// Delete a post or comment
    pub async fn delete(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/del", &form)
            .await?;

        Ok(())
    }
}
```

### 4.5 src/api/endpoints/comment.rs

```rust
use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

pub struct CommentEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct CommentResponse {
    pub json: CommentJson,
}

#[derive(Debug, Deserialize)]
pub struct CommentJson {
    pub data: CommentData,
    pub errors: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CommentData {
    pub things: Vec<ThingData>,
}

#[derive(Debug, Deserialize)]
pub struct ThingData {
    pub data: CommentThingData,
}

#[derive(Debug, Deserialize)]
pub struct CommentThingData {
    pub id: String,
    pub name: String,
    pub link_id: String,
}

impl<'a> CommentEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Submit a new comment
    /// parent: fullname of the thing being replied to
    ///   - t3_xxx for post (top-level comment)
    ///   - t1_xxx for comment (reply)
    pub async fn submit(&self, parent: &str, text: &str) -> Result<CommentResponse> {
        let form = vec![
            ("api_type", "json"),
            ("thing_id", parent),
            ("text", text),
        ];

        self.client
            .post_authenticated("/api/comment", &form)
            .await
    }

    /// Edit a comment
    pub async fn edit(&self, id: &str, text: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("thing_id", id),
            ("text", text),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/editusertext", &form)
            .await?;

        Ok(())
    }

    /// Delete a comment
    pub async fn delete(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/del", &form)
            .await?;

        Ok(())
    }

    /// Enable/disable inbox replies for a post or comment
    pub async fn set_inbox_replies(&self, id: &str, enabled: bool) -> Result<()> {
        let form = vec![
            ("id", id),
            ("state", &enabled.to_string()),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/sendreplies", &form)
            .await?;

        Ok(())
    }
}
```

### 4.6 src/api/endpoints/follow.rs

```rust
use crate::api::Client;
use crate::error::Result;

pub struct FollowEndpoint<'a> {
    client: &'a Client,
}

impl<'a> FollowEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Follow a post
    pub async fn follow(&self, post_id: &str) -> Result<()> {
        let form = vec![
            ("follow", "true"),
            ("fullname", post_id),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/follow_post", &form)
            .await?;

        Ok(())
    }

    /// Unfollow a post
    pub async fn unfollow(&self, post_id: &str) -> Result<()> {
        let form = vec![
            ("follow", "false"),
            ("fullname", post_id),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/follow_post", &form)
            .await?;

        Ok(())
    }
}
```

---

## CLI 커맨드 구현

### src/cli/root.rs (업데이트)

```rust
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands

    /// Upvote a post or comment
    Upvote {
        /// Fullname (t3_xxx or t1_xxx)
        id: String,
    },
    /// Downvote a post or comment
    Downvote {
        id: String,
    },
    /// Remove vote from a post or comment
    Unvote {
        id: String,
    },
    /// Save a post or comment
    Save {
        id: String,
        /// Category for saved item
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Unsave a post or comment
    Unsave {
        id: String,
    },
    /// Hide a post
    Hide {
        id: String,
    },
    /// Unhide a post
    Unhide {
        id: String,
    },
    /// Subscribe to a subreddit
    Subscribe {
        /// Subreddit name (without r/)
        subreddit: String,
    },
    /// Unsubscribe from a subreddit
    Unsubscribe {
        subreddit: String,
    },
    /// Submit a post
    Submit {
        #[command(subcommand)]
        command: SubmitCommands,
    },
    /// Comment on a post or reply to a comment
    Comment {
        /// Parent fullname (t3_xxx for post, t1_xxx for comment)
        parent: String,
        /// Comment text (markdown)
        #[arg(short, long)]
        text: String,
    },
    /// Edit a post or comment
    Edit {
        /// Fullname of the thing to edit
        id: String,
        /// New text (markdown)
        #[arg(short, long)]
        text: String,
    },
    /// Delete a post or comment
    Delete {
        id: String,
    },
    /// Follow a post
    Follow {
        /// Post fullname (t3_xxx)
        id: String,
    },
    /// Unfollow a post
    Unfollow {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum SubmitCommands {
    /// Submit a link post
    Link {
        /// Subreddit name
        #[arg(short = 'r', long)]
        subreddit: String,
        /// Post title
        #[arg(short, long)]
        title: String,
        /// URL
        #[arg(short, long)]
        url: String,
        /// Mark as NSFW
        #[arg(long)]
        nsfw: bool,
        /// Mark as spoiler
        #[arg(long)]
        spoiler: bool,
    },
    /// Submit a self (text) post
    Self {
        #[arg(short = 'r', long)]
        subreddit: String,
        #[arg(short, long)]
        title: String,
        /// Post body (markdown)
        #[arg(short, long)]
        text: Option<String>,
        /// Read text from file
        #[arg(short = 'f', long)]
        file: Option<String>,
        #[arg(long)]
        nsfw: bool,
        #[arg(long)]
        spoiler: bool,
    },
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        // ...

        match &self.command {
            Commands::Upvote { id } => {
                ensure_authenticated()?;
                api::endpoints::VoteEndpoint::new(&client)
                    .upvote(id).await?;
                println!("Upvoted {}", id);
            }

            Commands::Downvote { id } => {
                ensure_authenticated()?;
                api::endpoints::VoteEndpoint::new(&client)
                    .downvote(id).await?;
                println!("Downvoted {}", id);
            }

            Commands::Unvote { id } => {
                ensure_authenticated()?;
                api::endpoints::VoteEndpoint::new(&client)
                    .unvote(id).await?;
                println!("Vote removed from {}", id);
            }

            Commands::Save { id, category } => {
                ensure_authenticated()?;
                api::endpoints::SaveEndpoint::new(&client)
                    .save(id, category.as_deref()).await?;
                println!("Saved {}", id);
            }

            Commands::Unsave { id } => {
                ensure_authenticated()?;
                api::endpoints::SaveEndpoint::new(&client)
                    .unsave(id).await?;
                println!("Unsaved {}", id);
            }

            Commands::Hide { id } => {
                ensure_authenticated()?;
                api::endpoints::SaveEndpoint::new(&client)
                    .hide(id).await?;
                println!("Hidden {}", id);
            }

            Commands::Unhide { id } => {
                ensure_authenticated()?;
                api::endpoints::SaveEndpoint::new(&client)
                    .unhide(id).await?;
                println!("Unhidden {}", id);
            }

            Commands::Subscribe { subreddit } => {
                ensure_authenticated()?;
                api::endpoints::SubscribeEndpoint::new(&client)
                    .subscribe(subreddit).await?;
                println!("Subscribed to r/{}", subreddit);
            }

            Commands::Unsubscribe { subreddit } => {
                ensure_authenticated()?;
                api::endpoints::SubscribeEndpoint::new(&client)
                    .unsubscribe(subreddit).await?;
                println!("Unsubscribed from r/{}", subreddit);
            }

            Commands::Submit { command } => {
                ensure_authenticated()?;
                match command {
                    SubmitCommands::Link { subreddit, title, url, nsfw, spoiler } => {
                        let result = api::endpoints::SubmitEndpoint::new(&client)
                            .submit(&api::endpoints::submit::SubmitOptions {
                                subreddit: subreddit.clone(),
                                title: title.clone(),
                                kind: api::endpoints::submit::SubmitKind::Link,
                                url: Some(url.clone()),
                                text: None,
                                flair_id: None,
                                flair_text: None,
                                nsfw: *nsfw,
                                spoiler: *spoiler,
                                send_replies: true,
                            }).await?;
                        println!("Posted: {}", result.json.data.url);
                    }
                    SubmitCommands::Self { subreddit, title, text, file, nsfw, spoiler } => {
                        let body = if let Some(path) = file {
                            std::fs::read_to_string(path)?
                        } else {
                            text.clone().unwrap_or_default()
                        };

                        let result = api::endpoints::SubmitEndpoint::new(&client)
                            .submit(&api::endpoints::submit::SubmitOptions {
                                subreddit: subreddit.clone(),
                                title: title.clone(),
                                kind: api::endpoints::submit::SubmitKind::SelfPost,
                                url: None,
                                text: Some(body),
                                flair_id: None,
                                flair_text: None,
                                nsfw: *nsfw,
                                spoiler: *spoiler,
                                send_replies: true,
                            }).await?;
                        println!("Posted: {}", result.json.data.url);
                    }
                }
            }

            Commands::Comment { parent, text } => {
                ensure_authenticated()?;
                let result = api::endpoints::CommentEndpoint::new(&client)
                    .submit(parent, text).await?;
                println!("Comment posted: {}", result.json.data.things[0].data.name);
            }

            Commands::Edit { id, text } => {
                ensure_authenticated()?;
                api::endpoints::SubmitEndpoint::new(&client)
                    .edit(id, text).await?;
                println!("Edited {}", id);
            }

            Commands::Delete { id } => {
                ensure_authenticated()?;
                api::endpoints::SubmitEndpoint::new(&client)
                    .delete(id).await?;
                println!("Deleted {}", id);
            }

            Commands::Follow { id } => {
                ensure_authenticated()?;
                api::endpoints::FollowEndpoint::new(&client)
                    .follow(id).await?;
                println!("Following {}", id);
            }

            Commands::Unfollow { id } => {
                ensure_authenticated()?;
                api::endpoints::FollowEndpoint::new(&client)
                    .unfollow(id).await?;
                println!("Unfollowed {}", id);
            }

            // ... other commands
        }

        Ok(())
    }
}

fn ensure_authenticated() -> anyhow::Result<()> {
    if api::cache::token::CachedToken::load()?.is_none() {
        anyhow::bail!("Not authenticated. Run `reddit auth login` first.");
    }
    Ok(())
}
```

---

## 완료 기준

1. `reddit upvote t3_15bfi0` - 업보트
2. `reddit downvote t3_15bfi0` - 다운보트
3. `reddit save t3_15bfi0` - 저장
4. `reddit subscribe rust` - 서브레딧 구독
5. `reddit submit link -r rust -t "Title" -u "https://..."` - 링크 게시
6. `reddit submit self -r rust -t "Title" --text "Body"` - 셀프 포스트
7. `reddit comment t3_15bfi0 --text "Great post!"` - 댓글 작성
8. `reddit edit t1_xxx --text "Updated text"` - 수정
9. `reddit delete t3_xxx` - 삭제

---

## 다음 단계

Phase 4 완료 후 → [PHASE5.md](PHASE5.md): 메시지 & 모더레이션 구현
