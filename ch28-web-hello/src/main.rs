use axum::{
    extract::Json,
    response::IntoResponse,
    routing::{get, post},
    Router,
};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CreateUser {
    name: String,
}

#[tokio::main]
async fn main() {
    println!("=== Web 开发初探 (Web Dev) Demo ===");
    println!("Starting server on http://localhost:3000");

    // 构建我们的应用路由
    let app = Router::new()
        .route("/", get(handler))
        .route("/users", post(create_user));

    // 运行应用
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, Rust Web!"
}

async fn create_user(Json(payload): Json<CreateUser>) -> impl IntoResponse {
    format!("Created user: {}", payload.name)
}
