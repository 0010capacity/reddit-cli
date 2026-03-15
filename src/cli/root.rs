use clap::{Parser, Subcommand};
use crate::api::Client;
use crate::api::endpoints::{AccountEndpoint, ListingEndpoint, SubredditEndpoint, UserEndpoint, SearchEndpoint, LinkEndpoint};
use crate::api::endpoints::{CommentEndpoint, FollowEndpoint, SaveEndpoint, SubmitEndpoint, SubscribeEndpoint, VoteEndpoint, SubmitKind, SubmitOptions};
use crate::api::OAuthClient;
use crate::config::Settings;
use crate::output::{get_output, OutputFormat};
use crate::models::TimePeriod;

#[derive(Parser)]
#[command(name = "reddit")]
#[command(about = "A CLI client for Reddit API", long_about = None)]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,

    /// Output format (table, json, plain)
    #[arg(short, long, global = true, default_value = "table")]
    format: String,

    /// Number of items to fetch
    #[arg(short = 'n', long, global = true, default_value = "25")]
    limit: u32,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,
}

#[derive(Subcommand)]
pub enum Commands {
    /// View hot posts
    Hot {
        /// Subreddit name (without r/)
        #[arg(short, long)]
        subreddit: Option<String>,
    },
    /// View new posts
    New {
        #[arg(short, long)]
        subreddit: Option<String>,
    },
    /// View top posts
    Top {
        #[arg(short, long)]
        subreddit: Option<String>,
        /// Time period: hour, day, week, month, year, all
        #[arg(short = 't', long, default_value = "day")]
        time: String,
    },
    /// View rising posts
    Rising {
        #[arg(short, long)]
        subreddit: Option<String>,
    },
    /// View controversial posts
    Controversial {
        #[arg(short, long)]
        subreddit: Option<String>,
        #[arg(short = 't', long, default_value = "day")]
        time: String,
    },
    /// Subreddit commands
    #[command(subcommand)]
    Subreddit(SubredditCommands),
    /// User commands
    #[command(subcommand)]
    User(UserCommands),
    /// Search posts
    Search {
        /// Search query
        query: String,
        /// Restrict to subreddit
        #[arg(short, long)]
        subreddit: Option<String>,
        /// Sort by: relevance, hot, top, new, comments
        #[arg(short, long, default_value = "relevance")]
        sort: String,
    },
    /// View a post
    Post {
        /// Post ID (base36)
        id: String,
    },
    /// Authentication commands
    #[command(subcommand)]
    Auth(AuthCommands),
    /// Account commands (requires authentication)
    #[command(subcommand)]
    Me(MeCommands),
    /// Upvote a post or comment
    Upvote {
        /// Fullname (t3_xxx for posts, t1_xxx for comments)
        id: String,
    },
    /// Downvote a post or comment
    Downvote {
        /// Fullname (t3_xxx for posts, t1_xxx for comments)
        id: String,
    },
    /// Remove vote from a post or comment
    Unvote {
        /// Fullname (t3_xxx for posts, t1_xxx for comments)
        id: String,
    },
    /// Save a post or comment
    Save {
        /// Fullname (t3_xxx for posts, t1_xxx for comments)
        id: String,
        /// Category for saved item
        #[arg(short, long)]
        category: Option<String>,
    },
    /// Unsave a post or comment
    Unsave {
        /// Fullname
        id: String,
    },
    /// Hide a post
    Hide {
        /// Fullname (t3_xxx)
        id: String,
    },
    /// Unhide a post
    Unhide {
        /// Fullname (t3_xxx)
        id: String,
    },
    /// Subscribe to a subreddit
    Subscribe {
        /// Subreddit name (without r/)
        subreddit: String,
    },
    /// Unsubscribe from a subreddit
    Unsubscribe {
        /// Subreddit name (without r/)
        subreddit: String,
    },
    /// Submit a post
    Submit {
        #[command(subcommand)]
        command: SubmitCommands,
    },
    /// Comment on a post or reply to a comment
    Comment {
        /// Parent fullname (t3_xxx for post, t1_xxx for comment)
        parent: String,
        /// Comment text (markdown)
        #[arg(short, long)]
        text: String,
    },
    /// Edit a post or comment
    Edit {
        /// Fullname of the thing to edit
        id: String,
        /// New text (markdown)
        #[arg(short, long)]
        text: String,
    },
    /// Delete a post or comment
    Delete {
        /// Fullname to delete
        id: String,
    },
    /// Follow a post
    Follow {
        /// Post fullname (t3_xxx)
        id: String,
    },
    /// Unfollow a post
    Unfollow {
        /// Post fullname (t3_xxx)
        id: String,
    },
}

