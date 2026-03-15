use crate::api::Client;
use crate::error::Result;
use crate::models::{Link, ListingResponse};

pub struct SearchEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Clone)]
pub struct SearchParams {
    pub query: String,
    pub subreddit: Option<String>,
    pub sort: Option<String>,
    pub time: Option<String>,
    pub limit: Option<u32>,
    pub restrict_sr: bool,
}

impl<'a> SearchEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Search posts
    pub async fn search(&self, params: &SearchParams) -> Result<ListingResponse<Link>> {
        let path = match &params.subreddit {
            Some(sr) => format!("/r/{}/search", sr),
            None => "/search".to_string(),
        };

        let mut query: Vec<(&str, String)> = vec![("q", params.query.clone())];

        if let Some(ref sort) = params.sort {
            query.push(("sort", sort.clone()));
        }
        if let Some(ref time) = params.time {
            query.push(("t", time.clone()));
        }
        if let Some(l) = params.limit {
            query.push(("limit", l.to_string()));
        }
        if params.restrict_sr {
            query.push(("restrict_sr", "true".to_string()));
        }

        let query_refs: Vec<(&str, &str)> = query.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.client.get_with_query(&path, &query_refs).await
    }
}
