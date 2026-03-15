use crate::models::{Comment, Link, Subreddit, User};

pub struct TableOutput;

impl super::Output for TableOutput {
    fn format_links(&self, links: &[Link]) -> String {
        let mut output = String::new();

        for link in links {
            let score = format!("{:+}", link.score);
            let comments = link.num_comments;
            let subreddit = &link.subreddit;
            let title = if link.title.len() > 80 {
                format!("{}...", &link.title[..77])
            } else {
                link.title.clone()
            };

            let nsfw = if link.over_18 { "[NSFW] " } else { "" };
            let sticky = if link.stickied { "[STICKY] " } else { "" };

            output.push_str(&format!(
                "{}{} {:>6} {:>4}c r/{:<15} {}\n",
                sticky, nsfw, score, comments, subreddit, title
            ));
        }

        output
    }

    fn format_subreddit(&self, sr: &Subreddit) -> String {
        let mut output = String::new();

        output.push_str(&format!("r/{}\n", sr.display_name));
        output.push_str(&format!("{}\n", sr.title));
        output.push_str(&format!("{} subscribers\n", sr.subscribers));
        if let Some(active) = sr.active_user_count {
            output.push_str(&format!("{} online\n", active));
        }
        output.push_str(&format!("\n{}\n", sr.public_description));

        output
    }

    fn format_user(&self, user: &User) -> String {
        let mut output = String::new();

        output.push_str(&format!("u/{}\n", user.name));
        output.push_str(&format!("Link karma: {}\n", user.link_karma));
        output.push_str(&format!("Comment karma: {}\n", user.comment_karma));
        if let Some(total) = user.total_karma {
            output.push_str(&format!("Total karma: {}\n", total));
        }
        output.push_str(&format!("Created: {}\n", user.created_utc.format("%Y-%m-%d")));

        output
    }

    fn format_comments(&self, comments: &[Comment], depth: usize) -> String {
        let mut output = String::new();
        let indent = "  ".repeat(depth);

        for comment in comments {
            let body_preview = comment
                .body
                .chars()
                .take(100)
                .collect::<String>()
                .replace('\n', " ");

            output.push_str(&format!(
                "{}[{:>5}] u/{:<15}: {}\n",
                indent, comment.score, comment.author, body_preview
            ));

            // Recursively format replies
            if let Some(ref replies) = comment.replies {
                let child_comments: Vec<Comment> = replies
                    .data
                    .children
                    .iter()
                    .map(|t| t.data.clone())
                    .collect();
                if !child_comments.is_empty() {
                    output.push_str(&self.format_comments(&child_comments, depth + 1));
                }
            }
        }

        output
    }
}
