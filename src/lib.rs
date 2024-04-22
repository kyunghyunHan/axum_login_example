pub mod db;
pub mod model;
pub mod router;
use axum::{
    routing::{get, post},
    Router,
};

use router::{
    auth::login,
    pages::{error, index, success},
};
use tokio;
pub async fn run() {
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/success", get(success))
        .route("/error", get(error));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
