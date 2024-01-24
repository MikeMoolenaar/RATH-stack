use crate::AppState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct EmailForm {
    email: String,
}
pub async fn register_email_validate(
    State(state): State<Arc<AppState>>,
    query: Query<EmailForm>,
) -> (StatusCode, Html<String>) {
    let email_exists = sqlx::query!("SELECT email FROM users WHERE email = ?", query.email)
        .fetch_optional(&state.db)
        .await
        .unwrap();
    if email_exists.is_some() {
        return (
            StatusCode::OK,
            Html(String::from("<p class=\"text-error\">Email already exists</p>")),
        );
    }
    return (StatusCode::OK, Html(String::from("<p class=\"text-error\"></p>")));
}
