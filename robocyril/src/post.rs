use robocyril_api::{open_db, insert_post, read_stdin, json_ok, json_error, require_auth, NewPost};

fn main() {
    if !require_auth() {
        return;
    }

    let input = read_stdin();

    let post: NewPost = match serde_json::from_str(&input) {
        Ok(p) => p,
        Err(e) => {
            json_error(400, &format!("Invalid JSON: {}", e));
            return;
        }
    };

    if post.title.trim().is_empty() {
        json_error(400, "Title is required");
        return;
    }

    if post.content.trim().is_empty() {
        json_error(400, "Content is required");
        return;
    }

    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            json_error(500, &format!("Database error: {}", e));
            return;
        }
    };

    match insert_post(&conn, &post) {
        Ok(id) => {
            json_ok(&serde_json::json!({
                "success": true,
                "id": id,
                "slug": slug::slugify(&post.title)
            }));
        }
        Err(e) => {
            json_error(500, &format!("Insert failed: {}", e));
        }
    }
}
