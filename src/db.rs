use tokio::fs;
use postgres::{Client, NoTls, Error};
use dotenvy::dotenv;
use std::env;

pub fn connect_db() -> Result<Client, Error> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let client = Client::connect(&database_url, NoTls)?;

    Ok(client)
}

// Note: not used yet, for use when building out scalable db arch
pub async fn load_query(file_path: &str) -> Result<String, std::io::Error> {
    fs::read_to_string(file_path).await
}