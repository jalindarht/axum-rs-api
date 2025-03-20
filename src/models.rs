use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub description: String,
    pub status: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateTaskPayload {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    pub description: String,
}


#[derive(Debug, Deserialize, Validate)]
pub struct UpdateTaskPayload {
    #[validate(length(min = 1, max = 255, message = "Title must be between 1 and 255 characters"))]
    pub title: String,
    pub description: String,
    #[validate(length(min = 1, max = 50, message = "Status must be between 1 and 255 characters"))]
    pub status: String,
}