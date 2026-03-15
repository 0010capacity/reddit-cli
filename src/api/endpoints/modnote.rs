use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

pub struct ModNoteEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct ModNote {
    pub id: String,
    pub user: ModNoteUser,
    pub operator: ModNoteOperator,
    pub subreddit: ModNoteSubreddit,
    pub note: String,
    pub label: Option<String>,
    pub created_at: f64,
}

#[derive(Debug, Deserialize)]
pub struct ModNoteUser {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ModNoteOperator {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ModNoteSubreddit {
    pub name: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct ModNoteListResponse {
    pub created_notes: Vec<ModNote>,
}

#[derive(Debug, Clone, Copy)]
pub enum ModNoteLabel {
    Note,
    Abuse,
    Ban,
    Helpful,
    Spam,
}

impl std::fmt::Display for ModNoteLabel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ModNoteLabel::Note => write!(f, "NOTE"),
            ModNoteLabel::Abuse => write!(f, "ABUSE"),
            ModNoteLabel::Ban => write!(f, "BAN"),
            ModNoteLabel::Helpful => write!(f, "HELPFUL"),
            ModNoteLabel::Spam => write!(f, "SPAM"),
        }
    }
}

impl<'a> ModNoteEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get mod notes for a user in a subreddit
    pub async fn list(
        &self,
        subreddit: &str,
        user: &str,
        limit: Option<u32>,
    ) -> Result<ModNoteListResponse> {
        let limit_str;
        let mut query: Vec<(&str, &str)> = vec![
            ("subreddit", subreddit),
            ("user", user),
        ];

        if let Some(l) = limit {
            limit_str = l.to_string();
            query.push(("limit", &limit_str));
        }

        self.client
            .get_authenticated_with_query("/api/mod/notes", &query)
            .await
    }

    /// Get recent mod notes for a subreddit
    pub async fn recent(
        &self,
        subreddit: &str,
        limit: Option<u32>,
    ) -> Result<ModNoteListResponse> {
        let limit_str;
        let mut query: Vec<(&str, &str)> = vec![("subreddit", subreddit)];

        if let Some(l) = limit {
            limit_str = l.to_string();
            query.push(("limit", &limit_str));
        }

        self.client
            .get_authenticated_with_query("/api/mod/notes/recent", &query)
            .await
    }

    /// Create a mod note
    pub async fn create(
        &self,
        subreddit: &str,
        user: &str,
        note: &str,
        label: Option<ModNoteLabel>,
    ) -> Result<ModNote> {
        let label_str;
        let mut form: Vec<(&str, &str)> = vec![
            ("subreddit", subreddit),
            ("user", user),
            ("note", note),
        ];

        if let Some(l) = label {
            label_str = l.to_string();
            form.push(("label", &label_str));
        }

        self.client
            .post_authenticated("/api/mod/notes", &form)
            .await
    }

    /// Delete a mod note
    pub async fn delete(
        &self,
        subreddit: &str,
        user: &str,
        note_id: &str,
    ) -> Result<()> {
        let form = vec![
            ("subreddit", subreddit),
            ("user", user),
            ("note_id", note_id),
        ];

        let _: serde_json::Value = self.client
            .delete_authenticated_with_body("/api/mod/notes", &form)
            .await?;

        Ok(())
    }

    /// Delete all mod notes for a user in a subreddit
    pub async fn delete_all(&self, subreddit: &str, user: &str) -> Result<()> {
        let form = vec![
            ("subreddit", subreddit),
            ("user", user),
        ];

        let _: serde_json::Value = self.client
            .delete_authenticated_with_body("/api/mod/notes", &form)
            .await?;

        Ok(())
    }
}
