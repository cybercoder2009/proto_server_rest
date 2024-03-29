use serde::Deserialize;

#[derive(Debug,Deserialize)]
pub struct Config {
    
    pub server_rest: String,
    pub mongo_binding: String,
    pub mongo_db: String,

    pub title: String,
    pub keywords: String,
    pub description: String,
}