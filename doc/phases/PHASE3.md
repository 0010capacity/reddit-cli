# Phase 3: OAuth 인증

> **목표**: OAuth2 인증 흐름 구현, 토큰 관리, 내 계정 관련 API

**전제조건**: Phase 2 완료

---

## 체크리스트

### 3.1 OAuth2 설정
- [ ] `oauth2` crate 의존성 추가
- [ ] `src/api/auth.rs` - OAuth2 클라이언트
- [ ] 로컬 콜백 서버 구현
- [ ] 브라우저 자동 열기

### 3.2 토큰 관리
- [ ] `src/cache/token.rs` - 토큰 저장/로드
- [ ] 토큰 갱신 로직
- [ ] 토큰 만료 확인

### 3.3 인증 CLI
- [ ] `reddit auth login` 커맨드
- [ ] `reddit auth logout` 커맨드
- [ ] `reddit auth status` 커맨드
- [ ] `reddit auth refresh` 커맨드

### 3.4 내 계정 API
- [ ] `src/api/endpoints/account.rs` - 계정 API
- [ ] `reddit me` - 내 정보
- [ ] `reddit me karma` - 내 카르마
- [ ] `reddit me preferences` - 내 설정
- [ ] `reddit me trophies` - 내 트로피

### 3.5 내 서브레딧 API
- [ ] `reddit me subreddits` - 구독한 서브레딧
- [ ] `reddit me contributor` - 기여자인 서브레딧
- [ ] `reddit me moderator` - 모더레이터인 서브레딧

---

## 상세 구현 가이드

### 3.1 Cargo.toml 의존성 추가

```toml
[dependencies]
# ... existing dependencies

# OAuth2
oauth2 = "4"
openidconnect = "3"  # Optional, for more complete OAuth
tokio = { version = "1", features = ["full", "net"] }
```

### 3.2 src/api/auth.rs

