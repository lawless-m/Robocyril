# Robocyril

![Cyril](cyril.jpg)

A blog system where Claude Code writes posts in the voice of Cyril, a grumpy-but-pragmatic maintenance manager. The beautiful irony: Cyril is vocally anti-vibe-coding but is entirely vibe-coded into existence.

## The Setup

**Backend**: Rust CGI scripts, SQLite, nginx - 1990s simplicity
**Frontend**: Svelte SPA - 2020s overkill

The architectural equivalent of a mullet. Business in the back, party in the front.

## What It Does

1. You type `/blog <topic>` in Claude Code
2. Claude drafts a post in Cyril's voice
3. POST to the API creates the blog entry
4. Visitors see a gloriously over-engineered SPA serving content from CGI scripts

## The Persona: Cyril

Maintenance manager energy. 45 years of watching new hotness become old reliable become legacy nightmare. East Midlands matter-of-fact delivery. The Abbott and Costello Defence as a signature move.

**Sample opinions:**
- systemd: hates it, admits it's better than inetd
- Rust: good but the compiler is smug about it
- Ubuntu: it's Debian with marketing
- AI slop: suspicious of it, while being it

**The catchphrase:** "I'm glad you like me."

See [HANDOVER.md](robocyril/HANDOVER.md) for the complete persona specification and technical details.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     steponnopets.net                         │
│  ┌─────────────────────────────────────────────────────────┐ │
│  │                        nginx                             │ │
│  │  ┌─────────────────┐         ┌────────────────────────┐ │ │
│  │  │   /api/*        │         │  / (static)            │ │ │
│  │  │   fcgiwrap      │         │  Svelte SPA            │ │ │
│  │  │   ↓             │         │                        │ │ │
│  │  │  Rust CGI bins  │         │                        │ │ │
│  │  └────────┬────────┘         └────────────────────────┘ │ │
│  │           │                                              │ │
│  │           ▼                                              │ │
│  │  ┌─────────────────┐                                    │ │
│  │  │  SQLite         │                                    │ │
│  │  │  blog.db        │                                    │ │
│  │  └─────────────────┘                                    │ │
│  └─────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────┘
```

## Project Structure

```
Robocyril/
├── robocyril/              # Rust backend (CGI binaries)
├── robocyril-frontend/     # Svelte SPA (public blog)
├── devblog-frontend/       # Svelte SPA (dev/internal blog)
├── .claude/
│   ├── commands/
│   │   ├── blog.md         # /blog command for creating posts
│   │   ├── commit.md       # /commit for git commits
│   │   └── push.md         # /push for commit + push
│   └── skills/             # Reusable Claude Code skills
└── first-post.md           # Example blog post
```

## API Endpoints

All mutating endpoints require `X-Cyril-Key` header.

- `POST /api/posts` - Create post
- `GET /api/posts` - List posts (add `?drafts=true` to include drafts)
- `GET /api/post?slug=xxx` - Get single post
- `PATCH /api/post?slug=xxx` - Update post
- `DELETE /api/post?slug=xxx` - Delete post

See [HANDOVER.md](robocyril/HANDOVER.md) for complete API specification.

## Development

### Backend (Rust)

```bash
cd robocyril
cargo build --release
```

Binaries:
- `blog-init` - Initialize database
- `blog-post`, `blog-get`, `blog-list`, `blog-update`, `blog-delete` - CGI handlers

### Frontend (Svelte)

```bash
cd robocyril-frontend
npm install
npm run dev
```

Build for production:
```bash
npm run build
# Output: dist/
```

## Deployment

See [HANDOVER.md](robocyril/HANDOVER.md) for complete deployment instructions including:
- nginx configuration
- fcgiwrap setup
- Database initialization
- API key management
- SSL setup

## The /blog Command

Create blog posts from Claude Code:

```bash
/blog <topic hint>
```

The command:
1. Checks if the repo is on GitHub (includes link if so)
2. Drafts a post in Cyril's voice
3. Shows you for approval
4. POSTs to the API

Example:
```
/blog AI PR spam and fuzzing for real bugs
```

## Philosophy

This is not a dev diary. Posts should:

1. **Hook** - An interesting problem or observation
2. **Substance** - Something the reader actually learns
3. **Reveal** - Your work emerges as a natural solution

The product is the punchline, not the headline.

## License

See individual component licenses. The Cyril persona and blog content are... well, who owns what an AI writes? Cyril's got opinions on that too.

---

*"Some people say building your own blog platform is a waste of time. Well, I think it's a perfectly reasonable way to avoid writing actual content."*

— Cyril (probably)
