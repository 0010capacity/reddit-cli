use crate::api::Client;
use crate::error::Result;

/// Subscribe endpoint for subscribing/unsubscribing to subreddits
pub struct SubscribeEndpoint<'a> {
    client: &'a Client,
}

impl<'a> SubscribeEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Subscribe to a subreddit
    pub async fn subscribe(&self, subreddit: &str) -> Result<()> {
        let form = [("action", "sub"), ("sr_name", subreddit)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }

    /// Unsubscribe from a subreddit
    pub async fn unsubscribe(&self, subreddit: &str) -> Result<()> {
        let form = [("action", "unsub"), ("sr_name", subreddit)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }

    /// Subscribe to multiple subreddits at once
    pub async fn subscribe_multiple(&self, subreddits: &[&str]) -> Result<()> {
        let sr_names = subreddits.join(",");
        let form = [("action", "sub"), ("sr_name", &sr_names)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }

    /// Unsubscribe from multiple subreddits at once
    pub async fn unsubscribe_multiple(&self, subreddits: &[&str]) -> Result<()> {
        let sr_names = subreddits.join(",");
        let form = [("action", "unsub"), ("sr_name", &sr_names)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/subscribe", &form)
            .await?;

        Ok(())
    }
}
