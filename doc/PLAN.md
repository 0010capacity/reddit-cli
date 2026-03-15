# Reddit CLI - 프로젝트 계획서

## 1. 프로젝트 개요

Rust로 작성된 Reddit API CLI 클라이언트. Reddit의 방대한 API 기능을 커맨드라인에서 사용할 수 있도록 구현하는 것이 목표.

### 기술 스택
- **언어**: Rust (Edition 2021)
- **CLI 프레임워크**: `clap` (derive 기반)
- **HTTP 클라이언트**: `reqwest` + `tokio`
- **JSON 직렬화**: `serde` + `serde_json`
- **OAuth2**: `oauth2` crate
- **터미널 UI**: `crossterm` + `ratatui` (선택적 TUI)
- **출력 포맷팅**: `prettytable-rs`, `syntect` (마크다운 렌더링)
- **설정 관리**: `config` + `toml`
- **에러 처리**: `thiserror`, `anyhow`
- **로깅**: `tracing` + `tracing-subscriber`

---

## 2. 아키텍처 설계

```
reddit-cli/
├── Cargo.toml
├── src/
│   ├── main.rs                 # 진입점
│   ├── cli/                    # CLI 정의 (clap)
│   │   ├── mod.rs
│   │   ├── root.rs             # 루트 커맨드
│   │   ├── auth.rs             # 인증 관련 커맨드
│   │   ├── listing.rs          # 게시물 목록 커맨드
│   │   ├── subreddit.rs        # 서브레딧 관련 커맨드
│   │   ├── user.rs             # 사용자 관련 커맨드
│   │   ├── comment.rs          # 댓글 관련 커맨드
│   │   ├── message.rs          # 메시지 관련 커맨드
│   │   ├── search.rs           # 검색 커맨드
│   │   └── moderation.rs       # 모더레이션 커맨드
│   ├── api/                    # Reddit API 클라이언트
│   │   ├── mod.rs
│   │   ├── client.rs           # HTTP 클라이언트 래퍼
│   │   ├── auth.rs             # OAuth2 인증 로직
│   │   ├── endpoints/          # API 엔드포인트별 모듈
│   │   │   ├── mod.rs
│   │   │   ├── account.rs      # 계정 API
│   │   │   ├── listing.rs      # 리스트업 API (hot, new, top, etc.)
│   │   │   ├── subreddit.rs    # 서브레딧 API
│   │   │   ├── user.rs         # 사용자 API
│   │   │   ├── comment.rs      # 댓글 API
│   │   │   ├── link.rs         # 링크/게시물 API
│   │   │   ├── message.rs      # 개인 메시지 API
│   │   │   ├── search.rs       # 검색 API
│   │   │   ├── moderation.rs   # 모더레이션 API
│   │   │   ├── modmail.rs      # 새 모드메일 API
│   │   │   ├── wiki.rs         # 위키 API
│   │   │   ├── flair.rs        # 플레어 API
│   │   │   ├── collection.rs   # 컬렉션 API
│   │   │   ├── live.rs         # 라이브 스레드 API
│   │   │   ├── multi.rs        # 멀티레딧 API
│   │   │   ├── widget.rs       # 위젯 API
│   │   │   ├── emoji.rs        # 이모지 API
│   │   │   └── trophy.rs       # 트로피 API
│   │   └── ratelimit.rs        # Rate limiting 처리
│   ├── models/                 # 데이터 모델
│   │   ├── mod.rs
│   │   ├── common.rs           # 공통 타입 (Thing, Listing, etc.)
│   │   ├── account.rs          # 계정 모델
│   │   ├── subreddit.rs        # 서브레딧 모델
│   │   ├── user.rs             # 사용자 모델
│   │   ├── comment.rs          # 댓글 모델
│   │   ├── link.rs             # 링크/게시물 모델
│   │   ├── message.rs          # 메시지 모델
│   │   ├── award.rs            # 어워드 모델
│   │   ├── flair.rs            # 플레어 모델
│   │   ├── trophy.rs           # 트로피 모델
│   │   ├── wiki.rs             # 위키 모델
│   │   └── moderation.rs       # 모더레이션 로그 모델
│   ├── output/                 # 출력 포맷팅
│   │   ├── mod.rs
│   │   ├── table.rs            # 테이블 출력
│   │   ├── json.rs             # JSON 출력
│   │   ├── markdown.rs         # 마크다운 렌더링
│   │   └── pager.rs            # 페이저 지원
│   ├── config/                 # 설정 관리
│   │   ├── mod.rs
│   │   └── settings.rs         # 설정 파일 로드/저장
│   ├── cache/                  # 캐싱 (선택적)
│   │   ├── mod.rs
│   │   └── token.rs            # OAuth 토큰 캐시
│   └── utils/                  # 유틸리티
│       ├── mod.rs
│       ├── time.rs             # 시간 포맷팅
│       └── text.rs             # 텍스트 처리
├── tests/                      # 통합 테스트
├── doc/
│   └── PLAN.md                 # 이 파일
└── .gitignore
```

