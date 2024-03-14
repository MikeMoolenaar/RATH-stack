use crate::{models::user::User, turso_helper::fetch_optional, AppState};
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
    // TODO: fetch is a little overkill, just use a count or something
    let email_exists = fetch_optional::<User>(
        &state.db_conn,
        "SELECT email FROM users WHERE email = $1",
        params![query.email.clone()],
    )
    .await
    .unwrap();

    if email_exists.is_some() {
        return (
            StatusCode::BAD_REQUEST,
            Html(String::from("<p class=\"text-error\">Email already exists</p>")),
        );
    }
    return (StatusCode::OK, Html(String::from("<p class=\"text-error\"></p>")));
}