```rust
use crate::config::Settings;
use crate::error::{RedditError, Result};
use oauth2::{
    AuthUrl, AuthorizationCode, Client, ClientId, ClientSecret,
    CsrfToken, RedirectUrl, Scope, TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::path::PathBuf;
use tokio::process::Command;

const REDDIT_AUTH_URL: &str = "https://www.reddit.com/api/v1/authorize";
const REDDIT_TOKEN_URL: &str = "https://www.reddit.com/api/v1/access_token";

/// OAuth scopes needed for the CLI
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

#[derive(Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: chrono::DateTime<chrono::Utc>,
    pub token_type: String,
}

pub struct OAuthClient {
    client_id: String,
    client_secret: Option<String>,
    redirect_uri: String,
}

impl OAuthClient {
    pub fn new(settings: &Settings) -> Self {
        Self {
            client_id: settings.auth.client_id.clone().unwrap_or_default(),
            client_secret: settings.auth.client_secret.clone(),
            redirect_uri: settings.auth.redirect_uri.clone(),
        }
    }

    /// Start the OAuth login flow
    pub async fn login(&self) -> Result<TokenInfo> {
        let redirect_url = RedirectUrl::new(self.redirect_uri.clone())
            .map_err(|e| RedditError::Auth(format!("Invalid redirect URL: {}", e)))?;

        let mut client_builder = Client::new(
            ClientId::new(self.client_id.clone()),
            Some(AuthUrl::new(REDDIT_AUTH_URL.to_string()).unwrap()),
        )
        .set_redirect_uri(redirect_url);

        if let Some(secret) = &self.client_secret {
            client_builder = client_builder.set_client_secret(ClientSecret::new(secret.clone()));
        }

        client_builder = client_builder
            .set_token_uri(TokenUrl::new(REDDIT_TOKEN_URL.to_string()).unwrap());

        // Generate authorization URL
        let scopes: Vec<Scope> = SCOPES.iter().map(|s| Scope::new(s.to_string())).collect();

        let (auth_url, csrf_token) = client_builder
            .authorize_url(CsrfToken::new_random)
            .add_scopes(scopes)
            .url();

        println!("Opening browser for authentication...");
        println!("If the browser doesn't open, visit this URL:");
        println!("{}", auth_url);

        // Open browser
        self.open_browser(&auth_url)?;

        // Start local server to receive callback
        let code = self.wait_for_callback()?;

        // Exchange code for token
        let token_info = self.exchange_code(&client_builder, code).await?;

        // Save token
        self.save_token(&token_info)?;

        println!("Successfully authenticated!");
        Ok(token_info)
    }

    fn open_browser(&self, url: &str) -> Result<()> {
        #[cfg(target_os = "macos")]
        {
            Command::new("open").arg(url).spawn()?;
        }

        #[cfg(target_os = "linux")]
        {
            Command::new("xdg-open").arg(url).spawn()?;
        }

        #[cfg(target_os = "windows")]
        {
            Command::new("cmd")
                .args(["/C", "start", url])
                .spawn()?;
        }

        Ok(())
    }

    fn wait_for_callback(&self) -> Result<String> {
        // Extract port from redirect_uri
        let port = self.redirect_uri
            .parse::<url::Url>()
            .map_err(|e| RedditError::Auth(format!("Invalid redirect URI: {}", e)))?
            .port()
            .unwrap_or(65010);

        let listener = TcpListener::bind(format!("127.0.0.1:{}", port))
            .map_err(|e| RedditError::Auth(format!("Failed to bind port: {}", e)))?;

        println!("Waiting for authorization callback...");

        let (mut stream, _) = listener.accept()
            .map_err(|e| RedditError::Auth(format!("Failed to accept connection: {}", e)))?;

        let mut reader = BufReader::new(&stream);
        let mut request_line = String::new();
        reader.read_line(&mut request_line)?;

        // Parse the request to get the code
        let request: Vec<&str> = request_line.split_whitespace().collect();
        if request.len() < 2 {
            return Err(RedditError::Auth("Invalid callback request".into()));
        }

        let path = request[1];
        let url: url::Url = format!("http://localhost{}", path).parse()
            .map_err(|e| RedditError::Auth(format!("Invalid callback URL: {}", e)))?;

        let code = url.query_pairs()
            .find(|(key, _)| key == "code")
            .map(|(_, value)| value.to_string())
            .ok_or_else(|| RedditError::Auth("No code in callback".into()))?;

        // Send success response
        let response = "HTTP/1.1 200 OK\r\n\r\n\
            <html><body>\
            <h1>Authentication successful!</h1>\
            <p>You can close this window now.</p>\
            </body></html>";
        stream.write_all(response.as_bytes())?;

        Ok(code)
    }

    async fn exchange_code(
        &self,
        client: &Client<oauth2::basic::BasicClient>,
        code: String,
    ) -> Result<TokenInfo> {
        // Reddit uses HTTP Basic Auth with client_id:client_secret
        let token_url = format!("{}?grant_type=authorization_code&code={}&redirect_uri={}",
            REDDIT_TOKEN_URL, code, self.redirect_uri);

        let http_client = reqwest::Client::new();

        let mut request = http_client
            .post(&token_url)
            .header("Accept", "application/json");

        if let Some(secret) = &self.client_secret {
            request = request.basic_auth(&self.client_id, Some(secret));
        } else {
            request = request.basic_auth(&self.client_id, Some(""));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Auth(format!("Token exchange failed: {}", body)));
        }

        let token_response: RedditTokenResponse = response.json().await?;

        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(token_response.expires_in);

        Ok(TokenInfo {
            access_token: token_response.access_token,
            refresh_token: Some(token_response.refresh_token),
            expires_at,
            token_type: token_response.token_type,
        })
    }

    fn save_token(&self, token: &TokenInfo) -> Result<()> {
        let path = crate::config::Settings::token_path()?;
        let parent = path.parent().unwrap();

        std::fs::create_dir_all(parent)?;
        let json = serde_json::to_string_pretty(token)?;
        std::fs::write(&path, json)?;

        // Set file permissions to 600 on Unix
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    pub fn load_token() -> Result<Option<TokenInfo>> {
        let path = crate::config::Settings::token_path()?;

        if !path.exists() {
            return Ok(None);
        }

        let json = std::fs::read_to_string(&path)?;
        let token: TokenInfo = serde_json::from_str(&json)?;

        Ok(Some(token))
    }

    pub fn logout() -> Result<()> {
        let path = crate::config::Settings::token_path()?;

        if path.exists() {
            std::fs::remove_file(&path)?;
        }

        println!("Logged out successfully.");
        Ok(())
    }

    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenInfo> {
        let http_client = reqwest::Client::new();

        let body = format!(
            "grant_type=refresh_token&refresh_token={}",
            refresh_token
        );

        let mut request = http_client
            .post(REDDIT_TOKEN_URL)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(body);

        if let Some(secret) = &self.client_secret {
            request = request.basic_auth(&self.client_id, Some(secret));
        } else {
            request = request.basic_auth(&self.client_id, Some(""));
        }

        let response = request.send().await?;

        if !response.status().is_success() {
            let body = response.text().await.unwrap_or_default();
            return Err(RedditError::Auth(format!("Token refresh failed: {}", body)));
        }

        let token_response: RedditTokenResponse = response.json().await?;

        let expires_at = chrono::Utc::now() + chrono::Duration::seconds(token_response.expires_in);

        let token_info = TokenInfo {
            access_token: token_response.access_token,
            refresh_token: Some(refresh_token.to_string()),
            expires_at,
            token_type: token_response.token_type,
        };

        self.save_token(&token_info)?;

        Ok(token_info)
    }
}

#[derive(Debug, Deserialize)]
struct RedditTokenResponse {
    access_token: String,
    refresh_token: String,
    expires_in: i64,
    token_type: String,
    scope: String,
}
```

