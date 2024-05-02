use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Upload {
    pub image: Vec<u8>,
}
