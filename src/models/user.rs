use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    #[serde(default)]
    pub id: i64,
    pub email: String,
    pub password: String,
    #[serde(default)]
    pub created_at: i64,
}
