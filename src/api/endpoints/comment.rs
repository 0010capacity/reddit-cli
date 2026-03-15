use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

/// Comment endpoint for creating and managing comments
pub struct CommentEndpoint<'a> {
    client: &'a Client,
}

/// Response from submitting a comment
#[derive(Debug, Deserialize)]
pub struct CommentResponse {
    pub json: CommentJson,
}

#[derive(Debug, Deserialize)]
pub struct CommentJson {
    pub data: Option<CommentData>,
    pub errors: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct CommentData {
    pub things: Vec<CommentThing>,
}

#[derive(Debug, Deserialize)]
pub struct CommentThing {
    pub kind: String,
    pub data: CommentThingData,
}

#[derive(Debug, Deserialize)]
pub struct CommentThingData {
    pub id: String,
    pub name: String,
    pub link_id: String,
    pub parent_id: String,
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
        let form = [("api_type", "json"), ("thing_id", parent), ("text", text)];

        self.client.post_authenticated("/api/comment", &form).await
    }

    /// Edit a comment
    pub async fn edit(&self, id: &str, text: &str) -> Result<()> {
        let form = [("api_type", "json"), ("thing_id", id), ("text", text)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/editusertext", &form)
            .await?;

        Ok(())
    }

    /// Delete a comment
    pub async fn delete(&self, id: &str) -> Result<()> {
        let form = [("id", id)];

        let _: serde_json::Value = self.client.post_authenticated("/api/del", &form).await?;

        Ok(())
    }

    /// Enable/disable inbox replies for a post or comment
    pub async fn set_inbox_replies(&self, id: &str, enabled: bool) -> Result<()> {
        let state_str = enabled.to_string();
        let form = [("id", id), ("state", &state_str)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/sendreplies", &form)
            .await?;

        Ok(())
    }
}
