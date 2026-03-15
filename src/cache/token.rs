use crate::error::{RedditError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Cached OAuth token
#[derive(Debug, Serialize, Deserialize)]
pub struct CachedToken {
    /// OAuth2 access token
    pub access_token: String,
    /// OAuth2 refresh token (optional for some flows)
    pub refresh_token: Option<String>,
    /// Token expiration time
    pub expires_at: DateTime<Utc>,
    /// Scopes granted to this token
    pub scopes: Vec<String>,
}

impl CachedToken {
    /// Check if the token is expired or about to expire
    /// Returns true if the token expires within 5 minutes
    pub fn is_expired(&self) -> bool {
        let now = Utc::now();
        // Consider expired 5 minutes before actual expiry
        now >= self.expires_at - chrono::Duration::minutes(5)
    }

    /// Get the token cache file path
    fn token_path() -> Result<std::path::PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| RedditError::Config("Could not find config directory".into()))?;
        Ok(config_dir.join("reddit-cli").join("token.json"))
    }

    /// Load token from cache file
    pub fn load() -> Result<Option<Self>> {
        let path = Self::token_path()?;

        if !path.exists() {
            return Ok(None);
        }

        let content = std::fs::read_to_string(&path)?;
        let token: Self = serde_json::from_str(&content)?;
        Ok(Some(token))
    }

    /// Save token to cache file
    pub fn save(&self) -> Result<()> {
        let path = Self::token_path()?;
        let parent = path
            .parent()
            .ok_or_else(|| RedditError::Config("Could not determine parent directory".into()))?;
        std::fs::create_dir_all(parent)?;

        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        // Set file permissions to 600 on Unix (only owner can read/write)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(&path, std::fs::Permissions::from_mode(0o600))?;
        }

        Ok(())
    }

    /// Delete the cached token (logout)
    pub fn delete() -> Result<()> {
        let path = Self::token_path()?;
        if path.exists() {
            std::fs::remove_file(&path)?;
        }
        Ok(())
    }
}
