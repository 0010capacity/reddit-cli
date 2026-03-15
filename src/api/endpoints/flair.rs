use crate::api::Client;
use crate::error::Result;
use crate::models::ListingResponse;
use serde::Deserialize;

pub struct FlairEndpoint<'a> {
    client: &'a Client,
    subreddit: String,
}

#[derive(Debug, Deserialize)]
pub struct Flair {
    pub user: String,
    pub flair_text: Option<String>,
    pub flair_css_class: Option<String>,
    pub flair_template_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct FlairTemplate {
    pub id: String,
    pub text: Option<String>,
    pub text_color: Option<String>,
    pub background_color: Option<String>,
    pub css_class: Option<String>,
    pub text_editable: bool,
    pub mod_only: bool,
}

#[derive(Debug, Deserialize)]
pub struct FlairSelectorResponse {
    pub choices: Vec<FlairChoice>,
    pub current: Option<CurrentFlair>,
}

#[derive(Debug, Deserialize)]
pub struct FlairChoice {
    pub flair_template_id: String,
    pub flair_text: Option<String>,
    pub flair_css_class: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CurrentFlair {
    pub flair_text: Option<String>,
    pub flair_css_class: Option<String>,
    pub flair_template_id: Option<String>,
}

impl<'a> FlairEndpoint<'a> {
    pub fn new(client: &'a Client, subreddit: &str) -> Self {
        Self {
            client,
            subreddit: subreddit.to_string(),
        }
    }

    /// Get list of user flairs in subreddit
    pub async fn list(
        &self,
        limit: Option<u32>,
        after: Option<&str>,
        user: Option<&str>,
    ) -> Result<ListingResponse<Flair>> {
        let limit_str;
        let mut query: Vec<(&str, &str)> = Vec::new();
        if let Some(l) = limit {
            limit_str = l.to_string();
            query.push(("limit", &limit_str));
        }
        if let Some(a) = after {
            query.push(("after", a));
        }
        if let Some(u) = user {
            query.push(("name", u));
        }

        self.client
            .get_authenticated_with_query(&format!("/r/{}/api/flairlist", self.subreddit), &query)
            .await
    }

    /// Set user flair
    pub async fn set_user_flair(
        &self,
        user: &str,
        text: Option<&str>,
        css_class: Option<&str>,
        template_id: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("name", user),
        ];

        if let Some(t) = text {
            form.push(("text", t));
        }
        if let Some(c) = css_class {
            form.push(("css_class", c));
        }
        if let Some(t) = template_id {
            form.push(("flair_template_id", t));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/flair", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Delete user flair
    pub async fn delete_user_flair(&self, user: &str) -> Result<()> {
        let form = vec![
            ("api_type", "json"),
            ("name", user),
        ];

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/deleteflair", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Get user flair options
    pub async fn user_flair_selector(&self) -> Result<FlairSelectorResponse> {
        self.client
            .post_authenticated_empty(&format!("/r/{}/api/flairselector", self.subreddit))
            .await
    }

    /// Get link flair options for a post
    pub async fn link_flair_selector(&self, link: &str) -> Result<FlairSelectorResponse> {
        let form = vec![("link", link)];

        self.client
            .post_authenticated(&format!("/r/{}/api/flairselector", self.subreddit), &form)
            .await
    }

    /// Get all user flair templates (v2)
    pub async fn user_flair_templates(&self) -> Result<Vec<FlairTemplate>> {
        self.client
            .get_authenticated(&format!("/r/{}/api/user_flair_v2", self.subreddit))
            .await
    }

    /// Get all link flair templates (v2)
    pub async fn link_flair_templates(&self) -> Result<Vec<FlairTemplate>> {
        self.client
            .get_authenticated(&format!("/r/{}/api/link_flair_v2", self.subreddit))
            .await
    }

    /// Select flair for user
    pub async fn select_user_flair(
        &self,
        template_id: &str,
        text: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("flair_template_id", template_id),
        ];

        if let Some(t) = text {
            form.push(("text", t));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/selectflair", self.subreddit), &form)
            .await?;

        Ok(())
    }

    /// Select flair for a post
    pub async fn select_link_flair(
        &self,
        link: &str,
        template_id: &str,
        text: Option<&str>,
    ) -> Result<()> {
        let mut form = vec![
            ("api_type", "json"),
            ("link", link),
            ("flair_template_id", template_id),
        ];

        if let Some(t) = text {
            form.push(("text", t));
        }

        let _: serde_json::Value = self.client
            .post_authenticated(&format!("/r/{}/api/selectflair", self.subreddit), &form)
            .await?;

        Ok(())
    }
}
