use std::fs;
use std::io::Error as IOError;
use postgres::{Client, NoTls, Error as PgError};
use dotenvy::dotenv;
use std::{env, path::Path};

pub fn connect_db() -> Result<Client, PgError> {
    dotenv().ok();
    
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env");
    let mut client = Client::connect(&database_url, NoTls)?;
    apply_migrations(&mut client)?;
    Ok(client)
}

fn apply_migrations(client: &mut Client) -> Result<(), PgError> {
    let migrations_path = Path::new("src/sql/migrations");

    if migrations_path.exists() {
        for entry in fs::read_dir(migrations_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.extension().unwrap_or_default() == "sql" {
                let sql = fs::read_to_string(&path).unwrap();
                client.batch_execute(&sql)?;
                info!("Migration executed: {:?}", path.file_name().unwrap());
            }
        }
    } else {
        info!("No migrations folder found");
    }
    Ok(())
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