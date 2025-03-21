mod db;
mod error;
mod handlers;
mod models;
mod services;

use axum::{routing, Router};
use db::create_pool;
use handlers::task_handler::{create_task, delete_task, get_task, update_task};
use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenv::dotenv().expect("Unable to access .env file");
    //set variables from enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    // create a connection pool
    let pool = create_pool().await.expect("Failed to create connection pool");

    // Configure CORS to allow all origins
    let cors = CorsLayer::new()
        .allow_origin(Any) // Allow all origins
        .allow_methods(Any) // Allow all HTTP methods
        .allow_headers(Any); // Allow all headers

    // Build application with routes and middleware
    let app = Router::new()
        .route("/", routing::get(|| async { "Hello world" }))
        .route("/tasks", routing::post(create_task))
        .route("/tasks/:id", routing::get(get_task))
        .route("/tasks/:id", routing::put(update_task))
        .route("/tasks/:id", routing::delete(delete_task))
        .with_state(pool)
        .layer(cors); // Add the CORS middleware

    // run app, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(server_address).await.expect("Could not create tcp listener");
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
