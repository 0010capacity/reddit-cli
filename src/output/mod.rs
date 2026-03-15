mod json;
mod table;

pub use json::JsonOutput;
pub use table::TableOutput;

pub enum OutputFormat {
    Table,
    Json,
    Plain,
}

impl std::str::FromStr for OutputFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "table" => Ok(OutputFormat::Table),
            "json" => Ok(OutputFormat::Json),
            "plain" => Ok(OutputFormat::Plain),
            _ => Err(format!("Unknown output format: {}", s)),
        }
    }
}

pub trait Output {
    fn format_links(&self, links: &[crate::models::Link]) -> String;
    fn format_subreddit(&self, subreddit: &crate::models::Subreddit) -> String;
    fn format_user(&self, user: &crate::models::User) -> String;
    fn format_comments(&self, comments: &[crate::models::Comment], depth: usize) -> String;
}

pub fn get_output(format: OutputFormat) -> Box<dyn Output> {
    match format {
        OutputFormat::Table => Box::new(TableOutput),
        OutputFormat::Json => Box::new(JsonOutput),
        OutputFormat::Plain => Box::new(PlainOutput),
    }
}

struct PlainOutput;

impl Output for PlainOutput {
    fn format_links(&self, links: &[crate::models::Link]) -> String {
        links
            .iter()
            .map(|l| format!("{} - {} by u/{}", l.id, l.title, l.author))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn format_subreddit(&self, sr: &crate::models::Subreddit) -> String {
        format!("r/{} - {} subscribers", sr.display_name, sr.subscribers)
    }

    fn format_user(&self, user: &crate::models::User) -> String {
        format!("u/{} - {} karma", user.name, user.total_karma.unwrap_or(0))
    }

    fn format_comments(&self, comments: &[crate::models::Comment], depth: usize) -> String {
        let indent = "  ".repeat(depth);
        comments
            .iter()
            .map(|c| {
                format!(
                    "{}[{}] u/{}: {}",
                    indent,
                    c.score,
                    c.author,
                    c.body.lines().next().unwrap_or("")
                )
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
