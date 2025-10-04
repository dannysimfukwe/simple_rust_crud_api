use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Deserialize)]
pub struct CreateItem {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateItem {
    pub name: String,
    pub description: Option<String>,
}

#[derive(Serialize, FromRow)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
}
