use crate::serde_converters;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, sqlx::FromRow)]
pub struct TodoItem {
    #[serde(default)]
    pub id: i64,
    #[serde(deserialize_with = "serde_converters::html_encode")]
    pub title: String,
    #[serde(deserialize_with = "serde_converters::date_to_timestamp")]
    pub date: i64,
}

#[derive(Serialize)]
pub struct Info {
    pub name: String,
    pub age: u32,
}
