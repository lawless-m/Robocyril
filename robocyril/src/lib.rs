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

// Project structs for project tagging
#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub repo: String,
    pub description: String,
    pub short_description: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewProject {
    pub id: String,
    pub name: String,
    pub repo: String,
    pub description: String,
    pub short_description: String,
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

        CREATE TABLE IF NOT EXISTS projects (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            repo TEXT NOT NULL,
            description TEXT NOT NULL,
            short_description TEXT NOT NULL,
            created_at TEXT NOT NULL
        );
        CREATE INDEX IF NOT EXISTS idx_projects_created ON projects(created_at);
        ",
    )
}

pub fn insert_post(conn: &Connection, post: &NewPost) -> Result<i64> {
    let slug = slug::slugify(&post.title);
    let now = Utc::now();
    let is_published = post.publish.unwrap_or(false);
    let published_at = if is_published {
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

    // Sync project if post is published and has a project tag
    if is_published {
        let _ = sync_project_from_post(conn, post);
    }

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
    let is_publishing = update.publish == Some(true);
    if is_publishing {
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

    // Sync project when publishing
    if is_publishing && rows > 0 {
        if let Ok(Some(post)) = get_post_by_slug(conn, slug) {
            let new_post = NewPost {
                title: post.title,
                content: post.content,
                repo: post.repo,
                tags: Some(post.tags),
                commit_range: post.commit_range,
                publish: Some(true),
            };
            let _ = sync_project_from_post(conn, &new_post);
        }
    }

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

pub fn list_posts_full(conn: &Connection, limit: Option<usize>) -> Result<Vec<Post>> {
    let sql = "SELECT id, slug, title, content, repo, created_at, published_at, tags, commit_range
               FROM posts WHERE published_at IS NOT NULL ORDER BY published_at DESC";

    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query([])?;
    let mut posts = Vec::new();

    while let Some(row) = rows.next()? {
        if let Some(max) = limit {
            if posts.len() >= max {
                break;
            }
        }

        let tags_str: String = row.get(7)?;
        let tags: Vec<String> = serde_json::from_str(&tags_str).unwrap_or_default();
        let created_str: String = row.get(5)?;
        let published_str: Option<String> = row.get(6)?;

        posts.push(Post {
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
        });
    }

    Ok(posts)
}

// Project functions

/// Extract the first paragraph from markdown content (for project snippets)
pub fn extract_first_paragraph(content: &str) -> String {
    // Skip any leading headers or blank lines
    let lines: Vec<&str> = content.lines().collect();
    let mut paragraph = String::new();
    let mut in_paragraph = false;

    for line in lines {
        let trimmed = line.trim();

        // Skip headers and blank lines at the start
        if !in_paragraph {
            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }
            in_paragraph = true;
        }

        // End paragraph on blank line or header
        if in_paragraph && (trimmed.is_empty() || trimmed.starts_with('#')) {
            break;
        }

        if !paragraph.is_empty() {
            paragraph.push(' ');
        }
        paragraph.push_str(trimmed);
    }

    // Truncate to ~200 chars for short description
    if paragraph.len() > 200 {
        let truncated: String = paragraph.chars().take(197).collect();
        format!("{}...", truncated.trim_end())
    } else {
        paragraph
    }
}

/// Parse a project tag to get (id, name)
/// "® Robocyril" → ("robocyril", "Robocyril")
pub fn parse_project_tag(tag: &str) -> Option<(String, String)> {
    if !tag.starts_with('®') {
        return None;
    }
    let name = tag.trim_start_matches('®').trim();
    if name.is_empty() {
        return None;
    }
    let id = name.to_lowercase();
    Some((id, name.to_string()))
}

/// Sync project from post data - creates or updates project when post has project tag
pub fn sync_project_from_post(conn: &Connection, post: &NewPost) -> Result<()> {
    let tags = post.tags.as_ref().map(|t| t.as_slice()).unwrap_or(&[]);

    // Find first project tag
    let project_info = tags.iter()
        .filter_map(|tag| parse_project_tag(tag))
        .next();

    if let Some((id, name)) = project_info {
        // Need a repo URL to create a project
        let repo = match &post.repo {
            Some(r) if !r.is_empty() => r.clone(),
            _ => return Ok(()), // No repo, skip project creation
        };

        let short_desc = extract_first_paragraph(&post.content);
        let description = format!("{}: {}", post.title, short_desc);

        let project = NewProject {
            id,
            name,
            repo,
            description,
            short_description: short_desc,
        };

        insert_project(conn, &project)?;
    }

    Ok(())
}

pub fn insert_project(conn: &Connection, project: &NewProject) -> Result<()> {
    let now = Utc::now();
    conn.execute(
        "INSERT OR REPLACE INTO projects (id, name, repo, description, short_description, created_at)
         VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
        (
            &project.id,
            &project.name,
            &project.repo,
            &project.description,
            &project.short_description,
            now.to_rfc3339(),
        ),
    )?;
    Ok(())
}

pub fn list_projects(conn: &Connection) -> Result<Vec<Project>> {
    let sql = "SELECT id, name, repo, description, short_description, created_at
               FROM projects ORDER BY created_at ASC";

    let mut stmt = conn.prepare(sql)?;
    let mut rows = stmt.query([])?;
    let mut projects = Vec::new();

    while let Some(row) = rows.next()? {
        let created_str: String = row.get(5)?;

        projects.push(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            repo: row.get(2)?,
            description: row.get(3)?,
            short_description: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc),
        });
    }

    Ok(projects)
}

pub fn get_project_by_id(conn: &Connection, id: &str) -> Result<Option<Project>> {
    let mut stmt = conn.prepare(
        "SELECT id, name, repo, description, short_description, created_at
         FROM projects WHERE id = ?1",
    )?;

    let mut rows = stmt.query([id])?;

    if let Some(row) = rows.next()? {
        let created_str: String = row.get(5)?;

        Ok(Some(Project {
            id: row.get(0)?,
            name: row.get(1)?,
            repo: row.get(2)?,
            description: row.get(3)?,
            short_description: row.get(4)?,
            created_at: DateTime::parse_from_rfc3339(&created_str)
                .unwrap()
                .with_timezone(&Utc),
        }))
    } else {
        Ok(None)
    }
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