---

## 3. 구현할 기능 목록

### Phase 1: 기본 인프라 (OAuth 불필요)

#### 3.1 기본 읽기 기능
| 커맨드 | 설명 | API 엔드포인트 | OAuth |
|--------|------|----------------|-------|
| `reddit hot` | 인기 게시물 | `GET /hot` | X |
| `reddit new` | 최신 게시물 | `GET /new` | X |
| `reddit top` | 탑 게시물 | `GET /top` | X |
| `reddit rising` | 라이징 게시물 | `GET /rising` | X |
| `reddit controversial` | 논란 게시물 | `GET /controversial` | X |
| `reddit best` | 베스트 게시물 | `GET /best` | O (선택) |

#### 3.2 서브레딧 조회
| 커맨드 | 설명 | API 엔드포인트 | OAuth |
|--------|------|----------------|-------|
| `reddit subreddit show <name>` | 서브레딧 정보 | `GET /r/{subreddit}/about` | X |
| `reddit subreddit hot <name>` | 서브레딧 인기글 | `GET /r/{subreddit}/hot` | X |
| `reddit subreddit new <name>` | 서브레딧 최신글 | `GET /r/{subreddit}/new` | X |
| `reddit subreddit top <name>` | 서브레딧 탑글 | `GET /r/{subreddit}/top` | X |
| `reddit subreddit rules <name>` | 서브레딧 규칙 | `GET /r/{subreddit}/about/rules` | X |
| `reddit subreddit sidebar <name>` | 사이드바 | `GET /r/{subreddit}/sidebar` | X |
| `reddit subreddit traffic <name>` | 트래픽 통계 | `GET /r/{subreddit}/about/traffic` | X |
| `reddit subreddits popular` | 인기 서브레딧 | `GET /subreddits/popular` | X |
| `reddit subreddits new` | 새 서브레딧 | `GET /subreddits/new` | X |
| `reddit subreddits search <q>` | 서브레딧 검색 | `GET /subreddits/search` | X |

#### 3.3 게시물/댓글 조회
| 커맨드 | 설명 | API 엔드포인트 | OAuth |
|--------|------|----------------|-------|
| `reddit post view <id>` | 게시물 + 댓글 | `GET /comments/{article}` | X |
| `reddit post info <id>` | 게시물 정보 | `GET /api/info` | X |
| `reddit post duplicates <id>` | 중복 게시물 | `GET /duplicates/{article}` | X |

#### 3.4 사용자 조회
| 커맨드 | 설명 | API 엔드포인트 | OAuth |
|--------|------|----------------|-------|
| `reddit user show <username>` | 사용자 정보 | `GET /user/{username}/about` | X |
| `reddit user posts <username>` | 사용자 게시물 | `GET /user/{username}/submitted` | X |
| `reddit user comments <username>` | 사용자 댓글 | `GET /user/{username}/comments` | X |
| `reddit user overview <username>` | 개요 | `GET /user/{username}/overview` | X |
| `reddit user saved <username>` | 저장한 글 | `GET /user/{username}/saved` | O |
| `reddit user upvoted <username>` | 업보트한 글 | `GET /user/{username}/upvoted` | O |
| `reddit user downvoted <username>` | 다운보트한 글 | `GET /user/{username}/downvoted` | O |
| `reddit user gilded <username>` | 길드받은 글 | `GET /user/{username}/gilded` | X |
| `reddit user trophies <username>` | 트로피 | `GET /user/{username}/trophies` | X |

