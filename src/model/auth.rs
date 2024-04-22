use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct LoginUser {
    pub id: String,
    pub pw: String,
}
