use crate::api::Client;
use crate::error::Result;

/// Save endpoint for saving/hiding posts and comments
pub struct SaveEndpoint<'a> {
    client: &'a Client,
}

impl<'a> SaveEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Save a post or comment
    /// id: fullname (t3_xxx for posts, t1_xxx for comments)
    pub async fn save(&self, id: &str, category: Option<&str>) -> Result<()> {
        match category {
            Some(cat) => {
                let form = [("id", id), ("category", cat)];
                let _: serde_json::Value =
                    self.client.post_authenticated("/api/save", &form).await?;
            }
            None => {
                let form = [("id", id)];
                let _: serde_json::Value =
                    self.client.post_authenticated("/api/save", &form).await?;
            }
        }

        Ok(())
    }

    /// Unsave a post or comment
    pub async fn unsave(&self, id: &str) -> Result<()> {
        let form = [("id", id)];

        let _: serde_json::Value = self.client.post_authenticated("/api/unsave", &form).await?;

        Ok(())
    }

    /// Hide a post
    pub async fn hide(&self, id: &str) -> Result<()> {
        let form = [("id", id)];

        let _: serde_json::Value = self.client.post_authenticated("/api/hide", &form).await?;

        Ok(())
    }

    /// Unhide a post
    pub async fn unhide(&self, id: &str) -> Result<()> {
        let form = [("id", id)];

        let _: serde_json::Value = self.client.post_authenticated("/api/unhide", &form).await?;

        Ok(())
    }
}
