pub mod db;
pub mod model;
pub mod router;
pub mod schema;
use axum::{
    routing::{get, post},
    Router,
};

use router::{
    auth::{get_users, login, sign_up,secession,update},
    pages::{admin, error, index, sign},

};
use tokio;

pub async fn run() {
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/error", get(error))
        .route("/upload", post(update))
        .route("/sign", get(sign))
        .route("/signup", post(sign_up))
        .route("/admin", get(admin))
        .route("/get_users", post(get_users))
        .route("/secession", post(secession));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
