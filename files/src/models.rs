use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct DocumentIdentifier {
    pub document_id: String,
}

#[derive(Debug)]
#[derive(Deserialize,Serialize)]
pub struct Document{
    pub content: String,
    pub user: String,
    pub created_at: String,
    pub updated_at: String
}

#[derive(Deserialize,Serialize)]
pub struct UpdateDocument{
    pub content: String,
    pub user: String,
    pub updated_at: String
}

pub struct Config {
    pub host: String,
    pub port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_uri: String,
}