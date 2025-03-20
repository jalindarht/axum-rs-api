use axum::{
    extract::{Path, State},
    Json,
};
use validator::Validate;

use crate::db::{get_conn, ConnectionPool};
use crate::error::AppError;
use crate::models::{CreateTaskPayload, Task, UpdateTaskPayload};

// Handler to create a task
pub async fn create_task(
    State(pool): State<ConnectionPool>,
    Json(payload): Json<CreateTaskPayload>,
) -> Result<Json<Task>, AppError> {
    // Validate the payload
    payload.validate().map_err(|e| AppError::BadRequest(e.to_string()))?;
    // Get the connection from connection pool
    let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
    
    // Execute the query to insert task
    conn.execute(
        "INSERT INTO tasks (title, description, status) VALUES ($1, $2, $3)",
        &[&payload.title, &payload.description, &"pending".to_string()],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    //Redshift does not support RETURNING, so need to use a SELECT query to fetch the inserted record
    let row = conn
        .query_one("SELECT * FROM tasks WHERE id = (SELECT MAX(id) FROM tasks)", &[])
        .await
        .map_err(|_| AppError::TaskNotFound)?;

    // Construct the Task struct from the returned row
    let task = Task {
        id: row.get("id"),
        title: row.get("title"),
        description: row.get("description"),
        status: row.get("status"),
    };

    Ok(Json(task))
}

// Handler to get a task
pub async fn get_task(
    Path(id): Path<i64>,
    State(pool): State<ConnectionPool>,
) -> Result<Json<Task>, AppError> {
    // Get the connection from connection pool
    let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
    let row = conn
        .query_one("SELECT * FROM tasks WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::TaskNotFound)?;

    let task = Task {
        id: row.get("id"),
        title: row.get("title"),
        description: row.get("description"),
        status: row.get("status"),
    };

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
    // Get the connection from connection pool
    let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
    // Get the task from the database to check if it exists
    conn
        .query_one("SELECT * FROM tasks WHERE id = $1", &[&id])
        .await
        .map_err(|_| AppError::TaskNotFound)?;
    // Execute the query to update task
    conn.execute(
        "UPDATE tasks SET title = $1, description = $2, status = $3 WHERE id = $4",
        &[&payload.title, &payload.description, &payload.status, &id],
    )
    .await
    .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    let task = Task {
        id,
        title: payload.title,
        description: payload.description,
        status: payload.status,
    };

    Ok(Json(task))
}

// Handler to delete a task
pub async fn delete_task(
    Path(id): Path<i64>,
    State(pool): State<ConnectionPool>,
) -> Result<(), AppError> {
    // Get the connection from connection pool
    let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
    // Execute the query to delete task
    conn.execute("DELETE FROM tasks WHERE id = $1", &[&id])
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

    Ok(())
}