use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct LiveEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct LiveThreadResponse {
    pub data: LiveThread,
}

#[derive(Debug, Deserialize)]
pub struct LiveThread {
    pub id: String,
    pub name: String,
    pub title: String,
    pub description: Option<String>,
    pub resources: Option<String>,
    pub state: String,
    pub created_utc: f64,
    pub websocket_url: Option<String>,
    pub viewer_count: Option<u64>,
    pub nsfw: bool,
}

#[derive(Debug, Deserialize)]
pub struct LiveUpdate {
    pub id: String,
    pub name: String,
    pub body: String,
    pub body_html: String,
    pub author: String,
    pub created_utc: f64,
    pub embeds: Vec<serde_json::Value>,
    pub stricken: bool,
}

#[derive(Debug, Deserialize)]
pub struct LiveContributorsResponse {
    pub data: LiveContributorsData,
}

#[derive(Debug, Deserialize)]
pub struct LiveContributorsData {
    pub contributors: Vec<LiveContributor>,
}

#[derive(Debug, Deserialize)]
pub struct LiveContributor {
    pub name: String,
    pub id: String,
    pub permissions: Vec<String>,
}

impl<'a> LiveEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get live thread info
    pub async fn about(&self, thread_id: &str) -> Result<LiveThread> {
        let result: LiveThreadResponse = self
            .client
            .get_authenticated(&format!("/live/{}/about", thread_id))
            .await?;

        Ok(result.data)
    }

    /// Get live thread updates
    pub async fn updates(
        &self,
        thread_id: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<LiveUpdate>> {
        let limit_str;
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            limit_str = l.to_string();
            query.push(("limit", &limit_str));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }

        self.client
            .get_authenticated_with_query(&format!("/live/{}", thread_id), &query)
            .await
    }

    /// Get contributors
    pub async fn contributors(&self, thread_id: &str) -> Result<Vec<LiveContributor>> {
        let result: LiveContributorsResponse = self
            .client
            .get_authenticated(&format!("/live/{}/contributors", thread_id))
            .await?;

        Ok(result.data.contributors)
    }

    /// Get discussions linking to live thread
    pub async fn discussions(
        &self,
        thread_id: &str,
        limit: Option<u32>,
    ) -> Result<serde_json::Value> {
        let limit_str;
        let query: Vec<(&str, &str)> = if let Some(l) = limit {
            limit_str = l.to_string();
            vec![("limit", &limit_str)]
        } else {
            vec![]
        };

        self.client
            .get_authenticated_with_query(&format!("/live/{}/discussions", thread_id), &query)
            .await
    }

    /// Create a live thread
    pub async fn create(
        &self,
        title: &str,
        description: Option<&str>,
        resources: Option<&str>,
        nsfw: bool,
    ) -> Result<LiveThread> {
        let nsfw_str = nsfw.to_string();
        let mut form = vec![("api_type", "json"), ("title", title), ("nsfw", &nsfw_str)];

        if let Some(d) = description {
            form.push(("description", d));
        }
        if let Some(r) = resources {
            form.push(("resources", r));
        }

        self.client
            .post_authenticated("/api/live/create", &form)
            .await
    }

    /// Post an update to a live thread
    pub async fn update(&self, thread_id: &str, body: &str) -> Result<LiveUpdate> {
        let form = vec![("api_type", "json"), ("body", body)];

        self.client
            .post_authenticated(&format!("/api/live/{}/update", thread_id), &form)
            .await
    }

    /// Strike (mark incorrect) an update
    pub async fn strike_update(&self, thread_id: &str, update_id: &str) -> Result<()> {
        let form = vec![("api_type", "json"), ("id", update_id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/api/live/{}/strike_update", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Delete an update
    pub async fn delete_update(&self, thread_id: &str, update_id: &str) -> Result<()> {
        let form = vec![("api_type", "json"), ("id", update_id)];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/api/live/{}/delete_update", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Edit live thread settings
    pub async fn edit(
        &self,
        thread_id: &str,
        title: Option<&str>,
        description: Option<&str>,
        resources: Option<&str>,
        nsfw: Option<bool>,
    ) -> Result<()> {
        let nsfw_str;
        let mut form: Vec<(&str, &str)> = vec![("api_type", "json")];

        if let Some(t) = title {
            form.push(("title", t));
        }
        if let Some(d) = description {
            form.push(("description", d));
        }
        if let Some(r) = resources {
            form.push(("resources", r));
        }
        if let Some(n) = nsfw {
            nsfw_str = n.to_string();
            form.push(("nsfw", &nsfw_str));
        }

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/api/live/{}/edit", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Close a live thread
    pub async fn close(&self, thread_id: &str) -> Result<()> {
        let form = vec![("api_type", "json")];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/api/live/{}/close_thread", thread_id), &form)
            .await?;

        Ok(())
    }

    /// Invite contributor
    pub async fn invite_contributor(
        &self,
        thread_id: &str,
        username: &str,
        permissions: &[&str],
    ) -> Result<()> {
        let perms = permissions.join(",");
        let form = vec![
            ("api_type", "json"),
            ("name", username),
            ("permissions", &perms),
            ("type", "liveupdate_contributor_invite"),
        ];

        let _: serde_json::Value = self
            .client
            .post_authenticated(
                &format!("/api/live/{}/invite_contributor", thread_id),
                &form,
            )
            .await?;

        Ok(())
    }

    /// Get featured live threads
    pub async fn happening_now(&self) -> Result<Vec<LiveThread>> {
        self.client
            .get_authenticated("/api/live/happening_now")
            .await
    }
}
