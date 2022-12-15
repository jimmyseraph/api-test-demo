extern crate redis;
use serde_derive::{Deserialize, Serialize};
use redis::Commands;

#[derive(Deserialize, Serialize, Clone)]
pub struct UserInfo {
    #[serde(rename = "accountId")]
    pub account_id: u32,
    #[serde(rename = "accountName")]
    pub account_name: String,
    pub cellphone: String,
    pub gender: u8,
}

impl UserInfo {
    pub fn new(account_id: u32, account_name: String, cellphone: String, gender: u8) -> Self {
        Self {
            account_id,
            account_name,
            cellphone,
            gender,
        }
    }
    
    pub fn prepare_login_user(self, url: &str, token: &str) -> Result<(), Box<dyn std::error::Error>>{
        let client = redis::Client::open(url)?;
        let mut con = client.get_connection()?;
        let value = serde_json::to_string(&self)?;
        let _ : () = con.set(token, value)?;
        Ok(())
    }
}

