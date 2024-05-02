use crate::model::upload::Upload;
use axum::response::Json;
use axum::extract::Multipart;
use serde_json::{json, Value};

pub async fn upload(mut multipart: Multipart) -> Json<Value> {
    println!("{}",1);
    println!("{:?}", multipart);
    while let Some(mut field) = multipart.next_field().await.unwrap() {
        let name = field.name().unwrap().to_string();
        let data = field.bytes().await.unwrap();
        println!("Length of `{}` is {} bytes", name, data.len());
        println!("{:?}",data);
    }
    Json(json!({ "result": true }))
}
