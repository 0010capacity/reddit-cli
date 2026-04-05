use crate::cache::CachedToken;
use crate::config::Settings;
use crate::error::{RedditError, Result};
use chrono::{Duration, Utc};
use reqwest::Client as HttpClient;
use serde::Deserialize;
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;

const REDDIT_AUTH_URL: &str = "https://www.reddit.com/api/v1/authorize";
const REDDIT_TOKEN_URL: &str = "https://www.reddit.com/api/v1/access_token";

const SCOPES: &[&str] = &[
    "identity",        // View account info
    "read",            // Read posts and comments
    "history",         // View voting history
    "submit",          // Submit posts and comments
    "edit",            // Edit posts and comments
    "vote",            // Vote on posts and comments
    "save",            // Save posts and comments
    "report",          // Report posts and comments
    "privatemessages", // Send/receive private messages
    "subscribe",       // Subscribe to subreddits
    "modposts",        // Moderate posts
    "modconfig",       // Configure subreddits
    "modflair",        // Manage flair
    "modlog",          // View mod log
    "modothers",       // Invite/remove moderators
    "modwiki",         // Manage wiki
    "modcontributors", // Manage approved users
    "wikiread",        // Read wiki
    "wikiedit",        // Edit wiki
    "livemanage",      // Manage live threads
];

#[derive(Debug, Deserialize)]
struct RedditTokenResponse {
    access_token: String,
    #[serde(default)]
    refresh_token: Option<String>,
    expires_in: i64,
    token_type: String,
    #[serde(default)]
    scope: Option<String>,
}

pub struct OAuthClient {
    client_id: String,
    redirect_uri: String,
    http: HttpClient,
}

impl OAuthClient {
    pub fn new(settings: &Settings) -> Result<Self> {
        let client_id = settings
            .auth
            .client_id
            .clone()
            .ok_or_else(|| RedditError::Auth("client_id not configured".to_string()))?;

        Ok(Self {
            client_id,
            redirect_uri: settings.auth.redirect_uri.clone(),
            http: HttpClient::new(),
        })
    }

    pub async fn login(&self) -> Result<CachedToken> {
        let state = Self::generate_state();
        let auth_url = self.build_auth_url(&state);

        println!("Opening browser for authentication...");
        println!("If the browser doesn't open automatically, visit this URL:");
        println!();
        println!("  {}", auth_url);
        println!();

        self.open_browser(&auth_url)?;

        println!("Waiting for authorization callback...");
        let code = self.wait_for_callback()?;

        let token = self.exchange_code(code).await?;
        token.save()?;

        println!("Successfully authenticated!");
        Ok(token)
    }

    pub fn logout() -> Result<()> {
        CachedToken::delete()?;
        println!("Logged out successfully.");
        Ok(())
    }

    pub fn status() -> Result<Option<CachedToken>> {
        CachedToken::load()
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<CachedToken> {
        let body = format!(
            "grant_type=refresh_token&refresh_token={}",
            urlencoding::encode(refresh_token)
        );

        let response = self
            .http
            .post(REDDIT_TOKEN_URL)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.client_id, Some(""))
            .body(body)
            .send()
            .await?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Auth(format!("Token refresh failed: {}", body)));
        }

        let token_response: RedditTokenResponse = response.json().await?;
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

        let token = CachedToken {
            access_token: token_response.access_token,
            refresh_token: Some(refresh_token.to_string()),
            expires_at,
            scopes: token_response
                .scope
                .map(|s| s.split(' ').map(String::from).collect())
                .unwrap_or_default(),
        };

        token.save()?;
        println!("Token refreshed successfully.");
        Ok(token)
    }

    fn build_auth_url(&self, state: &str) -> String {
        let scopes = SCOPES.join("%20");
        format!(
            "{}?client_id={}&response_type=code&state={}&redirect_uri={}&duration=permanent&scope={}",
            REDDIT_AUTH_URL,
            urlencoding::encode(&self.client_id),
            state,
            urlencoding::encode(&self.redirect_uri),
            scopes
        )
    }

    fn generate_state() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        format!("{}", timestamp)
    }

    fn open_browser(&self, url: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            std::process::Command::new("open").arg(url).spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            std::process::Command::new("xdg-open").arg(url).spawn()?;
        }

        #[cfg(target_os = "windows")]
        {
            std::process::Command::new("cmd")
                .args(["/C", "start", url])
                .spawn()?;
        }

        Ok(())
    }

    fn wait_for_callback(&self) -> Result<String> {
        let redirect_url: url::Url = self
            .redirect_uri
            .parse()
            .map_err(|e| RedditError::Auth(format!("Invalid redirect URI: {}", e)))?;

        let port = redirect_url.port().unwrap_or(65010);
        let host = redirect_url.host_str().unwrap_or("127.0.0.1");

        let listener = TcpListener::bind(format!("{}:{}", host, port))
            .map_err(|e| RedditError::Auth(format!("Failed to bind port {}: {}", port, e)))?;

        let (mut stream, _) = listener
            .accept()
            .map_err(|e| RedditError::Auth(format!("Failed to accept connection: {}", e)))?;

        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader
            .read_line(&mut request_line)
            .map_err(|e| RedditError::Auth(format!("Failed to read request: {}", e)))?;

        let request: Vec<&str> = request_line.split_whitespace().collect();
        if request.len() < 2 {
            return Err(RedditError::Auth("Invalid callback request".into()));
        }

        let path = request[1];
        let callback_url: url::Url = format!("http://localhost{}", path)
            .parse()
            .map_err(|e| RedditError::Auth(format!("Invalid callback URL: {}", e)))?;

        if let Some(error) = callback_url
            .query_pairs()
            .find(|(key, _)| key == "error")
            .map(|(_, value)| value.to_string())
        {
            return Err(RedditError::Auth(format!("OAuth error: {}", error)));
        }

        let code = callback_url
            .query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| RedditError::Auth("No authorization code in callback".into()))?;

        let response = "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n\
            <!DOCTYPE html>\
            <html><head><title>Authentication Successful</title></head>\
            <body style=\"font-family: sans-serif; text-align: center; padding: 50px;\">\
            <h1 style=\"color: #228B22;\">✓ Authentication Successful!</h1>\
            <p>You can close this window and return to the terminal.</p>\
            </body></html>";
        stream
            .write_all(response.as_bytes())
            .map_err(|e| RedditError::Auth(format!("Failed to send response: {}", e)))?;

        Ok(code)
    }

    async fn exchange_code(&self, code: String) -> Result<CachedToken> {
        let body = format!(
            "grant_type=authorization_code&code={}&redirect_uri={}",
            urlencoding::encode(&code),
            urlencoding::encode(&self.redirect_uri)
        );

        let response = self
            .http
            .post(REDDIT_TOKEN_URL)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.client_id, Some(""))
            .body(body)
            .send()
            .await?;

        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Auth(format!(
                "Token exchange failed (HTTP {}): {}",
                status, body
            )));
        }

        let token_response: RedditTokenResponse = response.json().await?;
        let expires_at = Utc::now() + Duration::seconds(token_response.expires_in);

        Ok(CachedToken {
            access_token: token_response.access_token,
            refresh_token: token_response.refresh_token,
            expires_at,
            scopes: token_response
                .scope
                .map(|s| s.split(' ').map(String::from).collect())
                .unwrap_or_default(),
        })
    }
}

mod urlencoding {
    pub fn encode(s: &str) -> String {
        url::form_urlencoded::byte_serialize(s.as_bytes()).collect()
    }
}
