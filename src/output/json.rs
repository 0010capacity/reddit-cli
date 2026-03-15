use crate::models::{Comment, Link, Subreddit, User};

pub struct JsonOutput;

impl super::Output for JsonOutput {
    fn format_links(&self, links: &[Link]) -> String {
        serde_json::to_string_pretty(links).unwrap_or_default()
    }

    fn format_subreddit(&self, sr: &Subreddit) -> String {
        serde_json::to_string_pretty(sr).unwrap_or_default()
    }

    fn format_user(&self, user: &User) -> String {
        serde_json::to_string_pretty(user).unwrap_or_default()
    }

    fn format_comments(&self, comments: &[Comment], _depth: usize) -> String {
        serde_json::to_string_pretty(comments).unwrap_or_default()
    }
}