#### 3.5 검색
| 커맨드 | 설명 | API 엔드포인트 | OAuth |
|--------|------|----------------|-------|
| `reddit search <query>` | 전체 검색 | `GET /search` | X |
| `reddit search <query> --subreddit <name>` | 서브레딧 내 검색 | `GET /r/{sr}/search` | X |

---

### Phase 2: OAuth 인증

#### 3.6 인증
| 커맨드 | 설명 | 비고 |
|--------|------|------|
| `reddit auth login` | OAuth 로그인 | 브라우저 열기 → 콜백 서버 |
| `reddit auth logout` | 로그아웃 | 토큰 삭제 |
| `reddit auth status` | 인증 상태 확인 | |
| `reddit auth refresh` | 토큰 갱신 | |

#### 3.7 내 계정 (OAuth 필요)
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit me` | 내 정보 | `GET /api/v1/me` |
| `reddit me karma` | 내 카르마 | `GET /api/v1/me/karma` |
| `reddit me preferences` | 내 설정 | `GET /api/v1/me/prefs` |
| `reddit me trophies` | 내 트로피 | `GET /api/v1/me/trophies` |
| `reddit me friends` | 친구 목록 | `GET /api/v1/me/friends` |
| `reddit me blocked` | 차단 목록 | `GET /api/v1/me/blocked` |

#### 3.8 내 서브레딧 (OAuth 필요)
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit me subreddits` | 구독한 서브레딧 | `GET /subreddits/mine/subscriber` |
| `reddit me contributor` | 기여자인 서브레딧 | `GET /subreddits/mine/contributor` |
| `reddit me moderator` | 모더레이터인 서브레딧 | `GET /subreddits/mine/moderator` |

---

### Phase 3: 쓰기 기능 (OAuth 필요)

#### 3.9 투표
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit upvote <id>` | 업보트 | `POST /api/vote` (dir=1) |
| `reddit downvote <id>` | 다운보트 | `POST /api/vote` (dir=-1) |
| `reddit unvote <id>` | 투표 취소 | `POST /api/vote` (dir=0) |

#### 3.10 저장/숨기기
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit save <id>` | 저장 | `POST /api/save` |
| `reddit unsave <id>` | 저장 취소 | `POST /api/unsave` |
| `reddit hide <id>` | 숨기기 | `POST /api/hide` |
| `reddit unhide <id>` | 숨기기 취소 | `POST /api/unhide` |

#### 3.11 구독
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit subscribe <subreddit>` | 서브레딧 구독 | `POST /api/subscribe` (action=sub) |
| `reddit unsubscribe <subreddit>` | 구독 취소 | `POST /api/subscribe` (action=unsub) |

#### 3.12 게시물 작성
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit submit link --sr <subreddit> --title <title> --url <url>` | 링크 게시 | `POST /api/submit` (kind=link) |
| `reddit submit self --sr <subreddit> --title <title> --text <text>` | 셀프 포스트 | `POST /api/submit` (kind=self) |
| `reddit submit image --sr <subreddit> --title <title> --file <path>` | 이미지 게시 | `POST /api/submit` (kind=image) |
| `reddit submit video --sr <subreddit> --title <title> --file <path>` | 비디오 게시 | `POST /api/submit` (kind=video) |
| `reddit edit <id> --text <text>` | 게시물 수정 | `POST /api/editusertext` |
| `reddit delete <id>` | 게시물 삭제 | `POST /api/del` |

#### 3.13 댓글
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit comment <parent> --text <text>` | 댓글 작성 | `POST /api/comment` |
| `reddit comment edit <id> --text <text>` | 댓글 수정 | `POST /api/editusertext` |
| `reddit comment delete <id>` | 댓글 삭제 | `POST /api/del` |

---

### Phase 4: 메시지 & 모더레이션

#### 3.14 개인 메시지 (OAuth 필요)
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit message inbox` | 받은 메시지 | `GET /message/inbox` |
| `reddit message unread` | 안읽은 메시지 | `GET /message/unread` |
| `reddit message sent` | 보낸 메시지 | `GET /message/sent` |
| `reddit message send --to <user> --subject <subj> --text <text>` | 메시지 보내기 | `POST /api/compose` |
| `reddit message read <id>` | 읽음 표시 | `POST /api/read_message` |
| `reddit message unread <id>` | 안읽음 표시 | `POST /api/unread_message` |
| `reddit message delete <id>` | 메시지 삭제 | `POST /api/del_msg` |
| `reddit message block <id>` | 사용자 차단 | `POST /api/block` |

