# Phase 5: 메시지 & 모더레이션

> **목표**: 개인 메시지, 모더레이션 기능 구현

**전제조건**: Phase 4 완료

---

## 체크리스트

### 5.1 개인 메시지 API
- [x] `src/api/endpoints/message.rs` - 메시지 API
- [x] `reddit message inbox` 커맨드
- [x] `reddit message unread` 커맨드
- [x] `reddit message sent` 커맨드
- [x] `reddit message send` 커맨드
- [x] `reddit message read` 커맨드
- [x] `reddit message unread` (동사) 커맨드
- [x] `reddit message delete` 커맨드
- [x] `reddit message block` 커맨드

### 5.2 모더레이션 기본 API
- [x] `src/api/endpoints/moderation.rs` - 모더레이션 API
- [x] `reddit mod reports` 커맨드
- [x] `reddit mod spam` 커맨드
- [x] `reddit mod modqueue` 커맨드
- [x] `reddit mod unmoderated` 커맨드
- [x] `reddit mod edited` 커맨드
- [x] `reddit mod log` 커맨드

### 5.3 모더레이션 액션 API
- [x] `reddit mod approve` 커맨드
- [x] `reddit mod remove` 커맨드
- [x] `reddit mod spam` (동사) 커맨드
- [x] `reddit mod distinguish` 커맨드
- [x] `reddit mod sticky` 커맨드
- [x] `reddit mod lock` 커맨드
- [x] `reddit mod nsfw` 커맨드
- [x] `reddit mod report` 커맨드

### 5.4 사용자 관리 API
- [x] `reddit mod ban` 커맨드
- [x] `reddit mod unban` 커맨드
- [x] `reddit mod mute` 커맨드
- [x] `reddit mod unmute` 커맨드
- [x] `reddit mod contributors` 커맨드
- [x] `reddit mod banned` 커맨드
- [x] `reddit mod muted` 커맨드
- [x] `reddit mod moderators` 커맨드

---

## 상세 구현 가이드

### 5.1 src/api/endpoints/message.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct MessageEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub id: String,
    pub name: String,
    pub author: Option<String>,
    pub dest: String,
    pub body: String,
    pub body_html: Option<String>,
    pub subject: String,
    pub subreddit: Option<String>,
    pub created_utc: f64,
    pub new: bool,
    #[serde(rename = "was_comment")]
    pub was_comment: bool,
}

#[derive(Debug, Deserialize)]
pub struct ComposeResponse {
    pub json: ComposeJson,
}

#[derive(Debug, Deserialize)]
pub struct ComposeJson {
    pub errors: Vec<Vec<String>>,
}

pub enum MessageFolder {
    Inbox,
    Unread,
    Sent,
}

impl std::fmt::Display for MessageFolder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageFolder::Inbox => write!(f, "inbox"),
            MessageFolder::Unread => write!(f, "unread"),
            MessageFolder::Sent => write!(f, "sent"),
        }
    }
}

