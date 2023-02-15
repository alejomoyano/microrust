use serde::{Deserialize, Serialize};
pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_uri: String,
}

#[derive(Deserialize,Serialize)]
pub struct User{
    pub name: String,
    pub password: String
}

#[derive(Deserialize)]
pub struct UserIdentifier {
    pub user_id: String,
}