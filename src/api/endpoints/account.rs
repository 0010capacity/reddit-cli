use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

/// Account endpoint for /api/v1/me endpoints
pub struct AccountEndpoint<'a> {
    client: &'a Client,
}

impl<'a> AccountEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get current authenticated user's account info
    /// GET /api/v1/me
    pub async fn me(&self) -> Result<Account> {
        self.client.get_authenticated("/api/v1/me").await
    }

    /// Get karma breakdown by subreddit
    /// GET /api/v1/me/karma
    pub async fn karma(&self) -> Result<KarmaResponse> {
        self.client.get_authenticated("/api/v1/me/karma").await
    }

    /// Get account preferences
    /// GET /api/v1/me/prefs
    pub async fn preferences(&self) -> Result<Preferences> {
        self.client.get_authenticated("/api/v1/me/prefs").await
    }

    /// Get trophies for the authenticated user
    /// GET /api/v1/me/trophies
    pub async fn trophies(&self) -> Result<TrophiesResponse> {
        self.client.get_authenticated("/api/v1/me/trophies").await
    }

    /// Get subreddits the user is subscribed to
    /// GET /subreddits/mine/subscriber
    pub async fn subscribed(&self, limit: Option<u32>) -> Result<SubredditListResponse> {
        let query = limit.map(|l| ("limit", l.to_string()));
        match query {
            Some((k, v)) => {
                self.client
                    .get_authenticated_with_query("/subreddits/mine/subscriber", &[(k, &v)])
                    .await
            }
            None => self
                .client
                .get_authenticated("/subreddits/mine/subscriber")
                .await,
        }
    }

    /// Get subreddits where the user is an approved submitter
    /// GET /subreddits/mine/contributor
    pub async fn contributor(&self, limit: Option<u32>) -> Result<SubredditListResponse> {
        let query = limit.map(|l| ("limit", l.to_string()));
        match query {
            Some((k, v)) => {
                self.client
                    .get_authenticated_with_query("/subreddits/mine/contributor", &[(k, &v)])
                    .await
            }
            None => self
                .client
                .get_authenticated("/subreddits/mine/contributor")
                .await,
        }
    }

    /// Get subreddits where the user is a moderator
    /// GET /subreddits/mine/moderator
    pub async fn moderator(&self, limit: Option<u32>) -> Result<SubredditListResponse> {
        let query = limit.map(|l| ("limit", l.to_string()));
        match query {
            Some((k, v)) => {
                self.client
                    .get_authenticated_with_query("/subreddits/mine/moderator", &[(k, &v)])
                    .await
            }
            None => self
                .client
                .get_authenticated("/subreddits/mine/moderator")
                .await,
        }
    }
}

/// Account information for the authenticated user
#[derive(Debug, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    pub created_utc: f64,
    pub link_karma: i64,
    pub comment_karma: i64,
    pub total_karma: i64,
    pub has_verified_email: bool,
    pub is_gold: bool,
    pub is_mod: bool,
    pub is_employee: bool,
    pub over_18: bool,
    pub pref_nightmode: bool,
    pub inbox_count: u32,
    #[serde(default)]
    pub snoovatar_img: Option<String>,
    #[serde(default)]
    pub icon_img: Option<String>,
}

/// Response from /api/v1/me/karma
#[derive(Debug, Deserialize)]
pub struct KarmaResponse {
    pub data: Vec<KarmaEntry>,
}

/// Karma breakdown for a single subreddit
#[derive(Debug, Deserialize)]
pub struct KarmaEntry {
    /// Subreddit name
    pub sr: String,
    /// Subreddit ID (t5_xxxxx)
    pub sr_t5: String,
    /// Link karma in this subreddit
    pub link_karma: i64,
    /// Comment karma in this subreddit
    pub comment_karma: i64,
}

/// User preferences
#[derive(Debug, Deserialize)]
pub struct Preferences {
    #[serde(default)]
    pub over_18: bool,
    #[serde(default)]
    pub enable_followers: bool,
    #[serde(default)]
    pub hide_from_robots: bool,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub nightmode: bool,
    #[serde(default)]
    pub show_flair: bool,
    #[serde(default)]
    pub show_link_flair: bool,
    #[serde(default)]
    pub email_messages: bool,
    #[serde(default)]
    pub email_digests: bool,
    #[serde(default)]
    pub default_comment_sort: Option<String>,
    #[serde(default)]
    pub num_comments: Option<i32>,
    #[serde(default)]
    pub num_sites: Option<i32>,
}

/// Response from /api/v1/me/trophies
#[derive(Debug, Deserialize)]
pub struct TrophiesResponse {
    pub data: TrophiesData,
}

#[derive(Debug, Deserialize)]
pub struct TrophiesData {
    pub trophies: Vec<Trophy>,
}

/// A trophy/badge on a user's profile
#[derive(Debug, Deserialize)]
pub struct Trophy {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub icon_40: Option<String>,
    #[serde(default)]
    pub icon_70: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

/// Response from subreddit listing endpoints
#[derive(Debug, Deserialize)]
pub struct SubredditListResponse {
    pub data: SubredditListData,
}

#[derive(Debug, Deserialize)]
pub struct SubredditListData {
    pub children: Vec<SubredditChild>,
}

#[derive(Debug, Deserialize)]
pub struct SubredditChild {
    pub kind: String,
    pub data: SubscribedSubreddit,
}

/// A subreddit the user is subscribed to or moderates
#[derive(Debug, Deserialize)]
pub struct SubscribedSubreddit {
    pub id: String,
    pub display_name: String,
    pub title: String,
    pub url: String,
    pub over18: bool,
    pub subscribers: u64,
    pub description: Option<String>,
    pub public_description: Option<String>,
    pub created_utc: f64,
    pub user_is_subscriber: Option<bool>,
    pub user_is_moderator: Option<bool>,
    pub user_is_contributor: Option<bool>,
}
