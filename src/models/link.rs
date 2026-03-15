use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Reddit link (post)
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Link {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub fullname: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "subreddit")]
    pub subreddit: String,
    #[serde(rename = "subreddit_id")]
    pub subreddit_id: String,
    #[serde(rename = "selftext")]
    pub selftext: String,
    #[serde(rename = "selftext_html")]
    pub selftext_html: Option<String>,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "domain")]
    pub domain: String,
    #[serde(rename = "permalink")]
    pub permalink: String,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "score")]
    pub score: i64,
    #[serde(rename = "upvote_ratio")]
    pub upvote_ratio: f64,
    #[serde(rename = "num_comments")]
    pub num_comments: u32,
    #[serde(rename = "over_18")]
    pub over_18: bool,
    #[serde(rename = "spoiler")]
    pub spoiler: bool,
    #[serde(rename = "stickied")]
    pub stickied: bool,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "is_self")]
    pub is_self: bool,
    #[serde(rename = "link_flair_text")]
    pub link_flair_text: Option<String>,
    #[serde(rename = "link_flair_css_class")]
    pub link_flair_css_class: Option<String>,
    #[serde(rename = "thumbnail")]
    pub thumbnail: Option<String>,
    #[serde(rename = "preview")]
    pub preview: Option<Preview>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Preview {
    #[serde(rename = "images")]
    pub images: Vec<PreviewImage>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PreviewImage {
    #[serde(rename = "source")]
    pub source: ImageSource,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ImageSource {
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "width")]
    pub width: u32,
    #[serde(rename = "height")]
    pub height: u32,
}
