pub mod db;
pub mod model;
pub mod router;
pub mod schema;
use axum::{
    routing::{get, post},
    Router,
};

use router::{
    auth::{get_user, get_users, login, secession, sign_up, update},
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
        .route("/get_user", post(get_user))
        .route("/secession", post(secession))
        .nest("/img", axum_static::static_router("img"));


    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
