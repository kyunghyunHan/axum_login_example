use crate::db::connection::establish_connection;
use crate::model::auth::{DeleteUser, LoginUser, NewUser, User, UserID};
use crate::schema::users::user_id;
use crate::model::upload::Upload;
use axum::extract::Multipart;
use axum::{http::Error, response::Json, response::Json as JsonResponse};
use diesel::prelude::*;
use serde_json::{json, value, Value};
use std::{fs::File, io::Write};
use uuid::Uuid;
pub async fn login(user: axum::Json<LoginUser>) -> JsonResponse<Value> {
    use crate::schema::users::dsl::{user_id, user_pw, users};
    let connection = &mut establish_connection();
    let results = users
        .filter(user_id.eq(&user.user_id))
        .first::<User>(connection)
        .expect("Error loading posts");

    Json(json!({ "result": true,"id":results.id}))
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

pub async fn get_user(user: axum::Json<UserID>) -> JsonResponse<Value> {
    use crate::schema::users::dsl::{users,id}; // user_id, user_pw 필요 없음
    let connection = &mut establish_connection();
    let results = users
    .filter(id.eq(&user.id))
    .first::<User>(connection)
    .expect("Error loading posts");
    println!("{:?}",results);
    Json(json!({ "id": results.user_id,"pw":results.user_pw,"img":results.img}))
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
    let connection = &mut establish_connection();
    use crate::schema::users::dsl::{users,img};
    let filename = format!("./img/{}.jpg", Uuid::new_v4());

    while let Some(mut field) = multipart.next_field().await.unwrap() {
        

        if field.name().unwrap() == "id" {
            let id = field.text().await.unwrap().parse::<i32>().unwrap();
            diesel::update(users.find(id))
            .set(img.eq(&filename))
            .returning(User::as_returning())
            .get_result(connection)
            .unwrap();
        } else if field.name().unwrap() == "image" {
            let mut fs = File::create(&filename).unwrap();
            fs.write(&field.bytes().await.unwrap()).unwrap();
        }
    }
    Json(json!({ "result": true }))
}
