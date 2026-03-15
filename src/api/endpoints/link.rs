use crate::api::Client;
use crate::error::Result;
use crate::models::{Link, Thing};
use serde::Deserialize;

pub struct LinkEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct LinkInfoResponse {
    pub data: LinkInfo,
}

#[derive(Debug, Deserialize)]
pub struct LinkInfo {
    pub children: Vec<Thing<Link>>,
}

/// Post with comments response
/// Returns an array: [post_listing, comments_listing]
pub type PostWithComments = Vec<serde_json::Value>;

impl<'a> LinkEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get post with comments
    pub async fn comments(&self, article_id: &str, limit: Option<u32>) -> Result<PostWithComments> {
        let query = if let Some(l) = limit {
            vec![("limit", l.to_string())]
        } else {
            vec![]
        };
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&format!("/comments/{}", article_id), &query_refs).await
    }

    /// Get post info by ID (fullname like t3_xxxxx)
    pub async fn info(&self, id: &str) -> Result<LinkInfoResponse> {
        let query = vec![("id", id.to_string())];
        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query("/api/info", &query_refs).await
    }

    /// Get duplicate posts (crossposts)
    pub async fn duplicates(&self, article_id: &str) -> Result<serde_json::Value> {
        self.client.get(&format!("/duplicates/{}", article_id)).await
    }
}
