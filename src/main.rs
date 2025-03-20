mod db;
mod error;
mod handlers;
mod models;

use axum::{routing, Router};
use db::create_pool;
use handlers::{create_task, delete_task, get_task, update_task};

#[tokio::main]
async fn main() {
    //expose environment variables from .env file
    dotenv::dotenv().expect("Unable to access .env file");
    //set variables from enviroment variables
    let server_address = std::env::var("SERVER_ADDRESS").unwrap_or("127.0.0.1:3000".to_owned());

    // create a connection pool
    let pool = create_pool().await.expect("Failed to create connection pool");

    // build application with routes
    let app = Router::new()
        .route("/", routing::get(|| async { "Hello world" }))
        .route("/tasks", routing::post(create_task))
        .route("/tasks/:id", routing::get(get_task))
        .route("/tasks/:id", routing::put(update_task))
        .route("/tasks/:id", routing::delete(delete_task))
        .with_state(pool);

    // run app, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind(server_address).await.expect("Could not create tcp listener");
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
