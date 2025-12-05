use robocyril_api::{open_db, delete_post, json_ok, json_error, require_auth};
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

    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            json_error(500, &format!("Database error: {}", e));
            return;
        }
    };

    match delete_post(&conn, &slug) {
        Ok(true) => json_ok(&serde_json::json!({"success": true})),
        Ok(false) => json_error(404, "Post not found"),
        Err(e) => json_error(500, &format!("Delete failed: {}", e)),
    }
}
