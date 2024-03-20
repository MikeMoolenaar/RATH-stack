use crate::serde_converters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct TodoItemRequest {
    #[serde(default)]
    pub id: i64,
    pub title: String,
    #[serde(deserialize_with = "serde_converters::date_to_timestamp")]
    pub date: i64,
    #[serde(default)]
    pub user_id: i64,
}

#[derive(Deserialize, Serialize)]
pub struct TodoItem {
    pub id: i64,
    pub title: String,
    pub date: i64,
    pub user_id: i64,
}
