use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

pub struct CollectionEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct Collection {
    pub collection_id: String,
    pub title: String,
    pub description: Option<String>,
    pub display_layout: Option<String>,
    pub permalink: Option<String>,
    pub created_at_utc: Option<f64>,
    pub author: Option<String>,
    pub subreddit_id: Option<String>,
    pub link_ids: Vec<String>,
}

impl<'a> CollectionEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get a collection
    pub async fn get(&self, collection_id: &str) -> Result<Collection> {
        let query = vec![("collection_id", collection_id)];

        self.client
            .get_authenticated_with_query("/api/v1/collections/collection", &query)
            .await
    }

    /// Get collections for a subreddit
    pub async fn subreddit(&self, subreddit_fullname: &str) -> Result<Vec<Collection>> {
        let query = vec![("sr_fullname", subreddit_fullname)];

        self.client
            .get_authenticated_with_query("/api/v1/collections/subreddit_collections", &query)
            .await
    }

    /// Create a collection
    pub async fn create(
        &self,
        title: &str,
        sr_fullname: &str,
        description: Option<&str>,
        display_layout: Option<&str>,
    ) -> Result<Collection> {
        let mut form: Vec<(&str, &str)> = vec![
            ("title", title),
            ("sr_fullname", sr_fullname),
        ];

        if let Some(ref desc) = description {
            form.push(("description", desc));
        }
        if let Some(ref layout) = display_layout {
            form.push(("display_layout", layout));
        }

        self.client
            .post_authenticated("/api/v1/collections/create_collection", &form)
            .await
    }

    /// Delete a collection
    pub async fn delete(&self, collection_id: &str) -> Result<()> {
        let form = vec![("collection_id", collection_id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/delete_collection", &form)
            .await?;

        Ok(())
    }

    /// Update collection metadata
    pub async fn update(
        &self,
        collection_id: &str,
        title: Option<&str>,
        description: Option<&str>,
    ) -> Result<()> {
        let mut form: Vec<(&str, &str)> = vec![("collection_id", collection_id)];

        if let Some(t) = title {
            form.push(("title", t));
        }
        if let Some(d) = description {
            form.push(("description", d));
        }

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/update_collection_title_and_description", &form)
            .await?;

        Ok(())
    }

    /// Add a post to a collection
    pub async fn add_post(&self, collection_id: &str, link_fullname: &str) -> Result<()> {
        let form = vec![
            ("collection_id", collection_id),
            ("link_fullname", link_fullname),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/add_post_to_collection", &form)
            .await?;

        Ok(())
    }

    /// Remove a post from a collection
    pub async fn remove_post(&self, collection_id: &str, link_fullname: &str) -> Result<()> {
        let form = vec![
            ("collection_id", collection_id),
            ("link_fullname", link_fullname),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/remove_post_in_collection", &form)
            .await?;

        Ok(())
    }

    /// Reorder posts in a collection
    pub async fn reorder(&self, collection_id: &str, link_ids: &[&str]) -> Result<()> {
        let link_ids_json = serde_json::to_string(link_ids)?;
        let form = vec![
            ("collection_id", collection_id),
            ("link_ids", &link_ids_json),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/reorder_collection", &form)
            .await?;

        Ok(())
    }

    /// Follow a collection
    pub async fn follow(&self, collection_id: &str) -> Result<()> {
        let form = vec![("collection_id", collection_id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/follow_collection", &form)
            .await?;

        Ok(())
    }

    /// Unfollow a collection
    pub async fn unfollow(&self, collection_id: &str) -> Result<()> {
        let form = vec![("collection_id", collection_id)];

        let _: serde_json::Value = self.client
            .post_authenticated("/api/v1/collections/unfollow_collection", &form)
            .await?;

        Ok(())
    }
}
