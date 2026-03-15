# Phase 6: 고급 기능

> **목표**: 플레어, 위키, 멀티레딧, 라이브 스레드, 컬렉션, 모드메일, 모드 노트 구현

**전제조건**: Phase 5 완료

---

## 체크리스트

### 6.1 플레어 API
- [ ] `src/api/endpoints/flair.rs` - 플레어 API
- [ ] `reddit flair list` 커맨드
- [ ] `reddit flair set` 커맨드
- [ ] `reddit flair delete` 커맨드
- [ ] `reddit flair templates` 커맨드
- [ ] `reddit link-flair templates` 커맨드

### 6.2 위키 API
- [ ] `src/api/endpoints/wiki.rs` - 위키 API
- [ ] `reddit wiki pages` 커맨드
- [ ] `reddit wiki view` 커맨드
- [ ] `reddit wiki revisions` 커맨드
- [ ] `reddit wiki edit` 커맨드

### 6.3 멀티레딧 API
- [ ] `src/api/endpoints/multi.rs` - 멀티레딧 API
- [ ] `reddit multi list` 커맨드
- [ ] `reddit multi show` 커맨드
- [ ] `reddit multi create` 커맨드
- [ ] `reddit multi delete` 커맨드
- [ ] `reddit multi add` 커맨드
- [ ] `reddit multi remove` 커맨드

### 6.4 라이브 스레드 API
- [ ] `src/api/endpoints/live.rs` - 라이브 스레드 API
- [ ] `reddit live show` 커맨드
- [ ] `reddit live about` 커맨드
- [ ] `reddit live contributors` 커맨드
- [ ] `reddit live create` 커맨드
- [ ] `reddit live update` 커맨드

### 6.5 컬렉션 API
- [ ] `src/api/endpoints/collection.rs` - 컬렉션 API
- [ ] `reddit collection show` 커맨드
- [ ] `reddit collection list` 커맨드
- [ ] `reddit collection create` 커맨드
- [ ] `reddit collection add` 커맨드
- [ ] `reddit collection remove` 커맨드

### 6.6 모드메일 API
- [ ] `src/api/endpoints/modmail.rs` - 모드메일 API
- [ ] `reddit modmail list` 커맨드
- [ ] `reddit modmail show` 커맨드
- [ ] `reddit modmail reply` 커맨드
- [ ] `reddit modmail create` 커맨드
- [ ] `reddit modmail archive` 커맨드
- [ ] `reddit modmail highlight` 커맨드
- [ ] `reddit modmail mute` 커맨드
- [ ] `reddit modmail ban` 커맨드

### 6.7 모드 노트 API
- [ ] `src/api/endpoints/modnote.rs` - 모드 노트 API
- [ ] `reddit modnote show` 커맨드
- [ ] `reddit modnote add` 커맨드
- [ ] `reddit modnote delete` 커맨드

---

## 상세 구현 가이드

### 6.1 src/api/endpoints/flair.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct FlairEndpoint<'a> {
    client: &'a Client,
    subreddit: String,
}

