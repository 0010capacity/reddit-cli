# Phase 7: 품질 개선

> **목표**: 테스트 코드 작성, 문서화, CI/CD 설정, 성능 최적화

**전제조건**: Phase 6 완료

---

## 체크리스트

### 7.1 테스트 코드
- [ ] 단위 테스트 작성
- [ ] 통합 테스트 작성
- [ ] Mock HTTP 서버 설정
- [ ] 테스트 커버리지 측정

### 7.2 문서화
- [ ] README.md 작성
- [ ] API 문서 (rustdoc)
- [ ] CLI 도움말 개선
- [ ] 예제 작성

### 7.3 에러 처리 개선
- [ ] 사용자 친화적 에러 메시지
- [ ] 에러 복구 가이드
- [ ] 디버그 모드

### 7.4 성능 최적화
- [ ] Rate limiting 구현
- [ ] 요청 캐싱
- [ ] 병렬 요청 처리

### 7.5 CI/CD
- [ ] GitHub Actions 설정
- [ ] 자동 빌드
- [ ] 자동 테스트
- [ ] 릴리즈 자동화

### 7.6 배포
- [ ] 크로스 컴파일 설정
- [ ] 바이너리 릴리즈
- [ ] 패키지 매니저 지원 (Homebrew, AUR, etc.)

---

## 상세 구현 가이드

### 7.1 테스트 코드

#### tests/common/mod.rs

```rust
use once_cell::sync::Lazy;
use serde_json::json;
use wiremock::{Mock, MockServer, ResponseTemplate};
use wiremock::matchers::{method, path};

pub static MOCK_SERVER: Lazy<MockServer> = Lazy::new(|| {
    tokio::runtime::Runtime::new()
        .unwrap()
        .block_on(async {
            MockServer::start().await
        })
});

pub fn setup_hot_posts_mock(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/hot.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "after": "t3_test123",
                "before": null,
                "children": [
                    {
                        "kind": "t3",
                        "data": {
                            "id": "test123",
                            "name": "t3_test123",
                            "title": "Test Post",
                            "author": "testuser",
                            "subreddit": "test",
                            "subreddit_id": "t5_test",
                            "selftext": "",
                            "url": "https://example.com",
                            "domain": "example.com",
                            "permalink": "/r/test/comments/test123/",
                            "created_utc": 1234567890.0,
                            "score": 100,
                            "upvote_ratio": 0.95,
                            "num_comments": 10,
                            "over_18": false,
                            "spoiler": false,
                            "stickied": false,
                            "locked": false,
                            "is_self": false
                        }
                    }
                ]
            }
        })))
        .mount(&server)
        .expect(1);
}

pub fn setup_subreddit_about_mock(server: &MockServer) {
    Mock::given(method("GET"))
        .and(path("/r/rust/about.json"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({
            "data": {
                "id": "2qhuy",
                "name": "t5_2qhuy",
                "display_name": "rust",
                "title": "The Rust Programming Language",
                "description": "A place for all things Rust.",
                "public_description": "A place for all things Rust.",
                "subscribers": 250000,
                "active_user_count": 500,
                "created_utc": 1284672000.0,
                "over18": false,
                "url": "/r/rust/"
            }
        })))
        .mount(&server)
        .expect(1);
}
```

#### tests/integration_test.rs

```rust
mod common;

use crate::common::*;

#[tokio::test]
async fn test_hot_posts() {
    let mock_server = MockServer::start().await;
    setup_hot_posts_mock(&mock_server);

    let settings = reddit_cli::config::Settings {
        api: reddit_cli::config::ApiSettings {
            www_url: mock_server.uri(),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = reddit_cli::api::Client::new(&settings).unwrap();
    let endpoint = reddit_cli::api::endpoints::ListingEndpoint::new(&client);

    let result = endpoint.hot(None, Some(25), None).await.unwrap();

    assert_eq!(result.data.children.len(), 1);
    assert_eq!(result.data.children[0].data.title, "Test Post");
}

#[tokio::test]
async fn test_subreddit_about() {
    let mock_server = MockServer::start().await;
    setup_subreddit_about_mock(&mock_server);

    let settings = reddit_cli::config::Settings {
        api: reddit_cli::config::ApiSettings {
            www_url: mock_server.uri(),
            ..Default::default()
        },
        ..Default::default()
    };

    let client = reddit_cli::api::Client::new(&settings).unwrap();
    let endpoint = reddit_cli::api::endpoints::SubredditEndpoint::new(&client);

    let result = endpoint.about("rust").await.unwrap();

    assert_eq!(result.data.display_name, "rust");
    assert_eq!(result.data.subscribers, 250000);
}
```

