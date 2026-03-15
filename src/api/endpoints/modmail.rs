use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

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

#[derive(Debug, Deserialize)]
pub struct ModmailListResponse {
    pub conversation_ids: serde_json::Value,
    pub conversations: std::collections::HashMap<String, ModmailConversation>,
}

#[derive(Debug, Deserialize)]
pub struct ModmailConversationResponse {
    pub conversation: ModmailConversation,
    pub messages: std::collections::HashMap<String, ModmailMessage>,
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
        let entity_str;
        let state_str;
        let limit_str;

        let mut query: Vec<(&str, &str)> = Vec::new();

        if let Some(e) = entity {
            entity_str = e.join(",");
            query.push(("entity", &entity_str));
        }
        if let Some(s) = state {
            state_str = s.to_string();
            query.push(("state", &state_str));
        }
        if let Some(l) = limit {
            limit_str = l.to_string();
            query.push(("limit", &limit_str));
        }

        self.client
            .get_authenticated_with_query("/api/mod/conversations", &query)
            .await
    }

    /// Get a specific modmail conversation
    pub async fn get(&self, conversation_id: &str) -> Result<ModmailConversationResponse> {
        self.client
            .get_authenticated(&format!("/api/mod/conversations/{}", conversation_id))
            .await
    }

    /// Create a new modmail conversation
    pub async fn create(
        &self,
        body: &str,
        subject: &str,
        sr_name: &str,
        to: Option<&str>,
        is_author_hidden: Option<bool>,
    ) -> Result<ModmailConversationResponse> {
        let hidden_str;
        let mut form: Vec<(&str, &str)> =
            vec![("body", body), ("subject", subject), ("srName", sr_name)];

        if let Some(t) = to {
            form.push(("to", t));
        }
        if let Some(h) = is_author_hidden {
            hidden_str = h.to_string();
            form.push(("isAuthorHidden", &hidden_str));
        }

        self.client
            .post_authenticated("/api/mod/conversations", &form)
            .await
    }

    /// Reply to a modmail conversation
    pub async fn reply(
        &self,
        conversation_id: &str,
        body: &str,
        is_internal: bool,
        author_hidden: bool,
    ) -> Result<ModmailMessage> {
        let internal_str = is_internal.to_string();
        let hidden_str = author_hidden.to_string();

        let form = vec![
            ("body", body),
            ("isInternal", internal_str.as_str()),
            ("isAuthorHidden", hidden_str.as_str()),
        ];

        self.client
            .post_authenticated(
                &format!("/api/mod/conversations/{}/participant", conversation_id),
                &form,
            )
            .await
    }

    /// Archive a modmail conversation
    pub async fn archive(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post_authenticated_empty(&format!(
                "/api/mod/conversations/{}/archive",
                conversation_id
            ))
            .await?;

        Ok(())
    }

    /// Unarchive a modmail conversation
    pub async fn unarchive(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post_authenticated_empty(&format!(
                "/api/mod/conversations/{}/unarchive",
                conversation_id
            ))
            .await?;

        Ok(())
    }

    /// Highlight a modmail conversation
    pub async fn highlight(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post_authenticated_empty(&format!(
                "/api/mod/conversations/{}/highlight",
                conversation_id
            ))
            .await?;

        Ok(())
    }

    /// Remove highlight from a modmail conversation
    pub async fn unhighlight(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post_authenticated_empty(&format!(
                "/api/mod/conversations/{}/unhighlight",
                conversation_id
            ))
            .await?;

        Ok(())
    }

    /// Mark a modmail conversation as read
    pub async fn mark_read(&self, conversation_ids: &[&str]) -> Result<()> {
        let ids = conversation_ids.join(",");
        let form = vec![("ids", ids.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/mod/conversations/read", &form)
            .await?;

        Ok(())
    }

    /// Mark a modmail conversation as unread
    pub async fn mark_unread(&self, conversation_ids: &[&str]) -> Result<()> {
        let ids = conversation_ids.join(",");
        let form = vec![("ids", ids.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/mod/conversations/unread", &form)
            .await?;

        Ok(())
    }

    /// Mute the participant of a modmail conversation
    pub async fn mute(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post_authenticated_empty(&format!("/api/mod/conversations/{}/mute", conversation_id))
            .await?;

        Ok(())
    }

    /// Unmute the participant of a modmail conversation
    pub async fn unmute(&self, conversation_id: &str) -> Result<()> {
        let _: serde_json::Value = self
            .client
            .post_authenticated_empty(&format!(
                "/api/mod/conversations/{}/unmute",
                conversation_id
            ))
            .await?;

        Ok(())
    }

    /// Ban the participant of a modmail conversation
    pub async fn ban(
        &self,
        conversation_id: &str,
        duration: Option<u32>,
        reason: Option<&str>,
        note: Option<&str>,
    ) -> Result<()> {
        let duration_str;
        let mut form: Vec<(&str, &str)> = Vec::new();

        if let Some(d) = duration {
            duration_str = d.to_string();
            form.push(("duration", &duration_str));
        }
        if let Some(r) = reason {
            form.push(("banReason", r));
        }
        if let Some(n) = note {
            form.push(("note", n));
        }

        let _: serde_json::Value = self
            .client
            .post_authenticated(
                &format!("/api/mod/conversations/{}/ban", conversation_id),
                &form,
            )
            .await?;

        Ok(())
    }

    /// Get modmail unsubscribed count
    pub async fn unsubscribed_count(&self) -> Result<u64> {
        let result: serde_json::Value = self
            .client
            .get_authenticated("/api/mod/conversations/unsubscribed/count")
            .await?;

        Ok(result["count"].as_u64().unwrap_or(0))
    }
}
