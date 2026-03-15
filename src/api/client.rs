use crate::cache::CachedToken;
use crate::config::Settings;
use crate::error::{RedditError, Result};
use reqwest::header::{ACCEPT, ACCEPT_LANGUAGE, HeaderMap, HeaderValue, USER_AGENT};
use std::time::Duration;

pub struct Client {
    http: reqwest::Client,
    base_url: String,
    www_url: String,
}

impl Client {
    pub fn new(settings: &Settings) -> Result<Self> {
        let mut headers = HeaderMap::new();
        headers.insert(
            USER_AGENT,
            HeaderValue::from_str(&settings.api.user_agent)
                .map_err(|e| RedditError::Config(format!("Invalid user agent: {}", e)))?,
        );
        headers.insert(
            ACCEPT,
            HeaderValue::from_static("application/json, text/plain, */*"),
        );
        headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.9"));

        let http = reqwest::Client::builder()
            .default_headers(headers)
            .timeout(Duration::from_secs(30))
            .build()?;

        Ok(Self {
            http,
            base_url: settings.api.base_url.clone(),
            www_url: settings.api.www_url.clone(),
        })
    }

    /// Make a GET request to the www (non-OAuth) API
    pub async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let url = format!("{}{}.json", self.www_url, path);

        tracing::debug!("GET {}", url);

        let response = self
            .http
            .get(&url)
            .query(&[("raw_json", "1")])
            .send()
            .await?;

        let status = response.status();
        if status == reqwest::StatusCode::FORBIDDEN {
            return Err(RedditError::Auth(
                "Reddit requires OAuth authentication. Run `reddit auth login` first.".to_string(),
            ));
        }
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        // Check rate limit headers
        if let Some(remaining) = response.headers().get("x-ratelimit-remaining") {
            tracing::debug!("Rate limit remaining: {:?}", remaining);
        }

        response.json().await.map_err(Into::into)
    }

    /// Make a GET request with query parameters
    pub async fn get_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let url = format!("{}{}.json", self.www_url, path);

        tracing::debug!("GET {} with query {:?}", url, query);

        let mut params: Vec<(&str, &str)> = query.to_vec();
        params.push(("raw_json", "1"));

        let response = self.http.get(&url).query(&params).send().await?;

        let status = response.status();
        if status == reqwest::StatusCode::FORBIDDEN {
            return Err(RedditError::Auth(
                "Reddit requires OAuth authentication. Run `reddit auth login` first.".to_string(),
            ));
        }
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated GET request to the OAuth API
    pub async fn get_authenticated<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("GET {} (authenticated)", url);

        let response = self
            .http
            .get(&url)
            .query(&[("raw_json", "1")])
            .bearer_auth(&token)
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated GET request with query parameters
    pub async fn get_authenticated_with_query<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        query: &[(&str, &str)],
    ) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("GET {} with query {:?} (authenticated)", url, query);

        let mut params: Vec<(&str, &str)> = query.to_vec();
        params.push(("raw_json", "1"));

        let response = self
            .http
            .get(&url)
            .query(&params)
            .bearer_auth(&token)
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated POST request
    pub async fn post_authenticated<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("POST {} (authenticated)", url);

        let response = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .form(form)
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated POST request with empty body
    pub async fn post_authenticated_empty<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("POST {} (authenticated, empty body)", url);

        let response = self
            .http
            .post(&url)
            .bearer_auth(&token)
            .header("Content-Length", "0")
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated PUT request
    pub async fn put_authenticated<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("PUT {} (authenticated)", url);

        let response = self
            .http
            .put(&url)
            .bearer_auth(&token)
            .form(form)
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated DELETE request
    pub async fn delete_authenticated<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("DELETE {} (authenticated)", url);

        let response = self.http.delete(&url).bearer_auth(&token).send().await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Make an authenticated DELETE request with form body
    pub async fn delete_authenticated_with_body<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        form: &[(&str, &str)],
    ) -> Result<T> {
        let token = self.get_valid_token().await?;

        let url = format!("{}{}", self.base_url, path);

        tracing::debug!("DELETE {} with body (authenticated)", url);

        let response = self
            .http
            .delete(&url)
            .bearer_auth(&token)
            .form(form)
            .send()
            .await?;

        let status = response.status();
        if status.is_client_error() || status.is_server_error() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Api(format!("HTTP {}: {}", status, body)));
        }

        response.json().await.map_err(Into::into)
    }

    /// Get a valid access token, refreshing if necessary
    async fn get_valid_token(&self) -> Result<String> {
        let token = CachedToken::load()?.ok_or(RedditError::NotAuthenticated)?;

        if token.is_expired() {
            // Need to refresh token
            let refresh_token = token
                .refresh_token
                .ok_or_else(|| RedditError::Auth("No refresh token available".into()))?;

            let settings = Settings::load()
                .map_err(|e| RedditError::Config(format!("Failed to load settings: {}", e)))?;
            let oauth = super::OAuthClient::new(&settings);
            let new_token = oauth.refresh_token(&refresh_token).await?;

            Ok(new_token.access_token)
        } else {
            Ok(token.access_token)
        }
    }
}
