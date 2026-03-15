use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Reddit user
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "link_karma")]
    pub link_karma: i64,
    #[serde(rename = "comment_karma")]
    pub comment_karma: i64,
    #[serde(rename = "total_karma")]
    pub total_karma: Option<i64>,
    #[serde(rename = "created_utc")]
    pub created_utc: DateTime<Utc>,
    #[serde(rename = "has_verified_email")]
    pub has_verified_email: Option<bool>,
    #[serde(rename = "is_gold")]
    pub is_gold: Option<bool>,
    #[serde(rename = "is_mod")]
    pub is_mod: Option<bool>,
    #[serde(rename = "is_employee")]
    pub is_employee: Option<bool>,
    #[serde(rename = "icon_img")]
    pub icon_img: Option<String>,
    #[serde(rename = "snoovatar_img")]
    pub snoovatar_img: Option<String>,
    #[serde(rename = "subreddit")]
    pub subreddit: Option<UserSubreddit>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct UserSubreddit {
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "public_description")]
    pub public_description: Option<String>,
}
