use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct METADATA{
    pub id: i64,
    pub user_name: String,
    pub name: String,
    pub size: f64,
    pub path: String,
    pub ip: String,
    pub port: String,
}