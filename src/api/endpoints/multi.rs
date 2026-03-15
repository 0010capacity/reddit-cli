use crate::api::Client;
use crate::error::Result;
use serde::{Deserialize, Serialize};

pub struct MultiEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct MultiResponse {
    pub data: MultiData,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct Multi {
    pub data: MultiData,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct MultiData {
    pub path: String,
    pub display_name: String,
    pub description_md: Option<String>,
    pub icon_name: Option<String>,
    pub key_color: Option<String>,
    pub visibility: String,
    pub subreddits: Vec<MultiSubreddit>,
    pub owner: Option<String>,
    pub owner_id: Option<String>,
    pub num_subscribers: Option<u64>,
    pub created_utc: Option<f64>,
}

#[derive(Debug, Deserialize)]
pub struct MultiSubreddit {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct CreateMultiRequest {
    pub display_name: String,
    pub subreddits: Vec<MultiSubredditInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description_md: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub key_color: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visibility: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct MultiSubredditInput {
    pub name: String,
}

#[derive(Debug, Deserialize)]
struct MultiDescription {
    body_md: Option<String>,
}

impl<'a> MultiEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get user's multis
    pub async fn mine(&self) -> Result<Vec<Multi>> {
        self.client
            .get_authenticated("/api/multi/mine")
            .await
    }

    /// Get public multis for a user
    pub async fn user(&self, username: &str) -> Result<Vec<Multi>> {
        self.client
            .get_authenticated(&format!("/api/multi/user/{}", username))
            .await
    }

    /// Get a specific multi
    pub async fn get(&self, path: &str) -> Result<Multi> {
        self.client
            .get_authenticated(&format!("/api/multi/{}", path))
            .await
    }

    /// Create a multi
    pub async fn create(&self, path: &str, request: &CreateMultiRequest) -> Result<Multi> {
        let model = serde_json::to_string(request)?;
        let model_str = model;

        let form = vec![
            ("model", model_str.as_str()),
        ];

        self.client
            .post_authenticated(&format!("/api/multi/{}", path), &form)
            .await
    }

    /// Update a multi
    pub async fn update(&self, path: &str, request: &CreateMultiRequest) -> Result<Multi> {
        let model = serde_json::to_string(request)?;
        let model_str = model;

        let form = vec![
            ("model", model_str.as_str()),
        ];

        self.client
            .put_authenticated(&format!("/api/multi/{}", path), &form)
            .await
    }

    /// Delete a multi
    pub async fn delete(&self, path: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .delete_authenticated(&format!("/api/multi/{}", path))
            .await?;

        Ok(())
    }

    /// Copy a multi
    pub async fn copy(&self, from: &str, to: &str, display_name: Option<&str>) -> Result<Multi> {
        let mut form: Vec<(&str, &str)> = vec![
            ("from", from),
            ("to", to),
        ];

        if let Some(name) = display_name {
            form.push(("display_name", name));
        }

        self.client
            .post_authenticated("/api/multi/copy", &form)
            .await
    }

    /// Add subreddit to multi
    pub async fn add_subreddit(&self, multi_path: &str, subreddit: &str) -> Result<()> {
        let model = serde_json::json!({"name": subreddit}).to_string();
        let model_str = model;

        let form = vec![("model", model_str.as_str())];

        let _: serde_json::Value = self.client
            .put_authenticated(&format!("/api/multi/{}/r/{}", multi_path, subreddit), &form)
            .await?;

        Ok(())
    }

    /// Remove subreddit from multi
    pub async fn remove_subreddit(&self, multi_path: &str, subreddit: &str) -> Result<()> {
        let _: serde_json::Value = self.client
            .delete_authenticated(&format!("/api/multi/{}/r/{}", multi_path, subreddit))
            .await?;

        Ok(())
    }

    /// Get multi description
    pub async fn description(&self, path: &str) -> Result<String> {
        let result: MultiDescription = self.client
            .get_authenticated(&format!("/api/multi/{}/description", path))
            .await?;

        Ok(result.body_md.unwrap_or_default())
    }

    /// Update multi description
    pub async fn update_description(&self, path: &str, description: &str) -> Result<()> {
        let model = serde_json::json!({"body_md": description}).to_string();
        let model_str = model;

        let form = vec![("model", model_str.as_str())];

        let _: serde_json::Value = self.client
            .put_authenticated(&format!("/api/multi/{}/description", path), &form)
            .await?;

        Ok(())
    }
}
