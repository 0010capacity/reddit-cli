use crate::api::Client;
use crate::error::Result;
use crate::models::{Comment, Link, ListingResponse, User};
use serde::Deserialize;

pub struct UserEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct UserResponse {
    pub data: User,
}

#[derive(Debug, Deserialize)]
pub struct TrophyResponse {
    pub data: Trophies,
}

#[derive(Debug, Deserialize)]
pub struct Trophies {
    pub trophies: Vec<Trophy>,
}

#[derive(Debug, Deserialize)]
pub struct Trophy {
    pub name: String,
    pub description: Option<String>,
    pub icon_40: Option<String>,
}

impl<'a> UserEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get user info
    pub async fn about(&self, username: &str) -> Result<UserResponse> {
        self.client.get(&format!("/user/{}/about", username)).await
    }

    /// Get user's posts
    pub async fn submitted(
        &self,
        username: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&format!("/user/{}/submitted", username), &query_refs)
            .await
    }

    /// Get user's comments
    pub async fn comments(
        &self,
        username: &str,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Comment>> {
        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&format!("/user/{}/comments", username), &query_refs)
            .await
    }

    /// Get user's overview (posts + comments)
    pub async fn overview(&self, username: &str, limit: Option<u32>) -> Result<serde_json::Value> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client
            .get_with_query(&format!("/user/{}/overview", username), &query_refs)
            .await
    }

    /// Get user's trophies
    pub async fn trophies(&self, username: &str) -> Result<TrophyResponse> {
        self.client
            .get(&format!("/user/{}/trophies", username))
            .await
    }
}
