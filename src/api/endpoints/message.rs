use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::{Deserialize, Serialize};

pub struct MessageEndpoint<'a> {
    client: &'a Client,
}

/// Private message
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Message {
    pub id: String,
    pub name: String,
    /// Author username (None for system messages)
    pub author: Option<String>,
    /// Destination username
    pub dest: String,
    /// Message body (markdown)
    pub body: String,
    /// Message body as HTML
    #[serde(default)]
    pub body_html: Option<String>,
    /// Message subject
    pub subject: String,
    /// Subreddit name if sent from a subreddit
    #[serde(rename = "subreddit_name_prefixed")]
    pub subreddit: Option<String>,
    /// Creation timestamp (UTC)
    pub created_utc: f64,
    /// Whether the message is unread
    #[serde(rename = "new")]
    pub is_new: bool,
    /// Whether this is a comment reply
    #[serde(rename = "was_comment")]
    pub was_comment: bool,
    /// Context for comment replies
    #[serde(default)]
    pub context: Option<String>,
    /// Parent message ID
    #[serde(default)]
    pub parent_id: Option<String>,
    /// First message in thread
    #[serde(default)]
    pub first_message: Option<String>,
}

/// Response from compose API
#[derive(Debug, Deserialize)]
pub struct ComposeResponse {
    pub json: ComposeJson,
}

#[derive(Debug, Deserialize)]
pub struct ComposeJson {
    pub errors: Vec<Vec<String>>,
}

/// Message folder type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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

    /// Get messages from a folder (inbox, unread, sent)
    pub async fn get(
        &self,
        folder: MessageFolder,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Message>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }

        let query_ref: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.client
            .get_authenticated_with_query(&format!("/message/{}", folder), &query_ref)
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
        let mut form: Vec<(&str, &str)> = vec![
            ("api_type", "json"),
            ("to", to),
            ("subject", subject),
            ("text", text),
        ];

        if let Some(sr) = from_subreddit {
            form.push(("from_sr", sr));
        }

        self.client.post_authenticated("/api/compose", &form).await
    }

    /// Mark messages as read
    pub async fn read(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", ids_str.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/read_message", &form)
            .await?;

        Ok(())
    }

    /// Mark all messages as read
    pub async fn read_all(&self) -> Result<()> {
        let form: Vec<(&str, &str)> = vec![];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/read_all_messages", &form)
            .await?;

        Ok(())
    }

    /// Mark messages as unread
    pub async fn unread(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", ids_str.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/unread_message", &form)
            .await?;

        Ok(())
    }

    /// Delete a message
    pub async fn delete(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/del_msg", &form)
            .await?;

        Ok(())
    }

    /// Block the author of a message
    pub async fn block(&self, id: &str) -> Result<()> {
        let form = vec![("id", id)];

        let _: serde_json::Value = self.client.post_authenticated("/api/block", &form).await?;

        Ok(())
    }

    /// Collapse messages
    pub async fn collapse(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", ids_str.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/collapse_message", &form)
            .await?;

        Ok(())
    }

    /// Uncollapse messages
    pub async fn uncollapse(&self, ids: &[&str]) -> Result<()> {
        let ids_str = ids.join(",");
        let form = vec![("id", ids_str.as_str())];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/uncollapse_message", &form)
            .await?;

        Ok(())
    }
}