### 3.3 src/cache/token.rs

```rust
use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct CachedToken {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at: i64, // Unix timestamp
    pub scopes: Vec<String>,
}

impl CachedToken {
    pub fn is_expired(&self) -> bool {
        let now = chrono::Utc::now().timestamp();
        // Consider expired 5 minutes before actual expiry
        now >= self.expires_at - 300
    }

    pub fn load() -> Result<Option<Self>> {
        let path = Self::path()?;

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)?;
        let token: Self = serde_json::from_str(&content)?;
        Ok(Some(token))
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::path()?;
        let parent = path.parent().unwrap();
        std::fs::create_dir_all(parent)?;

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    pub fn delete() -> Result<()> {
        let path = Self::path()?;
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }

    fn path() -> Result<PathBuf> {
        crate::config::Settings::token_path()
    }
}
```

### 3.4 src/api/endpoints/account.rs

```rust
use crate::api::Client;
use crate::error::Result;
use serde::Deserialize;

pub struct AccountEndpoint<'a> {
    client: &'a Client,
}

#[derive(Debug, Deserialize)]
pub struct MeResponse {
    pub data: Account,
}

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
}

#[derive(Debug, Deserialize)]
pub struct KarmaResponse {
    pub data: Vec<KarmaEntry>,
}

#[derive(Debug, Deserialize)]
pub struct KarmaEntry {
    pub sr: String,
    pub sr_t5: String,
    pub link_karma: i64,
    pub comment_karma: i64,
}

#[derive(Debug, Deserialize)]
pub struct PreferencesResponse {
    pub data: Preferences,
}

#[derive(Debug, Deserialize)]
pub struct Preferences {
    pub over_18: bool,
    pub enable_followers: bool,
    pub hide_from_robots: bool,
    pub lang: String,
    pub nightmode: bool,
    pub show_flair: bool,
    pub show_link_flair: bool,
    pub email_messages: bool,
    pub email_digests: bool,
    // ... many more fields
}

#[derive(Debug, Deserialize)]
pub struct TrophiesResponse {
    pub data: TrophiesData,
}

#[derive(Debug, Deserialize)]
pub struct TrophiesData {
    pub trophies: Vec<Trophy>,
}

#[derive(Debug, Deserialize)]
pub struct Trophy {
    pub name: String,
    pub description: Option<String>,
    pub icon_40: Option<String>,
    pub icon_70: Option<String>,
}

impl<'a> AccountEndpoint<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get current user info
    pub async fn me(&self) -> Result<MeResponse> {
        self.client.get_authenticated("/api/v1/me").await
    }

    /// Get karma breakdown
    pub async fn karma(&self) -> Result<KarmaResponse> {
        self.client.get_authenticated("/api/v1/me/karma").await
    }

    /// Get preferences
    pub async fn preferences(&self) -> Result<PreferencesResponse> {
        self.client.get_authenticated("/api/v1/me/prefs").await
    }

    /// Get trophies
    pub async fn trophies(&self) -> Result<TrophiesResponse> {
        self.client.get_authenticated("/api/v1/me/trophies").await
    }
}
```

