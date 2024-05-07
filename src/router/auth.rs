use crate::db::connection::establish_connection;
use crate::model::auth::{User,LoginUser,NewUser};
// use crate::schema::users::dsl::*;
use axum::{http::Error, response::Json, response::Json as JsonResponse};
use diesel::prelude::*;
use serde_json::{json, value, Value};


pub async fn login(user: axum::Json<LoginUser>) -> JsonResponse<Value> {
    use crate::schema::users::dsl::{user_id, user_pw, users};
    let connection = &mut establish_connection();
    let results = users
        .filter(user_id.eq(&user.user_id))
        .load::<User>(connection)
        .expect("Error loading posts");
    if results.len() == 0 {
        
    }
    Json(json!({ "result": true }))
}



pub async fn sign_up(user: axum::Json<NewUser>) -> JsonResponse<Value> {
    use crate::schema::users;

    println!("{:?}",user);
    let user = NewUser{
        user_id:  user.user_id.to_string(),
        user_pw:  user.user_pw.to_string(),
        img:user.img.to_string(),
    };
    
    let connection = &mut establish_connection();
    diesel::insert_into(users::table)
    .values(&user)
    .returning(User::as_returning())
    .get_result(connection)
    .expect("Error saving new post");
    
    Json(json!({ "result": true }))
}