#### src/models/common.rs (테스트 추가)

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_period_from_str() {
        assert!(matches!(TimePeriod::from_str("hour"), Ok(TimePeriod::Hour)));
        assert!(matches!(TimePeriod::from_str("DAY"), Ok(TimePeriod::Day)));
        assert!(TimePeriod::from_str("invalid").is_err());
    }

    #[test]
    fn test_time_period_display() {
        assert_eq!(TimePeriod::Hour.to_string(), "hour");
        assert_eq!(TimePeriod::All.to_string(), "all");
    }
}
```

#### Cargo.toml (dev dependencies)

```toml
[dev-dependencies]
tokio-test = "0.4"
wiremock = "0.5"
serial_test = "3"
tempfile = "3"
assert_cmd = "2"
predicates = "3"
```

### 7.2 문서화

#### README.md

```markdown
# Reddit CLI

A powerful command-line interface for Reddit, written in Rust.

## Features

- Browse Reddit without authentication (hot, new, top, etc.)
- Full OAuth2 authentication support
- Post, comment, vote, and save
- Subreddit management (subscribe, flair, wiki)
- Moderation tools (modqueue, reports, ban/mute users)
- Modmail support
- Live thread support
- And much more!

## Installation

### From Binary

Download the latest release from [Releases](https://github.com/yourname/reddit-cli/releases).

### From Source

```bash
git clone https://github.com/yourname/reddit-cli.git
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

# Search
reddit search "rust programming" --subreddit rust

# Login for full access
reddit auth login

# After login, access your account
reddit me
reddit me karma

# Subscribe to a subreddit
reddit subscribe rust

# Post a self post
reddit submit self -r rust -t "My First Post" --text "Hello from CLI!"

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
| `reddit submit self -r <sr> -t <title> --text <body>` | Submit a text post |
| `reddit comment <parent> --text <text>` | Post a comment |

### Moderation (Auth + Mod Required)

| Command | Description |
|---------|-------------|
| `reddit mod reports <subreddit>` | View reports |
| `reddit mod queue <subreddit>` | View mod queue |
| `reddit mod approve <id>` | Approve post/comment |
| `reddit mod remove <id>` | Remove post/comment |
| `reddit mod ban <sr> --user <user>` | Ban a user |

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
```

### 7.3 Rate Limiting

#### src/api/ratelimit.rs

```rust
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
pub struct RateLimiter {
    state: Arc<Mutex<RateLimitState>>,
}

#[derive(Debug)]
struct RateLimitState {
    remaining: u32,
    reset_at: Instant,
    min_interval: Duration,
    last_request: Instant,
}

impl RateLimiter {
    pub fn new(requests_per_minute: u32) -> Self {
        let min_interval = Duration::from_secs(60) / requests_per_minute;

        Self {
            state: Arc::new(Mutex::new(RateLimitState {
                remaining: requests_per_minute,
                reset_at: Instant::now() + Duration::from_secs(60),
                min_interval,
                last_request: Instant::now() - min_interval,
            })),
        }
    }

    pub async fn acquire(&self) {
        let mut state = self.state.lock().await;

        // Check if we need to wait for rate limit reset
        if state.remaining == 0 {
            let wait_duration = state.reset_at.saturating_duration_since(Instant::now());
            if !wait_duration.is_zero() {
                tracing::warn!("Rate limited, waiting {:?}", wait_duration);
                tokio::time::sleep(wait_duration).await;
            }
            state.remaining = 60; // Reset
            state.reset_at = Instant::now() + Duration::from_secs(60);
        }

        // Ensure minimum interval between requests
        let elapsed = state.last_request.elapsed();
        if elapsed < state.min_interval {
            tokio::time::sleep(state.min_interval - elapsed).await;
        }

        state.remaining = state.remaining.saturating_sub(1);
        state.last_request = Instant::now();
    }

    pub async fn update_from_headers(&self, remaining: Option<u32>, reset_seconds: Option<u64>) {
        let mut state = self.state.lock().await;

        if let Some(r) = remaining {
            state.remaining = r;
        }

        if let Some(s) = reset_seconds {
            state.reset_at = Instant::now() + Duration::from_secs(s);
        }
    }
}
```

### 7.4 CI/CD

#### .github/workflows/ci.yml

```yaml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

env:
  CARGO_TERM_COLOR: always

jobs:
  test:
    name: Test
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Cache cargo registry
        uses: actions/cache@v4
        with:
          path: ~/.cargo/registry
          key: ${{ runner.os }}-cargo-registry-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache cargo index
        uses: actions/cache@v4
        with:
          path: ~/.cargo/git
          key: ${{ runner.os }}-cargo-git-${{ hashFiles('**/Cargo.lock') }}

      - name: Cache target directory
        uses: actions/cache@v4
        with:
          path: target
          key: ${{ runner.os }}-target-${{ hashFiles('**/Cargo.lock') }}

      - name: Run tests
        run: cargo test --verbose

  lint:
    name: Lint
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy

      - name: Check formatting
        run: cargo fmt --check

      - name: Run clippy
        run: cargo clippy -- -D warnings

  build:
    name: Build
    strategy:
      matrix:
        target:
          - x86_64-unknown-linux-gnu
          - x86_64-apple-darwin
          - aarch64-apple-darwin
          - x86_64-pc-windows-msvc
        include:
          - target: x86_64-unknown-linux-gnu
            os: ubuntu-latest
          - target: x86_64-apple-darwin
            os: macos-latest
          - target: aarch64-apple-darwin
            os: macos-latest
          - target: x86_64-pc-windows-msvc
            os: windows-latest

    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          targets: ${{ matrix.target }}

      - name: Build
        run: cargo build --release --target ${{ matrix.target }}

      - name: Upload artifact
        uses: actions/upload-artifact@v4
        with:
          name: reddit-cli-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/reddit-cli*

  release:
    name: Release
    needs: [test, lint, build]
    if: startsWith(github.ref, 'refs/tags/')
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4

      - name: Download artifacts
        uses: actions/download-artifact@v4

      - name: Create Release
        uses: softprops/action-gh-release@v1
        with:
          files: |
            reddit-cli-*/reddit-cli*
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

### 7.5 Cross-Compilation

#### Cargo.toml

```toml
# ... existing config

[package.metadata.cross.build]
pre-build = ["apt-get install -y libssl-dev"]

# Binary size optimization
[profile.release]
opt-level = "z"
lto = true
codegen-units = 1
panic = "abort"
strip = true
```

#### Cross.toml (for cross-rs)

```toml
[target.x86_64-unknown-linux-gnu]
pre-build = ["apt-get install -y libssl-dev"]

[target.aarch64-unknown-linux-gnu]
pre-build = ["apt-get install -y libssl-dev"]

[target.x86_64-pc-windows-gnu]
pre-build = ["apt-get install -y mingw-w64"]
```

### 7.6 Homebrew Formula

#### Formula/reddit-cli.rb

```ruby
class RedditCli < Formula
  desc "A CLI client for Reddit"
  homepage "https://github.com/yourname/reddit-cli"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_intel do
      url "https://github.com/yourname/reddit-cli/releases/download/v0.1.0/reddit-cli-x86_64-apple-darwin.tar.gz"
      sha256 "..."
    end
    on_arm do
      url "https://github.com/yourname/reddit-cli/releases/download/v0.1.0/reddit-cli-aarch64-apple-darwin.tar.gz"
      sha256 "..."
    end
  end

  on_linux do
    on_intel do
      url "https://github.com/yourname/reddit-cli/releases/download/v0.1.0/reddit-cli-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "..."
    end
  end

  def install
    bin.install "reddit-cli"
  end

  test do
    assert_match "reddit", shell_output("#{bin}/reddit-cli --version")
  end
end
```

---

## 완료 기준

1. `cargo test` 모든 테스트 통과
2. `cargo clippy` 경고 없음
3. `cargo fmt --check` 포맷팅 OK
4. GitHub Actions CI 통과
5. README.md 완성
6. 바이너리 릴리즈 생성
7. `cargo doc --open` 문서 확인

---

## 프로젝트 완료

모든 Phase 완료 시 Reddit CLI 프로젝트가 완성됩니다.

### 최종 기능 요약

| 카테고리 | 기능 |
|----------|------|
| **읽기** | hot/new/top/rising/controversial, 서브레딧, 사용자, 게시물, 검색 |
| **인증** | OAuth2 로그인, 토큰 관리, 자동 갱신 |
| **내 계정** | 정보, 카르마, 설정, 트로피, 구독 서브레딧 |
| **투표/저장** | 업보트, 다운보트, 저장, 숨기기 |
| **게시** | 링크/셀프/이미지/비디오 포스트, 댓글, 수정, 삭제 |
| **구독** | 서브레딧 구독/구독취소 |
| **메시지** | 받은/보낸/안읽은 메시지, 메시지 보내기 |
| **모더레이션** | 모드큐, 신고, 승인, 삭제, 밴, 뮤트 |
| **플레어** | 조회, 설정, 템플릿 |
| **위키** | 페이지 목록, 보기, 수정 |
| **멀티레딧** | 생성, 삭제, 관리 |
| **라이브** | 스레드 조회, 업데이트, 생성 |
| **컬렉션** | 생성, 관리 |
| **모드메일** | 대화 조회, 답장, 관리 |
| **모드 노트** | 조회, 추가, 삭제 |

축하합니다! 🎉
