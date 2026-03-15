use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Reddit comment
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Comment {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub fullname: String,
    #[serde(rename = "author")]
    pub author: String,
    #[serde(rename = "body")]
    pub body: String,
    #[serde(rename = "body_html")]
    pub body_html: Option<String>,
    #[serde(rename = "subreddit")]
    pub subreddit: String,
    #[serde(rename = "link_id")]
    pub link_id: String,
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "permalink")]
    pub permalink: String,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "score")]
    pub score: i64,
    #[serde(rename = "controversiality")]
    pub controversiality: u8,
    #[serde(rename = "stickied")]
    pub stickied: bool,
    #[serde(rename = "locked")]
    pub locked: bool,
    #[serde(rename = "edited")]
    pub edited: Edited,
    #[serde(rename = "replies")]
    pub replies: Option<CommentReplies>,
}

/// Edited can be either bool or timestamp
#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum Edited {
    Bool(bool),
    Timestamp(f64),
}

impl Edited {
    pub fn is_edited(&self) -> bool {
        match self {
            Edited::Bool(b) => *b,
            Edited::Timestamp(_) => true,
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommentReplies {
    #[serde(rename = "data")]
    pub data: CommentListing,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CommentListing {
    #[serde(rename = "children")]
    pub children: Vec<crate::models::common::Thing<Comment>>,
}
