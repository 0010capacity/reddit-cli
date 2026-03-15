use clap::{Parser, Subcommand};
use crate::api::Client;
use crate::api::endpoints::{AccountEndpoint, ListingEndpoint, SubredditEndpoint, UserEndpoint, SearchEndpoint, LinkEndpoint};
use crate::api::endpoints::{CommentEndpoint, FollowEndpoint, SaveEndpoint, SubmitEndpoint, SubscribeEndpoint, VoteEndpoint, SubmitKind, SubmitOptions};
use crate::api::endpoints::{MessageEndpoint, MessageFolder, ModerationEndpoint, ModQueueLocation, DistinguishType, UserManagementEndpoint};
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
    /// Message commands (requires authentication)
    #[command(subcommand)]
    Message(MessageCommands),
    /// Moderation commands (requires authentication)
    #[command(subcommand)]
    Mod(ModCommands),
    /// Flair commands
    #[command(subcommand)]
    Flair(FlairCommands),
    /// Wiki commands
    #[command(subcommand)]
    Wiki(WikiCommands),
    /// Multi commands
    #[command(subcommand)]
    Multi(MultiCommands),
    /// Live thread commands
    #[command(subcommand)]
    Live(LiveCommands),
    /// Collection commands
    #[command(subcommand)]
    Collection(CollectionCommands),
    /// Modmail commands
    #[command(subcommand)]
    Modmail(ModmailCommands),
    /// Mod note commands
    #[command(subcommand)]
    Modnote(ModnoteCommands),
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

