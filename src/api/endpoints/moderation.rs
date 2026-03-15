use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::{Deserialize, Serialize};

pub struct ModerationEndpoint<'a> {
    client: &'a Client,
}

/// Mod action log entry
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModAction {
    pub id: String,
    /// Moderator who performed the action (using raw identifier since 'mod' is a keyword)
    #[serde(rename = "mod")]
    pub mod_name: String,
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

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct SubredditInfo {
    pub display_name: String,
    pub name: String,
}

/// Report information
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Report {
    pub id: String,
    pub name: String,
    pub author: String,
    pub title: Option<String>,
    #[serde(default)]
    pub body: Option<String>,
    #[serde(default)]
    pub user_reports: Vec<UserReport>,
    #[serde(default)]
    pub mod_reports: Vec<ModReport>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct UserReport {
    #[serde(rename = "user_report_reason")]
    pub reason: String,
    #[serde(rename = "user_reports_count")]
    pub count: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModReport {
    pub reason: String,
    pub moderator: String,
}

/// Mod queue location type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

/// Distinguish type for mod tags
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DistinguishType {
    Yes,
    No,
    Admin,
    Special,
}

impl std::fmt::Display for DistinguishType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DistinguishType::Yes => write!(f, "yes"),
            DistinguishType::No => write!(f, "no"),
            DistinguishType::Admin => write!(f, "admin"),
            DistinguishType::Special => write!(f, "special"),
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
        only: Option<&str>,
    ) -> Result<serde_json::Value> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(o) = only {
            query.push(("only", o.to_string()));
        }

        let query_ref: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.client
            .get_authenticated_with_query(
                &format!("/r/{}/about/{}", subreddit, location),
                &query_ref,
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
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(m) = mod_filter {
            query.push(("mod", m.to_string()));
        }
        if let Some(t) = action_type {
            query.push(("type", t.to_string()));
        }

        let query_ref: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.client
            .get_authenticated_with_query(&format!("/r/{}/about/log", subreddit), &query_ref)
            .await
    }

    // ========== Moderation Actions ==========

    /// Approve a post or comment
    pub async fn approve(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/approve", &form)
            .await?;

        Ok(())
    }

    /// Remove a post or comment
    pub async fn remove(&self, id: &str, is_spam: bool) -> Result<()> {
        let spam_str = is_spam.to_string();
        let form = vec![("id", id), ("spam", spam_str.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/remove", &form)
            .await?;

        Ok(())
    }

    /// Distinguish a post or comment (mod tag)
    pub async fn distinguish(&self, id: &str, how: DistinguishType, sticky: bool) -> Result<()> {
        let how_str = how.to_string();
        let sticky_str = sticky.to_string();
        let form = vec![
            ("api_type", "json"),
            ("how", how_str.as_str()),
            ("id", id),
            ("sticky", sticky_str.as_str()),
        ];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/distinguish", &form)
            .await?;

        Ok(())
    }

    /// Set sticky status on a post
    pub async fn sticky(&self, id: &str, state: bool, num: Option<u8>) -> Result<()> {
        let state_str = state.to_string();
        let mut form: Vec<(&str, &str)> = vec![
            ("api_type", "json"),
            ("id", id),
            ("state", state_str.as_str()),
        ];

        let num_str;
        if let Some(n) = num {
            num_str = n.to_string();
            form.push(("num", num_str.as_str()));
        }

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/set_subreddit_sticky", &form)
            .await?;

        Ok(())
    }

    /// Lock a post or comment
    pub async fn lock(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/lock", &form)
            .await?;

        Ok(())
    }

    /// Unlock a post or comment
    pub async fn unlock(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/unlock", &form)
            .await?;

        Ok(())
    }

    /// Mark a post as NSFW
    pub async fn mark_nsfw(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/marknsfw", &form)
            .await?;

        Ok(())
    }

    /// Unmark a post as NSFW
    pub async fn unmark_nsfw(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/unmarknsfw", &form)
            .await?;

        Ok(())
    }

    /// Mark as spoiler
    pub async fn spoiler(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/spoiler", &form)
            .await?;

        Ok(())
    }

    /// Unmark as spoiler
    pub async fn unspoiler(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/unspoiler", &form)
            .await?;

        Ok(())
    }

    /// Report a post or comment
    pub async fn report(&self, id: &str, reason: &str, other_reason: Option<&str>) -> Result<()> {
        let mut form: Vec<(&str, &str)> = vec![
            ("api_type", "json"),
            ("thing_id", id),
            ("reason", reason),
        ];

        if let Some(other) = other_reason {
            form.push(("other_reason", other));
        }

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/report", &form)
            .await?;

        Ok(())
    }

    /// Set contest mode
    pub async fn contest_mode(&self, id: &str, state: bool) -> Result<()> {
        let state_str = state.to_string();
        let form = vec![
            ("api_type", "json"),
            ("id", id),
            ("state", state_str.as_str()),
        ];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/set_contest_mode", &form)
            .await?;

        Ok(())
    }

    /// Set suggested sort
    pub async fn suggested_sort(&self, id: &str, sort: &str) -> Result<()> {
        let form = vec![("api_type", "json"), ("id", id), ("sort", sort)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/set_suggested_sort", &form)
            .await?;

        Ok(())
    }

    /// Ignore reports on a thing
    pub async fn ignore_reports(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/ignore_reports", &form)
            .await?;

        Ok(())
    }

    /// Unignore reports on a thing
    pub async fn unignore_reports(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/unignore_reports", &form)
            .await?;

        Ok(())
    }
}

// ========== User Management ==========

pub struct UserManagementEndpoint<'a> {
    client: &'a Client,
}

/// Banned user info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BannedUser {
    pub name: String,
    pub date: f64,
    #[serde(default)]
    pub note: Option<String>,
    #[serde(default)]
    pub rel_id: Option<String>,
}

/// Muted user info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MutedUser {
    pub name: String,
    pub date: f64,
    #[serde(default)]
    pub mute_reason: Option<String>,
}

/// Contributor info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Contributor {
    pub name: String,
    pub date: f64,
}