### 3.5 src/api/client.rs 업데이트

```rust
impl Client {
    /// Make an authenticated GET request
    pub async fn get_authenticated<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
    ) -> Result<T> {
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

    async fn get_valid_token(&self) -> Result<String> {
        let token_info = cache::token::CachedToken::load()?
            .ok_or(RedditError::NotAuthenticated)?;

        if token_info.is_expired() {
            // Need to refresh token
            let refresh_token = token_info.refresh_token
                .ok_or(RedditError::Auth("No refresh token available".into()))?;

            let settings = config::Settings::load()?;
            let oauth = api::auth::OAuthClient::new(&settings);
            let new_token = oauth.refresh_token(&refresh_token).await?;

            Ok(new_token.access_token)
        } else {
            Ok(token_info.access_token)
        }
    }
}
```

---

## CLI 커맨드 구현

### src/cli/root.rs (업데이트)

```rust
#[derive(Subcommand)]
pub enum Commands {
    // ... existing commands

    /// Account commands (requires authentication)
    #[command(subcommand)]
    Me(MeCommands),
}

#[derive(Subcommand)]
pub enum MeCommands {
    /// Show current user info
    Info,
    /// Show karma breakdown
    Karma,
    /// Show preferences
    Prefs,
    /// Show trophies
    Trophies,
    /// Show subscribed subreddits
    Subreddits {
        /// Limit results
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        // ...

        match &self.command {
            // ... existing commands

            Commands::Auth(cmd) => {
                match cmd {
                    AuthCommands::Login => {
                        let settings = config::Settings::load()?;
                        let oauth = api::auth::OAuthClient::new(&settings);
                        oauth.login().await?;
                    }
                    AuthCommands::Logout => {
                        api::auth::OAuthClient::logout()?;
                    }
                    AuthCommands::Status => {
                        match api::cache::token::CachedToken::load()? {
                            Some(token) => {
                                println!("Logged in");
                                println!("Expires at: {}", token.expires_at);
                            }
                            None => {
                                println!("Not logged in");
                            }
                        }
                    }
                    AuthCommands::Refresh => {
                        let settings = config::Settings::load()?;
                        let oauth = api::auth::OAuthClient::new(&settings);
                        let token = api::cache::token::CachedToken::load()?
                            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;
                        let refresh = token.refresh_token
                            .ok_or_else(|| anyhow::anyhow!("No refresh token"))?;
                        oauth.refresh_token(&refresh).await?;
                        println!("Token refreshed successfully");
                    }
                }
            }

            Commands::Me(cmd) => {
                match cmd {
                    MeCommands::Info => {
                        let account = api::endpoints::AccountEndpoint::new(&client)
                            .me().await?;
                        // Format and display
                    }
                    MeCommands::Karma => {
                        let karma = api::endpoints::AccountEndpoint::new(&client)
                            .karma().await?;
                        // Format and display
                    }
                    // ... other me commands
                }
            }
        }

        Ok(())
    }
}
```

---

## 완료 기준

1. `reddit auth login` - 브라우저 열리고 인증 완료
2. `reddit auth status` - 인증 상태 표시
3. `reddit auth logout` - 로그아웃
4. `reddit auth refresh` - 토큰 갱신
5. `reddit me` - 내 계정 정보 표시
6. `reddit me karma` - 카르마 분석 표시
7. `reddit me subreddits` - 구독한 서브레딧 목록
8. 토큰 자동 갱신 동작

---

## 다음 단계

Phase 3 완료 후 → [PHASE4.md](PHASE4.md): 쓰기 API 구현
