use crate::model::auth::LoginUser;
use axum::response::Json;
use serde_json::{json, Value};

pub async fn login(Json(user): Json<LoginUser>) -> Json<Value> {
    println!("{}", user.id);
    println!("{}", user.pw);
    if user.id == "abc" && user.pw == "1234" {
        Json(json!({ "result": true }))
    } else {
        Json(json!({ "data": false }))
    }
}