/// Moderator info
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Moderator {
    pub name: String,
    pub date: f64,
    #[serde(default)]
    pub mod_permissions: Vec<String>,
    #[serde(default)]
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
        let mut form: Vec<(&str, &str)> = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "banned"),
        ];

        let duration_str;
        if let Some(d) = duration {
            duration_str = d.to_string();
            form.push(("duration", duration_str.as_str()));
        }
        if let Some(r) = reason {
            form.push(("ban_reason", r));
        }
        if let Some(n) = note {
            form.push(("note", n));
        }

        let _: serde_json::Value = self
            .client
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

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/r/{}/api/unfriend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Mute a user in a subreddit
    pub async fn mute(&self, subreddit: &str, user: &str, note: Option<&str>) -> Result<()> {
        let mut form: Vec<(&str, &str)> = vec![
            ("api_type", "json"),
            ("name", user),
            ("type", "muted"),
        ];

        if let Some(n) = note {
            form.push(("note", n));
        }

        let _: serde_json::Value = self
            .client
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

        let _: serde_json::Value = self
            .client
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

        let _: serde_json::Value = self
            .client
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

        let _: serde_json::Value = self
            .client
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
            ("permissions", perms.as_str()),
        ];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/r/{}/api/friend", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Leave moderator position
    pub async fn leave_moderator(&self, subreddit: &str) -> Result<()> {
        let form = vec![("api_type", "json")];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/r/{}/api/leavemoderator", subreddit), &form)
            .await?;

        Ok(())
    }

    /// Accept moderator invite
    pub async fn accept_moderator_invite(&self, subreddit: &str) -> Result<()> {
        let form = vec![("api_type", "json")];

        let _: serde_json::Value = self
            .client
            .post_authenticated(
                &format!("/r/{}/api/accept_moderator_invite", subreddit),
                &form,
            )
            .await?;

        Ok(())
    }
}
