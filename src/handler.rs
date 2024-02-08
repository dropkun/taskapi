use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use mongodb::Collection;
use serde_json;
use std::str::FromStr;

mod error;
use crate::task::{create_task, delete_task, get_all_task, get_task, Task};

pub async fn create_task_handler(
    State(collection): State<Collection<Task>>,
    Json(payload): Json<Task>,
) -> Result<(StatusCode, String), error::AppError> {
    create_task(&collection, payload).await?;
    Ok((StatusCode::CREATED, "Task created".to_string()))
}
pub async fn delete_task_handler(
    State(collection): State<Collection<Task>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, String), error::AppError> {
    let oid = mongodb::bson::oid::ObjectId::from_str(&id).unwrap();
    delete_task(&collection, oid).await?;
    Ok((StatusCode::OK, "Task deleted".to_string()))
}

pub async fn get_task_handler(
    State(collection): State<Collection<Task>>,
    Path(id): Path<String>,
) -> Result<(StatusCode, String), error::AppError> {
    let oid = mongodb::bson::oid::ObjectId::from_str(&id).unwrap();
    let task = get_task(&collection, oid).await?;
    let json = serde_json::to_string(&task).unwrap();
    Ok((StatusCode::OK, json))
}
pub async fn get_all_task_handler(
    State(collection): State<Collection<Task>>,
) -> Result<(StatusCode, String), error::AppError> {
    let tasks = get_all_task(&collection).await?;
    let json = serde_json::to_string(&tasks).unwrap();
    Ok((StatusCode::OK, json))
}
