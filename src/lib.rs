pub mod db;
pub mod model;
pub mod router;
use axum::{response::Html, routing::get, Router};
use tokio;

pub async fn run() {
    let app = Router::new().route("/", get(index));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn index() -> Html<&'static str> {
    let html_content = include_str!("../index.html");
    Html(html_content)
}
