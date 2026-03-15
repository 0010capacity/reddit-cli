# Reddit CLI

A powerful command-line interface for Reddit, written in Rust.

## Features

- **Browse Reddit** - View hot, new, top, rising, and controversial posts
- **OAuth Authentication** - Secure login with OAuth2
- **Subreddit Management** - View info, rules, and subscribe/unsubscribe
- **User Profiles** - View user info, posts, and comments
- **Post & Comment** - Create posts, comments, and replies
- **Voting & Saving** - Upvote, downvote, save, and hide
- **Messages** - View and send private messages
- **Moderation Tools** - Mod queue, reports, approve/remove, ban/mute users
- **Modmail** - Manage modmail conversations
- **Wiki** - View and edit wiki pages
- **Flair** - Manage user and post flair
- **Multireddits** - Create and manage custom feeds
- **Live Threads** - View and manage live threads
- **Collections** - Organize posts into collections
- **Mod Notes** - Track user notes in moderated subreddits

## Installation

### From Source

```bash
git clone https://github.com/0010capacity/reddit-cli.git
cd reddit-cli
cargo install --path .
```

### Using Cargo

```bash
cargo install reddit-cli
```

## Quick Start

```bash
# View hot posts
reddit hot

# View a specific subreddit
reddit subreddit show rust

# Search Reddit
reddit search "rust programming" --subreddit rust

# Login for full access
reddit auth login

# After login, access your account
reddit me
reddit me karma

# Subscribe to a subreddit
reddit subscribe rust

# Post a self post
reddit submit text -r rust -t "My First Post" --text "Hello from CLI!"

# Comment on a post
reddit comment t3_xxx --text "Great post!"
```

## Authentication

Reddit CLI uses OAuth2 for authentication. On first use:

```bash
reddit auth login
```

This will open your browser for authentication. After approving, you'll be automatically logged in.

## Configuration

Configuration is stored in `~/.config/reddit-cli/config.toml`:

```toml
[api]
user_agent = "cli:reddit-cli:0.1.0 (by /u/your_username)"

[output]
format = "table"  # table, json, plain
pager = true
color = true

[auth]
client_id = "your_client_id"  # Optional, uses default
```

## Commands

### Browsing (No Auth Required)

| Command | Description |
|---------|-------------|
| `reddit hot` | View hot posts |
| `reddit new` | View new posts |
| `reddit top [--time hour/day/week/month/year/all]` | View top posts |
| `reddit rising` | View rising posts |
| `reddit controversial` | View controversial posts |
| `reddit subreddit show <name>` | View subreddit info |
| `reddit user show <username>` | View user info |
| `reddit search <query>` | Search Reddit |

### Account (Auth Required)

| Command | Description |
|---------|-------------|
| `reddit me` | View your account info |
| `reddit me karma` | View karma breakdown |
| `reddit me prefs` | Show preferences |
| `reddit me trophies` | Show trophies |
| `reddit me subreddits` | View subscribed subreddits |

### Interactions (Auth Required)

| Command | Description |
|---------|-------------|
| `reddit upvote <id>` | Upvote a post/comment |
| `reddit downvote <id>` | Downvote a post/comment |
| `reddit unvote <id>` | Remove vote |
| `reddit save <id>` | Save a post/comment |
| `reddit unsave <id>` | Unsave a post/comment |
| `reddit hide <id>` | Hide a post |
| `reddit unhide <id>` | Unhide a post |
| `reddit subscribe <subreddit>` | Subscribe to subreddit |
| `reddit unsubscribe <subreddit>` | Unsubscribe from subreddit |

### Posting (Auth Required)

| Command | Description |
|---------|-------------|
| `reddit submit link -r <sr> -t <title> -u <url>` | Submit a link |
| `reddit submit text -r <sr> -t <title> --text <body>` | Submit a text post |
| `reddit comment <parent> --text <text>` | Post a comment |
| `reddit edit <id> --text <text>` | Edit a post or comment |
| `reddit delete <id>` | Delete a post or comment |

### Messages (Auth Required)