#### 3.15 모더레이션 (OAuth + mod 권한 필요)
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit mod reports <subreddit>` | 신고 목록 | `GET /r/{sr}/about/reports` |
| `reddit mod spam <subreddit>` | 스팸 목록 | `GET /r/{sr}/about/spam` |
| `reddit mod modqueue <subreddit>` | 모드큐 | `GET /r/{sr}/about/modqueue` |
| `reddit mod unmoderated <subreddit>` | 미승인 목록 | `GET /r/{sr}/about/unmoderated` |
| `reddit mod edited <subreddit>` | 수정된 목록 | `GET /r/{sr}/about/edited` |
| `reddit mod log <subreddit>` | 모드 로그 | `GET /r/{sr}/about/log` |
| `reddit mod approve <id>` | 승인 | `POST /api/approve` |
| `reddit mod remove <id>` | 삭제 | `POST /api/remove` |
| `reddit mod spam <id>` | 스팸 처리 | `POST /api/remove` (spam=true) |
| `reddit mod distinguish <id>` | 구분 표시 | `POST /api/distinguish` |
| `reddit mod sticky <id>` | 스티키 설정 | `POST /api/set_subreddit_sticky` |
| `reddit mod lock <id>` | 잠금 | `POST /api/lock` |
| `reddit mod unlock <id>` | 잠금 해제 | `POST /api/unlock` |
| `reddit mod nsfw <id>` | NSFW 표시 | `POST /api/marknsfw` |
| `reddit mod report <id> --reason <reason>` | 신고 | `POST /api/report` |

#### 3.16 사용자 관리 (모더레이터)
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit mod ban <subreddit> --user <user>` | 밴 | `POST /api/friend` (type=banned) |
| `reddit mod unban <subreddit> --user <user>` | 밴 해제 | `POST /api/unfriend` |
| `reddit mod mute <subreddit> --user <user>` | 뮤트 | `POST /api/friend` (type=muted) |
| `reddit mod unmute <subreddit> --user <user>` | 뮤트 해제 | `POST /api/unfriend` |
| `reddit mod contributors <subreddit>` | 기여자 목록 | `GET /r/{sr}/about/contributors` |
| `reddit mod banned <subreddit>` | 밴 목록 | `GET /r/{sr}/about/banned` |
| `reddit mod muted <subreddit>` | 뮤트 목록 | `GET /r/{sr}/about/muted` |
| `reddit mod moderators <subreddit>` | 모더레이터 목록 | `GET /r/{sr}/about/moderators` |

---

### Phase 5: 고급 기능

#### 3.17 플레어
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit flair list <subreddit>` | 플레어 목록 | `GET /r/{sr}/api/flairlist` |
| `reddit flair set <subreddit> --user <user> --text <text>` | 플레어 설정 | `POST /api/flair` |
| `reddit flair delete <subreddit> --user <user>` | 플레어 삭제 | `POST /api/deleteflair` |
| `reddit flair templates <subreddit>` | 플레어 템플릿 | `GET /api/user_flair_v2` |
| `reddit link-flair templates <subreddit>` | 링크 플레어 템플릿 | `GET /api/link_flair_v2` |

#### 3.18 위키
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit wiki pages <subreddit>` | 위키 페이지 목록 | `GET /r/{sr}/wiki/pages` |
| `reddit wiki view <subreddit> <page>` | 위키 페이지 보기 | `GET /r/{sr}/wiki/{page}` |
| `reddit wiki revisions <subreddit> <page>` | 수정 이력 | `GET /r/{sr}/wiki/revisions/{page}` |
| `reddit wiki edit <subreddit> <page> --content <content>` | 위키 수정 | `POST /api/wiki/edit` |

