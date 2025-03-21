use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::db::ConnectionPool;
use crate::error::AppError;
use crate::models::task_model::{CreateTaskPayload, Task, UpdateTaskPayload};
use crate::services::task_service::TaskService;

// Handler to create a task
pub async fn create_task(
    State(pool): State<ConnectionPool>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<Task>, AppError> {
    // Validate the payload
    payload.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    
    let task = TaskService::create_task(&pool, payload).await?;
    Ok(Json(task))
}

// Handler to get a task
pub async fn get_task(
    Path(id): Path<i64>,
    State(pool): State<ConnectionPool>,
) -> Result<Json<Task>, AppError> {
    
    // Use TaskService to create the task
    let task = TaskService::get_task(&pool, id).await?;
    Ok(Json(task))
}

// Handler to update a task
pub async fn update_task(
    Path(id): Path<i64>,
    State(pool): State<ConnectionPool>,
    Json(payload): Json<UpdateTaskPayload>,
) -> Result<Json<Task>, AppError> {
    // Validate the payload
    payload.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    
    // Use TaskService to update the task
    let task = TaskService::update_task(&pool, id, payload).await?;

    Ok(Json(task))
}

// Handler to delete a task
pub async fn delete_task(
    Path(id): Path<i64>,
    State(pool): State<ConnectionPool>,
) -> Result<(), AppError> {
    
    // Use TaskService to update the task
    TaskService::delete_task(&pool, id).await?;

    Ok(())
}