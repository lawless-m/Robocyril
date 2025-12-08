use robocyril_api::{open_db, list_projects, json_ok, json_error};

fn main() {
    let conn = match open_db() {
        Ok(c) => c,
        Err(e) => {
            json_error(500, &format!("Database error: {}", e));
            return;
        }
    };

    match list_projects(&conn) {
        Ok(projects) => json_ok(&projects),
        Err(e) => json_error(500, &format!("Query failed: {}", e)),
    }
}
