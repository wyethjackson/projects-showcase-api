use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use crate::db::{connect_db, load_query};
use rocket::tokio::task;

#[derive(Serialize)]
pub struct Project {
    name: String,
    description: String,
    url: String
}

#[get("/")]
pub async fn index() -> Result<Json<Vec<Project>>, Status> {
    let result = task::spawn_blocking(|| {
        let mut client = connect_db().map_err(|e| {
            error!("Database connection error: {}", e);
            Status::InternalServerError
        })?;

        let query = match load_query("./sql/select-all-projects.sql") {
            Ok(q) => q,
            Err(e) => {
                error!("Failed to load SQL query: {}", e);
                return Err(Status::InternalServerError);
            }
        };
        match client.query(&query, &[]) {
            Ok(rows) => {
                if rows.is_empty() {
                    info!("No projects found in database.");
                    Ok(vec![])
                } else {
                    let projects: Vec<Project> = rows.iter().map(|row| Project {
                        name: row.get("name"),
                        description: row.get("description"),
                        url: row.get("url"),
                    }).collect();
                    Ok(projects)
                }
            }
            Err(e) => {
                error!("Database query failed: {}", e);
                Err(Status::InternalServerError)
            }
        }
    }).await;

    match result {
        Ok(Ok(projects)) => Ok(Json(projects)),
        Ok(Err(status)) => Err(status),
        Err(e) => {
            error!("Thread panic: {}", e);
            Err(Status::InternalServerError)
        }
    }
}