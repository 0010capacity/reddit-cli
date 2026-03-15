use crate::api::Client;
use crate::error::Result;
use crate::models::{ListingResponse, Subreddit};
use serde::Deserialize;

pub struct SubredditEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct SubredditResponse {
    pub data: Subreddit,
}

#[derive(Debug, Deserialize)]
pub struct SubredditRulesResponse {
    pub rules: Vec<SubredditRuleData>,
}

#[derive(Debug, Deserialize)]
pub struct SubredditRuleData {
    pub short_name: String,
    pub description: String,
    #[serde(default)]
    pub priority: u32,
}

impl<'a> SubredditEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get subreddit info
    pub async fn about(&self, name: &str) -> Result<SubredditResponse> {
        self.client.get(&format!("/r/{}/about", name)).await
    }

    /// Get subreddit rules
    pub async fn rules(&self, name: &str) -> Result<SubredditRulesResponse> {
        self.client.get(&format!("/r/{}/about/rules", name)).await
    }

    /// Get popular subreddits
    pub async fn popular(&self, limit: Option<u32>) -> Result<ListingResponse<Subreddit>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/subreddits/popular", &query_refs)
            .await
    }

    /// Get new subreddits
    pub async fn new_subreddits(&self, limit: Option<u32>) -> Result<ListingResponse<Subreddit>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/subreddits/new", &query_refs)
            .await
    }

    /// Search subreddits
    pub async fn search(
        &self,
        query: &str,
        limit: Option<u32>,
    ) -> Result<ListingResponse<Subreddit>> {
        let mut params: Vec<(&str, String)> = vec![("q", query.to_string())];
        if let Some(l) = limit {
            params.push(("limit", l.to_string()));
        }
        let query_refs: Vec<(&str, &str)> = params.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query("/subreddits/search", &query_refs)
            .await
    }
}
