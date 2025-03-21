use crate::db::{get_conn, ConnectionPool};
use crate::error::AppError;
use crate::models::task_model::{CreateTaskPayload, Task, UpdateTaskPayload};
pub struct TaskService;

impl TaskService {
    pub async fn create_task(pool: &ConnectionPool, payload: CreateTaskPayload) -> Result<Task, AppError> {
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

        Ok(task)
    }

    pub async fn get_task(pool: &ConnectionPool, task_id: i64) -> Result<Task, AppError> {
        // Get the connection from connection pool
        let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
        let row = conn
            .query_one("SELECT * FROM tasks WHERE id = $1", &[&task_id])
            .await
            .map_err(|_| AppError::TaskNotFound)?;

        let task = Task {
            id: row.get("id"),
            title: row.get("title"),
            description: row.get("description"),
            status: row.get("status"),
        };

        Ok(task)
    }

    pub async fn update_task(pool: &ConnectionPool, task_id: i64, payload: UpdateTaskPayload) -> Result<Task, AppError> {
        // Get the connection from connection pool
        let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
        // Get the task from the database to check if it exists
        conn
            .query_one("SELECT * FROM tasks WHERE id = $1", &[&task_id])
            .await
            .map_err(|_| AppError::TaskNotFound)?;
        // Execute the query to update task
        conn.execute(
            "UPDATE tasks SET title = $1, description = $2, status = $3 WHERE id = $4",
            &[&payload.title, &payload.description, &payload.status, &task_id],
        )
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let task = Task {
            id: task_id,
            title: payload.title,
            description: payload.description,
            status: payload.status,
        };

        Ok(task)
    }

    pub async fn delete_task(pool: &ConnectionPool, task_id: i64) -> Result<(), AppError> {
        // Get the connection from connection pool
        let conn = get_conn(&pool).await.map_err(|e| AppError::DatabaseConnectionError(e.to_string()))?; // Map the error to AppError;
        // Execute the query to delete task
        conn.execute("DELETE FROM tasks WHERE id = $1", &[&task_id])
            .await
            .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}