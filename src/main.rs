use axum::{
    routing::{delete, get, post},
    Router,
};

use dotenv::dotenv;
use mongodb::{Client, Collection};
use std::env;

mod handler;
mod task;

use task::Task;

#[tokio::main]
async fn main() {
    dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGO_URI is not set");
    let client = Client::with_uri_str(uri).await.expect("invalid uri");
    let database = client.database("todolist");
    let collection: Collection<Task> = database.collection("task");

    let app = Router::new()
        .route("/", get(root))
        .route("/task", post(handler::create_task_handler))
        .route("/tasks", get(handler::get_all_task_handler))
        .route("/task/{:id}", delete(handler::delete_task_handler))
        .with_state(collection);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "Hello, this is task api!"
}
