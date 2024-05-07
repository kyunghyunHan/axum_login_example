use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable,Insertable, Debug,Deserialize,Serialize)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id : i32,
    pub user_id: String,
    pub user_pw: String,
    pub img: String,
}
#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub user_id: String,
    pub user_pw: String,
  
}