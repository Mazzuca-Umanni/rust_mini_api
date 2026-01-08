use axum::{Router, routing::{get,post, put, delete}, response::Json};

use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct Message { 
    status: String,
    message: String,
}

#[derive(Deserialize)]
struct NewUser { 
    name: String,
    age: u8,
}

async fn root() -> &'static str {
    "Hello, World!"
}

async fn post_user(Json(payload): Json<NewUser>) -> Json<Message> {
    let response = Message {
        status: "success".to_string(),
        message: format!("User {} of age {} created successfully", payload.name, payload.age),
    };
    Json(response)
}

async fn get_user() -> Json<Message> {
    let response = Message {
        status: "success".to_string(),
        message: "User data retrieved successfully".to_string(),
    };
    // -> item retornado automáticamente igual o ruby
    Json(response)
}

async fn put_user() -> Json<Message> {
    let response = Message {
        status: "success".to_string(),
        message: "User data updated successfully".to_string(),
    };
    Json(response)
}

async fn delete_user() -> Json<Message> {
    let response = Message {
        status: "success".to_string(),
        message: "User deleted successfully".to_string(),
    };
    Json(response)
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user))
    .route("/user", post(post_user))
    .route("/user", put(put_user))
    .route("/user", delete(delete_user));
    

    let addr = SocketAddr::from(([127, 0, 0, 1], 3001));
    println!("Listening on http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
