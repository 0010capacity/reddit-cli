use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct WikiEndpoint<'a> {
    client: &'a Client,
    subreddit: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiPageResponse {
    pub data: WikiPage,
}

#[derive(Debug, Deserialize)]
pub struct WikiPage {
    pub content: String,
    pub content_html: Option<String>,
    pub may_revise: bool,
    pub rev_id: String,
    pub revision_by: WikiUser,
    pub revisions_seen: u32,
}

#[derive(Debug, Deserialize)]
pub struct WikiUser {
    pub data: WikiUserData,
}

#[derive(Debug, Deserialize)]
pub struct WikiUserData {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiRevision {
    pub id: String,
    pub timestamp: f64,
    pub reason: Option<String>,
    pub author: WikiUser,
    pub page: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiPageList {
    pub data: Vec<String>,
    pub kind: String,
}

#[derive(Debug, Deserialize)]
pub struct WikiSettingsResponse {
    pub data: WikiSettings,
}

#[derive(Debug, Deserialize)]
pub struct WikiSettings {
    pub permlevel: u8,
    pub listed: bool,
}

impl<'a> WikiEndpoint<'a> {
    pub fn new(client: &'a Client, subreddit: &str) -> Self {
        Self {
            client,
            subreddit: subreddit.to_string(),
        }
    }

    /// Get list of wiki pages
    pub async fn pages(&self) -> Result<WikiPageList> {
        self.client
            .get_authenticated(&format!("/r/{}/wiki/pages", self.subreddit))
            .await
    }

    /// Get wiki page content
    pub async fn page(&self, page: &str) -> Result<WikiPage> {
        let result: WikiPageResponse = self
            .client
            .get_authenticated(&format!("/r/{}/wiki/{}", self.subreddit, page))
            .await?;

        Ok(result.data)
    }

    /// Get wiki page at specific revision
    pub async fn page_revision(&self, page: &str, revision: &str) -> Result<WikiPage> {
        let query = vec![("v", revision)];

        let result: WikiPageResponse = self
            .client
            .get_authenticated_with_query(&format!("/r/{}/wiki/{}", self.subreddit, page), &query)
            .await?;

        Ok(result.data)
    }

    /// Get revision history for a page
    pub async fn revisions(
        &self,
        page: &str,
        limit: Option<u32>,
    ) -> Result<ListingResponse<WikiRevision>> {
        let limit_str;
        let query: Vec<(&str, &str)> = if let Some(l) = limit {
            limit_str = l.to_string();
            vec![("limit", &limit_str)]
        } else {
            vec![]
        };

        self.client
            .get_authenticated_with_query(
                &format!("/r/{}/wiki/revisions/{}", self.subreddit, page),
                &query,
            )
            .await
    }

    /// Get recent wiki changes across all pages
    pub async fn recent_changes(
        &self,
        limit: Option<u32>,
    ) -> Result<ListingResponse<WikiRevision>> {
        let limit_str;
        let query: Vec<(&str, &str)> = if let Some(l) = limit {
            limit_str = l.to_string();
            vec![("limit", &limit_str)]
        } else {
            vec![]
        };

        self.client
            .get_authenticated_with_query(&format!("/r/{}/wiki/revisions", self.subreddit), &query)
            .await
    }

    /// Edit a wiki page
    pub async fn edit(
        &self,
        page: &str,
        content: &str,
        reason: Option<&str>,
        previous: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![("content", content), ("page", page)];

        if let Some(r) = reason {
            form.push(("reason", r));
        }
        if let Some(p) = previous {
            form.push(("previous", p));
        }

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/r/{}/api/wiki/edit", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Hide a wiki revision
    pub async fn hide_revision(&self, page: &str, revision: &str) -> Result<()> {
        let form = vec![("page", page), ("revision", revision)];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/r/{}/api/wiki/hide", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Revert to a revision
    pub async fn revert(&self, page: &str, revision: &str) -> Result<()> {
        let form = vec![("page", page), ("revision", revision)];

        let _: serde_json::Value = self
            .client
            .post_authenticated(&format!("/r/{}/api/wiki/revert", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Get wiki page settings
    pub async fn settings(&self, page: &str) -> Result<WikiSettings> {
        let result: WikiSettingsResponse = self
            .client
            .get_authenticated(&format!("/r/{}/wiki/settings/{}", self.subreddit, page))
            .await?;

        Ok(result.data)
    }

    /// Update wiki page settings
    pub async fn update_settings(&self, page: &str, perm_level: u8, listed: bool) -> Result<()> {
        let perm_str = perm_level.to_string();
        let listed_str = listed.to_string();
        let form = vec![
            ("page", page),
            ("permlevel", &perm_str),
            ("listed", &listed_str),
        ];

        let _: serde_json::Value = self
            .client
            .post_authenticated(
                &format!("/r/{}/wiki/settings/{}", self.subreddit, page),
                &form,
            )
            .await?;

        Ok(())
    }

    /// Allow user to edit wiki page
    pub async fn allow_editor(&self, page: &str, user: &str) -> Result<()> {
        let form = vec![("act", "add"), ("page", page), ("username", user)];

        let _: serde_json::Value = self
            .client
            .post_authenticated(
                &format!("/r/{}/api/wiki/alloweditor/add", self.subreddit),
                &form,
            )
            .await?;

        Ok(())
    }

    /// Remove user's wiki edit permission
    pub async fn disallow_editor(&self, page: &str, user: &str) -> Result<()> {
        let form = vec![("act", "del"), ("page", page), ("username", user)];

        let _: serde_json::Value = self
            .client
            .post_authenticated(
                &format!("/r/{}/api/wiki/alloweditor/del", self.subreddit),
                &form,
            )
            .await?;

        Ok(())
    }
}