impl<'a> MessageEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get messages from a folder
    pub async fn get(
        &self,
        folder: MessageFolder,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Message>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client
            .get_authenticated_with_query(&format!("/message/{}", folder), &query)
            .await
    }

    /// Send a private message
    pub async fn compose(
        &self,
        to: &str,
        subject: &str,
        text: &str,
        from_subreddit: Option<&str>,
    ) -> Result<ComposeResponse> {
        let mut form = vec![
            ("api_type", "json"),
            ("to", to),
            ("subject", subject),
            ("text", text),
        ];

        if let Some(sr) = from_subreddit {
            form.push(("from_sr", sr));
        }

        self.client
            .post_authenticated("/api/compose", &form)
            .await
    }

    /// Mark messages as read
    pub async fn read(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", &ids_str)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/read_message", &form)
            .await?;

        Ok(())
    }

    /// Mark all messages as read
    pub async fn read_all(&self) -> Result<()> {
        let form: Vec<(&str, &str)> = vec![];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/read_all_messages", &form)
            .await?;

        Ok(())
    }

    /// Mark messages as unread
    pub async fn unread(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", &ids_str)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unread_message", &form)
            .await?;

        Ok(())
    }

    /// Delete a message
    pub async fn delete(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/del_msg", &form)
            .await?;

        Ok(())
    }

    /// Block the author of a message
    pub async fn block(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/block", &form)
            .await?;

        Ok(())
    }

    /// Collapse a message
    pub async fn collapse(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", &ids_str)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/collapse_message", &form)
            .await?;

        Ok(())
    }

    /// Uncollapse a message
    pub async fn uncollapse(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", &ids_str)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/uncollapse_message", &form)
            .await?;

        Ok(())
    }
}
```

### 5.2 src/api/endpoints/moderation.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::{Comment, Link, ListingResponse};
use serde::Deserialize;

pub struct ModerationEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct ModAction {
    pub id: String,
    pub mod: String,
    pub action: String,
    pub target_author: Option<String>,
    pub target_fullname: Option<String>,
    pub target_permalink: Option<String>,
    pub target_title: Option<String>,
    pub details: Option<String>,
    pub description: Option<String>,
    pub created_utc: f64,
    pub subreddit: SubredditInfo,
}

#[derive(Debug, Deserialize)]
pub struct SubredditInfo {
    pub display_name: String,
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Report {
    pub id: String,
    pub name: String,
    pub author: String,
    pub title: Option<String>,
    pub body: Option<String>,
    pub user_reports: Vec<UserReport>,
    pub mod_reports: Vec<ModReport>,
}

#[derive(Debug, Deserialize)]
pub struct UserReport {
    pub reason: String,
    pub count: u32,
}

#[derive(Debug, Deserialize)]
pub struct ModReport {
    pub reason: String,
    pub moderator: String,
}

pub enum ModQueueLocation {
    Reports,
    Spam,
    Modqueue,
    Unmoderated,
    Edited,
}

impl std::fmt::Display for ModQueueLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModQueueLocation::Reports => write!(f, "reports"),
            ModQueueLocation::Spam => write!(f, "spam"),
            ModQueueLocation::Modqueue => write!(f, "modqueue"),
            ModQueueLocation::Unmoderated => write!(f, "unmoderated"),
            ModQueueLocation::Edited => write!(f, "edited"),
        }
    }
}

impl<'a> ModerationEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    // ========== Queue Operations ==========

    /// Get items from mod queue
    pub async fn get_queue(
        &self,
        subreddit: &str,
        location: ModQueueLocation,
        limit: Option<u32>,
        only: Option<&str>, // "links", "comments"
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(o) = only {
            query.push(("only", o));
        }

        self.client
            .get_authenticated_with_query(
                &format!("/r/{}/about/{}", subreddit, location),
                &query,
            )
            .await
    }

    /// Get mod log
    pub async fn log(
        &self,
        subreddit: &str,
        limit: Option<u32>,
        mod_filter: Option<&str>,
        action_type: Option<&str>,
    ) -> Result<ListingResponse<ModAction>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(m) = mod_filter {
            query.push(("mod", m));
        }
        if let Some(t) = action_type {
            query.push(("type", t));
        }

        self.client
            .get_authenticated_with_query(&format!("/r/{}/about/log", subreddit), &query)
            .await
    }

    // ========== Moderation Actions ==========

    /// Approve a post or comment
    pub async fn approve(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/approve", &form)
            .await?;

        Ok(())
    }

    /// Remove a post or comment
    pub async fn remove(&self, id: &str, is_spam: bool) -> Result<()> {
        let form = vec![
            ("id", id),
            ("spam", &is_spam.to_string()),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/remove", &form)
            .await?;

        Ok(())
    }

    /// Distinguish a post or comment (mod tag)
    pub async fn distinguish(
        &self,
        id: &str,
        how: DistinguishType,
        sticky: bool,
    ) -> Result<()> {
        let how_str = match how {
            DistinguishType::Yes => "yes",
            DistinguishType::No => "no",
            DistinguishType::Admin => "admin",
            DistinguishType::Special => "special",
        };

        let form = vec![
            ("api_type", "json"),
            ("how", how_str),
            ("id", id),
            ("sticky", &sticky.to_string()),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/distinguish", &form)
            .await?;

        Ok(())
    }

    /// Set sticky status on a post
    pub async fn sticky(&self, id: &str, state: bool, num: Option<u8>) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("id", id),
            ("state", &state.to_string()),
        ];

        if let Some(n) = num {
            form.push(("num", &n.to_string()));
        }

        let _: serde_json::Value = self.client
            .post_authenticated("/api/set_subreddit_sticky", &form)
            .await?;

        Ok(())
    }

    /// Lock a post or comment
    pub async fn lock(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/lock", &form)
            .await?;

        Ok(())
    }

    /// Unlock a post or comment
    pub async fn unlock(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unlock", &form)
            .await?;

        Ok(())
    }

    /// Mark a post as NSFW
    pub async fn mark_nsfw(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/marknsfw", &form)
            .await?;

        Ok(())
    }

    /// Unmark a post as NSFW
    pub async fn unmark_nsfw(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unmarknsfw", &form)
            .await?;

        Ok(())
    }

    /// Mark as spoiler
    pub async fn spoiler(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/spoiler", &form)
            .await?;

        Ok(())
    }

    /// Unmark as spoiler
    pub async fn unspoiler(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unspoiler", &form)
            .await?;

        Ok(())
    }

    /// Report a post or comment
    pub async fn report(
        &self,
        id: &str,
        reason: &str,
        other_reason: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("thing_id", id),
            ("reason", reason),
        ];

        if let Some(other) = other_reason {
            form.push(("other_reason", other));
        }

        let _: serde_json::Value = self.client
            .post_authenticated("/api/report", &form)
            .await?;

        Ok(())
    }

    /// Set contest mode
    pub async fn contest_mode(&self, id: &str, state: bool) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("id", id),
            ("state", &state.to_string()),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/set_contest_mode", &form)
            .await?;

        Ok(())
    }

    /// Set suggested sort
    pub async fn suggested_sort(&self, id: &str, sort: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("id", id),
            ("sort", sort),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/set_suggested_sort", &form)
            .await?;

        Ok(())
    }

    /// Ignore reports on a thing
    pub async fn ignore_reports(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/ignore_reports", &form)
            .await?;

        Ok(())
    }

    /// Unignore reports on a thing
    pub async fn unignore_reports(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/unignore_reports", &form)
            .await?;

        Ok(())
    }
}

pub enum DistinguishType {
    Yes,
    No,
    Admin,
    Special,
}
```

