pub mod db;
pub mod model;
pub mod router;
use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    response::Response,
    routing::{get, post},
    Router,
};
use opencv::core;
use opencv::prelude::MatTraitConst;
use opencv::prelude::MatTraitConstManual;
use router::{
    auth::login,
    pages::{error, index, success},
    upload::upload,
};
use tokio;
pub async fn run() {
    let app = Router::new()
        .route("/", get(index))
        .route("/login", post(login))
        .route("/success", get(success))
        .route("/error", get(error))
        .route("/upload", post(upload))
        .route("/ws", get(wsss));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}
async fn wsss(ws: WebSocketUpgrade) -> Response {
   
    ws.on_upgrade(handle_socket)
}
async fn handle_socket(mut socket: WebSocket) {
    while let Some(msg) = socket.recv().await {
        // println!("{:?}", socket);
 
        let msg = if let Ok(msg) = msg {
            msg
        } else {
            // client disconnected
            return;
        };
        if let Err(_) = socket.send(Message::Binary(vec![1u8])).await {
            println!("1");
            // client disconnected
            return;
        }
       
    }
}