#[derive(Subcommand)]
pub enum SubmitCommands {
    /// Submit a link post
    Link {
        /// Subreddit name
        #[arg(short = 'r', long)]
        subreddit: String,
        /// Post title
        #[arg(short, long)]
        title: String,
        /// URL
        url: String,
        /// Mark as NSFW
        #[arg(long)]
        nsfw: bool,
        /// Mark as spoiler
        #[arg(long)]
        spoiler: bool,
    },
    /// Submit a self (text) post
    Text {
        /// Subreddit name
        #[arg(short = 'r', long)]
        subreddit: String,
        /// Post title
        #[arg(short, long)]
        title: String,
        /// Post body (markdown)
        #[arg(short, long)]
        text: Option<String>,
        /// Read text from file
        #[arg(short = 'f', long)]
        file: Option<String>,
        /// Mark as NSFW
        #[arg(long)]
        nsfw: bool,
        /// Mark as spoiler
        #[arg(long)]
        spoiler: bool,
    },
}

#[derive(Subcommand)]
pub enum SubredditCommands {
    /// Show subreddit info
    Show { name: String },
    /// View subreddit posts (alias for hot)
    Hot { name: String },
    /// View subreddit new posts
    New { name: String },
    /// View subreddit top posts
    Top {
        name: String,
        #[arg(short = 't', long, default_value = "day")]
        time: String,
    },
    /// View subreddit rules
    Rules { name: String },
}

#[derive(Subcommand)]
pub enum UserCommands {
    /// Show user info
    Show { username: String },
    /// View user's posts
    Posts { username: String },
    /// View user's comments
    Comments { username: String },
    /// View user's overview
    Overview { username: String },
}