### 5.3 src/api/endpoints/user_management.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct UserManagementEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct BannedUser {
    pub name: String,
    pub date: f64,
    pub note: Option<String>,
    pub rel_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct MutedUser {
    pub name: String,
    pub date: f64,
    pub mute_reason: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Contributor {
    pub name: String,
    pub date: f64,
}

#[derive(Debug, Deserialize)]
pub struct Moderator {
    pub name: String,
    pub date: f64,
    pub mod_permissions: Vec<String>,
    pub author_flair_text: Option<String>,
}

impl<'a> UserManagementEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Ban a user from a subreddit
    pub async fn ban(
        &self,
        subreddit: &str,
        user: &str,
        duration: Option<u32>,
        reason: Option<&str>,
        note: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "banned"),
        ];

        if let Some(d) = duration {
            form.push(("duration", &d.to_string()));
        }
        if let Some(r) = reason {
            form.push(("ban_reason", r));
        }
        if let Some(n) = note {
            form.push(("note", n));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/friend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Unban a user from a subreddit
    pub async fn unban(&self, subreddit: &str, user: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "banned"),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/unfriend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Mute a user in a subreddit
    pub async fn mute(&self, subreddit: &str, user: &str, note: Option<&str>) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "muted"),
        ];

        if let Some(n) = note {
            form.push(("note", n));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/friend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Unmute a user in a subreddit
    pub async fn unmute(&self, subreddit: &str, user: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "muted"),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/unfriend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Add contributor (approved user)
    pub async fn add_contributor(&self, subreddit: &str, user: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "contributor"),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/friend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Remove contributor
    pub async fn remove_contributor(&self, subreddit: &str, user: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "contributor"),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/unfriend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Get list of banned users
    pub async fn banned(&self, subreddit: &str) -> Result<ListingResponse<BannedUser>> {
        self.client
            .get_authenticated(&format!("/r/{}/about/banned", subreddit))
            .await
    }

    /// Get list of muted users
    pub async fn muted(&self, subreddit: &str) -> Result<ListingResponse<MutedUser>> {
        self.client
            .get_authenticated(&format!("/r/{}/about/muted", subreddit))
            .await
    }

    /// Get list of contributors
    pub async fn contributors(&self, subreddit: &str) -> Result<ListingResponse<Contributor>> {
        self.client
            .get_authenticated(&format!("/r/{}/about/contributors", subreddit))
            .await
    }

    /// Get list of moderators
    pub async fn moderators(&self, subreddit: &str) -> Result<ListingResponse<Moderator>> {
        self.client
            .get_authenticated(&format!("/r/{}/about/moderators", subreddit))
            .await
    }

    /// Invite moderator
    pub async fn invite_moderator(
        &self,
        subreddit: &str,
        user: &str,
        permissions: Option<&[&str]>,
    ) -> Result<()> {
        let perms = permissions
            .map(|p| p.join(","))
            .unwrap_or_else(|| "+all".to_string());

        let form = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "moderator_invite"),
            ("permissions", &perms),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/friend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Leave moderator position
    pub async fn leave_moderator(&self, subreddit: &str) -> Result<()> {
        let form = vec![("api_type", "json")];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/leavemoderator", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Accept moderator invite
    pub async fn accept_moderator_invite(&self, subreddit: &str) -> Result<()> {
        let form = vec![("api_type", "json")];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/accept_moderator_invite", subreddit), &form)
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

    /// Message commands
    #[command(subcommand)]
    Message(MessageCommands),

    /// Moderation commands
    #[command(subcommand)]
    Mod(ModCommands),
}

#[derive(Subcommand)]
pub enum MessageCommands {
    /// View inbox
    Inbox {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View unread messages
    Unread {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View sent messages
    Sent {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// Send a message
    Send {
        /// Recipient username
        #[arg(short, long)]
        to: String,
        /// Subject
        #[arg(short, long)]
        subject: String,
        /// Message body
        #[arg(short, long)]
        text: String,
        /// Send as subreddit
        #[arg(short = 'r', long)]
        from: Option<String>,
    },
    /// Mark messages as read
    Read {
        /// Message IDs
        ids: Vec<String>,
    },
    /// Mark all messages as read
    ReadAll,
    /// Delete a message
    Delete {
        id: String,
    },
    /// Block the sender of a message
    Block {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ModCommands {
    /// View reports
    Reports {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View spam queue
    Spam {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View mod queue
    Queue {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View unmoderated posts
    Unmoderated {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View mod log
    Log {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// Approve a post or comment
    Approve {
        id: String,
    },
    /// Remove a post or comment
    Remove {
        id: String,
        /// Mark as spam
        #[arg(short, long)]
        spam: bool,
        /// Removal reason
        #[arg(short, long)]
        reason: Option<String>,
    },
    /// Distinguish a comment as mod
    Distinguish {
        id: String,
        /// Make sticky (for top-level comments)
        #[arg(short, long)]
        sticky: bool,
    },
    /// Undistinguish
    Undistinguish {
        id: String,
    },
    /// Sticky a post
    Sticky {
        id: String,
        /// Slot number (1-4)
        #[arg(short, long)]
        slot: Option<u8>,
    },
    /// Unsticky a post
    Unsticky {
        id: String,
    },
    /// Lock a post or comment
    Lock {
        id: String,
    },
    /// Unlock a post or comment
    Unlock {
        id: String,
    },
    /// Mark post as NSFW
    Nsfw {
        id: String,
    },
    /// Unmark post as NSFW
    Unnsfw {
        id: String,
    },
    /// Ban a user
    Ban {
        subreddit: String,
        /// Username
        #[arg(short, long)]
        user: String,
        /// Ban duration in days (None = permanent)
        #[arg(short, long)]
        days: Option<u32>,
        /// Reason
        #[arg(short, long)]
        reason: Option<String>,
        /// Mod note
        #[arg(short, long)]
        note: Option<String>,
    },
    /// Unban a user
    Unban {
        subreddit: String,
        #[arg(short, long)]
        user: String,
    },
    /// Mute a user
    Mute {
        subreddit: String,
        #[arg(short, long)]
        user: String,
    },
    /// Unmute a user
    Unmute {
        subreddit: String,
        #[arg(short, long)]
        user: String,
    },
    /// List banned users
    Banned {
        subreddit: String,
    },
    /// List muted users
    Muted {
        subreddit: String,
    },
    /// List moderators
    Mods {
        subreddit: String,
    },
}
```

---

## 완료 기준

1. `reddit message inbox` - 받은 메시지
2. `reddit message send --to user --subject "Hi" --text "Hello"` - 메시지 보내기
3. `reddit mod reports mysub` - 신고 목록
4. `reddit mod approve t3_xxx` - 승인
5. `reddit mod remove t3_xxx` - 삭제
6. `reddit mod ban mysub --user troll --days 7` - 밴
7. `reddit mod mods mysub` - 모더레이터 목록

---

## 다음 단계

Phase 5 완료 후 → [PHASE6.md](PHASE6.md): 고급 기능 구현
