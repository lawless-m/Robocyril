use robocyril_api::{open_db, update_post, read_stdin, json_ok, json_error, require_auth, UpdatePost};
use std::env;

fn main() {
    if !require_auth() {
        return;
    }

    let query = env::var("QUERY_STRING").unwrap_or_default();

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

    let input = read_stdin();
    let update: UpdatePost = match serde_json::from_str(&input) {
        Ok(u) => u,
        Err(e) => {
            json_error(400, &format!("Invalid JSON: {}", e));
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

    match update_post(&conn, &slug, &update) {
        Ok(true) => json_ok(&serde_json::json!({"success": true})),
        Ok(false) => json_error(404, "Post not found or no changes"),
        Err(e) => json_error(500, &format!("Update failed: {}", e)),
    }
}
