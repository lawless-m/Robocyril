use chrono::{DateTime, Utc};
use rusqlite::{Connection, Result};
use serde::{Deserialize, Serialize};

pub fn db_path() -> String {
    std::env::var("BLOG_DB_PATH").unwrap_or_else(|_| "/var/lib/robocyril/blog.db".to_string())
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub id: Option<i64>,
    pub slug: String,
    pub title: String,
    pub content: String,
    pub repo: Option<String>,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
    pub commit_range: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub repo: Option<String>,
    pub tags: Option<Vec<String>>,
    pub commit_range: Option<String>,
    pub publish: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostSummary {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
    pub published_at: Option<DateTime<Utc>>,
    pub tags: Vec<String>,
}

pub fn open_db() -> Result<Connection> {
    Connection::open(db_path())
}

pub fn init_db(conn: &Connection) -> Result<()> {
    conn.execute_batch(
        "
        CREATE TABLE IF NOT EXISTS posts (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            slug TEXT UNIQUE NOT NULL,
            title TEXT NOT NULL,
            content TEXT NOT NULL,
            repo TEXT,
            created_at TEXT NOT NULL,
            published_at TEXT,
            tags TEXT NOT NULL DEFAULT '[]',
            commit_range TEXT
        );
        CREATE INDEX IF NOT EXISTS idx_posts_published ON posts(published_at);
        CREATE INDEX IF NOT EXISTS idx_posts_created ON posts(created_at);
        ",
    )
}

pub fn insert_post(conn: &Connection, post: &NewPost) -> Result<i64> {
    let slug = slug::slugify(&post.title);
    let now = Utc::now();
    let published_at = if post.publish.unwrap_or(false) {
        Some(now)
    } else {
        None
    };
    let tags_json = serde_json::to_string(&post.tags.clone().unwrap_or_default()).unwrap();

    conn.execute(
        "INSERT INTO posts (slug, title, content, repo, created_at, published_at, tags, commit_range)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
        (
            &slug,
            &post.title,
            &post.content,
            &post.repo,
            now.to_rfc3339(),
            published_at.map(|t| t.to_rfc3339()),
            &tags_json,
            &post.commit_range,
        ),
    )?;

    Ok(conn.last_insert_rowid())
}

pub fn get_post_by_slug(conn: &Connection, slug: &str) -> Result<Option<Post>> {
    let mut stmt = conn.prepare(
        "SELECT id, slug, title, content, repo, created_at, published_at, tags, commit_range
         FROM posts WHERE slug = ?1",
    )?;

    let mut rows = stmt.query([slug])?;

    if let Some(row) = rows.next()? {
        let tags_str: String = row.get(7)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        let created_str: String = row.get(5)?;
        let published_str: Option<String> = row.get(6)?;

        Ok(Some(Post {
            id: Some(row.get(0)?),
            slug: row.get(1)?,
            title: row.get(2)?,
            content: row.get(3)?,
            repo: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc),
            published_at: published_str.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            tags,
            commit_range: row.get(8)?,
        }))
    } else {
        Ok(None)
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub publish: Option<bool>,
}

pub fn update_post(conn: &Connection, slug: &str, update: &UpdatePost) -> Result<bool> {
    // Build dynamic update query
    let mut sets = Vec::new();
    let mut params: Vec<Box<dyn rusqlite::ToSql>> = Vec::new();

    if let Some(title) = &update.title {
        sets.push("title = ?");
        params.push(Box::new(title.clone()));
    }
    if let Some(content) = &update.content {
        sets.push("content = ?");
        params.push(Box::new(content.clone()));
    }
    if update.publish == Some(true) {
        sets.push("published_at = ?");
        params.push(Box::new(Utc::now().to_rfc3339()));
    }

    if sets.is_empty() {
        return Ok(false);
    }

    let sql = format!("UPDATE posts SET {} WHERE slug = ?", sets.join(", "));
    params.push(Box::new(slug.to_string()));

    let param_refs: Vec<&dyn rusqlite::ToSql> = params.iter().map(|p| p.as_ref()).collect();
    let rows = conn.execute(&sql, param_refs.as_slice())?;

    Ok(rows > 0)
}

pub fn delete_post(conn: &Connection, slug: &str) -> Result<bool> {
    let rows = conn.execute("DELETE FROM posts WHERE slug = ?", [slug])?;
    Ok(rows > 0)
}

pub fn list_posts(conn: &Connection, include_drafts: bool) -> Result<Vec<PostSummary>> {
    let sql = if include_drafts {
        "SELECT id, slug, title, created_at, published_at, tags FROM posts ORDER BY created_at DESC"
    } else {
        "SELECT id, slug, title, created_at, published_at, tags FROM posts WHERE published_at IS NOT NULL ORDER BY published_at DESC"
    };

    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query([])?;
    let mut posts = Vec::new();

    while let Some(row) = rows.next()? {
        let tags_str: String = row.get(5)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        let created_str: String = row.get(3)?;
        let published_str: Option<String> = row.get(4)?;

        posts.push(PostSummary {
            id: row.get(0)?,
            slug: row.get(1)?,
            title: row.get(2)?,
            created_at: DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc),
            published_at: published_str.map(|s| {
                DateTime::parse_from_rfc3339(&s)
                    .unwrap()
                    .with_timezone(&Utc)
            }),
            tags,
        });
    }

    Ok(posts)
}

// Authentication
pub fn api_key_path() -> String {
    std::env::var("BLOG_API_KEY_PATH").unwrap_or_else(|_| "/etc/robocyril-api-key".to_string())
}

pub fn check_auth() -> bool {
    let expected = match std::fs::read_to_string(api_key_path()) {
        Ok(key) => key.trim().to_string(),
        Err(_) => return false,
    };

    let provided = std::env::var("HTTP_X_CYRIL_KEY").unwrap_or_default();
    !expected.is_empty() && provided == expected
}

pub fn require_auth() -> bool {
    if !check_auth() {
        json_error(401, "Unauthorized");
        false
    } else {
        true
    }
}

// CGI helpers
pub fn read_stdin() -> String {
    use std::io::Read;
    let mut buffer = String::new();
    std::io::stdin().read_to_string(&mut buffer).unwrap_or_default();
    buffer
}

pub fn cgi_response(status: u16, content_type: &str, body: &str) {
    println!("Status: {}", status);
    println!("Content-Type: {}", content_type);
    println!();
    print!("{}", body);
}

pub fn json_ok<T: Serialize>(data: &T) {
    let body = serde_json::to_string(data).unwrap();
    cgi_response(200, "application/json", &body);
}

pub fn json_error(status: u16, message: &str) {
    let body = serde_json::json!({"error": message}).to_string();
    cgi_response(status, "application/json", &body);
}
