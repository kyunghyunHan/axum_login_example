use crate::model::upload::Upload;
use axum::extract::Multipart;
use axum::response::Json;
use serde_json::{json, Value};
use std::{fs::File, io::Write};

pub async fn upload(mut multipart: Multipart) -> Json<Value> {
    println!("{}", 1);
    println!("{:?}", multipart);
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let mut a = File::create("./img.jpg").unwrap();
        // println!("{:?}",&field.bytes().await.unwrap());

        a.write(&field.bytes().await.unwrap()).unwrap();

        // let name = field.name().unwrap().to_string();
        // let data = field.bytes().await.unwrap();
        // println!("Length of `{}` is {} bytes", name, data.len());
        // println!("{:?}",data);
    }
    Json(json!({ "result": true }))
}
