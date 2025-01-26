use std::fs;
use std::io::Error as IOError;
use postgres::{Client, NoTls, Error as PgError};
use dotenvy::dotenv;
use std::env;
use refinery::Error as RefineryError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum DatabaseError {
    #[error("Postgres error: {0}")]
    PostgresError(#[from] PgError),
    
    #[error("Migration error: {0}")]
    MigrationError(#[from] RefineryError),
}

mod embedded {
    use refinery::embed_migrations;
    embed_migrations!("./src/migrations");
}

pub fn start_db() -> Result<Client, DatabaseError> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let mut client = Client::connect(&database_url, NoTls)
        .map_err(DatabaseError::PostgresError)?;
    apply_migrations(&mut client)?;
    info!("Database migrations executed successfully");
    Ok(client)
}

pub fn connect_db() -> Result<Client, PgError> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let client = Client::connect(&database_url, NoTls)?;
    Ok(client)
}

fn apply_migrations(client: &mut Client) -> Result<(), RefineryError> {
    let applied = embedded::migrations::runner().run(client);

    match applied {
        Ok(_) => {
            info!("Database migrations executed");
            Ok(())
        }
        Err(e) => {
            error!("Database migration failed: {}", e);
            Err(RefineryError::from(e))
        }
    }
}

pub fn load_query(file_path: &str) -> Result<String, IOError> {
    match fs::read_to_string(file_path) {
        Ok(q) => Ok(q),
        Err(e) => {
            error!("Failed to load SQL query: {}", e);
            Err(e)
        }
    }
}