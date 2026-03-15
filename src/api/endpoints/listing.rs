use crate::api::Client;
use crate::error::Result;
use crate::models::{Link, ListingResponse, TimePeriod};

pub struct ListingEndpoint<'a> {
    client: &'a Client,
}

impl<'a> ListingEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get hot posts
    pub async fn hot(
        &self,
        subreddit: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/hot", sr),
            None => "/hot".to_string(),
        };

        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&path, &query_refs).await
    }

    /// Get new posts
    pub async fn new_posts(
        &self,
        subreddit: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/new", sr),
            None => "/new".to_string(),
        };

        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&path, &query_refs).await
    }

    /// Get top posts
    pub async fn top(
        &self,
        subreddit: Option<&str>,
        time: TimePeriod,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/top", sr),
            None => "/top".to_string(),
        };

        let mut query: Vec<(&str, String)> = vec![("t", time.to_string())];
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&path, &query_refs).await
    }

    /// Get rising posts
    pub async fn rising(
        &self,
        subreddit: Option<&str>,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/rising", sr),
            None => "/rising".to_string(),
        };

        let mut query: Vec<(&str, String)> = Vec::new();
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&path, &query_refs).await
    }

    /// Get controversial posts
    pub async fn controversial(
        &self,
        subreddit: Option<&str>,
        time: TimePeriod,
        limit: Option<u32>,
        after: Option<&str>,
    ) -> Result<ListingResponse<Link>> {
        let path = match subreddit {
            Some(sr) => format!("/r/{}/controversial", sr),
            None => "/controversial".to_string(),
        };

        let mut query: Vec<(&str, String)> = vec![("t", time.to_string())];
        if let Some(l) = limit {
            query.push(("limit", l.to_string()));
        }
        if let Some(a) = after {
            query.push(("after", a.to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&path, &query_refs).await
    }
}
