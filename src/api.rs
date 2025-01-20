use rocket::serde::{Serialize, json::Json};
use rocket::http::Status;
use crate::db::connect_db;
use rocket::tokio::task;

#[derive(Serialize)]
pub struct Project {
    name: String,
    description: String,
    url: String
}

#[get("/")]
pub async fn index() -> Result<Json<Project>, Status> {
    let result = task::spawn_blocking(|| {
        let mut client = connect_db().map_err(|e| {
            error!("Database connection error: {}", e);
            Status::InternalServerError
        })?;

        let query = "SELECT name, description, url FROM projects LIMIT 1";

        client.query_one(query, &[]).map(|row| Project {
            name: row.get("name"),
            description: row.get("description"),
            url: row.get("url"),
        }).map_err(|e| {
            error!("Database query failed: {}", e);
            Status::InternalServerError
        })
    }).await;

    match result {
        Ok(Ok(project)) => Ok(Json(project)),
        Ok(Err(status)) => Err(status), 
        Err(e) => {
            error!("Thread panic: {}", e);
            Err(Status::InternalServerError)
        }
    }
}