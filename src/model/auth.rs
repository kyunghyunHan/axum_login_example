use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use crate::schema::users;

#[derive(Queryable,  Debug, Deserialize, Serialize,Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub user_id: String,
    pub user_pw: String,
    pub img: String,
}
#[derive(Debug,Queryable, Deserialize)]
pub struct LoginUser {
    pub user_id: String,
    pub user_pw: String,
}
#[derive(Insertable,Queryable,Selectable,  Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct NewUser{
    pub user_id: String,
    pub user_pw:String,
    pub img: String,
}
#[derive(Insertable,Queryable,Selectable,  Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct DeleteUser{
    pub user_id: String,
}
#[derive(Insertable,Queryable,Selectable,  Debug, Deserialize, Serialize)]
#[diesel(table_name = users)]
pub struct UserID{
    pub id: i32,
}