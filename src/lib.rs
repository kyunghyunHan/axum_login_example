pub mod db;
pub mod model;
pub mod router;
pub mod schema;
use axum::{
    routing::{get, post},
    Router,
};

use router::{
    auth::login,
    pages::{error, index, success, upload_page,sign_up_page},
    upload::upload,
};
use tokio;

pub async fn run() {
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/success", get(success))
        .route("/error", get(error))
        .route("/uploadpage", get(upload_page))
        .route("/upload", post(upload))
        .route("/sign_up", get(sign_up_page));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