#[derive(Subcommand)]
pub enum MessageCommands {
    /// View inbox messages
    Inbox {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View unread messages
    Unread {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View sent messages
    Sent {
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// Send a private message
    Send {
        /// Recipient username (or /r/subreddit for mod mail)
        #[arg(short, long)]
        to: String,
        /// Message subject
        #[arg(short, long)]
        subject: String,
        /// Message body (markdown)
        #[arg(short, long)]
        text: String,
        /// Send as subreddit (for mod mail)
        #[arg(short = 'r', long)]
        from: Option<String>,
    },
    /// Mark messages as read
    Read {
        /// Message IDs (space-separated)
        ids: Vec<String>,
    },
    /// Mark all messages as read
    ReadAll,
    /// Mark messages as unread
    UnreadMsg {
        /// Message IDs (space-separated)
        ids: Vec<String>,
    },
    /// Delete a message
    Delete {
        id: String,
    },
    /// Block the sender of a message
    Block {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ModCommands {
    /// View reports queue
    Reports {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View spam queue
    Spam {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View mod queue
    Queue {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View unmoderated posts
    Unmoderated {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View edited posts/comments
    Edited {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View mod log
    Log {
        subreddit: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
        /// Filter by moderator
        #[arg(short, long)]
        moderator: Option<String>,
    },
    /// Approve a post or comment
    Approve {
        id: String,
    },
    /// Remove a post or comment
    Remove {
        id: String,
        /// Mark as spam
        #[arg(short, long)]
        spam: bool,
    },
    /// Distinguish a comment as mod
    Distinguish {
        id: String,
        /// Make sticky (for top-level comments)
        #[arg(short, long)]
        sticky: bool,
    },
    /// Remove mod distinction from a comment
    Undistinguish {
        id: String,
    },
    /// Sticky a post
    Sticky {
        id: String,
        /// Slot number (1-4)
        #[arg(short, long)]
        slot: Option<u8>,
    },
    /// Unsticky a post
    Unsticky {
        id: String,
    },
    /// Lock a post or comment
    Lock {
        id: String,
    },
    /// Unlock a post or comment
    Unlock {
        id: String,
    },
    /// Mark post as NSFW
    Nsfw {
        id: String,
    },
    /// Unmark post as NSFW
    Unnsfw {
        id: String,
    },
    /// Mark post as spoiler
    Spoiler {
        id: String,
    },
    /// Unmark post as spoiler
    Unspoiler {
        id: String,
    },
    /// Report a post or comment
    Report {
        id: String,
        /// Report reason
        #[arg(short, long)]
        reason: String,
    },
    /// Ban a user from a subreddit
    Ban {
        subreddit: String,
        /// Username
        #[arg(short, long)]
        user: String,
        /// Ban duration in days (None = permanent)
        #[arg(short, long)]
        days: Option<u32>,
        /// Ban reason (visible to user)
        #[arg(short, long)]
        reason: Option<String>,
        /// Mod note (visible to mods only)
        #[arg(short, long)]
        note: Option<String>,
    },
    /// Unban a user from a subreddit
    Unban {
        subreddit: String,
        #[arg(short, long)]
        user: String,
    },
    /// Mute a user in a subreddit
    Mute {
        subreddit: String,
        #[arg(short, long)]
        user: String,
        /// Note (visible to mods)
        #[arg(short, long)]
        note: Option<String>,
    },
    /// Unmute a user in a subreddit
    Unmute {
        subreddit: String,
        #[arg(short, long)]
        user: String,
    },
    /// List banned users in a subreddit
    Banned {
        subreddit: String,
    },
    /// List muted users in a subreddit
    Muted {
        subreddit: String,
    },
    /// List contributors in a subreddit
    Contributors {
        subreddit: String,
    },
    /// List moderators of a subreddit
    Mods {
        subreddit: String,
    },
}

#[derive(Subcommand)]
pub enum FlairCommands {
    /// List user flairs in a subreddit
    List {
        subreddit: String,
        #[arg(short, long)]
        user: Option<String>,
    },
    /// Set user flair
    Set {
        subreddit: String,
        #[arg(short, long)]
        user: String,
        #[arg(short, long)]
        text: Option<String>,
        #[arg(short, long)]
        css: Option<String>,
    },
    /// Delete user flair
    Delete {
        subreddit: String,
        #[arg(short, long)]
        user: String,
    },
    /// List flair templates
    Templates {
        subreddit: String,
        /// Type: user or link
        #[arg(short = 't', long, default_value = "user")]
        flair_type: String,
    },
}

#[derive(Subcommand)]
pub enum WikiCommands {
    /// List wiki pages
    Pages {
        subreddit: String,
    },
    /// View wiki page
    View {
        subreddit: String,
        page: String,
    },
    /// View wiki page revisions
    Revisions {
        subreddit: String,
        #[arg(short, long)]
        page: Option<String>,
    },
    /// Edit wiki page
    Edit {
        subreddit: String,
        page: String,
        #[arg(short, long)]
        content: String,
        #[arg(short, long)]
        reason: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum MultiCommands {
    /// List your multis
    List,
    /// Show a multi
    Show {
        /// Multi path (e.g., user/username/m/multiname)
        path: String,
    },
    /// Create a multi
    Create {
        name: String,
        #[arg(short = 'r', long)]
        subreddits: Vec<String>,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete a multi
    Delete {
        path: String,
    },
    /// Add subreddit to multi
    Add {
        path: String,
        subreddit: String,
    },
    /// Remove subreddit from multi
    Remove {
        path: String,
        subreddit: String,
    },
}

#[derive(Subcommand)]
pub enum LiveCommands {
    /// Show live thread info
    Show {
        thread_id: String,
    },
    /// View live thread updates
    Updates {
        thread_id: String,
        #[arg(short, long, default_value = "25")]
        limit: u32,
    },
    /// View live thread contributors
    Contributors {
        thread_id: String,
    },
    /// Create a live thread
    Create {
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: Option<String>,
        #[arg(long)]
        nsfw: bool,
    },
    /// Post update to live thread
    Update {
        thread_id: String,
        #[arg(short, long)]
        body: String,
    },
    /// Close a live thread
    Close {
        thread_id: String,
    },
}

#[derive(Subcommand)]
pub enum CollectionCommands {
    /// Show a collection
    Show {
        collection_id: String,
    },
    /// List collections in a subreddit
    List {
        /// Subreddit fullname (t5_xxx)
        subreddit: String,
    },
    /// Create a collection
    Create {
        #[arg(short = 'r', long)]
        subreddit: String,
        #[arg(short, long)]
        title: String,
        #[arg(short, long)]
        description: Option<String>,
    },
    /// Delete a collection
    Delete {
        collection_id: String,
    },
    /// Add post to collection
    Add {
        collection_id: String,
        #[arg(short, long)]
        post: String,
    },
    /// Remove post from collection
    Remove {
        collection_id: String,
        #[arg(short, long)]
        post: String,
    },
}

#[derive(Subcommand)]
pub enum ModmailCommands {
    /// List modmail conversations
    List {
        #[arg(short, long)]
        subreddit: Option<String>,
        #[arg(short = 't', long, default_value = "all")]
        state: String,
    },
    /// Show a modmail conversation
    Show {
        conversation_id: String,
    },
    /// Reply to modmail
    Reply {
        conversation_id: String,
        #[arg(short, long)]
        body: String,
        #[arg(long)]
        internal: bool,
    },
    /// Create a new modmail
    Create {
        #[arg(short = 'r', long)]
        subreddit: String,
        #[arg(short, long)]
        to: String,
        #[arg(short, long)]
        subject: String,
        #[arg(short, long)]
        body: String,
    },
    /// Archive a modmail conversation
    Archive {
        conversation_id: String,
    },
    /// Highlight a modmail conversation
    Highlight {
        conversation_id: String,
    },
    /// Mute modmail participant
    Mmute {
        conversation_id: String,
    },
}

#[derive(Subcommand)]
pub enum ModnoteCommands {
    /// Show mod notes for a user
    Show {
        subreddit: String,
        user: String,
    },
    /// Add a mod note
    Add {
        subreddit: String,
        user: String,
        #[arg(short, long)]
        note: String,
        #[arg(short = 't', long)]
        label: Option<String>,
    },
    /// Delete a mod note
    Delete {
        subreddit: String,
        user: String,
        note_id: String,
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
            Commands::Message(cmd) => {
                ensure_authenticated()?;
                match cmd {
                    MessageCommands::Inbox { limit } => {
                        let listing = MessageEndpoint::new(&client)
                            .get(MessageFolder::Inbox, Some(*limit), None)
                            .await?;
                        println!("=== Inbox ===\n");
                        if listing.data.children.is_empty() {
                            println!("No messages.");
                        } else {
                            for msg in &listing.data.children {
                                let m = &msg.data;
                                let status = if m.is_new { "●" } else { "○" };
                                let author = m.author.as_deref().unwrap_or("Reddit");
                                println!("{} [{}] {} - from u/{}", status, m.id, m.subject, author);
                                if !m.body.is_empty() {
                                    let preview: String = m.body.chars().take(100).collect();
                                    println!("   {}", preview);
                                }
                                println!();
                            }
                        }
                    }
                    MessageCommands::Unread { limit } => {
                        let listing = MessageEndpoint::new(&client)
                            .get(MessageFolder::Unread, Some(*limit), None)
                            .await?;
                        println!("=== Unread Messages ===\n");
                        if listing.data.children.is_empty() {
                            println!("No unread messages.");
                        } else {
                            for msg in &listing.data.children {
                                let m = &msg.data;
                                let author = m.author.as_deref().unwrap_or("Reddit");
                                println!("[{}] {} - from u/{}", m.id, m.subject, author);
                                if !m.body.is_empty() {
                                    let preview: String = m.body.chars().take(100).collect();
                                    println!("   {}", preview);
                                }
                                println!();
                            }
                        }
                    }
                    MessageCommands::Sent { limit } => {
                        let listing = MessageEndpoint::new(&client)
                            .get(MessageFolder::Sent, Some(*limit), None)
                            .await?;
                        println!("=== Sent Messages ===\n");
                        if listing.data.children.is_empty() {
                            println!("No sent messages.");
                        } else {
                            for msg in &listing.data.children {
                                let m = &msg.data;
                                println!("[{}] {} - to {}", m.id, m.subject, m.dest);
                                if !m.body.is_empty() {
                                    let preview: String = m.body.chars().take(100).collect();
                                    println!("   {}", preview);
                                }
                                println!();
                            }
                        }
                    }
                    MessageCommands::Send { to, subject, text, from } => {
                        let result = MessageEndpoint::new(&client)
                            .compose(to, subject, text, from.as_deref())
                            .await?;
                        if result.json.errors.is_empty() {
                            println!("Message sent to {}", to);
                        } else {
                            anyhow::bail!("Failed to send message: {:?}", result.json.errors);
                        }
                    }
                    MessageCommands::Read { ids } => {
                        let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                        MessageEndpoint::new(&client).read(&ids).await?;
                        println!("Marked {} message(s) as read", ids.len());
                    }
                    MessageCommands::ReadAll => {
                        MessageEndpoint::new(&client).read_all().await?;
                        println!("All messages marked as read");
                    }
                    MessageCommands::UnreadMsg { ids } => {
                        let ids: Vec<&str> = ids.iter().map(|s| s.as_str()).collect();
                        MessageEndpoint::new(&client).unread(&ids).await?;
                        println!("Marked {} message(s) as unread", ids.len());
                    }
                    MessageCommands::Delete { id } => {
                        MessageEndpoint::new(&client).delete(id).await?;
                        println!("Message {} deleted", id);
                    }
                    MessageCommands::Block { id } => {
                        MessageEndpoint::new(&client).block(id).await?;
                        println!("Blocked sender of message {}", id);
                    }
                }
            }
            Commands::Mod(cmd) => {
                ensure_authenticated()?;
                match cmd {
                    ModCommands::Reports { subreddit, limit } => {
                        let result = ModerationEndpoint::new(&client)
                            .get_queue(subreddit, ModQueueLocation::Reports, Some(*limit), None)
                            .await?;
                        println!("=== Reports Queue for r/{} ===\n", subreddit);
                        if let Some(children) = result["data"]["children"].as_array() {
                            if children.is_empty() {
                                println!("No reports.");
                            } else {
                                for item in children {
                                    let kind = item["kind"].as_str().unwrap_or("");
                                    let data = &item["data"];
                                    let author = data["author"].as_str().unwrap_or("unknown");
                                    let id = data["id"].as_str().unwrap_or("");
                                    match kind {
                                        "t3" => {
                                            let title = data["title"].as_str().unwrap_or("");
                                            println!("[POST] {} by u/{} - {}", id, author, title);
                                        }
                                        "t1" => {
                                            let body = data["body"].as_str().unwrap_or("");
                                            let preview: String = body.chars().take(50).collect();
                                            println!("[COMMENT] {} by u/{} - {}...", id, author, preview);
                                        }
                                        _ => {
                                            println!("[{}] {} by u/{}", kind, id, author);
                                        }
                                    }
                                    // Show user reports
                                    if let Some(reports) = data["user_reports"].as_array() {
                                        for report in reports {
                                            if let Some(arr) = report.as_array() {
                                                if arr.len() >= 2 {
                                                    println!("   User report: {} ({}x)", arr[0], arr[1]);
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    ModCommands::Spam { subreddit, limit } => {
                        let result = ModerationEndpoint::new(&client)
                            .get_queue(subreddit, ModQueueLocation::Spam, Some(*limit), None)
                            .await?;
                        println!("=== Spam Queue for r/{} ===\n", subreddit);
                        if let Some(children) = result["data"]["children"].as_array() {
                            println!("{} items in spam queue", children.len());
                        }
                    }
                    ModCommands::Queue { subreddit, limit } => {
                        let result = ModerationEndpoint::new(&client)
                            .get_queue(subreddit, ModQueueLocation::Modqueue, Some(*limit), None)
                            .await?;
                        println!("=== Mod Queue for r/{} ===\n", subreddit);
                        if let Some(children) = result["data"]["children"].as_array() {
                            println!("{} items in mod queue", children.len());
                        }
                    }
                    ModCommands::Unmoderated { subreddit, limit } => {
                        let result = ModerationEndpoint::new(&client)
                            .get_queue(subreddit, ModQueueLocation::Unmoderated, Some(*limit), None)
                            .await?;
                        println!("=== Unmoderated Posts for r/{} ===\n", subreddit);
                        if let Some(children) = result["data"]["children"].as_array() {
                            println!("{} unmoderated items", children.len());
                        }
                    }
                    ModCommands::Edited { subreddit, limit } => {
                        let result = ModerationEndpoint::new(&client)
                            .get_queue(subreddit, ModQueueLocation::Edited, Some(*limit), None)
                            .await?;
                        println!("=== Edited Items for r/{} ===\n", subreddit);
                        if let Some(children) = result["data"]["children"].as_array() {
                            println!("{} edited items", children.len());
                        }
                    }
                    ModCommands::Log { subreddit, limit, moderator } => {
                        let listing = ModerationEndpoint::new(&client)
                            .log(subreddit, Some(*limit), moderator.as_deref(), None)
                            .await?;
                        println!("=== Mod Log for r/{} ===\n", subreddit);
                        if listing.data.children.is_empty() {
                            println!("No mod log entries.");
                        } else {
                            for entry in &listing.data.children {
                                let action = &entry.data;
                                let time = chrono::DateTime::from_timestamp(action.created_utc as i64, 0)
                                    .map(|d| d.format("%Y-%m-%d %H:%M").to_string())
                                    .unwrap_or_else(|| "Unknown".to_string());
                                println!("[{}] u/{} - {}", time, action.mod_name, action.action);
                                if let Some(target) = &action.target_title {
                                    println!("   Target: {}", target);
                                }
                                if let Some(author) = &action.target_author {
                                    println!("   By: u/{}", author);
                                }
                            }
                        }
                    }
                    ModCommands::Approve { id } => {
                        ModerationEndpoint::new(&client).approve(id).await?;
                        println!("Approved {}", id);
                    }
                    ModCommands::Remove { id, spam } => {
                        ModerationEndpoint::new(&client).remove(id, *spam).await?;
                        if *spam {
                            println!("Removed {} as spam", id);
                        } else {
                            println!("Removed {}", id);
                        }
                    }
                    ModCommands::Distinguish { id, sticky } => {
                        ModerationEndpoint::new(&client)
                            .distinguish(id, DistinguishType::Yes, *sticky)
                            .await?;
                        println!("Distinguished {}", id);
                    }
                    ModCommands::Undistinguish { id } => {
                        ModerationEndpoint::new(&client)
                            .distinguish(id, DistinguishType::No, false)
                            .await?;
                        println!("Undistinguished {}", id);
                    }
                    ModCommands::Sticky { id, slot } => {
                        ModerationEndpoint::new(&client).sticky(id, true, *slot).await?;
                        println!("Stickied {}", id);
                    }
                    ModCommands::Unsticky { id } => {
                        ModerationEndpoint::new(&client).sticky(id, false, None).await?;
                        println!("Unstickied {}", id);
                    }
                    ModCommands::Lock { id } => {
                        ModerationEndpoint::new(&client).lock(id).await?;
                        println!("Locked {}", id);
                    }
                    ModCommands::Unlock { id } => {
                        ModerationEndpoint::new(&client).unlock(id).await?;
                        println!("Unlocked {}", id);
                    }
                    ModCommands::Nsfw { id } => {
                        ModerationEndpoint::new(&client).mark_nsfw(id).await?;
                        println!("Marked {} as NSFW", id);
                    }
                    ModCommands::Unnsfw { id } => {
                        ModerationEndpoint::new(&client).unmark_nsfw(id).await?;
                        println!("Unmarked {} as NSFW", id);
                    }
                    ModCommands::Spoiler { id } => {
                        ModerationEndpoint::new(&client).spoiler(id).await?;
                        println!("Marked {} as spoiler", id);
                    }
                    ModCommands::Unspoiler { id } => {
                        ModerationEndpoint::new(&client).unspoiler(id).await?;
                        println!("Unmarked {} as spoiler", id);
                    }
                    ModCommands::Report { id, reason } => {
                        ModerationEndpoint::new(&client).report(id, reason, None).await?;
                        println!("Reported {}: {}", id, reason);
                    }
                    ModCommands::Ban { subreddit, user, days, reason, note } => {
                        UserManagementEndpoint::new(&client)
                            .ban(subreddit, user, *days, reason.as_deref(), note.as_deref())
                            .await?;
                        let duration = days.map(|d| format!("{} days", d)).unwrap_or_else(|| "permanently".to_string());
                        println!("Banned u/{} {} from r/{}", user, duration, subreddit);
                    }
                    ModCommands::Unban { subreddit, user } => {
                        UserManagementEndpoint::new(&client).unban(subreddit, user).await?;
                        println!("Unbanned u/{} from r/{}", user, subreddit);
                    }
                    ModCommands::Mute { subreddit, user, note } => {
                        UserManagementEndpoint::new(&client).mute(subreddit, user, note.as_deref()).await?;
                        println!("Muted u/{} in r/{}", user, subreddit);
                    }
                    ModCommands::Unmute { subreddit, user } => {
                        UserManagementEndpoint::new(&client).unmute(subreddit, user).await?;
                        println!("Unmuted u/{} in r/{}", user, subreddit);
                    }
                    ModCommands::Banned { subreddit } => {
                        let listing = UserManagementEndpoint::new(&client).banned(subreddit).await?;
                        println!("=== Banned Users in r/{} ===\n", subreddit);
                        if listing.data.children.is_empty() {
                            println!("No banned users.");
                        } else {
                            for entry in &listing.data.children {
                                let user = &entry.data;
                                let date = chrono::DateTime::from_timestamp(user.date as i64, 0)
                                    .map(|d| d.format("%Y-%m-%d").to_string())
                                    .unwrap_or_else(|| "Unknown".to_string());
                                println!("u/{} - banned on {}", user.name, date);
                                if let Some(note) = &user.note {
                                    println!("   Note: {}", note);
                                }
                            }
                        }
                    }
                    ModCommands::Muted { subreddit } => {
                        let listing = UserManagementEndpoint::new(&client).muted(subreddit).await?;
                        println!("=== Muted Users in r/{} ===\n", subreddit);
                        if listing.data.children.is_empty() {
                            println!("No muted users.");
                        } else {
                            for entry in &listing.data.children {
                                let user = &entry.data;
                                println!("u/{}", user.name);
                                if let Some(reason) = &user.mute_reason {
                                    println!("   Reason: {}", reason);
                                }
                            }
                        }
                    }
                    ModCommands::Contributors { subreddit } => {
                        let listing = UserManagementEndpoint::new(&client).contributors(subreddit).await?;
                        println!("=== Contributors in r/{} ===\n", subreddit);
                        if listing.data.children.is_empty() {
                            println!("No contributors.");
                        } else {
                            for entry in &listing.data.children {
                                println!("u/{}", entry.data.name);
                            }
                        }
                    }
                    ModCommands::Mods { subreddit } => {
                        let listing = UserManagementEndpoint::new(&client).moderators(subreddit).await?;
                        println!("=== Moderators of r/{} ===\n", subreddit);
                        if listing.data.children.is_empty() {
                            println!("No moderators found.");
                        } else {
                            for entry in &listing.data.children {
                                let mod_info = &entry.data;
                                let perms = if mod_info.mod_permissions.is_empty() {
                                    "No permissions".to_string()
                                } else {
                                    mod_info.mod_permissions.join(", ")
                                };
                                println!("u/{} - {}", mod_info.name, perms);
                                if let Some(flair) = &mod_info.author_flair_text {
                                    println!("   Flair: {}", flair);
                                }
                            }
                        }
                    }
                }
            }
            Commands::Flair(cmd) => {
                match cmd {
                    FlairCommands::List { subreddit, user } => {
                        use crate::api::endpoints::FlairEndpoint;
                        let endpoint = FlairEndpoint::new(&client, subreddit);
                        let listing = endpoint.list(Some(self.limit), None, user.as_deref()).await?;

                        println!("=== Flairs in r/{} ===\n", subreddit);
                        if listing.data.children.is_empty() {
                            println!("No flairs found.");
                        } else {
                            for flair in &listing.data.children {
                                println!("u/{}", flair.data.user);
                                if let Some(text) = &flair.data.flair_text {
                                    println!("   Text: {}", text);
                                }
                                if let Some(css) = &flair.data.flair_css_class {
                                    println!("   CSS: {}", css);
                                }
                            }
                        }
                    }
                    FlairCommands::Set { subreddit, user, text, css } => {
                        use crate::api::endpoints::FlairEndpoint;
                        let endpoint = FlairEndpoint::new(&client, subreddit);
                        endpoint.set_user_flair(user, text.as_deref(), css.as_deref(), None).await?;
                        println!("Set flair for u/{} in r/{}", user, subreddit);
                    }
                    FlairCommands::Delete { subreddit, user } => {
                        use crate::api::endpoints::FlairEndpoint;
                        let endpoint = FlairEndpoint::new(&client, subreddit);
                        endpoint.delete_user_flair(user).await?;
                        println!("Deleted flair for u/{} in r/{}", user, subreddit);
                    }
                    FlairCommands::Templates { subreddit, flair_type } => {
                        use crate::api::endpoints::FlairEndpoint;
                        let endpoint = FlairEndpoint::new(&client, subreddit);
                        let templates = if flair_type == "link" {
                            endpoint.link_flair_templates().await?
                        } else {
                            endpoint.user_flair_templates().await?
                        };

                        println!("=== {} Flair Templates in r/{} ===\n", flair_type, subreddit);
                        for template in &templates {
                            println!("{}", template.id);
                            if let Some(text) = &template.text {
                                println!("   Text: {}", text);
                            }
                            if let Some(color) = &template.background_color {
                                println!("   Background: {}", color);
                            }
                            if template.mod_only {
                                println!("   [Mod Only]");
                            }
                        }
                    }
                }
            }
            Commands::Wiki(cmd) => {
                match cmd {
                    WikiCommands::Pages { subreddit } => {
                        use crate::api::endpoints::WikiEndpoint;
                        let endpoint = WikiEndpoint::new(&client, subreddit);
                        let pages = endpoint.pages().await?;

                        println!("=== Wiki Pages in r/{} ===\n", subreddit);
                        for page in &pages.data {
                            println!("- {}", page);
                        }
                    }
                    WikiCommands::View { subreddit, page } => {
                        use crate::api::endpoints::WikiEndpoint;
                        let endpoint = WikiEndpoint::new(&client, subreddit);
                        let wiki_page = endpoint.page(page).await?;

                        println!("=== {} ===\n", page);
                        println!("{}", wiki_page.content);
                    }
                    WikiCommands::Revisions { subreddit, page } => {
                        use crate::api::endpoints::WikiEndpoint;
                        let endpoint = WikiEndpoint::new(&client, subreddit);
                        let revisions = if let Some(p) = page {
                            endpoint.revisions(p, Some(self.limit)).await?
                        } else {
                            endpoint.recent_changes(Some(self.limit)).await?
                        };

                        println!("=== Wiki Revisions in r/{} ===\n", subreddit);
                        for rev in &revisions.data.children {
                            let timestamp = chrono::DateTime::from_timestamp(rev.data.timestamp as i64, 0)
                                .map(|d| d.to_rfc2822())
                                .unwrap_or_else(|| "Unknown".to_string());
                            println!("{} - by u/{}", timestamp, rev.data.author.data.name);
                            if let Some(reason) = &rev.data.reason {
                                println!("   Reason: {}", reason);
                            }
                        }
                    }
                    WikiCommands::Edit { subreddit, page, content, reason } => {
                        use crate::api::endpoints::WikiEndpoint;
                        let endpoint = WikiEndpoint::new(&client, subreddit);
                        endpoint.edit(page, content, reason.as_deref(), None).await?;
                        println!("Updated wiki page '{}' in r/{}", page, subreddit);
                    }
                }
            }
            Commands::Multi(cmd) => {
                match cmd {
                    MultiCommands::List => {
                        use crate::api::endpoints::MultiEndpoint;
                        let endpoint = MultiEndpoint::new(&client);
                        let multis = endpoint.mine().await?;

                        println!("=== Your Multis ===\n");
                        for multi in &multis {
                            let data = &multi.data;
                            println!("- {} ({})", data.display_name, data.path);
                            println!("  {} subreddits", data.subreddits.len());
                        }
                    }
                    MultiCommands::Show { path } => {
                        use crate::api::endpoints::MultiEndpoint;
                        let endpoint = MultiEndpoint::new(&client);
                        let multi = endpoint.get(path).await?;

                        println!("=== {} ===\n", multi.data.display_name);
                        println!("Path: {}", multi.data.path);
                        if let Some(desc) = &multi.data.description_md {
                            println!("\n{}", desc);
                        }
                        println!("\nSubreddits:");
                        for sr in &multi.data.subreddits {
                            println!("- r/{}", sr.name);
                        }
                    }
                    MultiCommands::Create { name, subreddits, description } => {
                        use crate::api::endpoints::{MultiEndpoint, CreateMultiRequest, MultiSubredditInput};
                        let endpoint = MultiEndpoint::new(&client);
                        let request = CreateMultiRequest {
                            display_name: name.clone(),
                            subreddits: subreddits.iter().map(|s| MultiSubredditInput { name: s.clone() }).collect(),
                            description_md: description.clone(),
                            icon_name: None,
                            key_color: None,
                            visibility: Some("private".to_string()),
                        };
                        let path = format!("user/me/m/{}", name);
                        endpoint.create(&path, &request).await?;
                        println!("Created multi '{}' with {} subreddits", name, subreddits.len());
                    }
                    MultiCommands::Delete { path } => {
                        use crate::api::endpoints::MultiEndpoint;
                        let endpoint = MultiEndpoint::new(&client);
                        endpoint.delete(path).await?;
                        println!("Deleted multi '{}'", path);
                    }
                    MultiCommands::Add { path, subreddit } => {
                        use crate::api::endpoints::MultiEndpoint;
                        let endpoint = MultiEndpoint::new(&client);
                        endpoint.add_subreddit(path, subreddit).await?;
                        println!("Added r/{} to multi '{}'", subreddit, path);
                    }
                    MultiCommands::Remove { path, subreddit } => {
                        use crate::api::endpoints::MultiEndpoint;
                        let endpoint = MultiEndpoint::new(&client);
                        endpoint.remove_subreddit(path, subreddit).await?;
                        println!("Removed r/{} from multi '{}'", subreddit, path);
                    }
                }
            }
            Commands::Live(cmd) => {
                match cmd {
                    LiveCommands::Show { thread_id } => {
                        use crate::api::endpoints::LiveEndpoint;
                        let endpoint = LiveEndpoint::new(&client);
                        let thread = endpoint.about(thread_id).await?;

                        println!("=== {} ===\n", thread.title);
                        println!("ID: {}", thread.id);
                        println!("State: {}", thread.state);
                        if let Some(desc) = &thread.description {
                            println!("\n{}", desc);
                        }
                        if let Some(count) = thread.viewer_count {
                            println!("\nViewers: {}", count);
                        }
                    }
                    LiveCommands::Updates { thread_id, limit } => {
                        use crate::api::endpoints::LiveEndpoint;
                        let endpoint = LiveEndpoint::new(&client);
                        let updates = endpoint.updates(thread_id, Some(*limit), None).await?;

                        println!("=== Live Thread {} Updates ===\n", thread_id);
                        for update in &updates.data.children {
                            let time = chrono::DateTime::from_timestamp(update.data.created_utc as i64, 0)
                                .map(|d| d.format("%H:%M:%S").to_string())
                                .unwrap_or_else(|| "?".to_string());
                            println!("[{}] u/{}:", time, update.data.author);
                            println!("{}\n", update.data.body);
                        }
                    }
                    LiveCommands::Contributors { thread_id } => {
                        use crate::api::endpoints::LiveEndpoint;
                        let endpoint = LiveEndpoint::new(&client);
                        let contributors = endpoint.contributors(thread_id).await?;

                        println!("=== Contributors to Live Thread {} ===\n", thread_id);
                        for c in &contributors {
                            println!("u/{} - {}", c.name, c.permissions.join(", "));
                        }
                    }
                    LiveCommands::Create { title, description, nsfw } => {
                        use crate::api::endpoints::LiveEndpoint;
                        let endpoint = LiveEndpoint::new(&client);
                        let thread = endpoint.create(title, description.as_deref(), None, *nsfw).await?;
                        println!("Created live thread: {} (ID: {})", thread.title, thread.id);
                    }
                    LiveCommands::Update { thread_id, body } => {
                        use crate::api::endpoints::LiveEndpoint;
                        let endpoint = LiveEndpoint::new(&client);
                        endpoint.update(thread_id, body).await?;
                        println!("Posted update to live thread {}", thread_id);
                    }
                    LiveCommands::Close { thread_id } => {
                        use crate::api::endpoints::LiveEndpoint;
                        let endpoint = LiveEndpoint::new(&client);
                        endpoint.close(thread_id).await?;
                        println!("Closed live thread {}", thread_id);
                    }
                }
            }
            Commands::Collection(cmd) => {
                match cmd {
                    CollectionCommands::Show { collection_id } => {
                        use crate::api::endpoints::CollectionEndpoint;
                        let endpoint = CollectionEndpoint::new(&client);
                        let collection = endpoint.get(collection_id).await?;

                        println!("=== {} ===\n", collection.title);
                        if let Some(desc) = &collection.description {
                            println!("{}\n", desc);
                        }
                        println!("Posts: {}", collection.link_ids.len());
                        for link in &collection.link_ids {
                            println!("- {}", link);
                        }
                    }
                    CollectionCommands::List { subreddit } => {
                        use crate::api::endpoints::CollectionEndpoint;
                        let endpoint = CollectionEndpoint::new(&client);
                        let collections = endpoint.subreddit(subreddit).await?;

                        println!("=== Collections ===\n");
                        for c in &collections {
                            println!("- {} ({})", c.title, c.collection_id);
                        }
                    }
                    CollectionCommands::Create { subreddit, title, description } => {
                        use crate::api::endpoints::CollectionEndpoint;
                        let endpoint = CollectionEndpoint::new(&client);
                        let collection = endpoint.create(title, subreddit, description.as_deref(), None).await?;
                        println!("Created collection: {} (ID: {})", collection.title, collection.collection_id);
                    }
                    CollectionCommands::Delete { collection_id } => {
                        use crate::api::endpoints::CollectionEndpoint;
                        let endpoint = CollectionEndpoint::new(&client);
                        endpoint.delete(collection_id).await?;
                        println!("Deleted collection {}", collection_id);
                    }
                    CollectionCommands::Add { collection_id, post } => {
                        use crate::api::endpoints::CollectionEndpoint;
                        let endpoint = CollectionEndpoint::new(&client);
                        endpoint.add_post(collection_id, post).await?;
                        println!("Added {} to collection {}", post, collection_id);
                    }
                    CollectionCommands::Remove { collection_id, post } => {
                        use crate::api::endpoints::CollectionEndpoint;
                        let endpoint = CollectionEndpoint::new(&client);
                        endpoint.remove_post(collection_id, post).await?;
                        println!("Removed {} from collection {}", post, collection_id);
                    }
                }
            }
            Commands::Modmail(cmd) => {
                match cmd {
                    ModmailCommands::List { subreddit, state } => {
                        use crate::api::endpoints::{ModmailEndpoint, ModmailState};
                        let endpoint = ModmailEndpoint::new(&client);
                        let state_enum = match state.as_str() {
                            "new" => ModmailState::New,
                            "archived" => ModmailState::Archived,
                            "mod" => ModmailState::Mod,
                            "inbox" => ModmailState::Inbox,
                            _ => ModmailState::All,
                        };
                        let entities = subreddit.as_ref().map(|s| vec![s.as_str()]);
                        let response = endpoint.list(entities.as_deref(), Some(state_enum), Some(self.limit)).await?;

                        println!("=== Modmail ===\n");
                        for (id, conv) in &response.conversations {
                            println!("{} - {}", id, conv.subject);
                            println!("   Participant: {}", conv.participant.as_ref().map(|p| p.name.clone()).unwrap_or_else(|| "Unknown".to_string()));
                        }
                    }
                    ModmailCommands::Show { conversation_id } => {
                        use crate::api::endpoints::ModmailEndpoint;
                        let endpoint = ModmailEndpoint::new(&client);
                        let response = endpoint.get(conversation_id).await?;

                        println!("=== {} ===\n", response.conversation.subject);
                        for (_, msg) in &response.messages {
                            let time = chrono::DateTime::from_timestamp(msg.created_utc as i64, 0)
                                .map(|d| d.to_rfc2822())
                                .unwrap_or_else(|| "Unknown".to_string());
                            println!("[{}] u/{}:", time, msg.author.name);
                            println!("{}\n", msg.body);
                        }
                    }
                    ModmailCommands::Reply { conversation_id, body, internal } => {
                        use crate::api::endpoints::ModmailEndpoint;
                        let endpoint = ModmailEndpoint::new(&client);
                        endpoint.reply(conversation_id, body, *internal, false).await?;
                        println!("Replied to modmail {}", conversation_id);
                    }
                    ModmailCommands::Create { subreddit, to, subject, body } => {
                        use crate::api::endpoints::ModmailEndpoint;
                        let endpoint = ModmailEndpoint::new(&client);
                        endpoint.create(body, subject, subreddit, Some(to), None).await?;
                        println!("Created modmail in r/{}", subreddit);
                    }
                    ModmailCommands::Archive { conversation_id } => {
                        use crate::api::endpoints::ModmailEndpoint;
                        let endpoint = ModmailEndpoint::new(&client);
                        endpoint.archive(conversation_id).await?;
                        println!("Archived modmail {}", conversation_id);
                    }
                    ModmailCommands::Highlight { conversation_id } => {
                        use crate::api::endpoints::ModmailEndpoint;
                        let endpoint = ModmailEndpoint::new(&client);
                        endpoint.highlight(conversation_id).await?;
                        println!("Highlighted modmail {}", conversation_id);
                    }
                    ModmailCommands::Mmute { conversation_id } => {
                        use crate::api::endpoints::ModmailEndpoint;
                        let endpoint = ModmailEndpoint::new(&client);
                        endpoint.mute(conversation_id).await?;
                        println!("Muted participant in modmail {}", conversation_id);
                    }
                }
            }
            Commands::Modnote(cmd) => {
                match cmd {
                    ModnoteCommands::Show { subreddit, user } => {
                        use crate::api::endpoints::ModNoteEndpoint;
                        let endpoint = ModNoteEndpoint::new(&client);
                        let notes = endpoint.list(subreddit, user, Some(self.limit)).await?;

                        println!("=== Mod Notes for u/{} in r/{} ===\n", user, subreddit);
                        if notes.created_notes.is_empty() {
                            println!("No notes found.");
                        } else {
                            for note in &notes.created_notes {
                                let time = chrono::DateTime::from_timestamp(note.created_at as i64, 0)
                                    .map(|d| d.to_rfc2822())
                                    .unwrap_or_else(|| "Unknown".to_string());
                                println!("[{}] u/{}:", time, note.operator.name);
                                println!("   {}", note.note);
                                if let Some(label) = &note.label {
                                    println!("   Label: {}", label);
                                }
                            }
                        }
                    }
                    ModnoteCommands::Add { subreddit, user, note, label } => {
                        use crate::api::endpoints::{ModNoteEndpoint, ModNoteLabel};
                        let endpoint = ModNoteEndpoint::new(&client);
                        let label_enum = label.as_ref().and_then(|l| match l.to_uppercase().as_str() {
                            "NOTE" => Some(ModNoteLabel::Note),
                            "ABUSE" => Some(ModNoteLabel::Abuse),
                            "BAN" => Some(ModNoteLabel::Ban),
                            "HELPFUL" => Some(ModNoteLabel::Helpful),
                            "SPAM" => Some(ModNoteLabel::Spam),
                            _ => None,
                        });
                        endpoint.create(subreddit, user, note, label_enum).await?;
                        println!("Added mod note for u/{} in r/{}", user, subreddit);
                    }
                    ModnoteCommands::Delete { subreddit, user, note_id } => {
                        use crate::api::endpoints::ModNoteEndpoint;
                        let endpoint = ModNoteEndpoint::new(&client);
                        endpoint.delete(subreddit, user, note_id).await?;
                        println!("Deleted mod note {} for u/{} in r/{}", note_id, user, subreddit);
                    }
                }
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
