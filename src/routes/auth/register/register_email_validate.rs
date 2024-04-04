use crate::{turso_helper::count, AppState};
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use libsql::params;
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
    let count = count(
        &state.db_conn,
        "SELECT count(*) FROM users WHERE email = $1",
        params![query.email.clone()],
    )
    .await
    .unwrap();

    if count > 0 {
        return (
            StatusCode::BAD_REQUEST,
            Html(String::from("<p class=\"text-error\">Email already exists</p>")),
        );
    }
    return (StatusCode::OK, Html(String::from("<p class=\"text-error\"></p>")));
}
