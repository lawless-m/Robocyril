use robocyril_api::{open_db, list_posts, json_ok, json_error};
use std::env;

fn main() {
    let query = env::var("QUERY_STRING").unwrap_or_default();
    
    // Parse ?drafts=true
    let include_drafts = query
        .split('&')
        .any(|pair| pair == "drafts=true" || pair == "drafts=1");

    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            json_error(500, &format!("Database error: {}", e));
            return;
        }
    };

    match list_posts(&conn, include_drafts) {
        Ok(posts) => json_ok(&posts),
        Err(e) => json_error(500, &format!("Query failed: {}", e)),
    }
}
