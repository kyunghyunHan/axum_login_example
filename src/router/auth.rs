use crate::db::connection::establish_connection;
use crate::model::auth::{User,LoginUser};
// use crate::schema::users::dsl::*;
use axum::{http::Error, response::Json, response::Json as JsonResponse};
use diesel::prelude::*;
use serde_json::{json, value, Value};
pub async fn login(user: axum::Json<LoginUser>) -> JsonResponse<Value> {
    use crate::schema::users::dsl::{user_id, user_pw, users};
    println!("1");
    let connection = &mut establish_connection();
    let results = users
        .filter(user_id.eq(&user.user_id))
        .load::<User>(connection)
        .expect("Error loading posts");
    if results.len() == 0 {
        
    }
    Json(json!({ "result": true }))
}
