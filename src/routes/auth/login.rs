use crate::{models::user::User, render_html::*, turso_helper::fetch_optional, AppState};
use argon2::{password_hash::PasswordHash, Argon2, PasswordVerifier};
use axum::{extract::State, http::StatusCode, response::Html, Form};
use axum_htmx::{HxBoosted, HxLocation};
use libsql::params;
use minijinja::context;
use serde::Deserialize;
use std::{collections::HashMap, sync::Arc};
use tower_sessions::Session;

pub async fn login_get(State(_state): State<Arc<AppState>>, HxBoosted(boosted): HxBoosted) -> Html<String> {
    return render_html("login.html", context!(), boosted).unwrap();
}

#[derive(Deserialize)]
pub struct LoginForm {
    email: String,
    password: String,
}

pub async fn login_post(
    session: Session,
    State(state): State<Arc<AppState>>,
    Form(form): Form<LoginForm>,
) -> (StatusCode, Option<HxLocation>, Html<String>) {
    let mut errors = HashMap::new();

    let user = fetch_optional::<User>(
        &state.db_conn,
        "SELECT id, email, password, created_at FROM users WHERE email = ?",
        params![form.email.clone()],
    )
    .await
    .unwrap();

    if user.is_none() {
        // Prevent unknown email from returning faster than providing a known email.
        // This way, it cannot be known if the email exists.
        let password_hash = PasswordHash::new("something").unwrap();
        let _ = Argon2::default().verify_password(b"anything", &password_hash).is_ok();

        errors.insert("general", "Invalid email or password");
        return (
            StatusCode::UNAUTHORIZED,
            None,
            render_block("login.html", "errors", context! { errors }).unwrap(),
        );
    }

    let user = user.unwrap();
    let parsed_hash = PasswordHash::new(&user.password).unwrap();

    let password_matches = Argon2::default()
        .verify_password(form.password.as_bytes(), &parsed_hash)
        .is_ok();

    if !password_matches {
        errors.insert("general", "Invalid email or password");
        return (
            StatusCode::UNAUTHORIZED,
            None,
            render_block("login.html", "errors", context! { errors }).unwrap(),
        );
    }

    session.insert("user", user).await.unwrap();
    return (
        StatusCode::OK,
        Some(HxLocation::from_str("/").unwrap()),
        Html(String::from("")),
    );
}
