# @0010capacity/reddit-cli

A powerful CLI client for Reddit, written in Rust.

## Installation

```bash
npm install -g @0010capacity/reddit-cli
```

## Usage

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
| `reddit me subreddits` | View subscribed subreddits |

### Interactions (Auth Required)

| Command | Description |
|---------|-------------|
| `reddit upvote <id>` | Upvote a post/comment |
| `reddit downvote <id>` | Downvote a post/comment |
| `reddit save <id>` | Save a post/comment |
| `reddit subscribe <subreddit>` | Subscribe to subreddit |

### Posting (Auth Required)

| Command | Description |
|---------|-------------|
| `reddit submit link -r <sr> -t <title> -u <url>` | Submit a link |
| `reddit submit text -r <sr> -t <title> --text <body>` | Submit a text post |
| `reddit comment <parent> --text <text>` | Post a comment |

### Moderation (Auth + Mod Required)

| Command | Description |
|---------|-------------|
| `reddit mod reports <subreddit>` | View reports |
| `reddit mod queue <subreddit>` | View mod queue |
| `reddit mod approve <id>` | Approve post/comment |
| `reddit mod remove <id>` | Remove post/comment |

## Output Formats

```bash
# Table format (default)
reddit hot

# JSON output
reddit hot --format json

# Plain text
reddit hot --format plain
```

## Authentication

Reddit CLI uses OAuth2 for authentication. On first use:

```bash
reddit auth login
```

This will open your browser for authentication.

## License

MIT
