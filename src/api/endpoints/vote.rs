use crate::api::Client;
use crate::error::Result;

/// Vote endpoint for upvoting/downvoting posts and comments
pub struct VoteEndpoint<'a> {
    client: &'a Client,
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
        let dir_str = dir.to_string();
        let form = [("id", id), ("dir", &dir_str)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/vote", &form)
            .await?;

        Ok(())
    }
}