#[derive(Subcommand)]
pub enum AuthCommands {
    /// Login to Reddit (OAuth)
    Login,
    /// Logout
    Logout,
    /// Show auth status
    Status,
    /// Refresh access token
    Refresh,
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
    /// Show subreddits where you are a contributor
    Contributor {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// Show subreddits where you are a moderator
    Moderator {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
}

impl Cli {
    pub async fn run(&self) -> anyhow::Result<()> {
        // Load config
        let settings = Settings::load()?;

        // Initialize API client
        let client = Client::new(&settings)?;

        // Get output formatter
        let format: OutputFormat = self.format.parse()
            .unwrap_or(OutputFormat::Table);
        let output = get_output(format);

        match &self.command {
            Commands::Hot { subreddit } => {
                let listing = ListingEndpoint::new(&client)
                    .hot(subreddit.as_deref(), Some(self.limit), None)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                print!("{}", output.format_links(&links));
            }
            Commands::New { subreddit } => {
                let listing = ListingEndpoint::new(&client)
                    .new_posts(subreddit.as_deref(), Some(self.limit), None)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                print!("{}", output.format_links(&links));
            }
            Commands::Top { subreddit, time } => {
                let time_period: TimePeriod = time.parse()
                    .unwrap_or(TimePeriod::Day);

                let listing = ListingEndpoint::new(&client)
                    .top(subreddit.as_deref(), time_period, Some(self.limit), None)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                print!("{}", output.format_links(&links));
            }
            Commands::Rising { subreddit } => {
                let listing = ListingEndpoint::new(&client)
                    .rising(subreddit.as_deref(), Some(self.limit), None)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                print!("{}", output.format_links(&links));
            }
            Commands::Controversial { subreddit, time } => {
                let time_period: TimePeriod = time.parse()
                    .unwrap_or(TimePeriod::Day);

                let listing = ListingEndpoint::new(&client)
                    .controversial(subreddit.as_deref(), time_period, Some(self.limit), None)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                print!("{}", output.format_links(&links));
            }
            Commands::Subreddit(cmd) => {
                match cmd {
                    SubredditCommands::Show { name } => {
                        let response = SubredditEndpoint::new(&client)
                            .about(name)
                            .await?;
                        print!("{}", output.format_subreddit(&response.data));
                    }
                    SubredditCommands::Hot { name } => {
                        let listing = ListingEndpoint::new(&client)
                            .hot(Some(name), Some(self.limit), None)
                            .await?;

                        let links: Vec<_> = listing.data.children.into_iter()
                            .map(|t| t.data)
                            .collect();

                        print!("{}", output.format_links(&links));
                    }
                    SubredditCommands::New { name } => {
                        let listing = ListingEndpoint::new(&client)
                            .new_posts(Some(name), Some(self.limit), None)
                            .await?;

                        let links: Vec<_> = listing.data.children.into_iter()
                            .map(|t| t.data)
                            .collect();

                        print!("{}", output.format_links(&links));
                    }
                    SubredditCommands::Top { name, time } => {
                        let time_period: TimePeriod = time.parse()
                            .unwrap_or(TimePeriod::Day);

                        let listing = ListingEndpoint::new(&client)
                            .top(Some(name), time_period, Some(self.limit), None)
                            .await?;

                        let links: Vec<_> = listing.data.children.into_iter()
                            .map(|t| t.data)
                            .collect();

                        print!("{}", output.format_links(&links));
                    }
                    SubredditCommands::Rules { name } => {
                        let rules = SubredditEndpoint::new(&client)
                            .rules(name)
                            .await?;

                        for (i, rule) in rules.rules.iter().enumerate() {
                            println!("{}. {} - {}", i + 1, rule.short_name, rule.description);
                        }
                    }
                }
            }
            Commands::User(cmd) => {
                match cmd {
                    UserCommands::Show { username } => {
                        let response = UserEndpoint::new(&client)
                            .about(username)
                            .await?;
                        print!("{}", output.format_user(&response.data));
                    }
                    UserCommands::Posts { username } => {
                        let listing = UserEndpoint::new(&client)
                            .submitted(username, Some(self.limit), None)
                            .await?;

                        let links: Vec<_> = listing.data.children.into_iter()
                            .map(|t| t.data)
                            .collect();

                        print!("{}", output.format_links(&links));
                    }
                    UserCommands::Comments { username } => {
                        let listing = UserEndpoint::new(&client)
                            .comments(username, Some(self.limit), None)
                            .await?;

                        let comments: Vec<_> = listing.data.children.into_iter()
                            .map(|t| t.data)
                            .collect();

                        print!("{}", output.format_comments(&comments, 0));
                    }
                    UserCommands::Overview { username } => {
                        let _ = UserEndpoint::new(&client)
                            .overview(username, Some(self.limit))
                            .await?;
                        // Overview returns mixed content, just print for now
                        println!("Overview for u/{}", username);
                    }
                }
            }
            Commands::Search { query, subreddit, sort } => {
                let params = crate::api::endpoints::search::SearchParams {
                    query: query.clone(),
                    subreddit: subreddit.clone(),
                    sort: Some(sort.clone()),
                    time: None,
                    limit: Some(self.limit),
                    restrict_sr: subreddit.is_some(),
                };

                let listing = SearchEndpoint::new(&client)
                    .search(&params)
                    .await?;

                let links: Vec<_> = listing.data.children.into_iter()
                    .map(|t| t.data)
                    .collect();

                print!("{}", output.format_links(&links));
            }
            Commands::Post { id } => {
                let result = LinkEndpoint::new(&client)
                    .comments(id, Some(self.limit))
                    .await?;

                // Result is an array: [post_listing, comments_listing]
                if let Some(post_data) = result.get(0) {
                    if let Some(children) = post_data["data"]["children"].as_array() {
                        for child in children {
                            let link: crate::models::Link = serde_json::from_value(child["data"].clone())?;
                            println!("=== {} ===", link.title);
                            println!("Posted by u/{} in r/{}", link.author, link.subreddit);
                            println!("Score: {} | Comments: {}", link.score, link.num_comments);
                            if !link.selftext.is_empty() {
                                println!("\n{}", link.selftext);
                            }
                            println!("\n--- Comments ---\n");
                        }
                    }
                }

                if let Some(comments_data) = result.get(1) {
                    if let Some(children) = comments_data["data"]["children"].as_array() {
                        let comments: Vec<crate::models::Comment> = children
                            .iter()
                            .filter_map(|c| serde_json::from_value(c["data"].clone()).ok())
                            .collect();
                        print!("{}", output.format_comments(&comments, 0));
                    }
                }
            }
            Commands::Auth(cmd) => {
                match cmd {
                    AuthCommands::Login => {
                        let oauth = OAuthClient::new(&settings);
                        oauth.login().await?;
                    }
                    AuthCommands::Logout => {
                        OAuthClient::logout()?;
                    }
                    AuthCommands::Status => {
                        match OAuthClient::status()? {
                            Some(token) => {
                                println!("Logged in");
                                println!("Token expires at: {}", token.expires_at);
                                if token.is_expired() {
                                    println!("Status: EXPIRED (run 'reddit auth refresh')");
                                } else {
                                    println!("Status: Valid");
                                }
                                if !token.scopes.is_empty() {
                                    println!("Scopes: {}", token.scopes.join(", "));
                                }
                            }
                            None => {
                                println!("Not logged in");
                                println!("Run 'reddit auth login' to authenticate.");
                            }
                        }
                    }
                    AuthCommands::Refresh => {
                        let token = crate::cache::CachedToken::load()?
                            .ok_or_else(|| anyhow::anyhow!("Not logged in"))?;
                        let refresh_token = token.refresh_token
                            .ok_or_else(|| anyhow::anyhow!("No refresh token available"))?;
                        let oauth = OAuthClient::new(&settings);
                        oauth.refresh_token(&refresh_token).await?;
                    }
                }
            }
            Commands::Me(cmd) => {
                match cmd {
                    MeCommands::Info => {
                        let account = AccountEndpoint::new(&client)
                            .me()
                            .await?;
                        println!("=== Account Info ===");
                        println!("Username: u/{}", account.name);
                        println!("ID: {}", account.id);
                        println!("Created: {}", chrono::DateTime::from_timestamp(account.created_utc as i64, 0)
                            .map(|d| d.format("%Y-%m-%d %H:%M:%S UTC").to_string())
                            .unwrap_or_else(|| "Unknown".to_string()));
                        println!();
                        println!("Karma:");
                        println!("  Link Karma: {}", account.link_karma);
                        println!("  Comment Karma: {}", account.comment_karma);
                        println!("  Total Karma: {}", account.total_karma);
                        println!();
                        println!("Flags:");
                        println!("  Verified Email: {}", if account.has_verified_email { "Yes" } else { "No" });
                        println!("  Gold Member: {}", if account.is_gold { "Yes" } else { "No" });
                        println!("  Moderator: {}", if account.is_mod { "Yes" } else { "No" });
                        println!("  Employee: {}", if account.is_employee { "Yes" } else { "No" });
                        println!("  Over 18: {}", if account.over_18 { "Yes" } else { "No" });
                        println!("  Night Mode: {}", if account.pref_nightmode { "Yes" } else { "No" });
                        println!("  Inbox Count: {}", account.inbox_count);
                    }
                    MeCommands::Karma => {
                        let karma = AccountEndpoint::new(&client)
                            .karma()
                            .await?;
                        println!("=== Karma Breakdown ===\n");
                        if karma.data.is_empty() {
                            println!("No karma data available.");
                        } else {
                            println!("{:<30} {:>15} {:>15}", "Subreddit", "Link Karma", "Comment Karma");
                            println!("{}", "-".repeat(62));
                            for entry in &karma.data {
                                println!("{:<30} {:>15} {:>15}", entry.sr, entry.link_karma, entry.comment_karma);
                            }
                            println!("{}", "-".repeat(62));
                            let total_link: i64 = karma.data.iter().map(|k| k.link_karma).sum();
                            let total_comment: i64 = karma.data.iter().map(|k| k.comment_karma).sum();
                            println!("{:<30} {:>15} {:>15}", "TOTAL", total_link, total_comment);
                        }
                    }
                    MeCommands::Prefs => {
                        let prefs = AccountEndpoint::new(&client)
                            .preferences()
                            .await?;
                        println!("=== Preferences ===\n");
                        println!("Language: {}", prefs.lang.as_deref().unwrap_or("default"));
                        println!("Over 18: {}", prefs.over_18);
                        println!("Night Mode: {}", prefs.nightmode);
                        println!("Show Flair: {}", prefs.show_flair);
                        println!("Show Link Flair: {}", prefs.show_link_flair);
                        println!("Enable Followers: {}", prefs.enable_followers);
                        println!("Hide from Robots: {}", prefs.hide_from_robots);
                        println!("Email Messages: {}", prefs.email_messages);
                        println!("Email Digests: {}", prefs.email_digests);
                        if let Some(sort) = &prefs.default_comment_sort {
                            println!("Default Comment Sort: {}", sort);
                        }
                        if let Some(num) = prefs.num_comments {
                            println!("Comments per page: {}", num);
                        }
                        if let Some(num) = prefs.num_sites {
                            println!("Links per page: {}", num);
                        }
                    }
                    MeCommands::Trophies => {
                        let trophies = AccountEndpoint::new(&client)
                            .trophies()
                            .await?;
                        println!("=== Trophies ===\n");
                        if trophies.data.trophies.is_empty() {
                            println!("No trophies yet.");
                        } else {
                            for trophy in &trophies.data.trophies {
                                println!("* {}", trophy.name);
                                if let Some(desc) = &trophy.description {
                                    println!("  {}", desc);
                                }
                            }
                        }
                    }
                    MeCommands::Subreddits { limit } => {
                        let subs = AccountEndpoint::new(&client)
                            .subscribed(Some(*limit))
                            .await?;
                        println!("=== Subscribed Subreddits ===\n");
                        if subs.data.children.is_empty() {
                            println!("No subscribed subreddits found.");
                        } else {
                            println!("{:<25} {:>12} {:>10}", "Name", "Subscribers", "NSFW");
                            println!("{}", "-".repeat(49));
                            for sub in &subs.data.children {
                                println!("{:<25} {:>12} {:>10}",
                                    sub.data.display_name,
                                    sub.data.subscribers,
                                    if sub.data.over18 { "Yes" } else { "No" }
                                );
                            }
                            println!();
                            println!("Total: {} subreddits", subs.data.children.len());
                        }
                    }
                    MeCommands::Contributor { limit } => {
                        let subs = AccountEndpoint::new(&client)
                            .contributor(Some(*limit))
                            .await?;
                        println!("=== Contributor Subreddits ===\n");
                        if subs.data.children.is_empty() {
                            println!("You are not a contributor in any subreddits.");
                        } else {
                            for sub in &subs.data.children {
                                println!("* r/{} - {}", sub.data.display_name, sub.data.title);
                            }
                        }
                    }
                    MeCommands::Moderator { limit } => {
                        let subs = AccountEndpoint::new(&client)
                            .moderator(Some(*limit))
                            .await?;
                        println!("=== Moderated Subreddits ===\n");
                        if subs.data.children.is_empty() {
                            println!("You are not a moderator of any subreddits.");
                        } else {
                            for sub in &subs.data.children {
                                println!("* r/{} - {}", sub.data.display_name, sub.data.title);
                            }
                        }
                    }
                }
            }
            Commands::Upvote { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::VoteEndpoint::new(&client)
                    .upvote(id).await?;
                println!("Upvoted {}", id);
            }
            Commands::Downvote { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::VoteEndpoint::new(&client)
                    .downvote(id).await?;
                println!("Downvoted {}", id);
            }
            Commands::Unvote { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::VoteEndpoint::new(&client)
                    .unvote(id).await?;
                println!("Vote removed from {}", id);
            }
            Commands::Save { id, category } => {
                ensure_authenticated()?;
                crate::api::endpoints::SaveEndpoint::new(&client)
                    .save(id, category.as_deref()).await?;
                println!("Saved {}", id);
            }
            Commands::Unsave { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::SaveEndpoint::new(&client)
                    .unsave(id).await?;
                println!("Unsaved {}", id);
            }
            Commands::Hide { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::SaveEndpoint::new(&client)
                    .hide(id).await?;
                println!("Hidden {}", id);
            }
            Commands::Unhide { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::SaveEndpoint::new(&client)
                    .unhide(id).await?;
                println!("Unhidden {}", id);
            }
            Commands::Subscribe { subreddit } => {
                ensure_authenticated()?;
                crate::api::endpoints::SubscribeEndpoint::new(&client)
                    .subscribe(subreddit).await?;
                println!("Subscribed to r/{}", subreddit);
            }
            Commands::Unsubscribe { subreddit } => {
                ensure_authenticated()?;
                crate::api::endpoints::SubscribeEndpoint::new(&client)
                    .unsubscribe(subreddit).await?;
                println!("Unsubscribed from r/{}", subreddit);
            }
            Commands::Submit { command } => {
                ensure_authenticated()?;
                match command {
                    SubmitCommands::Link { subreddit, title, url, nsfw, spoiler } => {
                        let result = crate::api::endpoints::SubmitEndpoint::new(&client)
                            .submit(&crate::api::endpoints::SubmitOptions {
                                subreddit: subreddit.clone(),
                                title: title.clone(),
                                kind: crate::api::endpoints::SubmitKind::Link,
                                url: Some(url.clone()),
                                text: None,
                                flair_id: None,
                                flair_text: None,
                                nsfw: *nsfw,
                                spoiler: *spoiler,
                                send_replies: true,
                            }).await?;
                        if let Some(data) = result.json.data {
                            println!("Posted: {}", data.url);
                        } else if !result.json.errors.is_empty() {
                            anyhow::bail!("Failed to submit: {:?}", result.json.errors);
                        }
                    }
                    SubmitCommands::Text { subreddit, title, text, file, nsfw, spoiler } => {
                        let body = if let Some(path) = file {
                            std::fs::read_to_string(path)?
                        } else {
                            text.clone().unwrap_or_default()
                        };

                        let result = crate::api::endpoints::SubmitEndpoint::new(&client)
                            .submit(&crate::api::endpoints::SubmitOptions {
                                subreddit: subreddit.clone(),
                                title: title.clone(),
                                kind: crate::api::endpoints::SubmitKind::SelfPost,
                                url: None,
                                text: Some(body),
                                flair_id: None,
                                flair_text: None,
                                nsfw: *nsfw,
                                spoiler: *spoiler,
                                send_replies: true,
                            }).await?;
                        if let Some(data) = result.json.data {
                            println!("Posted: {}", data.url);
                        } else if !result.json.errors.is_empty() {
                            anyhow::bail!("Failed to submit: {:?}", result.json.errors);
                        }
                    }
                }
            }
            Commands::Comment { parent, text } => {
                ensure_authenticated()?;
                let result = crate::api::endpoints::CommentEndpoint::new(&client)
                    .submit(parent, text).await?;
                if let Some(data) = result.json.data {
                    if let Some(thing) = data.things.first() {
                        println!("Comment posted: {}", thing.data.name);
                    }
                } else if !result.json.errors.is_empty() {
                    anyhow::bail!("Failed to comment: {:?}", result.json.errors);
                }
            }
            Commands::Edit { id, text } => {
                ensure_authenticated()?;
                crate::api::endpoints::SubmitEndpoint::new(&client)
                    .edit(id, text).await?;
                println!("Edited {}", id);
            }
            Commands::Delete { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::SubmitEndpoint::new(&client)
                    .delete(id).await?;
                println!("Deleted {}", id);
            }
            Commands::Follow { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::FollowEndpoint::new(&client)
                    .follow(id).await?;
                println!("Following {}", id);
            }
            Commands::Unfollow { id } => {
                ensure_authenticated()?;
                crate::api::endpoints::FollowEndpoint::new(&client)
                    .unfollow(id).await?;
                println!("Unfollowed {}", id);
            }
        }

        Ok(())
    }
}

fn ensure_authenticated() -> anyhow::Result<()> {
    if crate::cache::CachedToken::load()?.is_none() {
        anyhow::bail!("Not authenticated. Run `reddit auth login` first.");
    }
    Ok(())
}
