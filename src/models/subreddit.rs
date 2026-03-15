use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Subreddit information
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Subreddit {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub fullname: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "public_description")]
    pub public_description: String,
    #[serde(rename = "subscribers")]
    pub subscribers: u64,
    #[serde(rename = "active_user_count")]
    pub active_user_count: Option<u64>,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "over18")]
    pub over18: bool,
    #[serde(rename = "url")]
    pub url: String,
    #[serde(rename = "icon_img")]
    pub icon_img: Option<String>,
    #[serde(rename = "banner_img")]
    pub banner_img: Option<String>,
    #[serde(rename = "header_img")]
    pub header_img: Option<String>,
    #[serde(rename = "user_is_subscriber")]
    pub user_is_subscriber: Option<bool>,
    #[serde(rename = "user_is_moderator")]
    pub user_is_moderator: Option<bool>,
}

/// Subreddit rule
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubredditRule {
    #[serde(rename = "short_name")]
    pub short_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "priority")]
    pub priority: u32,
}

/// Subreddit rules response
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SubredditRules {
    #[serde(rename = "rules")]
    pub rules: Vec<SubredditRule>,
}
