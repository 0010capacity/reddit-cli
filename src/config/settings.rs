use anyhow::Result;
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize, Clone, Default)]
pub struct Settings {
    #[serde(default)]
    pub api: ApiSettings,
    #[serde(default)]
    pub output: OutputSettings,
    #[serde(default)]
    pub auth: AuthSettings,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ApiSettings {
    #[serde(default = "default_api_base_url")]
    pub base_url: String,
    #[serde(default = "default_www_url")]
    pub www_url: String,
    #[serde(default = "default_user_agent")]
    pub user_agent: String,
}

impl Default for ApiSettings {
    fn default() -> Self {
        Self {
            base_url: default_api_base_url(),
            www_url: default_www_url(),
            user_agent: default_user_agent(),
        }
    }
}

fn default_api_base_url() -> String {
    "https://oauth.reddit.com".to_string()
}

fn default_www_url() -> String {
    "https://old.reddit.com".to_string()
}

fn default_user_agent() -> String {
    format!(
        "cli:reddit-cli:{} (by /u/anonymous)",
        env!("CARGO_PKG_VERSION")
    )
}

#[derive(Debug, Deserialize, Clone)]
pub struct OutputSettings {
    #[serde(default = "default_format")]
    pub format: String,
    #[serde(default = "default_pager")]
    pub pager: bool,
    #[serde(default = "default_color")]
    pub color: bool,
    #[serde(default = "default_page_size")]
    pub page_size: u32,
}

impl Default for OutputSettings {
    fn default() -> Self {
        Self {
            format: default_format(),
            pager: default_pager(),
            color: default_color(),
            page_size: default_page_size(),
        }
    }
}

fn default_format() -> String {
    "table".to_string()
}

fn default_pager() -> bool {
    true
}

fn default_color() -> bool {
    true
}

fn default_page_size() -> u32 {
    25
}

#[derive(Debug, Deserialize, Clone, Default)]
pub struct AuthSettings {
    #[serde(default)]
    pub client_id: Option<String>,
    #[serde(default)]
    pub client_secret: Option<String>,
    #[serde(default = "default_redirect_uri")]
    pub redirect_uri: String,
}

fn default_redirect_uri() -> String {
    "http://127.0.0.1:65010".to_string()
}

impl Settings {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let settings: Settings = toml::from_str(&content)?;
            Ok(settings)
        } else {
            Ok(Self::default())
        }
    }

    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("reddit-cli");
        Ok(config_dir.join("config.toml"))
    }

    pub fn token_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("reddit-cli");
        Ok(config_dir.join("token.json"))
    }
}
