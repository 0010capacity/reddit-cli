use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

/// Submit endpoint for creating posts
pub struct SubmitEndpoint<'a> {
    client: &'a Client,
}

/// Response from submitting a post
#[derive(Debug, Deserialize)]
pub struct SubmitResponse {
    pub json: SubmitJson,
}

#[derive(Debug, Deserialize)]
pub struct SubmitJson {
    pub data: Option<SubmitData>,
    pub errors: Vec<Vec<String>>,
}

#[derive(Debug, Deserialize)]
pub struct SubmitData {
    pub url: String,
    pub id: String,
    pub name: String,
}

/// Type of post to submit
pub enum SubmitKind {
    Link,
    SelfPost,
    Image,
    Video,
    VideoGif,
}

impl std::fmt::Display for SubmitKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SubmitKind::Link => write!(f, "link"),
            SubmitKind::SelfPost => write!(f, "self"),
            SubmitKind::Image => write!(f, "image"),
            SubmitKind::Video => write!(f, "video"),
            SubmitKind::VideoGif => write!(f, "videogif"),
        }
    }
}

/// Options for submitting a post
pub struct SubmitOptions {
    pub subreddit: String,
    pub title: String,
    pub kind: SubmitKind,
    pub url: Option<String>,
    pub text: Option<String>,
    pub flair_id: Option<String>,
    pub flair_text: Option<String>,
    pub nsfw: bool,
    pub spoiler: bool,
    pub send_replies: bool,
}

impl<'a> SubmitEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Submit a post
    pub async fn submit(&self, options: &SubmitOptions) -> Result<SubmitResponse> {
        let kind_str = options.kind.to_string();
        let send_replies = options.send_replies.to_string();

        let mut form: Vec<(&str, String)> = vec![
            ("api_type", "json".to_string()),
            ("sr", options.subreddit.clone()),
            ("title", options.title.clone()),
            ("kind", kind_str),
            ("resubmit", "true".to_string()),
            ("sendreplies", send_replies),
        ];

        if let Some(ref url) = options.url {
            form.push(("url", url.clone()));
        }
        if let Some(ref text) = options.text {
            form.push(("text", text.clone()));
        }
        if let Some(ref flair_id) = options.flair_id {
            form.push(("flair_id", flair_id.clone()));
        }
        if let Some(ref flair_text) = options.flair_text {
            form.push(("flair_text", flair_text.clone()));
        }
        if options.nsfw {
            form.push(("nsfw", "true".to_string()));
        }
        if options.spoiler {
            form.push(("spoiler", "true".to_string()));
        }

        // Convert to borrowed form
        let form_borrowed: Vec<(&str, &str)> = form.iter().map(|(k, v)| (*k, v.as_str())).collect();

        self.client
            .post_authenticated("/api/submit", &form_borrowed)
            .await
    }

    /// Edit a post or comment
    pub async fn edit(&self, id: &str, text: &str) -> Result<()> {
        let form = [("api_type", "json"), ("thing_id", id), ("text", text)];

        let _: serde_json::Value = self
            .client
            .post_authenticated("/api/editusertext", &form)
            .await?;

        Ok(())
    }

    /// Delete a post or comment
    pub async fn delete(&self, id: &str) -> Result<()> {
        let form = [("id", id)];

        let _: serde_json::Value = self.client.post_authenticated("/api/del", &form).await?;

        Ok(())
    }
}
