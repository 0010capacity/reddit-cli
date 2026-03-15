# Reddit CLI 진행 가이드

> **에이전트용 가이드**: 이 문서는 현재 프로젝트 상태와 다음 작업을 안내합니다.
> 세션 시작 시 이 파일을 먼저 읽고, 현재 진행 중인 Phase의 문서를 참조하여 작업을 이어가세요.

---

## 현재 상태

| 항목 | 상태 |
|------|------|
| **프로젝트 초기화** | ✅ 완료 |
| **계획서 작성** | ✅ 완료 |
| **Phase 문서 작성** | ✅ 완료 |
| **Phase 1: 기본 인프라** | ✅ 완료 |
| **Phase 2: 읽기 API** | ✅ 완료 |
| **Phase 3: OAuth 인증** | ✅ 완료 |
| **Phase 4: 쓰기 API** | 🔲 미시작 |
| **Phase 5: 메시지 & 모더레이션** | 🔲 미시작 |
| **Phase 6: 고급 기능** | 🔲 미시작 |
| **Phase 7: 품질 개선** | 🔲 미시작 |

**다음 작업**: Phase 4 시작 (`doc/phases/PHASE4.md` 참조)

---

## Phase 문서 구조

각 Phase는 `doc/phases/PHASE{N}.md`에 정의되어 있습니다.

| Phase | 문서 | 내용 | 예상 기간 |
|-------|------|------|-----------|
| 1 | [PHASE1.md](phases/PHASE1.md) | 기본 인프라 (Cargo, CLI, HTTP, 모델) | 1-2주 |
| 2 | [PHASE2.md](phases/PHASE2.md) | 읽기 API (hot/new/top, 서브레딧, 게시물, 사용자, 검색) | 1-2주 |
| 3 | [PHASE3.md](phases/PHASE3.md) | OAuth 인증 (로그인, 토큰 관리, me 커맨드) | 1주 |
| 4 | [PHASE4.md](phases/PHASE4.md) | 쓰기 API (투표, 저장, 구독, 게시물/댓글 작성) | 1-2주 |
| 5 | [PHASE5.md](phases/PHASE5.md) | 메시지 & 모더레이션 | 1-2주 |
| 6 | [PHASE6.md](phases/PHASE6.md) | 고급 기능 (플레어, 위키, 멀티, 라이브, 컬렉션, 모드메일) | 2주 |
| 7 | [PHASE7.md](phases/PHASE7.md) | 품질 개선 (테스트, 문서화, CI/CD) | 1주 |

---

## 에이전트 작업 가이드

### 세션 시작 시
1. 이 파일(`doc/PROGRESS.md`)을 읽어 현재 상태 확인
2. 현재 진행 중인 Phase 문서(`doc/phases/PHASE{N}.md`) 읽기
3. 체크리스트에서 미완료 항목 확인
4. 해당 항목 구현

### 작업 완료 시
1. Phase 문서의 체크리스트 업데이트
2. 이 파일의 "현재 상태" 섹션 업데이트
3. 변경사항 커밋

### Phase 완료 시
1. 해당 Phase의 모든 체크리스트 완료 확인
2. `cargo build` 및 `cargo test` 통과 확인
3. 이 파일에서 해당 Phase 상태를 ✅ 완료로 변경
4. 다음 Phase 시작

---

## 프로젝트 구조 (목표)

```
reddit-cli/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── cli/                    # CLI 커맨드 정의
│   │   ├── mod.rs
│   │   ├── root.rs
│   │   ├── auth.rs
│   │   ├── listing.rs
│   │   ├── subreddit.rs
│   │   ├── user.rs
│   │   ├── comment.rs
│   │   ├── message.rs
│   │   ├── search.rs
│   │   └── moderation.rs
│   ├── api/                    # Reddit API 클라이언트
│   │   ├── mod.rs
│   │   ├── client.rs
│   │   ├── auth.rs
│   │   ├── ratelimit.rs
│   │   └── endpoints/          # API 엔드포인트
│   │       ├── mod.rs
│   │       ├── account.rs
│   │       ├── listing.rs
│   │       ├── subreddit.rs
│   │       ├── user.rs
│   │       ├── comment.rs
│   │       ├── link.rs
│   │       ├── message.rs
│   │       ├── search.rs
│   │       ├── moderation.rs
│   │       ├── modmail.rs
│   │       ├── wiki.rs
│   │       ├── flair.rs
│   │       ├── collection.rs
│   │       ├── live.rs
│   │       ├── multi.rs
│   │       ├── widget.rs
│   │       └── emoji.rs
│   ├── models/                 # 데이터 모델
│   │   ├── mod.rs
│   │   ├── common.rs
│   │   ├── account.rs
│   │   ├── subreddit.rs
│   │   ├── user.rs
│   │   ├── comment.rs
│   │   ├── link.rs
│   │   ├── message.rs
│   │   ├── award.rs
│   │   ├── flair.rs
│   │   ├── trophy.rs
│   │   ├── wiki.rs
│   │   └── moderation.rs
│   ├── output/                 # 출력 포맷팅
│   │   ├── mod.rs
│   │   ├── table.rs
│   │   ├── json.rs
│   │   └── markdown.rs
│   ├── config/                 # 설정 관리
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── cache/                  # 토큰 캐시
│   │   ├── mod.rs
│   │   └── token.rs
│   └── utils/                  # 유틸리티
│       ├── mod.rs
│       ├── time.rs
│       └── text.rs
├── tests/
├── doc/
│   ├── PLAN.md
│   ├── PROGRESS.md
│   └── phases/
│       ├── PHASE1.md
│       ├── PHASE2.md
│       ├── PHASE3.md
│       ├── PHASE4.md
│       ├── PHASE5.md
│       ├── PHASE6.md
│       └── PHASE7.md
└── .gitignore
```

---

## 빠른 참조

### 주요 의존성
```toml
[dependencies]
clap = { version = "4", features = ["derive"] }
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
thiserror = "1"
anyhow = "1"
tracing = "0.1"
tracing-subscriber = "0.3"
config = "0.14"
toml = "0.8"
dirs = "5"
oauth2 = "4"
url = "2"
chrono = { version = "0.4", features = ["serde"] }
```

### Reddit API 기본 URL
- OAuth: `https://oauth.reddit.com`
- WWW (OAuth 없음): `https://www.reddit.com`

### User-Agent 형식
```
<platform>:<app_id>:<version> (by /u/<reddit_username>)
```
예: `cli:reddit-cli:0.1.0 (by /u/your_username)`

---

## 문제 해결

### 일반적인 문제
1. **빌드 실패**: `cargo clean` 후 다시 빌드
2. **API 429**: Rate limit 초과, 잠시 대기
3. **OAuth 실패**: 토큰 만료, `reddit auth refresh` 또는 재로그인

### 디버깅
```bash
RUST_LOG=debug cargo run -- hot --limit 5
```