#### 3.19 멀티레딧
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit multi list` | 내 멀티 목록 | `GET /api/multi/mine` |
| `reddit multi show <path>` | 멀티 정보 | `GET /api/multi/{path}` |
| `reddit multi create <path>` | 멀티 생성 | `POST /api/multi/{path}` |
| `reddit multi delete <path>` | 멀티 삭제 | `DELETE /api/multi/{path}` |
| `reddit multi add <path> --subreddit <sr>` | 서브레딧 추가 | `PUT /api/multi/{path}/r/{sr}` |
| `reddit multi remove <path> --subreddit <sr>` | 서브레딧 제거 | `DELETE /api/multi/{path}/r/{sr}` |

#### 3.20 라이브 스레드
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit live show <id>` | 라이브 스레드 | `GET /live/{thread}` |
| `reddit live about <id>` | 라이브 정보 | `GET /live/{thread}/about` |
| `reddit live contributors <id>` | 기여자 목록 | `GET /live/{thread}/contributors` |
| `reddit live create --title <title>` | 라이브 생성 | `POST /api/live/create` |
| `reddit live update <id> --body <body>` | 업데이트 게시 | `POST /api/live/{thread}/update` |

#### 3.21 컬렉션
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit collection show <id>` | 컬렉션 보기 | `GET /api/v1/collections/collection` |
| `reddit collection list <subreddit>` | 컬렉션 목록 | `GET /api/v1/collections/subreddit_collections` |
| `reddit collection create --title <title> --sr <sr>` | 컬렉션 생성 | `POST /api/v1/collections/create_collection` |
| `reddit collection add <id> --post <post_id>` | 게시물 추가 | `POST /api/v1/collections/add_post_to_collection` |

#### 3.22 모드메일 (새 모드메일)
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit modmail list` | 모드메일 목록 | `GET /api/mod/conversations` |
| `reddit modmail show <id>` | 모드메일 보기 | `GET /api/mod/conversations/{id}` |
| `reddit modmail reply <id> --body <body>` | 답장 | `POST /api/mod/conversations/{id}` |
| `reddit modmail create --to <user> --subject <subj> --body <body>` | 새 대화 | `POST /api/mod/conversations` |
| `reddit modmail archive <id>` | 보관 | `POST /api/mod/conversations/{id}/archive` |
| `reddit modmail highlight <id>` | 하이라이트 | `POST /api/mod/conversations/{id}/highlight` |
| `reddit modmail mute <id>` | 사용자 뮤트 | `POST /api/mod/conversations/{id}/mute` |
| `reddit modmail ban <id>` | 사용자 밴 | `POST /api/mod/conversations/{id}/temp_ban` |

#### 3.23 모드 노트
| 커맨드 | 설명 | API 엔드포인트 |
|--------|------|----------------|
| `reddit modnote show <subreddit> --user <user>` | 노트 보기 | `GET /api/mod/notes` |
| `reddit modnote add <subreddit> --user <user> --note <note>` | 노트 추가 | `POST /api/mod/notes` |
| `reddit modnote delete <subreddit> --user <user> --id <id>` | 노트 삭제 | `DELETE /api/mod/notes` |

---

## 4. OAuth2 인증 흐름

### 4.1 Reddit OAuth2 타입
1. **Installed App (권장)** - CLI에 적합, 로컬 콜백 서버 사용
2. **Web App** - client_secret 필요
3. **Script** - 사용자 비밀번호 직접 사용 (권장하지 않음)

### 4.2 구현 방식
```
1. 사용자: reddit auth login
2. CLI: 로컬 HTTP 서버 시작 (예: http://127.0.0.1:65010)
3. CLI: 브라우저로 Reddit 인증 페이지 열기
4. 사용자: 브라우저에서 인증 승인
5. Reddit: 콜백 URL로 리다이렉트 (code 포함)
6. CLI: 로컬 서버가 code 수신
7. CLI: code를 access_token으로 교환
8. CLI: 토큰을 안전하게 저장
```

### 4.3 필요한 OAuth 스코프
```toml
# 기본 읽기
identity, read, history

# 쓰기
submit, edit, vote, save, report

# 메시지
privatemessages, subscribe

# 모더레이션
modposts, modconfig, modflair, modlog, modothers, modwiki, modcontributors

# 위키
wikiread, wikiedit

# 라이브
livemanage, submit (라이브 스레드용)
```

---

## 5. 설정 파일 구조

### 5.1 설정 파일 위치
- **Linux/macOS**: `~/.config/reddit-cli/config.toml`
- **Windows**: `%APPDATA%\reddit-cli\config.toml`

