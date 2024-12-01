use actix_web::{web, HttpResponse};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use sled::Db;

// Todo item structure
#[derive(Serialize, Deserialize, Debug)]
struct Todo {
    id: String,
    text: String,
    completed: bool,
}

// Add a new to-do item
pub async fn add_todo(db: web::Data<Db>, item: web::Json<Todo>) -> HttpResponse {
    let todo = item.into_inner();
    let key = todo.id.clone();
    let value = serde_json::to_string(&todo).unwrap();

    // Convert String to bytes for sled
    match db.insert(key.as_bytes(), value.as_bytes()) {
        Ok(_) => HttpResponse::Ok().json("Todo added"),
        Err(_) => HttpResponse::InternalServerError().json("Failed to add todo"),
    }
}

// Get all to-do items
pub async fn get_todos(db: web::Data<Db>) -> HttpResponse {
    let todos: Result<Vec<Todo>> = db
        .iter()
        .values()
        .map(|v| {
            let bytes = v?.to_vec();
            Ok(serde_json::from_slice::<Todo>(&bytes)?)
        })
        .collect();

    match todos {
        Ok(todos) => HttpResponse::Ok().json(todos),
        Err(_) => HttpResponse::InternalServerError().json("Failed to fetch todos"),
    }
}
