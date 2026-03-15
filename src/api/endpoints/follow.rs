use crate::api::Client;
use crate::error::Result;

/// Follow endpoint for following/unfollowing posts
pub struct FollowEndpoint<'a> {
    client: &'a Client,
}

impl<'a> FollowEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Follow a post to receive notifications
    pub async fn follow(&self, post_id: &str) -> Result<()> {
        let post_id_val = post_id.to_string();
        let form = vec![("follow", "true"), ("fullname", &post_id_val)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/follow_post", &form)
            .await?;

        Ok(())
    }

    /// Unfollow a post
    pub async fn unfollow(&self, post_id: &str) -> Result<()> {
        let post_id_val = post_id.to_string();
        let form = vec![("follow", "false"), ("fullname", &post_id_val)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/follow_post", &form)
            .await?;

        Ok(())
    }
}
