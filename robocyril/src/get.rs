use robocyril_api::{open_db, get_post_by_slug, json_ok, json_error};
use std::env;

fn main() {
    // CGI passes query string in QUERY_STRING env var
    let query = env::var("QUERY_STRING").unwrap_or_default();
    
    // Parse ?slug=xxx
    let slug = query
        .split('&')
        .find_map(|pair| {
            let mut parts = pair.splitn(2, '=');
            match (parts.next(), parts.next()) {
                (Some("slug"), Some(v)) => Some(v.to_string()),
                _ => None,
            }
        });

    let slug = match slug {
        Some(s) => s,
        None => {
            json_error(400, "Missing slug parameter");
            return;
        }
    };

    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            json_error(500, &format!("Database error: {}", e));
            return;
        }
    };

    match get_post_by_slug(&conn, &slug) {
        Ok(Some(post)) => json_ok(&post),
        Ok(None) => json_error(404, "Post not found"),
        Err(e) => json_error(500, &format!("Query failed: {}", e)),
    }
}