| Command | Description |
|---------|-------------|
| `reddit message inbox` | View inbox |
| `reddit message unread` | View unread messages |
| `reddit message sent` | View sent messages |
| `reddit message send --to <user> --subject <subj> --text <body>` | Send a message |
| `reddit message read <ids...>` | Mark messages as read |
| `reddit message read-all` | Mark all messages as read |

### Moderation (Auth + Mod Required)

| Command | Description |
|---------|-------------|
| `reddit mod reports <subreddit>` | View reports |
| `reddit mod spam <subreddit>` | View spam queue |
| `reddit mod queue <subreddit>` | View mod queue |
| `reddit mod approve <id>` | Approve post/comment |
| `reddit mod remove <id>` | Remove post/comment |
| `reddit mod distinguish <id>` | Distinguish as mod |
| `reddit mod sticky <id>` | Sticky a post |
| `reddit mod lock <id>` | Lock a post/comment |
| `reddit mod nsfw <id>` | Mark as NSFW |
| `reddit mod spoiler <id>` | Mark as spoiler |
| `reddit mod ban <sr> --user <user>` | Ban a user |
| `reddit mod unban <sr> --user <user>` | Unban a user |
| `reddit mod mute <sr> --user <user>` | Mute a user |
| `reddit mod unmute <sr> --user <user>` | Unmute a user |

### Flair

| Command | Description |
|---------|-------------|
| `reddit flair list <subreddit>` | List user flairs |
| `reddit flair set <sr> --user <user>` | Set user flair |
| `reddit flair templates <subreddit>` | List flair templates |

### Wiki

| Command | Description |
|---------|-------------|
| `reddit wiki pages <subreddit>` | List wiki pages |
| `reddit wiki view <sr> <page>` | View wiki page |
| `reddit wiki revisions <sr>` | View wiki revisions |
| `reddit wiki edit <sr> <page> --content <text>` | Edit wiki page |

### Multireddits

| Command | Description |
|---------|-------------|
| `reddit multi list` | List your multis |
| `reddit multi show <path>` | Show a multi |
| `reddit multi create <name> -r <subreddits>` | Create a multi |
| `reddit multi delete <path>` | Delete a multi |
| `reddit multi add <path> <subreddit>` | Add subreddit to multi |
| `reddit multi remove <path> <subreddit>` | Remove subreddit from multi |

### Live Threads

| Command | Description |
|---------|-------------|
| `reddit live show <thread_id>` | Show live thread info |
| `reddit live updates <thread_id>` | View live updates |
| `reddit live contributors <thread_id>` | View contributors |
| `reddit live create --title <title>` | Create a live thread |
| `reddit live update <thread_id> --body <text>` | Post an update |
| `reddit live close <thread_id>` | Close a live thread |

### Modmail

| Command | Description |
|---------|-------------|
| `reddit modmail list` | List modmail conversations |
| `reddit modmail show <conversation_id>` | Show a conversation |
| `reddit modmail reply <id> --body <text>` | Reply to modmail |
| `reddit modmail create -r <sr> --to <user> --subject <subj> --body <text>` | Create modmail |
| `reddit modmail archive <id>` | Archive a conversation |

### Mod Notes

| Command | Description |
|---------|-------------|
| `reddit modnote show <sr> <user>` | Show mod notes |
| `reddit modnote add <sr> <user> --note <text>` | Add a mod note |
| `reddit modnote delete <sr> <user> <note_id>` | Delete a mod note |

## Output Formats

```bash
# Table format (default)
reddit hot

# JSON output
reddit hot --format json

# Plain text
reddit hot --format plain
```

## Environment Variables

| Variable | Description |
|----------|-------------|
| `RUST_LOG` | Set log level (debug, info, warn, error) |
| `REDDIT_CONFIG_DIR` | Custom config directory |

## Development

```bash
# Build
cargo build

# Run tests
cargo test

# Run with debug logging
RUST_LOG=debug cargo run -- hot

# Check formatting
cargo fmt --check

# Lint
cargo clippy
```

## License

MIT License
