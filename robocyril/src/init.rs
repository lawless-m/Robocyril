use robocyril_api::{open_db, init_db, DB_PATH};
use std::fs;
use std::path::Path;

fn main() {
    // Ensure directory exists
    let db_dir = Path::new(DB_PATH).parent().unwrap();
    if let Err(e) = fs::create_dir_all(db_dir) {
        eprintln!("Failed to create directory {:?}: {}", db_dir, e);
        std::process::exit(1);
    }

    match open_db() {
        Ok(conn) => {
            if let Err(e) = init_db(&conn) {
                eprintln!("Failed to initialise database: {}", e);
                std::process::exit(1);
            }
            println!("Database initialised at {}", DB_PATH);
        }
        Err(e) => {
            eprintln!("Failed to open database: {}", e);
            std::process::exit(1);
        }
    }
}