### 5.2 설정 파일 예시
```toml
[auth]
client_id = "your_client_id"
client_secret = "your_client_secret"  # 선택적
redirect_uri = "http://127.0.0.1:65010"

[api]
base_url = "https://oauth.reddit.com"
www_url = "https://www.reddit.com"
user_agent = "reddit-cli/0.1.0 by /u/your_username"

[output]
format = "table"  # table, json, plain
pager = true
color = true
page_size = 25

[cache]
enabled = true
ttl_seconds = 300
```

### 5.3 토큰 저장 (별도 파일, 권한 600)
```
~/.config/reddit-cli/token.json
```

---

## 6. 개발 로드맵

### Sprint 1: 기본 인프라 (1-2주)
- [ ] 프로젝트 구조 생성
- [ ] Cargo.toml 의존성 설정
- [ ] 기본 CLI 구조 (clap)
- [ ] HTTP 클라이언트 기본 구현
- [ ] 기본 데이터 모델 (Thing, Listing)
- [ ] 에러 처리 체계
- [ ] 설정 파일 로드/저장

### Sprint 2: 읽기 API (1-2주)
- [ ] hot/new/top/rising/controversial 커맨드
- [ ] 서브레딧 조회 커맨드
- [ ] 게시물 + 댓글 조회
- [ ] 사용자 정보 조회
- [ ] 검색 기능
- [ ] 출력 포맷팅 (table, json)
- [ ] 페이지네이션 (--after, --before)

### Sprint 3: OAuth 인증 (1주)
- [ ] OAuth2 흐름 구현
- [ ] 로컬 콜백 서버
- [ ] 토큰 저장/갱신
- [ ] me 커맨드 그룹

### Sprint 4: 쓰기 API (1-2주)
- [ ] 투표 (업/다운/취소)
- [ ] 저장/숨기기
- [ ] 구독/구독취소
- [ ] 게시물 작성/수정/삭제
- [ ] 댓글 작성/수정/삭제

### Sprint 5: 메시지 & 모더레이션 (1-2주)
- [ ] 개인 메시지
- [ ] 모더레이션 기본 기능
- [ ] 사용자 관리 (밴/뮤트)

### Sprint 6: 고급 기능 (2주)
- [ ] 플레어
- [ ] 위키
- [ ] 멀티레딧
- [ ] 라이브 스레드
- [ ] 컬렉션
- [ ] 모드메일
- [ ] 모드 노트

### Sprint 7: 품질 개선 (1주)
- [ ] 테스트 코드
- [ ] 문서화
- [ ] 에러 메시지 개선
- [ ] 성능 최적화
- [ ] CI/CD 설정

---

## 7. CLI 사용 예시

```bash
# 인기 게시물 보기
reddit hot --limit 50

# 특정 서브레딧
reddit subreddit show rust
reddit subreddit hot rust --limit 30
reddit subreddit top rust --time week

# 게시물 보기
reddit post view 15bfi0

# 사용자 정보
reddit user show spez
reddit user comments spez --limit 20

# 검색
reddit search "rust programming" --subreddit rust --sort top

# 로그인
reddit auth login

# 내 정보
reddit me
reddit me karma

# 투표
reddit upvote 15bfi0
reddit downvote 15bfi0

# 게시물 작성
reddit submit self --sr rust --title "Hello from CLI" --text "This is a test"

# 댓글 작성
reddit comment 15bfi0 --text "Great post!"

# 서브레딧 구독
reddit subscribe rust
reddit unsubscribe rust

# 메시지
reddit message inbox
reddit message send --to spez --subject "Hello" --text "Hi!"

# 모더레이션
reddit mod reports example_subreddit
reddit mod approve t3_15bfi0
reddit mod remove t3_15bfi0 --reason "Spam"
```

---

## 8. API Rate Limiting

Reddit API는 다음과 같은 속도 제한이 있음:
- OAuth: 60 requests/minute
- OAuth 없음: 더 엄격함

구현 필요:
- Rate limit 헤더 파싱 (`x-ratelimit-remaining`, `x-ratelimit-reset`)
- 대기열 관리
- 429 응답 시 자동 재시도

---

## 9. 참고 자료

- [Reddit API Documentation](https://www.reddit.com/dev/api/)
- [Reddit OAuth2 Documentation](https://github.com/reddit-archive/reddit/wiki/OAuth2)
- [clap Documentation](https://docs.rs/clap/)
- [reqwest Documentation](https://docs.rs/reqwest/)
