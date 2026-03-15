use thiserror::Error;

#[derive(Error, Debug)]
pub enum RedditError {
    #[error("API error: {0}")]
    Api(String),

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Config error: {0}")]
    Config(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Rate limited. Retry after {0} seconds")]
    RateLimited(u64),

    #[error("Not authenticated. Run `reddit auth login` first")]
    NotAuthenticated,

    #[error("Thing not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, RedditError>;