#[derive(Debug, Deserialize)]
pub struct Flair {
    pub user: String,
    pub flair_text: Option<String>,
    pub flair_css_class: Option<String>,
    pub flair_template_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FlairTemplate {
    pub id: String,
    pub text: String,
    pub text_color: String,
    pub background_color: String,
    pub css_class: Option<String>,
    pub text_editable: bool,
    pub mod_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct FlairSelectorResponse {
    pub choices: Vec<FlairChoice>,
    pub current: Option<CurrentFlair>,
}

#[derive(Debug, Deserialize)]
pub struct FlairChoice {
    pub flair_template_id: String,
    pub flair_text: Option<String>,
    pub flair_css_class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentFlair {
    pub flair_text: Option<String>,
    pub flair_css_class: Option<String>,
    pub flair_template_id: Option<String>,
}

impl<'a> FlairEndpoint<'a> {
    pub fn new(client: &'a Client, subreddit: &str) -> Self {
        Self {
            client,
            subreddit: subreddit.to_string(),
        }
    }

    /// Get list of user flairs in subreddit
    pub async fn list(
        &self,
        limit: Option<u32>,
        after: Option<&str>,
        user: Option<&str>,
    ) -> Result<ListingResponse<Flair>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }
        if let Some(u) = user {
            query.push(("name", u));
        }

        self.client
            .get_authenticated_with_query(&format!("/r/{}/api/flairlist", self.subreddit), &query)
            .await
    }

    /// Set user flair
    pub async fn set_user_flair(
        &self,
        user: &str,
        text: Option<&str>,
        css_class: Option<&str>,
        template_id: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("name", user),
        ];

        if let Some(t) = text {
            form.push(("text", t));
        }
        if let Some(c) = css_class {
            form.push(("css_class", c));
        }
        if let Some(t) = template_id {
            form.push(("flair_template_id", t));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/flair", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Delete user flair
    pub async fn delete_user_flair(&self, user: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("name", user),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/deleteflair", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Get user flair options
    pub async fn user_flair_selector(&self) -> Result<FlairSelectorResponse> {
        self.client
            .post_authenticated_empty(&format!("/r/{}/api/flairselector", self.subreddit))
            .await
    }

    /// Get link flair options for a post
    pub async fn link_flair_selector(&self, link: &str) -> Result<FlairSelectorResponse> {
        let form = vec![("link", link)];

        self.client
            .post_authenticated(&format!("/r/{}/api/flairselector", self.subreddit), &form)
            .await
    }

    /// Get all user flair templates (v2)
    pub async fn user_flair_templates(&self) -> Result<Vec<FlairTemplate>> {
        self.client
            .get_authenticated(&format!("/r/{}/api/user_flair_v2", self.subreddit))
            .await
    }

    /// Get all link flair templates (v2)
    pub async fn link_flair_templates(&self) -> Result<Vec<FlairTemplate>> {
        self.client
            .get_authenticated(&format!("/r/{}/api/link_flair_v2", self.subreddit))
            .await
    }

    /// Select flair for user
    pub async fn select_user_flair(
        &self,
        template_id: &str,
        text: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("flair_template_id", template_id),
        ];

        if let Some(t) = text {
            form.push(("text", t));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/selectflair", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Select flair for a post
    pub async fn select_link_flair(
        &self,
        link: &str,
        template_id: &str,
        text: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("link", link),
            ("flair_template_id", template_id),
        ];

        if let Some(t) = text {
            form.push(("text", t));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/selectflair", self.subreddit), &form)
            .await?;

        Ok(())
    }
}
```

### 6.2 src/api/endpoints/wiki.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct WikiEndpoint<'a> {
    client: &'a Client,
    subreddit: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiPage {
    pub content: String,
    pub content_html: Option<String>,
    pub may_revise: bool,
    pub rev_id: String,
    pub revision_by: WikiUser,
    pub revisions_seen: u32,
}

#[derive(Debug, Deserialize)]
pub struct WikiUser {
    pub data: WikiUserData,
}

#[derive(Debug, Deserialize)]
pub struct WikiUserData {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiRevision {
    pub id: String,
    pub timestamp: f64,
    pub reason: Option<String>,
    pub author: WikiUser,
    pub page: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiPageList {
    pub data: Vec<String>,
    pub kind: String,
}

impl<'a> WikiEndpoint<'a> {
    pub fn new(client: &'a Client, subreddit: &str) -> Self {
        Self {
            client,
            subreddit: subreddit.to_string(),
        }
    }

    /// Get list of wiki pages
    pub async fn pages(&self) -> Result<WikiPageList> {
        self.client
            .get_authenticated(&format!("/r/{}/wiki/pages", self.subreddit))
            .await
    }

    /// Get wiki page content
    pub async fn page(&self, page: &str) -> Result<WikiPage> {
        self.client
            .get_authenticated(&format!("/r/{}/wiki/{}", self.subreddit, page))
            .await
    }

    /// Get wiki page at specific revision
    pub async fn page_revision(&self, page: &str, revision: &str) -> Result<WikiPage> {
        let query = vec![("v", revision)];

        self.client
            .get_authenticated_with_query(&format!("/r/{}/wiki/{}", self.subreddit, page), &query)
            .await
    }

    /// Get diff between two revisions
    pub async fn diff(&self, page: &str, v1: &str, v2: &str) -> Result<String> {
        let query = vec![("v", v1), ("v2", v2)];

        // Returns HTML diff
        self.client
            .get_authenticated_with_query(&format!("/r/{}/wiki/{}", self.subreddit, page), &query)
            .await
    }

    /// Get revision history for a page
    pub async fn revisions(
        &self,
        page: &str,
        limit: Option<u32>,
    ) -> Result<ListingResponse<WikiRevision>> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };

        self.client
            .get_authenticated_with_query(&format!("/r/{}/wiki/revisions/{}", self.subreddit, page), &query)
            .await
    }

    /// Get recent wiki changes across all pages
    pub async fn recent_changes(&self, limit: Option<u32>) -> Result<ListingResponse<WikiRevision>> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };

        self.client
            .get_authenticated_with_query(&format!("/r/{}/wiki/revisions", self.subreddit), &query)
            .await
    }

    /// Edit a wiki page
    pub async fn edit(
        &self,
        page: &str,
        content: &str,
        reason: Option<&str>,
        previous: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("content", content),
            ("page", page),
        ];

        if let Some(r) = reason {
            form.push(("reason", r));
        }
        if let Some(p) = previous {
            form.push(("previous", p));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/wiki/edit", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Hide a wiki revision
    pub async fn hide_revision(&self, page: &str, revision: &str) -> Result<()> {
        let form = vec![
            ("page", page),
            ("revision", revision),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/wiki/hide", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Revert to a revision
    pub async fn revert(&self, page: &str, revision: &str) -> Result<()> {
        let form = vec![
            ("page", page),
            ("revision", revision),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/wiki/revert", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Get wiki page settings
    pub async fn settings(&self, page: &str) -> Result<WikiSettings> {
        self.client
            .get_authenticated(&format!("/r/{}/wiki/settings/{}", self.subreddit, page))
            .await
    }

    /// Update wiki page settings
    pub async fn update_settings(
        &self,
        page: &str,
        perm_level: u8,
        listed: bool,
    ) -> Result<()> {
        let form = vec![
            ("page", page),
            ("permlevel", &perm_level.to_string()),
            ("listed", &listed.to_string()),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/wiki/settings/{}", self.subreddit, page), &form)
            .await?;

        Ok(())
    }

    /// Allow user to edit wiki page
    pub async fn allow_editor(&self, page: &str, user: &str) -> Result<()> {
        let form = vec![
            ("act", "add"),
            ("page", page),
            ("username", user),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/wiki/alloweditor/add", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Remove user's wiki edit permission
    pub async fn disallow_editor(&self, page: &str, user: &str) -> Result<()> {
        let form = vec![
            ("act", "del"),
            ("page", page),
            ("username", user),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/wiki/alloweditor/del", self.subreddit), &form)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
pub struct WikiSettings {
    pub permlevel: u8,
    pub listed: bool,
}
```

### 6.3 src/api/endpoints/multi.rs

```rust
use crate::api::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub struct MultiEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct Multi {
    pub data: MultiData,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct MultiData {
    pub path: String,
    pub display_name: String,
    pub description_md: Option<String>,
    pub icon_name: Option<String>,
    pub key_color: Option<String>,
    pub visibility: String,
    pub subreddits: Vec<MultiSubreddit>,
    pub owner: Option<String>,
    pub owner_id: Option<String>,
    pub num_subscribers: Option<u64>,
    pub created_utc: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct MultiSubreddit {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateMultiRequest {
    pub display_name: String,
    pub subreddits: Vec<MultiSubredditInput>,
    pub description_md: Option<String>,
    pub icon_name: Option<String>,
    pub key_color: Option<String>,
    pub visibility: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MultiSubredditInput {
    pub name: String,
}

impl<'a> MultiEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get user's multis
    pub async fn mine(&self) -> Result<Vec<Multi>> {
        self.client
            .get_authenticated("/api/multi/mine")
            .await
    }

    /// Get public multis for a user
    pub async fn user(&self, username: &str) -> Result<Vec<Multi>> {
        self.client
            .get_authenticated(&format!("/api/multi/user/{}", username))
            .await
    }

    /// Get a specific multi
    pub async fn get(&self, path: &str) -> Result<Multi> {
        self.client
            .get_authenticated(&format!("/api/multi/{}", path))
            .await
    }

    /// Create a multi
    pub async fn create(&self, path: &str, request: &CreateMultiRequest) -> Result<Multi> {
        let model = serde_json::to_string(request)?;

        let form = vec![
            ("model", &model),
        ];

        self.client
            .post_authenticated(&format!("/api/multi/{}", path), &form)
            .await
    }

    /// Update a multi
    pub async fn update(&self, path: &str, request: &CreateMultiRequest) -> Result<Multi> {
        let model = serde_json::to_string(request)?;

        let form = vec![
            ("model", &model),
        ];

        self.client
            .put_authenticated(&format!("/api/multi/{}", path), &form)
            .await
    }

    /// Delete a multi
    pub async fn delete(&self, path: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .delete_authenticated(&format!("/api/multi/{}", path))
            .await?;

        Ok(())
    }

    /// Copy a multi
    pub async fn copy(&self, from: &str, to: &str, display_name: Option<&str>) -> Result<Multi> {
        let mut form = vec![
            ("from", from),
            ("to", to),
        ];

        if let Some(name) = display_name {
            form.push(("display_name", name));
        }

        self.client
            .post_authenticated("/api/multi/copy", &form)
            .await
    }

    /// Add subreddit to multi
    pub async fn add_subreddit(&self, multi_path: &str, subreddit: &str) -> Result<()> {
        let model = serde_json::json!({"name": subreddit}).to_string();

        let form = vec![("model", &model)];

        let _: serde_json::Value = self.client
            .put_authenticated(&format!("/api/multi/{}/r/{}", multi_path, subreddit), &form)
            .await?;

        Ok(())
    }

    /// Remove subreddit from multi
    pub async fn remove_subreddit(&self, multi_path: &str, subreddit: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .delete_authenticated(&format!("/api/multi/{}/r/{}", multi_path, subreddit))
            .await?;

        Ok(())
    }

    /// Get multi description
    pub async fn description(&self, path: &str) -> Result<String> {
        let result: MultiDescription = self.client
            .get_authenticated(&format!("/api/multi/{}/description", path))
            .await?;

        Ok(result.body_md.unwrap_or_default())
    }

    /// Update multi description
    pub async fn update_description(&self, path: &str, description: &str) -> Result<()> {
        let model = serde_json::json!({"body_md": description}).to_string();

        let form = vec![("model", &model)];

        let _: serde_json::Value = self.client
            .put_authenticated(&format!("/api/multi/{}/description", path), &form)
            .await?;

        Ok(())
    }
}

#[derive(Debug, Deserialize)]
struct MultiDescription {
    body_md: Option<String>,
}
```

### 6.4 src/api/endpoints/live.rs

```rust
use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct LiveEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct LiveThread {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub resources: Option<String>,
    pub state: String,
    pub created_utc: f64,
    pub websocket_url: Option<String>,
    pub viewer_count: Option<u64>,
    pub nsfw: bool,
}

#[derive(Debug, Deserialize)]
pub struct LiveUpdate {
    pub id: String,
    pub name: String,
    pub body: String,
    pub body_html: String,
    pub author: String,
    pub created_utc: f64,
    pub embeds: Vec<serde_json::Value>,
    pub stricken: bool,
}

#[derive(Debug, Deserialize)]
pub struct LiveContributor {
    pub name: String,
    pub id: String,
    pub permissions: Vec<String>,
}

impl<'a> LiveEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get live thread info
    pub async fn about(&self, thread_id: &str) -> Result<LiveThread> {
        let result: LiveThreadResponse = self.client
            .get_authenticated(&format!("/live/{}/about", thread_id))
            .await?;

        Ok(result.data)
    }

    /// Get live thread updates
    pub async fn updates(
        &self,
        thread_id: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<LiveUpdate>> {
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client
            .get_authenticated_with_query(&format!("/live/{}", thread_id), &query)
            .await
    }

    /// Get contributors
    pub async fn contributors(&self, thread_id: &str) -> Result<Vec<LiveContributor>> {
        let result: LiveContributorsResponse = self.client
            .get_authenticated(&format!("/live/{}/contributors", thread_id))
            .await?;

        Ok(result.data.contributors)
    }

    /// Get discussions linking to live thread
    pub async fn discussions(
        &self,
        thread_id: &str,
        limit: Option<u32>,
    ) -> Result<serde_json::Value> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };

        self.client
            .get_authenticated_with_query(&format!("/live/{}/discussions", thread_id), &query)
            .await
    }

    /// Create a live thread
    pub async fn create(
        &self,
        title: &str,
        description: Option<&str>,
        resources: Option<&str>,
        nsfw: bool,
    ) -> Result<LiveThread> {
        let mut form = vec![
            ("api_type", "json"),
            ("title", title),
            ("nsfw", &nsfw.to_string()),
        ];

        if let Some(d) = description {
            form.push(("description", d));
        }
        if let Some(r) = resources {
            form.push(("resources", r));
        }

        self.client
            .post_authenticated("/api/live/create", &form)
            .await
    }

    /// Post an update to a live thread
    pub async fn update(&self, thread_id: &str, body: &str) -> Result<LiveUpdate> {
        let form = vec![
            ("api_type", "json"),
            ("body", body),
        ];

        self.client
            .post_authenticated(&format!("/api/live/{}/update", thread_id), &form)
            .await
    }

    /// Strike (mark incorrect) an update
    pub async fn strike_update(&self, thread_id: &str, update_id: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("id", update_id),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/live/{}/strike_update", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Delete an update
    pub async fn delete_update(&self, thread_id: &str, update_id: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("id", update_id),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/live/{}/delete_update", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Edit live thread settings
    pub async fn edit(
        &self,
        thread_id: &str,
        title: Option<&str>,
        description: Option<&str>,
        resources: Option<&str>,
        nsfw: Option<bool>,
    ) -> Result<()> {
        let mut form = vec![("api_type", "json")];

        if let Some(t) = title {
            form.push(("title", t));
        }
        if let Some(d) = description {
            form.push(("description", d));
        }
        if let Some(r) = resources {
            form.push(("resources", r));
        }
        if let Some(n) = nsfw {
            form.push(("nsfw", &n.to_string()));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/live/{}/edit", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Close a live thread
    pub async fn close(&self, thread_id: &str) -> Result<()> {
        let form = vec![("api_type", "json")];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/live/{}/close_thread", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Invite contributor
    pub async fn invite_contributor(
        &self,
        thread_id: &str,
        username: &str,
        permissions: &[&str],
    ) -> Result<()> {
        let perms = permissions.join(",");
        let form = vec![
            ("api_type", "json"),
            ("name", username),
            ("permissions", &perms),
            ("type", "liveupdate_contributor_invite"),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/live/{}/invite_contributor", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Get featured live threads
    pub async fn happening_now(&self) -> Result<Option<LiveThread>> {
        let result: serde_json::Value = self.client
            .get_authenticated("/api/live/happening_now")
            .await?;

        // May be empty if no featured thread
        Ok(None) // TODO: Parse properly
    }
}

#[derive(Debug, Deserialize)]
struct LiveThreadResponse {
    data: LiveThread,
}

#[derive(Debug, Deserialize)]
struct LiveContributorsResponse {
    data: LiveContributorsData,
}

#[derive(Debug, Deserialize)]
struct LiveContributorsData {
    contributors: Vec<LiveContributor>,
}
```

### 6.5 src/api/endpoints/modmail.rs

```rust
use crate::api::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub struct ModmailEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct ModmailConversation {
    pub id: String,
    pub obj_ids: Vec<ModmailObjId>,
    pub authors: Vec<ModmailAuthor>,
    pub subject: String,
    pub state: u8,
    pub last_updated: f64,
    pub num_messages: u32,
    pub is_internal: bool,
    pub participant: Option<ModmailParticipant>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailObjId {
    pub id: String,
    pub key: String,
}

#[derive(Debug, Deserialize)]
pub struct ModmailAuthor {
    pub name: String,
    pub admin: bool,
    pub moderator: bool,
    pub hidden: bool,
    pub is_op: bool,
    #[serde(rename = "isAdmin")]
    pub is_admin: bool,
    #[serde(rename = "isMod")]
    pub is_mod: bool,
    #[serde(rename = "isParticipant")]
    pub is_participant: bool,
}

#[derive(Debug, Deserialize)]
pub struct ModmailParticipant {
    pub name: String,
    pub ban_status: Option<ModmailBanStatus>,
    pub mute_status: Option<ModmailMuteStatus>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailBanStatus {
    pub banned: bool,
    pub permanently_banned: bool,
    pub days_left: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailMuteStatus {
    pub muted: bool,
    pub mute_count: Option<u32>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailMessage {
    pub id: String,
    pub author: ModmailAuthor,
    pub body: String,
    pub body_md: Option<String>,
    pub created_utc: f64,
    pub is_internal: bool,
}

#[derive(Debug, Serialize)]
pub struct CreateModmailRequest {
    pub body: String,
    pub subject: String,
    pub srName: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub isAuthorHidden: Option<bool>,
}

pub enum ModmailState {
    All,
    New,
    InProgress,
    Mod,
    Archived,
    Appeals,
    Notifications,
    Filtered,
    Highlighted,
    Default,
    Inbox,
    JoinRequests,
}

impl std::fmt::Display for ModmailState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModmailState::All => write!(f, "all"),
            ModmailState::New => write!(f, "new"),
            ModmailState::InProgress => write!(f, "inprogress"),
            ModmailState::Mod => write!(f, "mod"),
            ModmailState::Archived => write!(f, "archived"),
            ModmailState::Appeals => write!(f, "appeals"),
            ModmailState::Notifications => write!(f, "notifications"),
            ModmailState::Filtered => write!(f, "filtered"),
            ModmailState::Highlighted => write!(f, "highlighted"),
            ModmailState::Default => write!(f, "default"),
            ModmailState::Inbox => write!(f, "inbox"),
            ModmailState::JoinRequests => write!(f, "join_requests"),
        }
    }
}

impl<'a> ModmailEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get modmail conversations
    pub async fn list(
        &self,
        entity: Option<&[&str]>,
        state: Option<ModmailState>,
        limit: Option<u32>,
    ) -> Result<ModmailListResponse> {
        let mut query: Vec<(&str, String)> = Vec::new();

        if let Some(e) = entity {
            query.push(("entity", e.join(",")));
        }
        if let Some(s) = state {
            query.push(("state", s.to_string()));
        }
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query
            .iter()
            .map(|(k, v)| (*k, v.as_str()))
            .collect();

        self.client
            .get_authenticated_with_query("/api/mod/conversations", &query_refs)
            .await
    }

    /// Get a specific conversation
    pub async fn get(&self, id: &str, mark_read: bool) -> Result<ModmailConversationResponse> {
        let query = vec![("markRead", mark_read.to_string())];

        self.client
            .get_authenticated_with_query(&format!("/api/mod/conversations/{}", id), &query)
            .await
    }

    /// Create a new modmail conversation
    pub async fn create(&self, request: &CreateModmailRequest) -> Result<ModmailConversationResponse> {
        self.client
            .post_authenticated_json("/api/mod/conversations", request)
            .await
    }

    /// Reply to a conversation
    pub async fn reply(
        &self,
        conversation_id: &str,
        body: &str,
        is_internal: bool,
        is_author_hidden: bool,
    ) -> Result<ModmailMessageResponse> {
        let form = vec![
            ("body", body),
            ("isInternal", &is_internal.to_string()),
            ("isAuthorHidden", &is_author_hidden.to_string()),
        ];

        self.client
            .post_authenticated(&format!("/api/mod/conversations/{}", conversation_id), &form)
            .await
    }

    /// Archive a conversation
    pub async fn archive(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/archive", conversation_id), &[])
            .await?;

        Ok(())
    }

    /// Unarchive a conversation
    pub async fn unarchive(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/unarchive", conversation_id), &[])
            .await?;

        Ok(())
    }

    /// Highlight a conversation
    pub async fn highlight(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/highlight", conversation_id), &[])
            .await?;

        Ok(())
    }

    /// Remove highlight
    pub async fn unhighlight(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .delete_authenticated(&format!("/api/mod/conversations/{}/highlight", conversation_id))
            .await?;

        Ok(())
    }

    /// Mute the user in a conversation
    pub async fn mute(&self, conversation_id: &str, hours: Option<u32>) -> Result<()> {
        let form = if let Some(h) = hours {
            vec![("num_hours", h.to_string())]
        } else {
            vec![]
        };

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/mute", conversation_id), &form)
            .await?;

        Ok(())
    }

    /// Unmute the user
    pub async fn unmute(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/unmute", conversation_id), &[])
            .await?;

        Ok(())
    }

    /// Ban the user (temporary)
    pub async fn temp_ban(&self, conversation_id: &str, duration: u32) -> Result<()> {
        let form = vec![("duration", duration.to_string())];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/temp_ban", conversation_id), &form)
            .await?;

        Ok(())
    }

    /// Unban the user
    pub async fn unban(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/unban", conversation_id), &[])
            .await?;

        Ok(())
    }

    /// Approve the user
    pub async fn approve(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/api/mod/conversations/{}/approve", conversation_id), &[])
            .await?;

        Ok(())
    }

    /// Mark conversations as read
    pub async fn read(&self, conversation_ids: &[&str]) -> Result<()> {
        let ids = conversation_ids.join(",");
        let form = vec![("conversationIds", &ids)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/mod/conversations/read", &form)
            .await?;

        Ok(())
    }

    /// Mark conversations as unread
    pub async fn unread(&self, conversation_ids: &[&str]) -> Result<()> {
        let ids = conversation_ids.join(",");
        let form = vec![("conversationIds", &ids)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/mod/conversations/unread", &form)
            .await?;

        Ok(())
    }

    /// Get unread count
    pub async fn unread_count(&self) -> Result<ModmailUnreadCount> {
        self.client
            .get_authenticated("/api/mod/conversations/unread/count")
            .await
    }

    /// Get subreddits for modmail
    pub async fn subreddits(&self) -> Result<Vec<ModmailSubreddit>> {
        let result: ModmailSubredditsResponse = self.client
            .get_authenticated("/api/mod/conversations/subreddits")
            .await?;

        Ok(result.data.subreddits)
    }
}

#[derive(Debug, Deserialize)]
pub struct ModmailListResponse {
    #[serde(rename = "conversationIds")]
    pub conversation_ids: Vec<String>,
    pub conversations: std::collections::HashMap<String, ModmailConversation>,
    pub messages: std::collections::HashMap<String, ModmailMessage>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailConversationResponse {
    pub conversation: ModmailConversation,
    pub messages: std::collections::HashMap<String, ModmailMessage>,
    pub mod_actions: Vec<serde_json::Value>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailMessageResponse {
    #[serde(rename = "modAction")]
    pub mod_action: Option<serde_json::Value>,
    pub message: ModmailMessage,
}

#[derive(Debug, Deserialize)]
pub struct ModmailUnreadCount {
    #[serde(rename = "modmailUnreadCount")]
    pub modmail_unread_count: u32,
    #[serde(rename = "notificationsUnreadCount")]
    pub notifications_unread_count: u32,
    #[serde(rename = "archivedUnreadCount")]
    pub archived_unread_count: u32,
    #[serde(rename = "newUnreadCount")]
    pub new_unread_count: u32,
    #[serde(rename = "inProgressUnreadCount")]
    pub in_progress_unread_count: u32,
    #[serde(rename = "appealsUnreadCount")]
    pub appeals_unread_count: u32,
    #[serde(rename = "joinRequestsUnreadCount")]
    pub join_requests_unread_count: u32,
}

#[derive(Debug, Deserialize)]
pub struct ModmailSubreddits {
    pub subreddits: Vec<ModmailSubreddit>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailSubreddit {
    pub name: String,
    pub display_name: String,
    pub subscribers: u64,
    pub public_description: String,
    pub icon_img: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ModmailSubredditsResponse {
    data: ModmailSubreddits,
}
```

### 6.6 src/api/endpoints/modnote.rs

```rust
use crate::api::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub struct ModNoteEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct ModNote {
    pub id: String,
    pub operator_id: String,
    pub user: ModNoteUser,
    pub subreddit: ModNoteSubreddit,
    pub note: String,
    pub label: Option<String>,
    pub created_at: f64,
    #[serde(rename = "type")]
    pub note_type: String,
    pub reddit_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ModNoteUser {
    pub id: String,
    pub name: String,
    pub created_utc: f64,
}

#[derive(Debug, Deserialize)]
pub struct ModNoteSubreddit {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateModNoteRequest {
    pub user: String,
    pub subreddit: String,
    pub note: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reddit_id: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ModNoteLabel {
    BotBan,
    PermaBan,
    Ban,
    AbuseWarning,
    SpamWarning,
    SpamWatch,
    SolidContributor,
    HelpfulUser,
}

impl std::fmt::Display for ModNoteLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNoteLabel::BotBan => write!(f, "BOT_BAN"),
            ModNoteLabel::PermaBan => write!(f, "PERMA_BAN"),
            ModNoteLabel::Ban => write!(f, "BAN"),
            ModNoteLabel::AbuseWarning => write!(f, "ABUSE_WARNING"),
            ModNoteLabel::SpamWarning => write!(f, "SPAM_WARNING"),
            ModNoteLabel::SpamWatch => write!(f, "SPAM_WATCH"),
            ModNoteLabel::SolidContributor => write!(f, "SOLID_CONTRIBUTOR"),
            ModNoteLabel::HelpfulUser => write!(f, "HELPFUL_USER"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum ModNoteFilter {
    Note,
    Approval,
    Removal,
    Ban,
    Mute,
    Invite,
    Spam,
    ContentChange,
    ModAction,
    All,
}

impl std::fmt::Display for ModNoteFilter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNoteFilter::Note => write!(f, "NOTE"),
            ModNoteFilter::Approval => write!(f, "APPROVAL"),
            ModNoteFilter::Removal => write!(f, "REMOVAL"),
            ModNoteFilter::Ban => write!(f, "BAN"),
            ModNoteFilter::Mute => write!(f, "MUTE"),
            ModNoteFilter::Invite => write!(f, "INVITE"),
            ModNoteFilter::Spam => write!(f, "SPAM"),
            ModNoteFilter::ContentChange => write!(f, "CONTENT_CHANGE"),
            ModNoteFilter::ModAction => write!(f, "MOD_ACTION"),
            ModNoteFilter::All => write!(f, "ALL"),
        }
    }
}

impl<'a> ModNoteEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get notes for a user in a subreddit
    pub async fn get(
        &self,
        subreddit: &str,
        user: &str,
        filter: Option<ModNoteFilter>,
        limit: Option<u32>,
        before: Option<&str>,
    ) -> Result<Vec<ModNote>> {
        let mut query: Vec<(&str, &str)> = vec![
            ("subreddit", subreddit),
            ("user", user),
        ];

        if let Some(f) = filter {
            query.push(("filter", &f.to_string()));
        }
        if let Some(l) = limit {
            query.push(("limit", &l.to_string()));
        }
        if let Some(b) = before {
            query.push(("before", b));
        }

        let result: ModNoteListResponse = self.client
            .get_authenticated_with_query("/api/mod/notes", &query)
            .await?;

        Ok(result.notes)
    }

    /// Create a mod note
    pub async fn create(
        &self,
        subreddit: &str,
        user: &str,
        note: &str,
        label: Option<ModNoteLabel>,
        reddit_id: Option<&str>,
    ) -> Result<ModNote> {
        let mut request = CreateModNoteRequest {
            subreddit: subreddit.to_string(),
            user: user.to_string(),
            note: note.to_string(),
            label: None,
            reddit_id: None,
        };

        if let Some(l) = label {
            request.label = Some(l.to_string());
        }
        if let Some(id) = reddit_id {
            request.reddit_id = Some(id.to_string());
        }

        self.client
            .post_authenticated_json("/api/mod/notes", &request)
            .await
    }

    /// Delete a mod note
    pub async fn delete(&self, subreddit: &str, user: &str, note_id: &str) -> Result<()> {
        let query = vec![
            ("subreddit", subreddit),
            ("user", user),
            ("note_id", note_id),
        ];

        let _: serde_json::Value = self.client
            .delete_authenticated_with_query("/api/mod/notes", &query)
            .await?;

        Ok(())
    }

    /// Get recent notes by moderator
    pub async fn recent(
        &self,
        subreddits: &[&str],
        users: &[&str],
    ) -> Result<Vec<Option<ModNote>>> {
        let query = vec![
            ("subreddits", subreddits.join(",").as_str()),
            ("users", users.join(",").as_str()),
        ];

        let result: Vec<Option<ModNote>> = self.client
            .get_authenticated_with_query("/api/mod/notes/recent", &query)
            .await?;

        Ok(result)
    }
}

#[derive(Debug, Deserialize)]
struct ModNoteListResponse {
    notes: Vec<ModNote>,
}
```

---

## CLI 커맨드 구현

### src/cli/root.rs (업데이트)

```rust
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands

    /// Flair commands
    #[command(subcommand)]
    Flair(FlairCommands),

    /// Wiki commands
    #[command(subcommand)]
    Wiki(WikiCommands),

    /// Multireddit commands
    #[command(subcommand)]
    Multi(MultiCommands),

    /// Live thread commands
    #[command(subcommand)]
    Live(LiveCommands),

    /// Collection commands
    #[command(subcommand)]
    Collection(CollectionCommands),

    /// Modmail commands
    #[command(subcommand)]
    Modmail(ModmailCommands),

    /// Mod note commands
    #[command(subcommand)]
    Modnote(ModnoteCommands),
}

// Add subcommand enums for each feature...
```

---

## 완료 기준

1. `reddit flair list mysub` - 플레어 목록
2. `reddit wiki pages mysub` - 위키 페이지 목록
3. `reddit wiki view mysub index` - 위키 페이지 보기
4. `reddit multi list` - 멀티레딧 목록
5. `reddit live show xxx` - 라이브 스레드
6. `reddit modmail list` - 모드메일 목록
7. `reddit modnote show mysub --user troll` - 모드 노트

---

## 다음 단계

Phase 6 완료 후 → [PHASE7.md](PHASE7.md): 품질 개선
