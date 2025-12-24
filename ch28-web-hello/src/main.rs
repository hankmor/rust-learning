use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    println!("=== Web 开发初探 (Web Dev) Demo ===");
    println!("Starting server on http://localhost:3000");

    // 构建我们的应用路由
    let app = Router::new().route("/", get(handler));

    // 运行应用
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> &'static str {
    "Hello, Rust Web!"
}
