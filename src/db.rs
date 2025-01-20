use std::fs;
use std::io::Error as IOError;
use postgres::{Client, NoTls, Error as PgError};
use dotenvy::dotenv;
use std::env;

pub fn connect_db() -> Result<Client, PgError> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let client = Client::connect(&database_url, NoTls)?;

    Ok(client)
}

// Note: not used yet, for use when building out scalable db arch
pub fn load_query(file_path: &str) -> Result<String, IOError> {
    match fs::read_to_string(file_path) {
        Ok(q) => Ok(q),
        Err(e) => {
            error!("Failed to load SQL query: {}", e);
            Err(e) // ✅ Return `std::io::Error`, not `Status`
        }
    }
}