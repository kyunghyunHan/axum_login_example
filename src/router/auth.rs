use crate::db::connection::establish_connection;
use crate::model::auth::{DeleteUser, LoginUser, NewUser, User};
use crate::schema::users::user_id;
// use crate::schema::users::dsl::*;
use axum::{http::Error, response::Json, response::Json as JsonResponse};
use diesel::prelude::*;
use serde_json::{json, value, Value};
use crate::model::upload::Upload;
use axum::extract::Multipart;
use std::{fs::File, io::Write};
pub async fn login(user: axum::Json<LoginUser>) -> JsonResponse<Value> {
    use crate::schema::users::dsl::{user_id, user_pw, users};
    let connection = &mut establish_connection();
    let results = users
        .filter(user_id.eq(&user.user_id))
        .load::<User>(connection)
        .expect("Error loading posts");
    if results.len() == 0 {}
    Json(json!({ "result": true }))
}

pub async fn sign_up(user: axum::Json<NewUser>) -> JsonResponse<Value> {
    use crate::schema::users;

    println!("{:?}", user);
    let user = NewUser {
        user_id: user.user_id.to_string(),
        user_pw: user.user_pw.to_string(),
        img: user.img.to_string(),
    };

    let connection = &mut establish_connection();
    diesel::insert_into(users::table)
        .values(&user)
        .returning(User::as_returning())
        .get_result(connection)
        .expect("Error saving new post");

    Json(json!({ "result": true }))
}

pub async fn get_users() -> JsonResponse<Value> {
    use crate::schema::users::dsl::users; // user_id, user_pw 필요 없음
    let connection = &mut establish_connection();
    let results = users.load::<User>(connection).expect("Error loading users");

    // 사용자 목록을 JSON으로 변환하여 반환
    Json(json!({ "users": results }))
}
pub async fn secession(user: axum::Json<DeleteUser>) -> JsonResponse<Value> {
    use crate::schema::users::dsl::users; // user_id, user_pw 필요 없음
    let connection = &mut establish_connection();
    diesel::delete(users.filter(user_id.eq(&user.user_id)))
        .execute(connection)
        .expect("error");

    Json(json!({ "result": true }))
}




pub async fn update(mut multipart: Multipart) -> Json<Value> {
    println!("{}", 1);
    println!("{:?}", multipart);
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        println!("{:?}",field.name());
        let mut a = File::create("./img.jpg").unwrap();
         a.write(&field.bytes().await.unwrap()).unwrap();
         
        // let name = field.name().unwrap().to_string();
        // let data = field.bytes().await.unwrap();
        // println!("Length of `{}` is {} bytes", name, data.len());
        // println!("{:?}",data);
    }
    Json(json!({ "result": true }))
}
