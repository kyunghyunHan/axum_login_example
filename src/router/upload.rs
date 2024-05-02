use crate::model::upload::Upload;
use axum::response::Json;
use serde_json::{json, Value};

pub async fn upload(Json(user): Json<Upload>) -> Json<Value> {
    println!("{}",1);
    println!("{:?}", user.image);

    Json(json!({ "result": true }))
}
